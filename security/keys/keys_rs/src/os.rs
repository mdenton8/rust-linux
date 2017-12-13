// This is so the bindings can generate unions with "non-Copyable" structs in them
#![feature(untagged_unions)]

// So we can tolerate the bindings generated
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include our c_types so they work (bindgen set to generate c_types::c_int and such)
use c_types;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));