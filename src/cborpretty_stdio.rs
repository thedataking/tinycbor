#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(libc)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: *mut __va_list_tag) -> libc::c_int;
    #[no_mangle]
    fn cbor_value_to_pretty_stream(
        streamFunction: CborStreamFunction,
        token: *mut libc::c_void,
        value: *mut CborValue_0,
        flags: libc::c_int,
    ) -> CborError_0;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type va_list = __builtin_va_list;
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
pub type CborStreamFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, ...) -> CborError_0>;
/* The following API requires a hosted C implementation (uses FILE*) */
#[no_mangle]
pub unsafe extern "C" fn cbor_value_to_pretty_advance_flags(
    mut out: *mut FILE,
    mut value: *mut CborValue_0,
    mut flags: libc::c_int,
) -> CborError_0 {
    return cbor_value_to_pretty_stream(Some(cbor_fprintf), out as *mut libc::c_void, value, flags);
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_to_pretty_advance(
    mut out: *mut FILE,
    mut value: *mut CborValue_0,
) -> CborError_0 {
    return cbor_value_to_pretty_stream(
        Some(cbor_fprintf),
        out as *mut libc::c_void,
        value,
        CborPrettyDefaultFlags as libc::c_int,
    );
}
