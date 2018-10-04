use libc;
extern "C" {
    #[no_mangle]
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn _cbor_value_copy_string(
        value: *const CborValue_0,
        buffer: *mut libc::c_void,
        buflen: *mut size_t,
        next: *mut CborValue_0,
    ) -> CborError_0;
    #[no_mangle]
    fn free(_: *mut libc::c_void) -> ();
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
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
#[no_mangle]
pub unsafe extern "C" fn _cbor_value_dup_string(
    mut value: *const CborValue_0,
    mut buffer: *mut *mut libc::c_void,
    mut buflen: *mut size_t,
    mut next: *mut CborValue_0,
) -> CborError_0 {
    let mut err: CborError_0 = CborNoError;
    if 0 != buffer.is_null() as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"_cbor_value_dup_string\x00",
            )).as_ptr(),
            b"src/cborparser_dup_string.c\x00" as *const u8 as *const libc::c_char,
            100i32,
            b"buffer\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 != buflen.is_null() as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"_cbor_value_dup_string\x00",
            )).as_ptr(),
            b"src/cborparser_dup_string.c\x00" as *const u8 as *const libc::c_char,
            101i32,
            b"buflen\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    *buflen = 18446744073709551615u64;
    err = _cbor_value_copy_string(value, 0 as *mut libc::c_void, buflen, 0 as *mut CborValue_0);
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
