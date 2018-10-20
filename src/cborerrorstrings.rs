#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(libc)]
extern crate libc;
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
    use super::libc;
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
    CborErrorUnsupportedType, CborNoError, CborUnknownError,
};
#[no_mangle]
pub unsafe extern "C" fn cbor_error_string(mut error: CborError) -> *const libc::c_char {
    match error as libc::c_int {
        0 => { return b"\x00" as *const u8 as *const libc::c_char }
        1 => {
            return b"unknown error\x00" as *const u8 as *const libc::c_char
        }
        -2147483648 => {
            return b"out of memory/need more memory\x00" as *const u8 as
                       *const libc::c_char
        }
        2 => {
            return b"unknown length (attempted to get the length of a map/array/string of indeterminate length\x00"
                       as *const u8 as *const libc::c_char
        }
        3 => {
            return b"attempted to advance past EOF\x00" as *const u8 as
                       *const libc::c_char
        }
        4 => { return b"I/O error\x00" as *const u8 as *const libc::c_char }
        256 => {
            return b"garbage after the end of the content\x00" as *const u8 as
                       *const libc::c_char
        }
        257 => {
            return b"unexpected end of data\x00" as *const u8 as
                       *const libc::c_char
        }
        258 => {
            return b"unexpected \'break\' byte\x00" as *const u8 as
                       *const libc::c_char
        }
        259 => {
            return b"illegal byte (encodes future extension type)\x00" as
                       *const u8 as *const libc::c_char
        }
        260 => {
            return b"mismatched string type in chunked string\x00" as
                       *const u8 as *const libc::c_char
        }
        261 => {
            return b"illegal initial byte (encodes unspecified additional information)\x00"
                       as *const u8 as *const libc::c_char
        }
        262 => {
            return b"illegal encoding of simple type smaller than 32\x00" as
                       *const u8 as *const libc::c_char
        }
        512 => {
            return b"unknown simple type\x00" as *const u8 as
                       *const libc::c_char
        }
        513 => {
            return b"unknown tag\x00" as *const u8 as *const libc::c_char
        }
        514 => {
            return b"inappropriate tag for type\x00" as *const u8 as
                       *const libc::c_char
        }
        515 => {
            return b"duplicate keys in object\x00" as *const u8 as
                       *const libc::c_char
        }
        516 => {
            return b"invalid UTF-8 content in string\x00" as *const u8 as
                       *const libc::c_char
        }
        517 => {
            return b"excluded type found\x00" as *const u8 as
                       *const libc::c_char
        }
        518 => {
            return b"excluded value found\x00" as *const u8 as
                       *const libc::c_char
        }
        519 | 520 => {
            return b"value encoded in non-canonical form\x00" as *const u8 as
                       *const libc::c_char
        }
        521 | 1281 => {
            return b"key in map is not a string\x00" as *const u8 as
                       *const libc::c_char
        }
        522 => {
            return b"map is not sorted\x00" as *const u8 as
                       *const libc::c_char
        }
        523 => {
            return b"map keys are not unique\x00" as *const u8 as
                       *const libc::c_char
        }
        768 => {
            return b"too many items added to encoder\x00" as *const u8 as
                       *const libc::c_char
        }
        769 => {
            return b"too few items added to encoder\x00" as *const u8 as
                       *const libc::c_char
        }
        1024 => {
            return b"internal error: data too large\x00" as *const u8 as
                       *const libc::c_char
        }
        1025 => {
            return b"internal error: too many nested containers found in recursive function\x00"
                       as *const u8 as *const libc::c_char
        }
        1026 => {
            return b"unsupported type\x00" as *const u8 as *const libc::c_char
        }
        1280 => {
            return b"conversion to JSON failed: key in object is an array or map\x00"
                       as *const u8 as *const libc::c_char
        }
        1282 => {
            return b"conversion to JSON failed: open_memstream unavailable\x00"
                       as *const u8 as *const libc::c_char
        }
        2147483647 => {
            return b"internal error\x00" as *const u8 as *const libc::c_char
        }
        _ => { return cbor_error_string(CborUnknownError) }
    };
}
