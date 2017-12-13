// for "language items" like eh_personality and panic_fmt
#![feature(lang_items)]

// coz we don't have an operating system
#![no_std]
#![feature(no_std, core)]

// so we can use unions with non-Copyable structs. Thought it should be in os.rs....
#![feature(untagged_unions)]

// so we can define a custom allocator (wrapper for Linux allocator)
#![feature(alloc)]

//#[macro_use]
extern crate alloc;

mod c_types;
mod os;


struct KeyLengths {
	quotalen : u16,
	datalen : u16,
}
struct KeyPerms {
	uid : os::kuid_t,
	gid : os::kgid_t,
	perm : u32,
}
// key_user? security data? key_type? description?
// key ring bits (payload, keyring payload)?
enum Key {
	Uninstantiated(), // expiry time?
	Instantiated(Payload, KeyLengths, KeyPerms, u64), // last is expiry time
	Negative(), // expiry time?
	Expired(u64), // expired time
	Revoked(u64), // revoked time
	Dead()
}

enum Payload {
	Keyring(alloc::LinkedList<Key>), // TODO BTreeMap between Description and Key
	Data(),
	RejectError(isize),
}

enum UserPtr {
	U8Ptr(*mut u8),
	U16Ptr(*mut u16),
	U32Ptr(*mut u32),
	U64Ptr(*mut u64),
	StrPtr(*mut u8),
	VarLenPtr(*mut u8, isize),
	AlreadyReadFrom(*mut u8),
	AlreadyReadFromVarLen(*mut u8, isize),
}

// TODO actual types?
#[no_mangle]
pub extern fn add_key(key_type : *mut u8, description : *mut u8, payload : *mut u8, plen : isize, ringid: i32) {
	// search for keyring

	// create or update key
}




#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

// #[lang = "panic_fmt"]
// extern fn panic_impl(args: core::fmt::Arguments, file : &'static str, line : u32) -> ! {
//     use ::core::fmt::Write;
//     use ::std::io::KernelDebugWriter;
//     let mut writer = KernelDebugWriter {};
//     print!("Panicked at '");
//     // If this fails to write, just leave the quotes empty.
//     let _ = writer.write_fmt(args);
//     println!("', {}:{}", file, line);
//     // Force a null pointer read to crash.
//     unsafe{ let _ = *(core::ptr::null::<i32>()); }
//     // If that doesn't work, loop forever.
//     loop{}
// }