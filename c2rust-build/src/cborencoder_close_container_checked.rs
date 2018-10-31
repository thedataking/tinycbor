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
    cbor_encoder_close_container, unnamed, CborEncoder, CborError, CborErrorAdvancePastEOF,
    CborErrorDataTooLarge, CborErrorDuplicateObjectKeys, CborErrorExcludedType,
    CborErrorExcludedValue, CborErrorGarbageAtEnd, CborErrorIO, CborErrorIllegalNumber,
    CborErrorIllegalSimpleType, CborErrorIllegalType, CborErrorImproperValue,
    CborErrorInappropriateTagForType, CborErrorInternalError, CborErrorInvalidUtf8TextString,
    CborErrorJsonNotImplemented, CborErrorJsonObjectKeyIsAggregate,
    CborErrorJsonObjectKeyNotString, CborErrorMapKeyNotString, CborErrorMapKeysNotUnique,
    CborErrorMapNotSorted, CborErrorNestingTooDeep, CborErrorOutOfMemory,
    CborErrorOverlongEncoding, CborErrorTooFewItems, CborErrorTooManyItems,
    CborErrorUnexpectedBreak, CborErrorUnexpectedEOF, CborErrorUnknownLength,
    CborErrorUnknownSimpleType, CborErrorUnknownTag, CborErrorUnknownType,
    CborErrorUnsupportedType, CborNoError, CborUnknownError,
};
use stdlib::{__uint8_t, ptrdiff_t, size_t, uint8_t};

#[no_mangle]
pub unsafe extern "C" fn cbor_encoder_close_container_checked(
    mut encoder: *mut CborEncoder,
    mut containerEncoder: *const CborEncoder,
) -> CborError {
    return cbor_encoder_close_container(encoder, containerEncoder);
}
