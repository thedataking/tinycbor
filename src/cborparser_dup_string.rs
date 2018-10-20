#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(libc)]
extern crate libc;
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
    use super::libc;
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"))]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
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
    use super::libc;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
    extern "C" {
        #[no_mangle]
        pub fn _cbor_value_copy_string(
            value: *const CborValue,
            buffer: *mut libc::c_void,
            buflen: *mut size_t,
            next: *mut CborValue,
        ) -> CborError;
    }
}
#[cfg(not(source_header = "/usr/include/stdlib.h"))]
pub mod stdlib_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void) -> ();
        #[no_mangle]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
use self::cbor_h::{
    CborError, CborErrorAdvancePastEOF, CborErrorDataTooLarge, CborErrorDuplicateObjectKeys,
    CborErrorExcludedType, CborErrorExcludedValue, CborErrorGarbageAtEnd, CborErrorIO,
    CborErrorIllegalNumber, CborErrorIllegalSimpleType, CborErrorIllegalType,
    CborErrorImproperValue, CborErrorInappropriateTagForType, CborErrorInternalError,
    CborErrorInvalidUtf8TextString, CborErrorJsonNotImplemented, CborErrorJsonObjectKeyIsAggregate,
    CborErrorJsonObjectKeyNotString, CborErrorMapKeyNotString, CborErrorMapKeysNotUnique,
    CborErrorMapNotSorted, CborErrorNestingTooDeep, CborErrorOutOfMemory,
    CborErrorOverlongEncoding, CborErrorTooFewItems, CborErrorTooManyItems,
    CborErrorUnexpectedBreak, CborErrorUnexpectedEOF, CborErrorUnknownLength,
    CborErrorUnknownSimpleType, CborErrorUnknownTag, CborErrorUnknownType,
    CborErrorUnsupportedType, CborNoError, CborParser, CborUnknownError, CborValue,
    _cbor_value_copy_string,
};
use self::stddef_h::size_t;
use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use self::stdlib_h::{free, malloc};
use self::types_h::{__uint16_t, __uint32_t, __uint8_t};
#[no_mangle]
pub unsafe extern "C" fn _cbor_value_dup_string(
    mut value: *const CborValue,
    mut buffer: *mut *mut libc::c_void,
    mut buflen: *mut size_t,
    mut next: *mut CborValue,
) -> CborError {
    let mut err: CborError = CborNoError;
    *buflen = 18446744073709551615u64;
    err = _cbor_value_copy_string(value, 0 as *mut libc::c_void, buflen, 0 as *mut CborValue);
    if 0 != err as u64 {
        return err;
    } else {
        *buflen = (*buflen).wrapping_add(1);
        *buffer = malloc(*buflen);
        if (*buffer).is_null() {
            /* out of memory */
            return CborErrorOutOfMemory;
        } else {
            err = _cbor_value_copy_string(value, *buffer, buflen, next);
            if 0 != err as u64 {
                free(*buffer);
                return err;
            } else {
                return CborNoError;
            }
        }
    };
}
