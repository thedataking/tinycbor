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
    CborErrorUnsupportedType, CborNoError, CborParser, CborUnknownError, CborValue,
    _cbor_value_copy_string,
};
use stdlib::{
    __uint16_t, __uint32_t, __uint8_t, free, malloc, size_t, uint16_t, uint32_t, uint8_t,
};

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
