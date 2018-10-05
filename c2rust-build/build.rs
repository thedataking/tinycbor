extern crate cc;

fn main() {
    // doesn't work; we need a static lib that only provides `cbor_fprintf`.
    // println!("cargo:rustc-link-search=native=/Users/perl/Work/tinycbor/lib/");
    // println!("cargo:rustc-link-lib=static=tinycbor");

    cc::Build::new()
        .file("../src/cborpretty_stdio_c2rust.c")
        .compile("tinycbor-c2rust");
}