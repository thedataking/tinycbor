#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(libc)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char) -> ();
    #[no_mangle]
    fn cbor_error_string(error: CborError_0) -> *const libc::c_char;
    #[no_mangle]
    fn cbor_parser_init(
        buffer: *const uint8_t,
        size: size_t,
        flags: uint32_t,
        parser: *mut CborParser_0,
        it: *mut CborValue_0,
    ) -> CborError_0;
    /* The following API requires a hosted C implementation (uses FILE*) */
    #[no_mangle]
    fn cbor_value_to_pretty_advance_flags(
        out: *mut FILE,
        value: *mut CborValue_0,
        flags: libc::c_int,
    ) -> CborError_0;
    #[no_mangle]
    fn cbor_value_to_json_advance(
        out: *mut FILE,
        value: *mut CborValue_0,
        flags: libc::c_int,
    ) -> CborError_0;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type CborError = libc::c_int;
/* INT_MAX on two's complement machines */
pub const CborErrorInternalError: CborError = 2147483647;
pub const CborErrorOutOfMemory: CborError = -2147483648;
pub const CborErrorJsonNotImplemented: CborError = 1282;
pub const CborErrorJsonObjectKeyNotString: CborError = 1281;
/* errors in converting to JSON */
pub const CborErrorJsonObjectKeyIsAggregate: CborError = 1280;
pub const CborErrorUnsupportedType: CborError = 1026;
pub const CborErrorNestingTooDeep: CborError = 1025;
/* internal implementation errors */
pub const CborErrorDataTooLarge: CborError = 1024;
pub const CborErrorTooFewItems: CborError = 769;
/* encoder errors */
pub const CborErrorTooManyItems: CborError = 768;
pub const CborErrorMapKeysNotUnique: CborError = 523;
pub const CborErrorMapNotSorted: CborError = 522;
pub const CborErrorMapKeyNotString: CborError = 521;
pub const CborErrorOverlongEncoding: CborError = 520;
pub const CborErrorImproperValue: CborError = 519;
pub const CborErrorExcludedValue: CborError = 518;
pub const CborErrorExcludedType: CborError = 517;
pub const CborErrorInvalidUtf8TextString: CborError = 516;
pub const CborErrorDuplicateObjectKeys: CborError = 515;
pub const CborErrorInappropriateTagForType: CborError = 514;
pub const CborErrorUnknownTag: CborError = 513;
/* parser errors in strict mode parsing only */
pub const CborErrorUnknownSimpleType: CborError = 512;
/* types of value less than 32 encoded in two bytes */
pub const CborErrorIllegalSimpleType: CborError = 262;
pub const CborErrorIllegalNumber: CborError = 261;
/* type not allowed here */
pub const CborErrorIllegalType: CborError = 260;
/* can only happen in major type 7 */
pub const CborErrorUnknownType: CborError = 259;
pub const CborErrorUnexpectedBreak: CborError = 258;
pub const CborErrorUnexpectedEOF: CborError = 257;
/* parser errors streaming errors */
pub const CborErrorGarbageAtEnd: CborError = 256;
pub const CborErrorIO: CborError = 4;
pub const CborErrorAdvancePastEOF: CborError = 3;
/* request for length in array, map, or string with indeterminate length */
pub const CborErrorUnknownLength: CborError = 2;
/* errors in all modes */
pub const CborUnknownError: CborError = 1;
pub const CborNoError: CborError = 0;
/* #define the constants so we can check with #ifdef */
/* Error API */
pub type CborError_0 = CborError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CborParser {
    pub end: *const uint8_t,
    pub flags: uint32_t,
}
pub type CborParser_0 = CborParser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CborValue {
    pub parser: *const CborParser_0,
    pub ptr: *const uint8_t,
    pub remaining: uint32_t,
    pub extra: uint16_t,
    pub type_0: uint8_t,
    pub flags: uint8_t,
}
pub type CborValue_0 = CborValue;
/* Human-readable (dump) API */
pub type CborPrettyFlags = libc::c_uint;
pub const CborPrettyDefaultFlags: CborPrettyFlags = 2;
pub const CborPrettyMergeStringFragments: CborPrettyFlags = 0;
pub const CborPrettyShowStringFragments: CborPrettyFlags = 256;
pub const CborPrettyIndicateOverlongNumbers: CborPrettyFlags = 4;
/* deprecated */
pub const CborPrettyIndicateIndetermineLength: CborPrettyFlags = 2;
pub const CborPrettyIndicateIndeterminateLength: CborPrettyFlags = 2;
pub const CborPrettyTextualEncodingIndicators: CborPrettyFlags = 0;
pub const CborPrettyNumericEncodingIndicators: CborPrettyFlags = 1;
/* ***************************************************************************
**
** Copyright (C) 2015 Intel Corporation
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and associated documentation files (the "Software"), to deal
** in the Software without restriction, including without limitation the rights
** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
** copies of the Software, and to permit persons to whom the Software is
** furnished to do so, subject to the following conditions:
**
** The above copyright notice and this permission notice shall be included in
** all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
** FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
** AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
** LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
** OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
** THE SOFTWARE.
**
****************************************************************************/
/* Conversion to JSON */
pub type CborToJsonFlags = libc::c_uint;
pub const CborConvertDefaultFlags: CborToJsonFlags = 0;
pub const CborConvertStringifyMapKeys: CborToJsonFlags = 8;
pub const CborConvertRequireMapStringKeys: CborToJsonFlags = 0;
pub const CborConvertByteStringsToBase64Url: CborToJsonFlags = 4;
pub const CborConvertObeyByteStringTags: CborToJsonFlags = 0;
pub const CborConvertIgnoreTags: CborToJsonFlags = 0;
pub const CborConvertTagsToObjects: CborToJsonFlags = 2;
pub const CborConvertAddMetadata: CborToJsonFlags = 1;
/* ***************************************************************************
**
** Copyright (C) 2015 Intel Corporation
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and associated documentation files (the "Software"), to deal
** in the Software without restriction, including without limitation the rights
** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
** copies of the Software, and to permit persons to whom the Software is
** furnished to do so, subject to the following conditions:
**
** The above copyright notice and this permission notice shall be included in
** all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
** FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
** AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
** LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
** OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
** THE SOFTWARE.
**
****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut old: *mut libc::c_void,
    mut size: size_t,
    mut fname: *const libc::c_char,
) -> *mut libc::c_void {
    old = realloc(old, size);
    if old.is_null() {
        fprintf(
            stderr,
            b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
            fname,
            strerror(*__errno_location()),
        );
        exit(1i32);
    } else {
        return old;
    };
}
#[no_mangle]
pub unsafe extern "C" fn printerror(mut err: CborError_0, mut fname: *const libc::c_char) -> () {
    fprintf(
        stderr,
        b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
        fname,
        cbor_error_string(err),
    );
    exit(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn dumpFile(
    mut in_0: *mut FILE,
    mut fname: *const libc::c_char,
    mut printJosn: bool,
    mut flags: libc::c_int,
) -> () {
    static mut chunklen: size_t = unsafe { (16i32 * 1024i32) as size_t };
    static mut bufsize: size_t = unsafe { 0i32 as size_t };
    static mut buffer: *mut uint8_t = unsafe { 0 as *const uint8_t as *mut uint8_t };
    let mut buflen: size_t = 0i32 as size_t;
    loop {
        if bufsize == buflen {
            bufsize = (bufsize as libc::c_ulong).wrapping_add(chunklen) as size_t as size_t;
            buffer = xrealloc(buffer as *mut libc::c_void, bufsize, fname) as *mut uint8_t
        }
        let mut n: size_t = fread(
            buffer.offset(buflen as isize) as *mut libc::c_void,
            1i32 as size_t,
            bufsize.wrapping_sub(buflen),
            in_0,
        );
        buflen = (buflen as libc::c_ulong).wrapping_add(n) as size_t as size_t;
        if n == 0i32 as libc::c_ulong {
            if !(0 == ferror(in_0)) {
                fprintf(
                    stderr,
                    b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
                    fname,
                    strerror(*__errno_location()),
                );
                exit(1i32);
            }
        }
        if !(0 == feof(in_0)) {
            break;
        }
    }
    let mut parser: CborParser_0 = CborParser {
        end: 0 as *const uint8_t,
        flags: 0,
    };
    let mut value: CborValue_0 = CborValue {
        parser: 0 as *const CborParser_0,
        ptr: 0 as *const uint8_t,
        remaining: 0,
        extra: 0,
        type_0: 0,
        flags: 0,
    };
    let mut err: CborError_0 =
        cbor_parser_init(buffer, buflen, 0i32 as uint32_t, &mut parser, &mut value);
    if 0 == err as u64 {
        if printJosn {
            err = cbor_value_to_json_advance(stdout, &mut value, flags)
        } else {
            err = cbor_value_to_pretty_advance_flags(stdout, &mut value, flags)
        }
        if 0 == err as u64 {
            puts(b"\x00" as *const u8 as *const libc::c_char);
        }
    }
    if 0 == err as u64 && value.ptr != buffer.offset(buflen as isize) {
        err = CborErrorGarbageAtEnd
    }
    if 0 != err as u64 {
        printerror(err, fname);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut printJson: bool = 0 != 0i32;
    let mut json_flags: libc::c_int = CborConvertDefaultFlags as libc::c_int;
    let mut cbor_flags: libc::c_int = CborPrettyDefaultFlags as libc::c_int;
    let mut c: libc::c_int = 0;
    loop {
        c = getopt(
            argc,
            argv,
            b"MOSUcjhfn\x00" as *const u8 as *const libc::c_char,
        );
        if !(c != -1i32) {
            break;
        }
        match c {
            99 => {
                printJson = 0 != 0i32;
                continue;
            }
            106 => {
                printJson = 0 != 1i32;
                continue;
            }
            102 => {
                cbor_flags |= CborPrettyShowStringFragments as libc::c_int;
                continue;
            }
            110 => {
                cbor_flags |= CborPrettyIndicateIndeterminateLength as libc::c_int
                    | CborPrettyNumericEncodingIndicators as libc::c_int;
                continue;
            }
            77 => {
                json_flags |= CborConvertAddMetadata as libc::c_int;
                continue;
            }
            79 => {
                json_flags |= CborConvertTagsToObjects as libc::c_int;
                continue;
            }
            83 => {
                json_flags |= CborConvertStringifyMapKeys as libc::c_int;
                continue;
            }
            85 => {
                json_flags |= CborConvertByteStringsToBase64Url as libc::c_int;
                continue;
            }
            63 => {
                fprintf(
                    stderr,
                    b"Unknown option -%c.\n\x00" as *const u8 as *const libc::c_char,
                    optopt,
                );
            }
            104 => {}
            _ => {
                /* fall through */
                continue;
            }
        }
        puts(b"Usage: cbordump [OPTION]... [FILE]...\nInterprets FILEs as CBOR binary data and dumps the content to stdout.\n\nOptions:\n -c       Print a CBOR dump (see RFC 7049) (default)\n -j       Print a JSON equivalent version\n -h       Print this help output and exit\nWhen JSON output is active, the following options are recognized:\n -M       Add metadata so converting back to CBOR is possible\n -O       Convert CBOR tags to JSON objects\n -S       Stringify non-text string map keys\n -U       Convert all CBOR byte strings to Base64url regardless of tags\nWhen CBOR dump is active, the following options are recognized:\n -f       Show text and byte string fragments\n -n       Show overlong encoding of CBOR numbers and length\x00"
                 as *const u8 as *const libc::c_char);
        return if c == '?' as i32 { 1i32 } else { 0i32 };
    }
    let mut fname: *mut *mut libc::c_char = argv.offset(optind as isize);
    if (*fname).is_null() {
        dumpFile(
            stdin,
            b"-\x00" as *const u8 as *const libc::c_char,
            printJson,
            if 0 != printJson as libc::c_int {
                json_flags
            } else {
                cbor_flags
            },
        );
    } else {
        while !(*fname).is_null() {
            let mut in_0: *mut FILE = fopen(*fname, b"rb\x00" as *const u8 as *const libc::c_char);
            if in_0.is_null() {
                perror(b"open\x00" as *const u8 as *const libc::c_char);
                return 1i32;
            } else {
                dumpFile(
                    in_0,
                    *fname,
                    printJson,
                    if 0 != printJson as libc::c_int {
                        json_flags
                    } else {
                        cbor_flags
                    },
                );
                fclose(in_0);
                fname = fname.offset(1isize)
            }
        }
    }
    return 0i32;
}
pub fn main() -> () {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
