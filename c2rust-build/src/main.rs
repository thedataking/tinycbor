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


extern crate libc;


pub mod cbordump;

pub mod cborencoder;

pub mod cborencoder_close_container_checked;

pub mod cborerrorstrings;

pub mod cborparser;

pub mod cborparser_dup_string;

pub mod cborpretty;

pub mod cborpretty_stdio;

pub mod cbortojson;

pub mod cborvalidation;

pub fn main() {
    cbordump::main()
}
