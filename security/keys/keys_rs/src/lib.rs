// for "language items" like eh_personality and panic_fmt
#![feature(lang_items)]

// This is so the bindings can generate unions with "non-Copyable" structs in them
#![feature(untagged_unions)]

// coz we don't have an operating system
#![no_std]
#![feature(core)]

// so we can use unions with non-Copyable structs. Thought it should be in os.rs....
#![feature(untagged_unions)]

// so we can define a custom allocator (wrapper for Linux allocator)
#![feature(alloc)]
#![feature(global_allocator)]
#![feature(allocator_api)]

// so we can avoid linker errors on intrinsics,
// I'm not sure why I need intrinsics but I don't want the errors....
#![feature(compiler_builtins_lib)]

//#[macro_use]
extern crate alloc;

// to prevent vmlinux linker errors on intrinsics like __udivti3
extern crate compiler_builtins;

mod c_types;
mod os;
mod linux_allocator;
#[macro_use]
mod io;
mod user_ptr;

use linux_allocator::LinuxAllocator;
#[global_allocator]
static MY_ALLOCATOR: LinuxAllocator = LinuxAllocator {};


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


#[no_mangle]
pub extern fn rust_hello() {
	println!("Hello from Rust!!!!");
}


// TODO actual types?
#[no_mangle]
pub extern fn rust_add_key(key_type : *mut u8, description : *mut u8, payload : *mut u8, plen : isize, ringid: i32) {
	// search for keyring
	println!("Hello from Rust!!!!");
	// create or update key
}




#[lang = "eh_personality"] extern fn eh_personality() {}
// #[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
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

// using this to avoid vmlinux linker error.... Not supported in
// compiler_builtins just yet, and why do I need floats here???
// I thought +soft-float would take care of this........
#[no_mangle]
pub extern "C" fn __floatundisf() {}