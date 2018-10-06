use libc;
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: *mut __va_list_tag) -> libc::c_int;
    fn cbor_fprintf(_: *mut libc::c_void, _: *const libc::c_char, ...) -> libc::c_int;
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
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_off_t = __int64_t;
pub type va_list = __darwin_va_list;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_char, _: libc::c_int)
            -> libc::c_int,
    >,
    pub _seek:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t, _: libc::c_int) -> fpos_t>,
    pub _write: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, _: libc::c_int)
            -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
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
