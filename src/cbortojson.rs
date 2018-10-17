use libc;
use cborparser::CborValue;
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn memcpy(
        __dst: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fputc(_: libc::c_int, _: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn open_memstream(__bufp: *mut *mut libc::c_char, __sizep: *mut size_t) -> *mut FILE;
    #[no_mangle]
    fn cbor_value_advance_fixed(it: *mut CborValue) -> CborError_0;
    #[no_mangle]
    fn cbor_value_enter_container(
        it: *const CborValue,
        recursed: *mut CborValue,
    ) -> CborError_0;
    #[no_mangle]
    fn cbor_value_leave_container(
        it: *mut CborValue,
        recursed: *const CborValue,
    ) -> CborError_0;
    #[no_mangle]
    fn _cbor_value_decode_int64_internal(value: *const CborValue) -> uint64_t;
    #[no_mangle]
    fn _cbor_value_copy_string(
        value: *const CborValue,
        buffer: *mut libc::c_void,
        buflen: *mut size_t,
        next: *mut CborValue,
    ) -> CborError_0;
    #[no_mangle]
    fn _cbor_value_dup_string(
        value: *const CborValue,
        buffer: *mut *mut libc::c_void,
        buflen: *mut size_t,
        next: *mut CborValue,
    ) -> CborError_0;
    #[no_mangle]
    fn cbor_value_calculate_string_length(
        value: *const CborValue,
        length: *mut size_t,
    ) -> CborError_0;
    #[no_mangle]
    fn cbor_value_get_half_float(
        value: *const CborValue,
        result: *mut libc::c_void,
    ) -> CborError_0;
    #[no_mangle]
    fn cbor_value_to_pretty_advance(out: *mut FILE, value: *mut CborValue) -> CborError_0;
    #[no_mangle]
    fn __fpclassifyl(_: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn __fpclassifyd(_: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn __fpclassifyf(_: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn free(_: *mut libc::c_void) -> ();
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type uint_least32_t = uint32_t;
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
/* ***************************************************************************
**
** Copyright (C) 2017 Intel Corporation
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and associated documentation files (the "Software"), to deal
** in the Software without restriction, including without limitation the rights
** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
** copies of the Software, and to permit persons to whom the Software is
** furnished to do so, subject to the following conditions:
**
** The above copyright notice and this permission notice shall be included in
** all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
** FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
** AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
** LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
** OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
** THE SOFTWARE.
**
****************************************************************************/
pub type CborType = libc::c_uint;
/* equivalent to the break byte, so it will never be used */
pub const CborInvalidType: CborType = 255;
pub const CborDoubleType: CborType = 251;
pub const CborFloatType: CborType = 250;
pub const CborHalfFloatType: CborType = 249;
pub const CborUndefinedType: CborType = 247;
pub const CborNullType: CborType = 246;
pub const CborBooleanType: CborType = 245;
pub const CborSimpleType: CborType = 224;
pub const CborTagType: CborType = 192;
pub const CborMapType: CborType = 160;
pub const CborArrayType: CborType = 128;
pub const CborTextStringType: CborType = 96;
pub const CborByteStringType: CborType = 64;
pub const CborIntegerType: CborType = 0;
pub type CborType_0 = CborType;
pub type CborTag = uint64_t;
pub type CborKnownTags = libc::c_uint;
pub const CborSignatureTag: CborKnownTags = 55799;
pub const CborCOSE_SignTag: CborKnownTags = 98;
pub const CborCOSE_MacTag: CborKnownTags = 97;
pub const CborCOSE_EncryptTag: CborKnownTags = 96;
pub const CborMimeMessageTag: CborKnownTags = 36;
pub const CborRegularExpressionTag: CborKnownTags = 35;
pub const CborBase64Tag: CborKnownTags = 34;
pub const CborBase64urlTag: CborKnownTags = 33;
pub const CborUrlTag: CborKnownTags = 32;
pub const CborEncodedCborTag: CborKnownTags = 24;
pub const CborExpectedBase16Tag: CborKnownTags = 23;
pub const CborExpectedBase64Tag: CborKnownTags = 22;
pub const CborExpectedBase64urlTag: CborKnownTags = 21;
pub const CborCOSE_Sign1Tag: CborKnownTags = 18;
pub const CborCOSE_Mac0Tag: CborKnownTags = 17;
pub const CborCOSE_Encrypt0Tag: CborKnownTags = 16;
pub const CborBigfloatTag: CborKnownTags = 5;
pub const CborDecimalTag: CborKnownTags = 4;
pub const CborNegativeBignumTag: CborKnownTags = 3;
pub const CborPositiveBignumTag: CborKnownTags = 2;
pub const CborUnixTime_tTag: CborKnownTags = 1;
pub const CborDateTimeStringTag: CborKnownTags = 0;
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
/* Parser API */
pub type CborParserIteratorFlags = libc::c_uint;
pub const CborIteratorFlag_ContainerIsMap: CborParserIteratorFlags = 32;
pub const CborIteratorFlag_UnknownLength: CborParserIteratorFlags = 4;
pub const CborIteratorFlag_IteratingStringChunks: CborParserIteratorFlags = 2;
pub const CborIteratorFlag_NegativeInteger: CborParserIteratorFlags = 2;
pub const CborIteratorFlag_IntegerValueTooLarge: CborParserIteratorFlags = 1;
/* ***************************************************************************
**
** Copyright (C) 2015 Intel Corporation
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and associated documentation files (the "Software"), to deal
** in the Software without restriction, including without limitation the rights
** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
** copies of the Software, and to permit persons to whom the Software is
** furnished to do so, subject to the following conditions:
**
** The above copyright notice and this permission notice shall be included in
** all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
** FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
** AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
** LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
** OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
** THE SOFTWARE.
**
****************************************************************************/
/* Conversion to JSON */
pub type CborToJsonFlags = libc::c_uint;
pub const CborConvertDefaultFlags: CborToJsonFlags = 0;
pub const CborConvertStringifyMapKeys: CborToJsonFlags = 8;
pub const CborConvertRequireMapStringKeys: CborToJsonFlags = 0;
pub const CborConvertByteStringsToBase64Url: CborToJsonFlags = 4;
pub const CborConvertObeyByteStringTags: CborToJsonFlags = 0;
pub const CborConvertIgnoreTags: CborToJsonFlags = 0;
pub const CborConvertTagsToObjects: CborToJsonFlags = 2;
pub const CborConvertAddMetadata: CborToJsonFlags = 1;
pub type ConversionStatus = ConversionStatus_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConversionStatus_0 {
    pub lastTag: CborTag,
    pub originalNumber: uint64_t,
    pub flags: libc::c_int,
}
/* anything but strings, boolean, null, arrays and maps */
pub const TypeWasNotNative: ConversionStatusFlags = 256;
/* only used with NumberWasInifite or NumberWasTooBig */
pub const NumberWasNegative: ConversionStatusFlags = 8192;
pub const NumberWasInfinite: ConversionStatusFlags = 4096;
pub const NumberWasNaN: ConversionStatusFlags = 2048;
pub const TypeWasTagged: ConversionStatusFlags = 512;
pub const NumberPrecisionWasLost: ConversionStatusFlags = 1024;
pub const FinalTypeMask: ConversionStatusFlags = 255;
/* ***************************************************************************
**
** Copyright (C) 2018 Intel Corporation
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and associated documentation files (the "Software"), to deal
** in the Software without restriction, including without limitation the rights
** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
** copies of the Software, and to permit persons to whom the Software is
** furnished to do so, subject to the following conditions:
**
** The above copyright notice and this permission notice shall be included in
** all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
** FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
** AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
** LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
** OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
** THE SOFTWARE.
**
****************************************************************************/
/* *
 * \defgroup CborToJson Converting CBOR to JSON
 * \brief Group of functions used to convert CBOR to JSON.
 *
 * This group contains two functions that can be used to convert a \ref
 * CborValue object to an equivalent JSON representation. This module attempts
 * to follow the recommendations from RFC 7049 section 4.1 "Converting from
 * CBOR to JSON", though it has a few differences. They are noted below.
 *
 * These functions produce a "minified" JSON output, with no spacing,
 * indentation or line breaks. If those are necessary, they need to be applied
 * in a post-processing phase.
 *
 * Note that JSON cannot support all CBOR types with fidelity, so the
 * conversion is usually lossy. For that reason, TinyCBOR supports adding a set
 * of metadata JSON values that can be used by a JSON-to-CBOR converter to
 * restore the original data types.
 *
 * The TinyCBOR library does not provide a way to convert from JSON
 * representation back to encoded form. However, it provides a tool called
 * \c json2cbor which can be used for that purpose. That tool supports the
 * metadata format that these functions may produce.
 *
 * Either of the functions in this section will attempt to convert exactly one
 * CborValue object to JSON. Those functions may return any error documented
 * for the functions for CborParsing. In addition, if the C standard library
 * stream functions return with error, the text conversion will return with
 * error CborErrorIO.
 *
 * These functions also perform UTF-8 validation in CBOR text strings. If they
 * encounter a sequence of bytes that is not permitted in UTF-8, they will return
 * CborErrorInvalidUtf8TextString. That includes encoding of surrogate points
 * in UTF-8.
 *
 * \warning The metadata produced by these functions is not guaranteed to
 * remain stable. A future update of TinyCBOR may produce different output for
 * the same input and parsers may be unable to handle it.
 *
 * \sa CborParsing, CborPretty, cbor_parser_init()
 */
/* *
 * \addtogroup CborToJson
 * @{
 * <h2 class="groupheader">Conversion limitations</h2>
 *
 * When converting from CBOR to JSON, there may be information loss. This
 * section lists the possible scenarios.
 *
 * \par Number precision:
 * ALL JSON numbers, due to its JavaScript heritage, are IEEE 754
 * double-precision floating point. This means JSON is not capable of
 * representing all integers numbers outside the range [-(2<sup>53</sup>)+1,
 * 2<sup>53</sup>-1] and is not capable of representing NaN or infinite. If the
 * CBOR data contains a number outside the valid range, the conversion will
 * lose precision. If the input was NaN or infinite, the result of the
 * conversion will be the JSON null value. In addition, the distinction between
 * half-, single- and double-precision is lost.
 *
 * \par
 * If enabled, the original value and original type are stored in the metadata.
 *
 * \par Non-native types:
 * CBOR's type system is richer than JSON's, which means some data values
 * cannot be represented when converted to JSON. The conversion silently turns
 * them into strings: CBOR simple types become "simple(nn)" where \c nn is the
 * simple type's value, with the exception of CBOR undefined, which becomes
 * "undefined", while CBOR byte strings are converted to an Base16, Base64, or
 * Base64url encoding
 *
 * \par
 * If enabled, the original type is stored in the metadata.
 *
 * \par Presence of tags:
 * JSON has no support for tagged values, so by default tags are dropped when
 * converting to JSON. However, if the CborConvertObeyByteStringTags option is
 * active (default), then certain known tags are honored and are used to format
 * the conversion of the tagged byte string to JSON.
 *
 * \par
 * If the CborConvertTagsToObjects option is active, then the tag and the
 * tagged value are converted to a JSON object. Otherwise, if enabled, the
 * last (innermost) tag is stored in the metadata.
 *
 * \par Non-string keys in maps:
 * JSON requires all Object keys to be strings, while CBOR does not. By
 * default, if a non-string key is found, the conversion fails with error
 * CborErrorJsonObjectKeyNotString. If the CborConvertStringifyMapKeys option
 * is active, then the conversion attempts to create a string representation
 * using CborPretty. Note that the \c json2cbor tool is not able to parse this
 * back to the original form.
 *
 * \par Duplicate keys in maps:
 * Neither JSON nor CBOR allow duplicated keys, but current TinyCBOR does not
 * validate that this is the case. If there are duplicated keys in the input,
 * they will be repeated in the output, which many JSON tools may flag as
 * invalid. In addition to that, if the CborConvertStringifyMapKeys option is
 * active, it is possible that a non-string key in a CBOR map will be converted
 * to a string form that is identical to another key.
 *
 * \par
 * When metadata support is active, the conversion will add extra key-value
 * pairs to the JSON output so it can store the metadata. It is possible that
 * the keys for the metadata clash with existing keys in the JSON map.
 */
pub type ConversionStatusFlags = libc::c_uint;
unsafe extern "C" fn cbor_value_at_end(mut it: *const CborValue) -> bool {
    return (*it).remaining == 0i32 as libc::c_uint;
}
unsafe extern "C" fn _cbor_value_extract_int64_helper(mut value: *const CborValue) -> uint64_t {
    return if 0
        != (*value).flags as libc::c_int & CborIteratorFlag_IntegerValueTooLarge as libc::c_int
    {
        _cbor_value_decode_int64_internal(value)
    } else {
        (*value).extra as libc::c_ulonglong
    };
}
unsafe extern "C" fn cbor_value_get_type(mut value: *const CborValue) -> CborType_0 {
    return (*value).type_0 as CborType_0;
}
/* Booleans */
unsafe extern "C" fn cbor_value_is_boolean(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborBooleanType as libc::c_int;
}
unsafe extern "C" fn cbor_value_get_boolean(
    mut value: *const CborValue,
    mut result: *mut bool,
) -> CborError_0 {
    if 0 != !cbor_value_is_boolean(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"cbor_value_get_boolean\x00",
            )).as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            332i32,
            b"cbor_value_is_boolean(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    *result = 0 != (*value).extra;
    return CborNoError;
}
/* Simple types */
unsafe extern "C" fn cbor_value_is_simple_type(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborSimpleType as libc::c_int;
}
unsafe extern "C" fn cbor_value_get_simple_type(
    mut value: *const CborValue,
    mut result: *mut uint8_t,
) -> CborError_0 {
    if 0 != !cbor_value_is_simple_type(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"cbor_value_get_simple_type\x00",
            )).as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            342i32,
            b"cbor_value_is_simple_type(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    *result = (*value).extra as uint8_t;
    return CborNoError;
}
/* Integers */
unsafe extern "C" fn cbor_value_is_integer(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborIntegerType as libc::c_int;
}
unsafe extern "C" fn cbor_value_is_negative_integer(mut value: *const CborValue) -> bool {
    return 0 != cbor_value_is_integer(value) as libc::c_int
        && 0 != (*value).flags as libc::c_int & CborIteratorFlag_NegativeInteger as libc::c_int;
}
unsafe extern "C" fn cbor_value_get_raw_integer(
    mut value: *const CborValue,
    mut result: *mut uint64_t,
) -> CborError_0 {
    if 0 != !cbor_value_is_integer(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"cbor_value_get_raw_integer\x00",
            )).as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            357i32,
            b"cbor_value_is_integer(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    *result = _cbor_value_extract_int64_helper(value);
    return CborNoError;
}
/* Tags */
unsafe extern "C" fn cbor_value_is_tag(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborTagType as libc::c_int;
}
unsafe extern "C" fn cbor_value_get_tag(
    mut value: *const CborValue,
    mut result: *mut CborTag,
) -> CborError_0 {
    if 0 != !cbor_value_is_tag(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"cbor_value_get_tag\x00"))
                .as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            398i32,
            b"cbor_value_is_tag(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    *result = _cbor_value_extract_int64_helper(value);
    return CborNoError;
}
/* Strings */
unsafe extern "C" fn cbor_value_is_byte_string(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborByteStringType as libc::c_int;
}
unsafe extern "C" fn cbor_value_is_text_string(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborTextStringType as libc::c_int;
}
unsafe extern "C" fn cbor_value_copy_byte_string(
    mut value: *const CborValue,
    mut buffer: *mut uint8_t,
    mut buflen: *mut size_t,
    mut next: *mut CborValue,
) -> CborError_0 {
    if 0 != !cbor_value_is_byte_string(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"cbor_value_copy_byte_string\x00",
            )).as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            439i32,
            b"cbor_value_is_byte_string(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    return _cbor_value_copy_string(value, buffer as *mut libc::c_void, buflen, next);
}
unsafe extern "C" fn cbor_value_dup_text_string(
    mut value: *const CborValue,
    mut buffer: *mut *mut libc::c_char,
    mut buflen: *mut size_t,
    mut next: *mut CborValue,
) -> CborError_0 {
    if 0 != !cbor_value_is_text_string(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"cbor_value_dup_text_string\x00",
            )).as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            446i32,
            b"cbor_value_is_text_string(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    return _cbor_value_dup_string(value, buffer as *mut *mut libc::c_void, buflen, next);
}
unsafe extern "C" fn cbor_value_is_float(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborFloatType as libc::c_int;
}
unsafe extern "C" fn cbor_value_get_float(
    mut value: *const CborValue,
    mut result: *mut libc::c_float,
) -> CborError_0 {
    let mut data: uint32_t = 0;
    if 0 != !cbor_value_is_float(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"cbor_value_get_float\x00"))
                .as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            502i32,
            b"cbor_value_is_float(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 != (0
        == (*value).flags as libc::c_int & CborIteratorFlag_IntegerValueTooLarge as libc::c_int)
        as libc::c_int as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"cbor_value_get_float\x00"))
                .as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            503i32,
            b"value->flags & CborIteratorFlag_IntegerValueTooLarge\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
    };
    data = _cbor_value_decode_int64_internal(value) as uint32_t;
    memcpy(
        result as *mut libc::c_void,
        &mut data as *mut uint32_t as *const libc::c_void,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    );
    return CborNoError;
}
unsafe extern "C" fn cbor_value_is_double(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborDoubleType as libc::c_int;
}
unsafe extern "C" fn cbor_value_get_double(
    mut value: *const CborValue,
    mut result: *mut libc::c_double,
) -> CborError_0 {
    let mut data: uint64_t = 0;
    if 0 != !cbor_value_is_double(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"cbor_value_get_double\x00",
            )).as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            514i32,
            b"cbor_value_is_double(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 != (0
        == (*value).flags as libc::c_int & CborIteratorFlag_IntegerValueTooLarge as libc::c_int)
        as libc::c_int as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"cbor_value_get_double\x00",
            )).as_ptr(),
            b"./src/cbor.h\x00" as *const u8 as *const libc::c_char,
            515i32,
            b"value->flags & CborIteratorFlag_IntegerValueTooLarge\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
    };
    data = _cbor_value_decode_int64_internal(value);
    memcpy(
        result as *mut libc::c_void,
        &mut data as *mut uint64_t as *const libc::c_void,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    return CborNoError;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_to_json_advance(
    mut out: *mut FILE,
    mut value: *mut CborValue,
    mut flags: libc::c_int,
) -> CborError_0 {
    let mut status: ConversionStatus = ConversionStatus_0 {
        lastTag: 0,
        originalNumber: 0,
        flags: 0,
    };
    return value_to_json(out, value, flags, cbor_value_get_type(value), &mut status);
}
unsafe extern "C" fn value_to_json(
    mut out: *mut FILE,
    mut it: *mut CborValue,
    mut flags: libc::c_int,
    mut type_0: CborType_0,
    mut status: *mut ConversionStatus,
) -> CborError_0 {
    let mut f16: uint16_t = 0;
    let mut n: size_t = 0;
    let mut val_1: libc::c_double = 0.;
    let mut f: libc::c_float = 0.;
    let mut current_block: u64;
    let mut err: CborError_0 = CborNoError;
    (*status).flags = 0i32;
    match type_0 as libc::c_uint {
        128 | 160 => {
            /* recursive type */
            let mut recursed: CborValue = CborValue::new();
            err = cbor_value_enter_container(it, &mut recursed);
            if 0 != err as u64 {
                (*it).ptr = recursed.ptr;
                (*it).idx = recursed.idx;
                /* parse error */
                return err;
            } else if fputc(
                if type_0 as libc::c_uint == CborArrayType as libc::c_int as libc::c_uint {
                    '[' as i32
                } else {
                    '{' as i32
                },
                out,
            ) < 0i32
            {
                return CborErrorIO;
            } else {
                err = (if type_0 as libc::c_uint == CborArrayType as libc::c_int as libc::c_uint {
                    array_to_json(out, &mut recursed, flags, status) as libc::c_int
                } else {
                    map_to_json(out, &mut recursed, flags, status) as libc::c_int
                }) as CborError_0;
                if 0 != err as u64 {
                    (*it).ptr = recursed.ptr;
                    (*it).idx = recursed.idx;
                    /* parse error */
                    return err;
                } else if fputc(
                    if type_0 as libc::c_uint == CborArrayType as libc::c_int as libc::c_uint {
                        ']' as i32
                    } else {
                        '}' as i32
                    },
                    out,
                ) < 0i32
                {
                    return CborErrorIO;
                } else {
                    err = cbor_value_leave_container(it, &mut recursed);
                    if 0 != err as u64 {
                        /* parse error */
                        return err;
                    } else {
                        /* reset, there are never conversion errors for us */
                        (*status).flags = 0i32;
                        return CborNoError;
                    }
                }
            }
        }
        0 => {
            /* JS numbers are IEEE double precision */
            let mut num: libc::c_double = 0.;
            let mut val: uint64_t = 0;
            /* can't fail */
            cbor_value_get_raw_integer(it, &mut val);
            num = val as libc::c_double;
            if cbor_value_is_negative_integer(it) {
                /* convert to negative */
                num = -num - 1i32 as libc::c_double;
                if (-num - 1i32 as libc::c_double) as uint64_t != val {
                    (*status).flags =
                        NumberPrecisionWasLost as libc::c_int | NumberWasNegative as libc::c_int;
                    (*status).originalNumber = val
                }
            } else if num as uint64_t != val {
                (*status).flags = NumberPrecisionWasLost as libc::c_int;
                (*status).originalNumber = val
            }
            /* this number has no fraction, so no decimal points please */
            if fprintf(out, b"%.0f\x00" as *const u8 as *const libc::c_char, num) < 0i32 {
                return CborErrorIO;
            } else {
                current_block = 9386390421034826751;
            }
        }
        64 | 96 => {
            let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
            if type_0 as libc::c_uint == CborByteStringType as libc::c_int as libc::c_uint {
                err = dump_bytestring_base64url(&mut str, it);
                (*status).flags = TypeWasNotNative as libc::c_int
            } else {
                n = 0i32 as size_t;
                err = cbor_value_dup_text_string(it, &mut str, &mut n, it)
            }
            if 0 != err as u64 {
                return err;
            } else {
                err = (if fprintf(out, b"\"%s\"\x00" as *const u8 as *const libc::c_char, str)
                    < 0i32
                {
                    CborErrorIO as libc::c_int
                } else {
                    CborNoError as libc::c_int
                }) as CborError_0;
                free(str as *mut libc::c_void);
                return err;
            }
        }
        192 => return tagged_value_to_json(out, it, flags, status),
        224 => {
            let mut simple_type: uint8_t = 0;
            /* can't fail */
            cbor_value_get_simple_type(it, &mut simple_type);
            (*status).flags = TypeWasNotNative as libc::c_int;
            (*status).originalNumber = simple_type as uint64_t;
            if fprintf(
                out,
                b"\"simple(%hhu)\"\x00" as *const u8 as *const libc::c_char,
                simple_type as libc::c_int,
            ) < 0i32
            {
                return CborErrorIO;
            } else {
                current_block = 9386390421034826751;
            }
        }
        246 => {
            if fprintf(out, b"null\x00" as *const u8 as *const libc::c_char) < 0i32 {
                return CborErrorIO;
            } else {
                current_block = 9386390421034826751;
            }
        }
        247 => {
            (*status).flags = TypeWasNotNative as libc::c_int;
            if fprintf(
                out,
                b"\"undefined\"\x00" as *const u8 as *const libc::c_char,
            ) < 0i32
            {
                return CborErrorIO;
            } else {
                current_block = 9386390421034826751;
            }
        }
        245 => {
            let mut val_0: bool = false;
            /* can't fail */
            cbor_value_get_boolean(it, &mut val_0);
            if fprintf(
                out,
                if 0 != val_0 as libc::c_int {
                    b"true\x00" as *const u8 as *const libc::c_char
                } else {
                    b"false\x00" as *const u8 as *const libc::c_char
                },
            ) < 0i32
            {
                return CborErrorIO;
            } else {
                current_block = 9386390421034826751;
            }
        }
        251 => {
            val_1 = 0.;
            cbor_value_get_double(it, &mut val_1);
            current_block = 13472856163611868459;
        }
        250 => {
            (*status).flags = TypeWasNotNative as libc::c_int;
            cbor_value_get_float(it, &mut f);
            val_1 = f as libc::c_double;
            current_block = 13472856163611868459;
        }
        249 => {
            (*status).flags = TypeWasNotNative as libc::c_int;
            cbor_value_get_half_float(it, &mut f16 as *mut uint16_t as *mut libc::c_void);
            val_1 = decode_half(f16);
            current_block = 13472856163611868459;
        }
        255 => return CborErrorUnknownType,
        _ => {
            current_block = 9386390421034826751;
        }
    }
    match current_block {
        13472856163611868459 => {
            let mut r: libc::c_int = if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
            {
                __fpclassifyf(val_1 as libc::c_float)
            } else if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
            {
                __fpclassifyd(val_1)
            } else {
                __fpclassifyl(val_1 as libc::c_double)
            };
            if r == 1i32 || r == 2i32 {
                if fprintf(out, b"null\x00" as *const u8 as *const libc::c_char) < 0i32 {
                    return CborErrorIO;
                } else {
                    (*status).flags |= if r == 1i32 {
                        NumberWasNaN as libc::c_int
                    } else {
                        NumberWasInfinite as libc::c_int | if val_1 < 0i32 as libc::c_double {
                            NumberWasNegative as libc::c_int
                        } else {
                            0i32
                        }
                    }
                }
            } else {
                let mut ival: uint64_t = fabs(val_1) as uint64_t;
                if ival as libc::c_double == fabs(val_1) {
                    /* print as integer so we get the full precision */
                    r = fprintf(
                        out,
                        b"%s%llu\x00" as *const u8 as *const libc::c_char,
                        if val_1 < 0i32 as libc::c_double {
                            b"-\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        },
                        ival,
                    );
                    /* mark this integer number as a double */
                    (*status).flags |= TypeWasNotNative as libc::c_int
                } else {
                    r = fprintf(out, b"%.17g\x00" as *const u8 as *const libc::c_char, val_1)
                }
                if r < 0i32 {
                    return CborErrorIO;
                }
            }
        }
        _ => {}
    }
    return cbor_value_advance_fixed(it);
}
/* this function was copied & adapted from RFC 7049 Appendix D */
unsafe extern "C" fn decode_half(mut half: libc::c_ushort) -> libc::c_double {
    let mut exp: libc::c_int = half as libc::c_int >> 10i32 & 0x1fi32;
    let mut mant: libc::c_int = half as libc::c_int & 0x3ffi32;
    let mut val: libc::c_double = 0.;
    if exp == 0i32 {
        val = ldexp(mant as libc::c_double, -24i32)
    } else if exp != 31i32 {
        val = ldexp((mant + 1024i32) as libc::c_double, exp - 25i32)
    } else {
        val = (if mant == 0i32 {
            ::std::f32::INFINITY
        } else {
            ::std::f32::NAN
        }) as libc::c_double
    }
    return if 0 != half as libc::c_int & 0x8000i32 {
        -val
    } else {
        val
    };
}
unsafe extern "C" fn tagged_value_to_json(
    mut out: *mut FILE,
    mut it: *mut CborValue,
    mut flags: libc::c_int,
    mut status: *mut ConversionStatus,
) -> CborError_0 {
    let mut tag: CborTag = 0;
    let mut err: CborError_0 = CborNoError;
    if 0 != flags & CborConvertTagsToObjects as libc::c_int {
        /* can't fail */
        cbor_value_get_tag(it, &mut tag);
        err = cbor_value_advance_fixed(it);
        if 0 != err as u64 {
            return err;
        } else if fprintf(
            out,
            b"{\"tag%llu\":\x00" as *const u8 as *const libc::c_char,
            tag,
        ) < 0i32
        {
            return CborErrorIO;
        } else {
            let mut type_0: CborType_0 = cbor_value_get_type(it);
            err = value_to_json(out, it, flags, type_0, status);
            if 0 != err as u64 {
                return err;
            } else {
                if 0 != flags & CborConvertAddMetadata as libc::c_int && 0 != (*status).flags {
                    if fprintf(
                        out,
                        b",\"tag%llu$cbor\":{\x00" as *const u8 as *const libc::c_char,
                        tag,
                    ) < 0i32
                        || add_value_metadata(out, type_0, status) as libc::c_int
                            != CborNoError as libc::c_int
                        || fputc('}' as i32, out) < 0i32
                    {
                        return CborErrorIO;
                    }
                }
                if fputc('}' as i32, out) < 0i32 {
                    return CborErrorIO;
                } else {
                    (*status).flags = TypeWasNotNative as libc::c_int | CborTagType as libc::c_int;
                    return CborNoError;
                }
            }
        }
    } else {
        let mut type_1: CborType_0 = CborIntegerType;
        err = find_tagged_type(it, &mut (*status).lastTag, &mut type_1);
        if 0 != err as u64 {
            return err;
        } else {
            tag = (*status).lastTag;
            /* special handling of byte strings? */
            if type_1 as libc::c_uint == CborByteStringType as libc::c_int as libc::c_uint
                && flags & CborConvertByteStringsToBase64Url as libc::c_int == 0i32
                && (tag == CborNegativeBignumTag as libc::c_int as libc::c_ulonglong
                    || tag == CborExpectedBase16Tag as libc::c_int as libc::c_ulonglong
                    || tag == CborExpectedBase64Tag as libc::c_int as libc::c_ulonglong)
            {
                let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut pre: *mut libc::c_char =
                    b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                if tag == CborNegativeBignumTag as libc::c_int as libc::c_ulonglong {
                    pre = b"~\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    err = dump_bytestring_base64url(&mut str, it)
                } else if tag == CborExpectedBase64Tag as libc::c_int as libc::c_ulonglong {
                    err = dump_bytestring_base64(&mut str, it)
                } else {
                    err = dump_bytestring_base16(&mut str, it)
                }
                if 0 != err as u64 {
                    return err;
                } else {
                    err = (if fprintf(
                        out,
                        b"\"%s%s\"\x00" as *const u8 as *const libc::c_char,
                        pre,
                        str,
                    ) < 0i32
                    {
                        CborErrorIO as libc::c_int
                    } else {
                        CborNoError as libc::c_int
                    }) as CborError_0;
                    free(str as *mut libc::c_void);
                    (*status).flags = TypeWasNotNative as libc::c_int
                        | TypeWasTagged as libc::c_int
                        | CborByteStringType as libc::c_int;
                    return err;
                }
            } else {
                /* no special handling */
                err = value_to_json(out, it, flags, type_1, status);
                (*status).flags = ((*status).flags as libc::c_uint
                    | (TypeWasTagged as libc::c_int as libc::c_uint | type_1 as libc::c_uint))
                    as libc::c_int;
                return err;
            }
        }
    };
}
unsafe extern "C" fn dump_bytestring_base16(
    mut result: *mut *mut libc::c_char,
    mut it: *mut CborValue,
) -> CborError_0 {
    static mut characters: [libc::c_char; 17] = [
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 0,
    ];
    let mut i: size_t = 0;
    let mut n: size_t = 0i32 as size_t;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut err: CborError_0 = cbor_value_calculate_string_length(it, &mut n);
    if 0 != err as u64 {
        return err;
    } else {
        /* a Base16 (hex) output is twice as big as our buffer */
        buffer = malloc(
            n.wrapping_mul(2i32 as libc::c_ulong)
                .wrapping_add(1i32 as libc::c_ulong),
        ) as *mut uint8_t;
        *result = buffer as *mut libc::c_char;
        /* let cbor_value_copy_byte_string know we have an extra byte for the terminating NUL */
        n = n.wrapping_add(1);
        err =
            cbor_value_copy_byte_string(it, buffer.offset(n as isize).offset(-1isize), &mut n, it);
        if 0 != !(err as libc::c_int == CborNoError as libc::c_int) as libc::c_int as libc::c_long {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"dump_bytestring_base16\x00",
                )).as_ptr(),
                b"src/cbortojson.c\x00" as *const u8 as *const libc::c_char,
                187i32,
                b"err == CborNoError\x00" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        i = 0i32 as size_t;
        while i < n {
            let mut byte: uint8_t = *buffer.offset(n.wrapping_add(i) as isize);
            *buffer.offset((2i32 as libc::c_ulong).wrapping_mul(i) as isize) =
                characters[(byte as libc::c_int >> 4i32) as usize] as uint8_t;
            *buffer.offset(
                (2i32 as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1i32 as libc::c_ulong) as isize,
            ) = characters[(byte as libc::c_int & 0xfi32) as usize] as uint8_t;
            i = i.wrapping_add(1)
        }
        return CborNoError;
    };
}
unsafe extern "C" fn dump_bytestring_base64(
    mut result: *mut *mut libc::c_char,
    mut it: *mut CborValue,
) -> CborError_0 {
    static mut alphabet: [libc::c_char; 66] = [
        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
        87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
        112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 43, 47, 61, 0,
    ];
    return generic_dump_base64(result, it, alphabet.as_ptr());
}
unsafe extern "C" fn generic_dump_base64(
    mut result: *mut *mut libc::c_char,
    mut it: *mut CborValue,
    mut alphabet: *const libc::c_char,
) -> CborError_0 {
    let mut n: size_t = 0i32 as size_t;
    let mut i: size_t = 0;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut out: *mut uint8_t = 0 as *mut uint8_t;
    let mut in_0: *mut uint8_t = 0 as *mut uint8_t;
    let mut err: CborError_0 = cbor_value_calculate_string_length(it, &mut n);
    if 0 != err as u64 {
        return err;
    } else {
        /* a Base64 output (untruncated) has 4 bytes for every 3 in the input */
        let mut len: size_t = n
            .wrapping_add(5i32 as libc::c_ulong)
            .wrapping_div(3i32 as libc::c_ulong)
            .wrapping_mul(4i32 as libc::c_ulong);
        buffer = malloc(len.wrapping_add(1i32 as libc::c_ulong)) as *mut uint8_t;
        out = buffer;
        *result = buffer as *mut libc::c_char;
        /* we read our byte string at the tail end of the buffer
         * so we can do an in-place conversion while iterating forwards */
        in_0 = buffer.offset(len as isize).offset(-(n as isize));
        /* let cbor_value_copy_byte_string know we have an extra byte for the terminating NUL */
        n = n.wrapping_add(1);
        err = cbor_value_copy_byte_string(it, in_0, &mut n, it);
        if 0 != !(err as libc::c_int == CborNoError as libc::c_int) as libc::c_int as libc::c_long {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"generic_dump_base64\x00",
                )).as_ptr(),
                b"src/cbortojson.c\x00" as *const u8 as *const libc::c_char,
                217i32,
                b"err == CborNoError\x00" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        let mut val: uint_least32_t = 0i32 as uint_least32_t;
        i = 0i32 as size_t;
        while n.wrapping_sub(i) >= 3i32 as libc::c_ulong {
            /* read 3 bytes x 8 bits = 24 bits */
            //#ifdef __GNUC__
            //        } else if (i) {
            //            __builtin_memcpy(&val, in + i - 1, sizeof(val));
            //            val = cbor_ntohl(val);
            //#endif
            val = ((*in_0.offset(i as isize) as libc::c_int) << 16i32
                | (*in_0.offset(i.wrapping_add(1i32 as libc::c_ulong) as isize) as libc::c_int)
                    << 8i32
                | *in_0.offset(i.wrapping_add(2i32 as libc::c_ulong) as isize) as libc::c_int)
                as uint_least32_t;
            /* write 4 chars x 6 bits = 24 bits */
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 =
                *alphabet.offset((val >> 18i32 & 0x3fi32 as libc::c_uint) as isize) as uint8_t;
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 =
                *alphabet.offset((val >> 12i32 & 0x3fi32 as libc::c_uint) as isize) as uint8_t;
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = *alphabet.offset((val >> 6i32 & 0x3fi32 as libc::c_uint) as isize) as uint8_t;
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 = *alphabet.offset((val & 0x3fi32 as libc::c_uint) as isize) as uint8_t;
            i = (i as libc::c_ulong).wrapping_add(3i32 as libc::c_ulong) as size_t as size_t
        }
        /* maybe 1 or 2 bytes left */
        if 0 != n.wrapping_sub(i) {
            /* we can read in[i + 1] even if it's past the end of the string because
             * we know (by construction) that it's a NUL byte */
            //#ifdef __GNUC__
            //        uint16_t val16;
            //        __builtin_memcpy(&val16, in + i, sizeof(val16));
            //        val = cbor_ntohs(val16);
            //#else
            val = ((*in_0.offset(i as isize) as libc::c_int) << 8i32
                | *in_0.offset(i.wrapping_add(1i32 as libc::c_ulong) as isize) as libc::c_int)
                as uint_least32_t;
            //#endif
            val <<= 8i32;
            /* the 65th character in the alphabet is our filler: either '=' or '\0' */
            *out.offset(4isize) = '\u{0}' as i32 as uint8_t;
            *out.offset(3isize) = *alphabet.offset(64isize) as uint8_t;
            if n.wrapping_sub(i) == 2i32 as libc::c_ulong {
                /* write the third char in 3 chars x 6 bits = 18 bits */
                *out.offset(2isize) =
                    *alphabet.offset((val >> 6i32 & 0x3fi32 as libc::c_uint) as isize) as uint8_t
            } else {
                *out.offset(2isize) = *alphabet.offset(64isize) as uint8_t
            }
            *out.offset(1isize) =
                *alphabet.offset((val >> 12i32 & 0x3fi32 as libc::c_uint) as isize) as uint8_t;
            *out.offset(0isize) =
                *alphabet.offset((val >> 18i32 & 0x3fi32 as libc::c_uint) as isize) as uint8_t
        } else {
            *out.offset(0isize) = '\u{0}' as i32 as uint8_t
        }
        return CborNoError;
    };
}
unsafe extern "C" fn dump_bytestring_base64url(
    mut result: *mut *mut libc::c_char,
    mut it: *mut CborValue,
) -> CborError_0 {
    static mut alphabet: [libc::c_char; 65] = 
    [
        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
        87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
        112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 45, 95, 0,
    ];
    return generic_dump_base64(result, it, alphabet.as_ptr());
}
unsafe extern "C" fn find_tagged_type(
    mut it: *mut CborValue,
    mut tag: *mut CborTag,
    mut type_0: *mut CborType_0,
) -> CborError_0 {
    let mut err: CborError_0 = CborNoError;
    *type_0 = cbor_value_get_type(it);
    while *type_0 as libc::c_uint == CborTagType as libc::c_int as libc::c_uint {
        /* can't fail */
        cbor_value_get_tag(it, tag);
        err = cbor_value_advance_fixed(it);
        if 0 != err as u64 {
            return err;
        } else {
            *type_0 = cbor_value_get_type(it)
        }
    }
    return err;
}
unsafe extern "C" fn add_value_metadata(
    mut out: *mut FILE,
    mut type_0: CborType_0,
    mut status: *const ConversionStatus,
) -> CborError_0 {
    let mut flags: libc::c_int = (*status).flags;
    if 0 != flags & TypeWasTagged as libc::c_int {
        /* extract the tagged type, which may be JSON native */
        type_0 = (flags & FinalTypeMask as libc::c_int) as CborType_0;
        flags &= !(FinalTypeMask as libc::c_int | TypeWasTagged as libc::c_int);
        if fprintf(
            out,
            b"\"tag\":\"%llu\"%s\x00" as *const u8 as *const libc::c_char,
            (*status).lastTag,
            if 0 != flags & !(TypeWasTagged as libc::c_int) {
                b",\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        ) < 0i32
        {
            return CborErrorIO;
        }
    }
    if 0 == flags {
        return CborNoError;
    } else if fprintf(
        out,
        b"\"t\":%d\x00" as *const u8 as *const libc::c_char,
        type_0 as libc::c_uint,
    ) < 0i32
    {
        return CborErrorIO;
    } else {
        if 0 != flags & NumberWasNaN as libc::c_int {
            if fprintf(
                out,
                b",\"v\":\"nan\"\x00" as *const u8 as *const libc::c_char,
            ) < 0i32
            {
                return CborErrorIO;
            }
        }
        if 0 != flags & NumberWasInfinite as libc::c_int {
            if fprintf(
                out,
                b",\"v\":\"%sinf\"\x00" as *const u8 as *const libc::c_char,
                if 0 != flags & NumberWasNegative as libc::c_int {
                    b"-\x00" as *const u8 as *const libc::c_char
                } else {
                    b"\x00" as *const u8 as *const libc::c_char
                },
            ) < 0i32
            {
                return CborErrorIO;
            }
        }
        if 0 != flags & NumberPrecisionWasLost as libc::c_int {
            if fprintf(
                out,
                b",\"v\":\"%c%llx\"\x00" as *const u8 as *const libc::c_char,
                if 0 != flags & NumberWasNegative as libc::c_int {
                    '-' as i32
                } else {
                    '+' as i32
                },
                (*status).originalNumber,
            ) < 0i32
            {
                return CborErrorIO;
            }
        }
        if type_0 as libc::c_uint == CborSimpleType as libc::c_int as libc::c_uint {
            if fprintf(
                out,
                b",\"v\":%d\x00" as *const u8 as *const libc::c_char,
                (*status).originalNumber as libc::c_int,
            ) < 0i32
            {
                return CborErrorIO;
            }
        }
        return CborNoError;
    };
}
unsafe extern "C" fn map_to_json(
    mut out: *mut FILE,
    mut it: *mut CborValue,
    mut flags: libc::c_int,
    mut status: *mut ConversionStatus,
) -> CborError_0 {
    let mut n: size_t = 0;
    let mut comma: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut err: CborError_0 = CborNoError;
    while !cbor_value_at_end(it) {
        let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
        if fprintf(out, b"%s\x00" as *const u8 as *const libc::c_char, comma) < 0i32 {
            return CborErrorIO;
        } else {
            comma = b",\x00" as *const u8 as *const libc::c_char;
            let mut keyType: CborType_0 = cbor_value_get_type(it);
            if 0 != (keyType as libc::c_uint == CborTextStringType as libc::c_int as libc::c_uint)
                as libc::c_int as libc::c_long
            {
                n = 0i32 as size_t;
                err = cbor_value_dup_text_string(it, &mut key, &mut n, it)
            } else if 0 != flags & CborConvertStringifyMapKeys as libc::c_int {
                err = stringify_map_key(&mut key, it, flags, keyType)
            } else {
                return CborErrorJsonObjectKeyNotString;
            }
            if 0 != err as u64 {
                return err;
            } else if fprintf(out, b"\"%s\":\x00" as *const u8 as *const libc::c_char, key) < 0i32 {
                free(key as *mut libc::c_void);
                return CborErrorIO;
            } else {
                /* then, print the value */
                let mut valueType: CborType_0 = cbor_value_get_type(it);
                err = value_to_json(out, it, flags, valueType, status);
                /* finally, print any metadata we may have */
                if 0 != flags & CborConvertAddMetadata as libc::c_int {
                    if 0 == err as u64
                        && keyType as libc::c_uint
                            != CborTextStringType as libc::c_int as libc::c_uint
                    {
                        if fprintf(
                            out,
                            b",\"%s$keycbordump\":true\x00" as *const u8 as *const libc::c_char,
                            key,
                        ) < 0i32
                        {
                            err = CborErrorIO
                        }
                    }
                    if 0 == err as u64 && 0 != (*status).flags {
                        if fprintf(
                            out,
                            b",\"%s$cbor\":{\x00" as *const u8 as *const libc::c_char,
                            key,
                        ) < 0i32
                            || add_value_metadata(out, valueType, status) as libc::c_int
                                != CborNoError as libc::c_int
                            || fputc('}' as i32, out) < 0i32
                        {
                            err = CborErrorIO
                        }
                    }
                }
                free(key as *mut libc::c_void);
                if !(0 != err as u64) {
                    continue;
                }
                return err;
            }
        }
    }
    return CborNoError;
}
unsafe extern "C" fn stringify_map_key(
    mut key: *mut *mut libc::c_char,
    mut it: *mut CborValue,
    mut flags: libc::c_int,
    mut type_0: CborType_0,
) -> CborError_0 {
    /* unused */
    /* unused */
    let mut size: size_t = 0;
    let mut memstream: *mut FILE = open_memstream(key, &mut size);
    if memstream.is_null() {
        /* could also be EMFILE, but it's unlikely */
        return CborErrorOutOfMemory;
    } else {
        let mut err: CborError_0 = cbor_value_to_pretty_advance(memstream, it);
        if 0 != (fclose(memstream) < 0i32 || (*key).is_null()) as libc::c_int as libc::c_long {
            return CborErrorInternalError;
        } else {
            return err;
        }
    };
}
unsafe extern "C" fn array_to_json(
    mut out: *mut FILE,
    mut it: *mut CborValue,
    mut flags: libc::c_int,
    mut status: *mut ConversionStatus,
) -> CborError_0 {
    let mut comma: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    while !cbor_value_at_end(it) {
        if fprintf(out, b"%s\x00" as *const u8 as *const libc::c_char, comma) < 0i32 {
            return CborErrorIO;
        } else {
            comma = b",\x00" as *const u8 as *const libc::c_char;
            let mut err: CborError_0 =
                value_to_json(out, it, flags, cbor_value_get_type(it), status);
            if !(0 != err as u64) {
                continue;
            }
            return err;
        }
    }
    return CborNoError;
}
