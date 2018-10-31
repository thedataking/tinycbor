#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
#![feature(libc)]
extern crate libc;
use cbor_h::{
    cbor_error_string, cbor_parser_init, cbor_value_to_pretty_advance_flags, CborError,
    CborErrorAdvancePastEOF, CborErrorDataTooLarge, CborErrorDuplicateObjectKeys,
    CborErrorExcludedType, CborErrorExcludedValue, CborErrorGarbageAtEnd, CborErrorIO,
    CborErrorIllegalNumber, CborErrorIllegalSimpleType, CborErrorIllegalType,
    CborErrorImproperValue, CborErrorInappropriateTagForType, CborErrorInternalError,
    CborErrorInvalidUtf8TextString, CborErrorJsonNotImplemented, CborErrorJsonObjectKeyIsAggregate,
    CborErrorJsonObjectKeyNotString, CborErrorMapKeyNotString, CborErrorMapKeysNotUnique,
    CborErrorMapNotSorted, CborErrorNestingTooDeep, CborErrorOutOfMemory,
    CborErrorOverlongEncoding, CborErrorTooFewItems, CborErrorTooManyItems,
    CborErrorUnexpectedBreak, CborErrorUnexpectedEOF, CborErrorUnknownLength,
    CborErrorUnknownSimpleType, CborErrorUnknownTag, CborErrorUnknownType,
    CborErrorUnsupportedType, CborNoError, CborParser, CborPrettyDefaultFlags, CborPrettyFlags,
    CborPrettyIndicateIndeterminateLength, CborPrettyIndicateIndetermineLength,
    CborPrettyIndicateOverlongNumbers, CborPrettyMergeStringFragments,
    CborPrettyNumericEncodingIndicators, CborPrettyShowStringFragments,
    CborPrettyTextualEncodingIndicators, CborUnknownError, CborValue,
};
use cborjson_h::{
    cbor_value_to_json_advance, CborConvertAddMetadata, CborConvertByteStringsToBase64Url,
    CborConvertDefaultFlags, CborConvertIgnoreTags, CborConvertObeyByteStringTags,
    CborConvertRequireMapStringKeys, CborConvertStringifyMapKeys, CborConvertTagsToObjects,
    CborToJsonFlags,
};
use stdlib::{
    _IO_lock_t, _IO_marker, __errno_location, __off64_t, __off_t, __uint16_t, __uint32_t,
    __uint8_t, exit, fclose, feof, ferror, fopen, fprintf, fread, getopt, optind, optopt, perror,
    puts, realloc, size_t, stderr, stdin, stdout, strerror, uint16_t, uint32_t, uint8_t, FILE,
    _IO_FILE,
};

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
pub unsafe extern "C" fn printerror(mut err: CborError, mut fname: *const libc::c_char) -> () {
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
    let mut parser: CborParser = CborParser {
        end: 0 as *const uint8_t,
        flags: 0,
    };
    let mut value: CborValue = CborValue {
        parser: 0 as *const CborParser,
        ptr: 0 as *const uint8_t,
        remaining: 0,
        extra: 0,
        type_0: 0,
        flags: 0,
    };
    let mut err: CborError =
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
