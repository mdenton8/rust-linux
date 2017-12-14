extern crate bindgen;
extern crate shlex;

use std::env;
use std::path::PathBuf;

// TODO us --rust-target nightly
// TODO use clang arg blacklist?

// https://github.com/tsgates/rust.ko/blob/master/std/build.rs
/// List of parameters not ever to pass to the clang parser of rust-bindgen
const CLANG_ARGS_BLACKLIST: [&'static str; 10] = [
    "-mno-80387", "-mno-fp-ret-in-387", "-mskip-rax-setup", "-maccumulate-outgoing-args",
    "-mpreferred-stack-boundary=3", "-mfentry",
    "-fno-var-tracking-assignments", "-fconserve-stack", "-DCC_HAVE_ASM_GOTO",
    "-fno-delete-null-pointer-checks"
];

fn main() {
    // tell cargo to only rerun the build script if linux_headers.h changes...
    println!("cargo:rerun-if-changed=linux_headers.h");

    // change to kernel working directory, save current wd first
    let curr_wd = env::current_dir().unwrap();
    println!("The current directory is {}", curr_wd.display());
    let k_dir = PathBuf::from(env::var("STD_KERNEL_PATH").unwrap());
    println!("STD_KERNEL_PATH is {}", k_dir.display());
    assert!(env::set_current_dir(&k_dir).is_ok());

    // block of code adapted from https://github.com/tsgates/rust.ko/blob/master/std/build.rs
    let clang_args = match std::env::var("STD_CLANG_ARGS") {
        Ok(string) =>
            match shlex::split(string.as_str()) {
                Some(mut args) => {
                    // Find positions of arguments to remove
                    let mut remove_indices = Vec::with_capacity(CLANG_ARGS_BLACKLIST.len());
                    for (index, clang_arg) in args.iter().enumerate() {
                        if CLANG_ARGS_BLACKLIST.contains(&clang_arg.as_str()) {
                            remove_indices.push(index);
                        }
                    }
                    
                    // Remove the found positions from the argument list
                    for index in remove_indices.iter().rev() {
                        args.swap_remove(*index);
                    }
                    
                    args
                },
                None => {
                    panic!("Malformed environment variable STD_CLANG_ARGS");
                }
            },
        Err(error) => {
            panic!("Missing environment variable STD_CLANG_ARGS: {:?}", error);
        }
    };

    for arg in clang_args.iter() {
        println!("{:?}", arg);
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Nightly)
        .derive_copy(false) // used because deriving copy for things like spinlock would be unsafe.
        .derive_debug(false)
        // .emit_ast(false)
        .use_core()
        .ctypes_prefix("c_types")
        .clang_arg("-Dfalse=__false")
        .clang_arg("-Dtrue=__true")
        .clang_arg("-Du64=__u64")
        .clang_args(clang_args.iter())
        .opaque_type("timex")
        .header(curr_wd.join("linux_headers.h").to_string_lossy())
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // change back to old working directory (rust directory)
    assert!(env::set_current_dir(&curr_wd).is_ok());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("{:?}", out_path.join("bindings.rs").display());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}