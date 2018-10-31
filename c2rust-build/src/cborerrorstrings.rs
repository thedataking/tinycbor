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
