#![feature(libc)]
#![feature(const_ptr_null)]
#![feature(offset_to)]
#![feature(const_ptr_null_mut)]
#![feature(extern_types)]
#![feature(asm)]
#![feature(ptr_wrapping_offset_from)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]

#![feature(extern_prelude)] // TODO: not sure this is necessary in newer nightlies
#![feature(const_vec_new)]  // for `<std::vec::Vec<T>>::new` in statics

extern crate libc;


#[path = "../tools/cbordump/cbordump.rs"] pub mod cbordump;

#[path = "../src/cborencoder.rs"] pub mod cborencoder;

#[path = "../src/cborencoder_close_container_checked.rs"] pub mod cborencoder_close_container_checked;

#[path = "../src/cborerrorstrings.rs"] pub mod cborerrorstrings;

#[path = "../src/cborparser.rs"] pub mod cborparser;

#[path = "../src/cborparser_dup_string.rs"] pub mod cborparser_dup_string;

#[path = "../src/cborpretty.rs"] pub mod cborpretty;

#[path = "../src/cborpretty_stdio.rs"] pub mod cborpretty_stdio;

#[path = "../src/cbortojson.rs"] pub mod cbortojson;

#[path = "../src/cborvalidation.rs"] pub mod cborvalidation;

pub fn main() {
    cbordump::main()
}