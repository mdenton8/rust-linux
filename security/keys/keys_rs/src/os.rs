// So we can tolerate the bindings generated
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// avoid this error......
#![allow(dead_code)]

// allow improper c_types because due to the kernel config there can
// be empty structs, like lock_class_key if lock_dep is disabled.
#![allow(improper_ctypes)]

// include our c_types so they work (bindgen set to generate c_types::c_int and such)
use c_types;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));