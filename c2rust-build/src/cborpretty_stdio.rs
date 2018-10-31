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
use stdarg_h::va_list;
use stdlib::{
    _IO_lock_t, _IO_marker, __builtin_va_list, __off64_t, __off_t, __uint16_t, __uint32_t,
    __uint8_t, __va_list_tag, size_t, uint16_t, uint32_t, uint8_t, vfprintf, FILE, _IO_FILE,
};

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
