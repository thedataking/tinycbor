#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(libc)]
extern crate libc;
#[cfg(not(source_header = "vararg"))]
pub mod vararg {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
    use super::libc;
}
#[cfg(
    not(
        source_header = "/home/miguelsaldivar/workspace/C2Rust/dependencies/llvm-6.0.1/build.donna/lib/clang/6.0.1/include/stddef.h"
    )
)]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::libc;
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/types.h"))]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __uint16_t = libc::c_ushort;
    pub type __uint32_t = libc::c_uint;
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
    use super::libc;
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"))]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/types/__FILE.h"))]
pub mod __FILE_h {
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
    use super::libc;
    use super::libio_h::{_IO_lock_t, _IO_marker};
    use super::stddef_h::size_t;
    use super::types_h::{__off64_t, __off_t};
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/libio.h"))]
pub mod libio_h {
    pub type _IO_lock_t = ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _IO_marker {
        pub _next: *mut _IO_marker,
        pub _sbuf: *mut _IO_FILE,
        pub _pos: libc::c_int,
    }
    use super::__FILE_h::_IO_FILE;
    use super::libc;
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h"))]
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::__FILE_h::_IO_FILE;
}
#[cfg(
    not(
        source_header = "/home/miguelsaldivar/workspace/C2Rust/dependencies/llvm-6.0.1/build.donna/lib/clang/6.0.1/include/stdarg.h"
    )
)]
pub mod stdarg_h {
    pub type va_list = __builtin_va_list;
    use super::vararg::__builtin_va_list;
}
#[cfg(not(source_header = "/home/miguelsaldivar/workspace/misc/tinycbor/src/cbor.h"))]
pub mod cbor_h {
    /* #define the constants so we can check with #ifdef */
    /* Error API */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct CborParser {
        pub end: *const uint8_t,
        pub flags: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct CborValue {
        pub parser: *const CborParser,
        pub ptr: *const uint8_t,
        pub remaining: uint32_t,
        pub extra: uint16_t,
        pub type_0: uint8_t,
        pub flags: uint8_t,
    }
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
    pub type CborStreamFunction = Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, ...) -> CborError,
    >;
    use super::libc;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        pub fn cbor_value_to_pretty_stream(
            streamFunction: CborStreamFunction,
            token: *mut libc::c_void,
            value: *mut CborValue,
            flags: libc::c_int,
        ) -> CborError;
    }
}
#[cfg(not(source_header = "/usr/include/stdio.h"))]
pub mod stdio_h {
    use super::libc;
    use super::vararg::__va_list_tag;
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: *mut __va_list_tag)
            -> libc::c_int;
    }
}
use self::__FILE_h::_IO_FILE;
use self::cbor_h::{
    cbor_value_to_pretty_stream, CborError, CborErrorAdvancePastEOF, CborErrorDataTooLarge,
    CborErrorDuplicateObjectKeys, CborErrorExcludedType, CborErrorExcludedValue,
    CborErrorGarbageAtEnd, CborErrorIO, CborErrorIllegalNumber, CborErrorIllegalSimpleType,
    CborErrorIllegalType, CborErrorImproperValue, CborErrorInappropriateTagForType,
    CborErrorInternalError, CborErrorInvalidUtf8TextString, CborErrorJsonNotImplemented,
    CborErrorJsonObjectKeyIsAggregate, CborErrorJsonObjectKeyNotString, CborErrorMapKeyNotString,
    CborErrorMapKeysNotUnique, CborErrorMapNotSorted, CborErrorNestingTooDeep,
    CborErrorOutOfMemory, CborErrorOverlongEncoding, CborErrorTooFewItems, CborErrorTooManyItems,
    CborErrorUnexpectedBreak, CborErrorUnexpectedEOF, CborErrorUnknownLength,
    CborErrorUnknownSimpleType, CborErrorUnknownTag, CborErrorUnknownType,
    CborErrorUnsupportedType, CborNoError, CborParser, CborPrettyDefaultFlags, CborPrettyFlags,
    CborPrettyIndicateIndeterminateLength, CborPrettyIndicateIndetermineLength,
    CborPrettyIndicateOverlongNumbers, CborPrettyMergeStringFragments,
    CborPrettyNumericEncodingIndicators, CborPrettyShowStringFragments,
    CborPrettyTextualEncodingIndicators, CborStreamFunction, CborUnknownError, CborValue,
};
use self::libio_h::{_IO_lock_t, _IO_marker};
use self::stdarg_h::va_list;
use self::stddef_h::size_t;
use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use self::stdio_h::vfprintf;
use self::types_h::{__off64_t, __off_t, __uint16_t, __uint32_t, __uint8_t};
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::FILE_h::FILE;

extern "C" {
#[no_mangle]
pub fn cbor_fprintf(out: *mut libc::c_void, fmt: *const libc::c_char, ...) -> CborError;
}
/* The following API requires a hosted C implementation (uses FILE*) */
#[no_mangle]
pub unsafe extern "C" fn cbor_value_to_pretty_advance_flags(
    mut out: *mut FILE,
    mut value: *mut CborValue,
    mut flags: libc::c_int,
) -> CborError {
    return cbor_value_to_pretty_stream(Some(cbor_fprintf), out as *mut libc::c_void, value, flags);
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_to_pretty_advance(
    mut out: *mut FILE,
    mut value: *mut CborValue,
) -> CborError {
    return cbor_value_to_pretty_stream(
        Some(cbor_fprintf),
        out as *mut libc::c_void,
        value,
        CborPrettyDefaultFlags as libc::c_int,
    );
}
