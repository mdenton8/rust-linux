// enum UserPtr {
// 	U8Ptr(*mut u8),
// 	U16Ptr(*mut u16),
// 	U32Ptr(*mut u32),
// 	U64Ptr(*mut u64),
// 	StrPtr(*mut u8),
// 	VarLenPtr(*mut u8, isize),
// 	AlreadyReadFrom(*mut u8),
// 	AlreadyReadFromVarLen(*mut u8, isize),
// }

// struct UserU8Ptr {
// 	ptr : *mut u8,
// 	read_once : bool
// }

use core::mem::size_of;

use c_types;
use os;


/*
 * Target API:
 * 1. Callers should create this structure or enum out of user pointers as soon
 *    as they are received.
 * 2. Callers should be able to create 3 types:
 *    - one for any primitive types which uses get_user/put_user.
 *      This takes a pointer.
 *    - one for strings which uses strncpy_from_user, TODO is there a strncpy_to_user?
 *      This takes a pointer.
 *    - one for variable length data, like buffers or key payloads.
 *		This takes a pointer and a length. (should length be passed to read fn?)
 * 3. Each of these types will have a read function that takes ownership: (ownership of self)
 *	  - the first will return the primitive type
 *	  - the second will return a String
 *    - the last will return a slice? (or something) For binary blobs.
 * 4. The read function will also return a writeable pointer (no longer readable):
 *	  - this is to avoid TOCTOU and double reads.
 * 5. Both readable (original) and writeable (returned by read) pointers
 *    will have a write function so you can write. (TODO not much point in
 *    reading after writing, perhaps there should be a warning for this case?)
 *
 *
 * target macros: (with ptr : mut *u8, u8 a type..., n: isize) TODO how to deal with negative isize...
 * let char_ptr = create_user_primitive_ptr[u8, ptr]
 * let str_ptr = create_user_string_ptr[ptr]
 * let blob_ptr = create_user_varlen_ptr[ptr, n]
 * 
 * let (x, writeable_ptr_x) = char_ptr.read()
 * let (str, writeable_ptr_str) = str_ptr.read(n) // TODO should string return writable ptr?
 * let (blob, writeable_ptr_blob) = blob_ptr.read(n) // TODO will need to check rust syntax for returning blobs.
 *
 * writeable_ptr_x.write(5) // TODO can I use DerefMut here for *writeable_ptr_x = 5;
 */

// TODO currently not returning any error values OR number of bytes written.....
macro_rules! create_user_rw_ptr {
    ($t:ty, $struct_name:ident, $writeable_struct_name:ident) => {
    	pub struct $struct_name(*mut $t);
    	pub struct $writeable_struct_name(*mut $t);
    	impl $struct_name {
			// fn new(x : *mut $t) -> $struct_name {
			// 	$struct_name(x)
			// }
			fn read(self) -> ($t, $writeable_struct_name) {
				// TODO use get_user/put_user... Difficult because they are macros.
				// TODO do I really need size_of? Will it be evaluated at compile_time?
				// TODO pretty sure _copy_from_user is fine...
				// Doesn't have compile time object size checking, but that should be fine
				// or runtime heap-size checking (does that exist here? HARDENED_USER_COPY?)
				let mut ret : $t = 0;
				unsafe { os::_copy_from_user(&mut ret as *mut $t as *mut c_types::c_void,
					self.0 as *mut c_types::c_void, size_of::<$t>()); }
				return (ret, $writeable_struct_name(self.0))
			}
			fn write(&self, x : $t) {
				// TODO have to take x as mut because we need a mut reference, and
				// the C implementations have different signatures from the header
				// files! they have the user annotation on the wrong one lol
				unsafe { os::_copy_to_user(self.0 as  *mut c_types::c_void,
					&x as *const $t as *const c_types::c_void, size_of::<$t>()); }
			}
		}
    	impl $writeable_struct_name {
			fn write(&self, mut x : $t) {
				unsafe { os::_copy_to_user(self.0 as  *mut c_types::c_void,
					&x as *const $t as *const c_types::c_void, size_of::<$t>()); }
			}
		}
    }
}

#![allow(dead_code)]
create_user_rw_ptr!(u8, UserRWPtrU8, UserWPtrU8);
create_user_rw_ptr!(u16, UserRWPtrU16, UserWPtrU16);
create_user_rw_ptr!(u32, UserRWPtrU32, UserWPtrU32);
create_user_rw_ptr!(u64, UserRWPtrU64, UserWPtrU64);
create_user_rw_ptr!(i8, UserRWPtrI8, UserWPtrI8);
create_user_rw_ptr!(i16, UserRWPtrI16, UserWPtrI16);
create_user_rw_ptr!(i32, UserRWPtrI32, UserWPtrI32);
create_user_rw_ptr!(i64, UserRWPtrI64, UserWPtrI64);


// now for string types....



// ________________________________________________________________________________
// can't seem to do it with generics because Rust wants the type initialized...
// but I can't do that!
// ...because I can't create something of any arbitrary type. There's no builtin
// trait for requiring primitives types, and any available crates use std.
// so I am going with a macro.....

// struct UserRWPtr<T>(*mut T);
// struct UserWPtr<T>(*mut T);

// struct UserStrPtr(*mut c_types::c_char);

// struct UserVarlenPtr(*mut u8, isize);


// impl<T> UserRWPtr<T> {
// 	fn new(x : *mut T) -> UserRWPtr<T> {
// 		UserRWPtr(x)
// 	}
// 	fn read(&mut self) -> T {
// 		// TODO pretty sure _copy_from_user is fine...
// 		// Doesn't have compile time object size checking, but that should be fine
// 		// or runtime heap-size checking (does that exist here? HARDENED_USER_COPY?)
// 		let mut ret : T;
// 		os::_copy_from_user(&mut ret as *mut T as *mut c_types::c_void, self.0 as *mut c_types::c_void, size_of::<T>());
// 		return ret
// 	}
// 	fn write(&self) {

// 	}
// }
//_____________________________________________________________________________

// implementing deref would be cool but you can't change the variant
// and DerefMut doesn't seem to work.
// And I want to encode read_once in the type system, not
// in a runtime-checked boolean.
// impl Deref for UserPtr::U8Ptr {
// 	fn deref(&self) {
// 		if(read_once)
// 	}
// }