#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(libc, ptr_wrapping_offset_from)]
extern crate libc;
#[cfg(
    not(
        source_header = "/home/miguelsaldivar/workspace/C2Rust/dependencies/llvm-6.0.1/build.donna/lib/clang/6.0.1/include/stddef.h"
    )
)]
pub mod stddef_h {
    pub type ptrdiff_t = libc::c_long;
    pub type size_t = libc::c_ulong;
    use super::libc;
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/types.h"))]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __uint16_t = libc::c_ushort;
    pub type __uint32_t = libc::c_uint;
    pub type __uint64_t = libc::c_ulong;
    use super::libc;
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"))]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type uint32_t = __uint32_t;
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
}
#[cfg(not(source_header = "/home/miguelsaldivar/workspace/misc/tinycbor/src/cbor.h"))]
pub mod cbor_h {
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
    pub type CborTag = uint64_t;
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
    /* Parser API */
    pub type CborParserIteratorFlags = libc::c_uint;
    pub const CborIteratorFlag_ContainerIsMap: CborParserIteratorFlags = 32;
    pub const CborIteratorFlag_UnknownLength: CborParserIteratorFlags = 4;
    pub const CborIteratorFlag_IteratingStringChunks: CborParserIteratorFlags = 2;
    pub const CborIteratorFlag_NegativeInteger: CborParserIteratorFlags = 2;
    pub const CborIteratorFlag_IntegerValueTooLarge: CborParserIteratorFlags = 1;
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
    /* Validation API */
    pub type CborValidationFlags = libc::c_int;
    pub const CborValidateBasic: CborValidationFlags = 0;
    pub const CborValidateStrictest: CborValidationFlags = -1;
    pub const CborValidateCompleteData: CborValidationFlags = -2147483648;
    pub const CborValidateNoUnknownTags: CborValidationFlags = 1879048192;
    pub const CborValidateNoUnknownTagsSR: CborValidationFlags = 805306368;
    pub const CborValidateNoUnknownTagsSA: CborValidationFlags = 268435456;
    pub const CborValidateNoUnknownSimpleTypes: CborValidationFlags = 201326592;
    /* unused                               = 0x1000000, */
    /* unused                               = 0x2000000, */
    pub const CborValidateNoUnknownSimpleTypesSA: CborValidationFlags = 67108864;
    pub const CborValidateFiniteFloatingPoint: CborValidationFlags = 8388608;
    pub const CborValidateNoTags: CborValidationFlags = 4194304;
    pub const CborValidateNoUndefined: CborValidationFlags = 2097152;
    pub const CborValidateMapKeysAreString: CborValidationFlags = 1048576;
    pub const CborValidateStrictMode: CborValidationFlags = 1048320;
    pub const CborValidateUtf8: CborValidationFlags = 16384;
    pub const CborValidateTagUse: CborValidationFlags = 8192;
    pub const CborValidateMapKeysAreUnique: CborValidationFlags = 4864;
    pub const CborValidateCanonicalFormat: CborValidationFlags = 4095;
    pub const CborValidateMapIsSorted: CborValidationFlags = 768;
    pub const CborValidateNoIndeterminateLength: CborValidationFlags = 256;
    pub const CborValidateShortestNumbers: CborValidationFlags = 3;
    pub const CborValidateShortestFloatingPoint: CborValidationFlags = 2;
    /* Bit mapping:
     *  bits 0-7 (8 bits):      canonical format
     *  bits 8-11 (4 bits):     canonical format & strict mode
     *  bits 12-20 (8 bits):    strict mode
     *  bits 21-31 (10 bits):   other
     */
    pub const CborValidateShortestIntegrals: CborValidationFlags = 1;
    use super::libc;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    extern "C" {
        #[no_mangle]
        pub fn cbor_value_advance_fixed(it: *mut CborValue) -> CborError;
        #[no_mangle]
        pub fn cbor_value_enter_container(
            it: *const CborValue,
            recursed: *mut CborValue,
        ) -> CborError;
        #[no_mangle]
        pub fn cbor_value_leave_container(
            it: *mut CborValue,
            recursed: *const CborValue,
        ) -> CborError;
        #[no_mangle]
        pub fn _cbor_value_decode_int64_internal(value: *const CborValue) -> uint64_t;
        #[no_mangle]
        pub fn cbor_value_skip_tag(it: *mut CborValue) -> CborError;
        #[no_mangle]
        pub fn cbor_value_get_half_float(
            value: *const CborValue,
            result: *mut libc::c_void,
        ) -> CborError;
    }
}
#[cfg(not(source_header = "/home/miguelsaldivar/workspace/misc/tinycbor/src/cborinternal_p.h"))]
pub mod cborinternal_p_h {
    pub const Value8Bit: unnamed = 24;
    pub type unnamed = libc::c_int;
    pub const BreakByte: unnamed = 255;
    pub const MajorTypeMask: unnamed = -32;
    pub const MajorTypeShift: unnamed = 5;
    pub const IndefiniteLength: unnamed = 31;
    pub const Value64Bit: unnamed = 27;
    pub const Value32Bit: unnamed = 26;
    pub const Value16Bit: unnamed = 25;
    /* 31 */
    pub const SmallValueMask: unnamed = 31;
    pub const SmallValueBitLength: unnamed = 5;
    use super::cbor_h::{CborError, CborValue};
    use super::libc;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::{uint64_t, uint8_t};
    extern "C" {
        #[no_mangle]
        pub fn _cbor_value_get_string_chunk(
            value: *const CborValue,
            bufferptr: *mut *const libc::c_void,
            len: *mut size_t,
            next: *mut CborValue,
        ) -> CborError;
        #[no_mangle]
        pub fn _cbor_value_extract_number(
            ptr: *mut *const uint8_t,
            end: *const uint8_t,
            len: *mut uint64_t,
        ) -> CborError;
        #[no_mangle]
        pub fn _cbor_value_prepare_string_iteration(it: *mut CborValue) -> CborError;
    }
}
#[cfg(not(source_header = "/usr/include/string.h"))]
pub mod string_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memcmp(
            _: *const libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> libc::c_int;
    }
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"))]
pub mod mathcalls_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        pub fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    }
}
#[cfg(not(source_header = "/usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h"))]
pub mod mathcalls_helper_functions_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        pub fn __fpclassifyl(__value: libc::c_double) -> libc::c_int;
        #[no_mangle]
        pub fn __fpclassify(__value: libc::c_double) -> libc::c_int;
        #[no_mangle]
        pub fn __fpclassifyf(__value: libc::c_float) -> libc::c_int;
    }
}
#[cfg(not(source_header = "/home/miguelsaldivar/workspace/misc/tinycbor/src/utf8_p.h"))]
pub mod utf8_p_h {}
use self::cbor_h::{
    CborArrayType, CborBooleanType, CborByteStringType, CborDoubleType, CborError,
    CborErrorAdvancePastEOF, CborErrorDataTooLarge, CborErrorDuplicateObjectKeys,
    CborErrorExcludedType, CborErrorExcludedValue, CborErrorGarbageAtEnd, CborErrorIO,
    CborErrorIllegalNumber, CborErrorIllegalSimpleType, CborErrorIllegalType,
    CborErrorImproperValue, CborErrorInappropriateTagForType, CborErrorInternalError,
    CborErrorInvalidUtf8TextString, CborErrorJsonNotImplemented, CborErrorJsonObjectKeyIsAggregate,
    CborErrorJsonObjectKeyNotString, CborErrorMapKeyNotString, CborErrorMapKeysNotUnique,
    CborErrorMapNotSorted, CborErrorNestingTooDeep, CborErrorOutOfMemory,
    CborErrorOverlongEncoding, CborErrorTooFewItems, CborErrorTooManyItems,
    CborErrorUnexpectedBreak, CborErrorUnexpectedEOF, CborErrorUnknownLength,
    CborErrorUnknownSimpleType, CborErrorUnknownTag, CborErrorUnknownType,
    CborErrorUnsupportedType, CborFloatType, CborHalfFloatType, CborIntegerType, CborInvalidType,
    CborIteratorFlag_ContainerIsMap, CborIteratorFlag_IntegerValueTooLarge,
    CborIteratorFlag_IteratingStringChunks, CborIteratorFlag_NegativeInteger,
    CborIteratorFlag_UnknownLength, CborMapType, CborNoError, CborNullType, CborParser,
    CborParserIteratorFlags, CborSimpleType, CborTag, CborTagType, CborTextStringType, CborType,
    CborUndefinedType, CborUnknownError, CborValidateBasic, CborValidateCanonicalFormat,
    CborValidateCompleteData, CborValidateFiniteFloatingPoint, CborValidateMapIsSorted,
    CborValidateMapKeysAreString, CborValidateMapKeysAreUnique, CborValidateNoIndeterminateLength,
    CborValidateNoTags, CborValidateNoUndefined, CborValidateNoUnknownSimpleTypes,
    CborValidateNoUnknownSimpleTypesSA, CborValidateNoUnknownTags, CborValidateNoUnknownTagsSA,
    CborValidateNoUnknownTagsSR, CborValidateShortestFloatingPoint, CborValidateShortestIntegrals,
    CborValidateShortestNumbers, CborValidateStrictMode, CborValidateStrictest, CborValidateTagUse,
    CborValidateUtf8, CborValidationFlags, CborValue, _cbor_value_decode_int64_internal,
    cbor_value_advance_fixed, cbor_value_enter_container, cbor_value_get_half_float,
    cbor_value_leave_container, cbor_value_skip_tag,
};
use self::cborinternal_p_h::{
    unnamed, BreakByte, IndefiniteLength, MajorTypeMask, MajorTypeShift, SmallValueBitLength,
    SmallValueMask, Value16Bit, Value32Bit, Value64Bit, Value8Bit, _cbor_value_extract_number,
    _cbor_value_get_string_chunk, _cbor_value_prepare_string_iteration,
};
use self::mathcalls_h::ldexp;
use self::mathcalls_helper_functions_h::{__fpclassify, __fpclassifyf, __fpclassifyl};
use self::stddef_h::{ptrdiff_t, size_t};
use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::string_h::{memcmp, memcpy};
use self::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
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
/* *
 * \addtogroup CborParsing
 * @{
 */
/* *
 * \enum CborValidationFlags
 * The CborValidationFlags enum contains flags that control the validation of a
 * CBOR stream.
 *
 * \value CborValidateBasic         Validates only the syntactic correctedness of the stream.
 * \value CborValidateCanonical     Validates that the stream is in canonical format, according to
 *                                  RFC 7049 section 3.9.
 * \value CborValidateStrictMode    Performs strict validation, according to RFC 7049 section 3.10.
 * \value CborValidateStrictest     Attempt to perform the strictest validation we know of.
 *
 * \value CborValidateShortestIntegrals     (Canonical) Validate that integral numbers and lengths are
 *                                          enconded in their shortest form possible.
 * \value CborValidateShortestFloatingPoint (Canonical) Validate that floating-point numbers are encoded
 *                                          in their shortest form possible.
 * \value CborValidateShortestNumbers       (Canonical) Validate both integral and floating-point numbers
 *                                          are in their shortest form possible.
 * \value CborValidateNoIndeterminateLength (Canonical) Validate that no string, array or map uses
 *                                          indeterminate length encoding.
 * \value CborValidateMapIsSorted           (Canonical & Strict mode) Validate that map keys appear in
 *                                          sorted order.
 * \value CborValidateMapKeysAreUnique      (Strict mode) Validate that map keys are unique.
 * \value CborValidateTagUse                (Strict mode) Validate that known tags are used with the
 *                                          correct types. This does not validate that the content of
 *                                          those types is syntactically correct. For example, this
 *                                          option validates that tag 1 (DateTimeString) is used with
 *                                          a Text String, but it does not validate that the string is
 *                                          a valid date/time representation.
 * \value CborValidateUtf8                  (Strict mode) Validate that text strings are appropriately
 *                                          encoded in UTF-8.
 * \value CborValidateMapKeysAreString      Validate that all map keys are text strings.
 * \value CborValidateNoUndefined           Validate that no elements of type "undefined" are present.
 * \value CborValidateNoTags                Validate that no tags are used.
 * \value CborValidateFiniteFloatingPoint   Validate that all floating point numbers are finite (no NaN or
 *                                          infinities are allowed).
 * \value CborValidateCompleteData          Validate that the stream is complete and there is no more data
 *                                          in the buffer.
 * \value CborValidateNoUnknownSimpleTypesSA Validate that all Standards Action simple types are registered
 *                                          with IANA.
 * \value CborValidateNoUnknownSimpleTypes  Validate that all simple types used are registered with IANA.
 * \value CborValidateNoUnknownTagsSA       Validate that all Standard Actions tags are registered with IANA.
 * \value CborValidateNoUnknownTagsSR       Validate that all Standard Actions and Specification Required tags
 *                                          are registered with IANA (see below for limitations).
 * \value CborValidateNoUnkonwnTags         Validate that all tags are registered with IANA
 *                                          (see below for limitations).
 *
 * \par Simple type registry
 * The CBOR specification requires that registration for use of the first 19
 * simple types must be done by way of Standards Action. The rest of the simple
 * types only require a specification. The official list can be obtained from
 *  https://www.iana.org/assignments/cbor-simple-values/cbor-simple-values.xhtml.
 *
 * \par
 * There are no registered simple types recognized by this release of TinyCBOR
 * (beyond those defined by RFC 7049).
 *
 * \par Tag registry
 * The CBOR specification requires that registration for use of the first 23
 * tags must be done by way of Standards Action. The next up to tag 255 only
 * require a specification. Finally, all other tags can be registered on a
 * first-come-first-serve basis. The official list can be ontained from
 *  https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml.
 *
 * \par
 * Given the variability of this list, TinyCBOR cannot recognize all tags
 * registered with IANA. Instead, the implementation only recognizes tags
 * that are backed by an RFC.
 *
 * \par
 * These are the tags known to the current TinyCBOR release:
<table>
  <tr>
    <th>Tag</th>
    <th>Data Item</th>
    <th>Semantics</th>
  </tr>
  <tr>
    <td>0</td>
    <td>UTF-8 text string</td>
    <td>Standard date/time string</td>
  </td>
  <tr>
    <td>1</td>
    <td>integer</td>
    <td>Epoch-based date/time</td>
  </td>
  <tr>
    <td>2</td>
    <td>byte string</td>
    <td>Positive bignum</td>
  </td>
  <tr>
    <td>3</td>
    <td>byte string</td>
    <td>Negative bignum</td>
  </td>
  <tr>
    <td>4</td>
    <td>array</td>
    <td>Decimal fraction</td>
  </td>
  <tr>
    <td>5</td>
    <td>array</td>
    <td>Bigfloat</td>
  </td>
  <tr>
    <td>16</td>
    <td>array</td>
    <td>COSE Single Recipient Encrypted Data Object (RFC 8152)</td>
  </td>
  <tr>
    <td>17</td>
    <td>array</td>
    <td>COSE Mac w/o Recipients Object (RFC 8152)</td>
  </td>
  <tr>
    <td>18</td>
    <td>array</td>
    <td>COSE Single Signer Data Object (RFC 8162)</td>
  </td>
  <tr>
    <td>21</td>
    <td>byte string, array, map</td>
    <td>Expected conversion to base64url encoding</td>
  </td>
  <tr>
    <td>22</td>
    <td>byte string, array, map</td>
    <td>Expected conversion to base64 encoding</td>
  </td>
  <tr>
    <td>23</td>
    <td>byte string, array, map</td>
    <td>Expected conversion to base16 encoding</td>
  </td>
  <tr>
    <td>24</td>
    <td>byte string</td>
    <td>Encoded CBOR data item</td>
  </td>
  <tr>
    <td>32</td>
    <td>UTF-8 text string</td>
    <td>URI</td>
  </td>
  <tr>
    <td>33</td>
    <td>UTF-8 text string</td>
    <td>base64url</td>
  </td>
  <tr>
    <td>34</td>
    <td>UTF-8 text string</td>
    <td>base64</td>
  </td>
  <tr>
    <td>35</td>
    <td>UTF-8 text string</td>
    <td>Regular expression</td>
  </td>
  <tr>
    <td>36</td>
    <td>UTF-8 text string</td>
    <td>MIME message</td>
  </td>
  <tr>
    <td>96</td>
    <td>array</td>
    <td>COSE Encrypted Data Object (RFC 8152)</td>
  </td>
  <tr>
    <td>97</td>
    <td>array</td>
    <td>COSE MACed Data Object (RFC 8152)</td>
  </td>
  <tr>
    <td>98</td>
    <td>array</td>
    <td>COSE Signed Data Object (RFC 8152)</td>
  </td>
  <tr>
    <td>55799</td>
    <td>any</td>
    <td>Self-describe CBOR</td>
  </td>
</table>
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KnownTagData {
    pub tag: uint32_t,
    pub types: uint32_t,
}
unsafe extern "C" fn cbor_value_at_end(mut it: *const CborValue) -> bool {
    return (*it).remaining == 0i32 as libc::c_uint;
}
unsafe extern "C" fn cbor_value_get_next_byte(mut it: *const CborValue) -> *const uint8_t {
    return (*it).ptr;
}
unsafe extern "C" fn _cbor_value_extract_int64_helper(mut value: *const CborValue) -> uint64_t {
    return if 0
        != (*value).flags as libc::c_int & CborIteratorFlag_IntegerValueTooLarge as libc::c_int
    {
        _cbor_value_decode_int64_internal(value)
    } else {
        (*value).extra as libc::c_ulong
    };
}
unsafe extern "C" fn cbor_value_get_type(mut value: *const CborValue) -> CborType {
    return (*value).type_0 as CborType;
}
unsafe extern "C" fn cbor_value_get_simple_type(
    mut value: *const CborValue,
    mut result: *mut uint8_t,
) -> CborError {
    *result = (*value).extra as uint8_t;
    return CborNoError;
}
unsafe extern "C" fn cbor_value_get_raw_integer(
    mut value: *const CborValue,
    mut result: *mut uint64_t,
) -> CborError {
    *result = _cbor_value_extract_int64_helper(value);
    return CborNoError;
}
unsafe extern "C" fn cbor_value_is_length_known(mut value: *const CborValue) -> bool {
    return (*value).flags as libc::c_int & CborIteratorFlag_UnknownLength as libc::c_int == 0i32;
}
unsafe extern "C" fn cbor_value_get_tag(
    mut value: *const CborValue,
    mut result: *mut CborTag,
) -> CborError {
    *result = _cbor_value_extract_int64_helper(value);
    return CborNoError;
}
unsafe extern "C" fn cbor_value_get_float(
    mut value: *const CborValue,
    mut result: *mut libc::c_float,
) -> CborError {
    let mut data: uint32_t = 0;
    data = _cbor_value_decode_int64_internal(value) as uint32_t;
    memcpy(
        result as *mut libc::c_void,
        &mut data as *mut uint32_t as *const libc::c_void,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    );
    return CborNoError;
}
unsafe extern "C" fn cbor_value_get_double(
    mut value: *const CborValue,
    mut result: *mut libc::c_double,
) -> CborError {
    let mut data: uint64_t = 0;
    data = _cbor_value_decode_int64_internal(value);
    memcpy(
        result as *mut libc::c_void,
        &mut data as *mut uint64_t as *const libc::c_void,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    return CborNoError;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_validate(
    mut it: *const CborValue,
    mut flags: uint32_t,
) -> CborError {
    let mut value: CborValue = *it;
    let mut err: CborError = validate_value(&mut value, flags, 1024i32);
    if 0 != err as u64 {
        return err;
    } else if 0 != flags & CborValidateCompleteData as libc::c_int as libc::c_uint
        && (*it).ptr != (*(*it).parser).end
    {
        return CborErrorGarbageAtEnd;
    } else {
        return CborNoError;
    };
}
unsafe extern "C" fn validate_value(
    mut it: *mut CborValue,
    mut flags: uint32_t,
    mut recursionLeft: libc::c_int,
) -> CborError {
    let mut err: CborError = CborNoError;
    let mut type_0: CborType = cbor_value_get_type(it);
    if cbor_value_is_length_known(it) {
        err = validate_number(it, type_0, flags);
        if 0 != err as u64 {
            return err;
        }
    } else if 0 != flags & CborValidateNoIndeterminateLength as libc::c_int as libc::c_uint {
        return CborErrorUnknownLength;
    }
    match type_0 as libc::c_uint {
        128 | 160 => {
            /* recursive type */
            let mut recursed: CborValue = CborValue {
                parser: 0 as *const CborParser,
                ptr: 0 as *const uint8_t,
                remaining: 0,
                extra: 0,
                type_0: 0,
                flags: 0,
            };
            err = cbor_value_enter_container(it, &mut recursed);
            if 0 == err as u64 {
                err = validate_container(
                    &mut recursed,
                    type_0 as libc::c_int,
                    flags,
                    recursionLeft - 1i32,
                )
            }
            if 0 != err as u64 {
                (*it).ptr = recursed.ptr;
                return err;
            } else {
                err = cbor_value_leave_container(it, &mut recursed);
                if 0 != err as u64 {
                    return err;
                } else {
                    return CborNoError;
                }
            }
        }
        0 => {
            let mut val: uint64_t = 0;
            err = cbor_value_get_raw_integer(it, &mut val)
        }
        64 | 96 => {
            /* can't fail */
            let mut n: size_t = 0i32 as size_t;
            let mut ptr: *const libc::c_void = 0 as *const libc::c_void;
            err = _cbor_value_prepare_string_iteration(it);
            if 0 != err as u64 {
                return err;
            } else {
                loop {
                    err = validate_number(it, type_0, flags);
                    if 0 != err as u64 {
                        return err;
                    } else {
                        err = _cbor_value_get_string_chunk(it, &mut ptr, &mut n, it);
                        if 0 != err as u64 {
                            return err;
                        } else {
                            if ptr.is_null() {
                                break;
                            }
                            if !(type_0 as libc::c_uint
                                == CborTextStringType as libc::c_int as libc::c_uint
                                && 0 != flags & CborValidateUtf8 as libc::c_int as libc::c_uint)
                            {
                                continue;
                            }
                            err = validate_utf8_string(ptr, n);
                            if !(0 != err as u64) {
                                continue;
                            }
                            return err;
                        }
                    }
                }
                return CborNoError;
            }
        }
        192 => {
            let mut tag: CborTag = 0;
            err = cbor_value_get_tag(it, &mut tag);
            /* can't fail */
            err = cbor_value_advance_fixed(it);
            if 0 != err as u64 {
                return err;
            } else {
                err = validate_tag(it, tag, flags, recursionLeft - 1i32);
                if 0 != err as u64 {
                    return err;
                } else {
                    return CborNoError;
                }
            }
        }
        224 => {
            let mut simple_type: uint8_t = 0;
            err = cbor_value_get_simple_type(it, &mut simple_type);
            /* can't fail */
            err = validate_simple_type(simple_type, flags);
            if 0 != err as u64 {
                return err;
            }
        }
        247 => {
            if 0 != flags & CborValidateNoUndefined as libc::c_int as libc::c_uint {
                return CborErrorExcludedType;
            }
        }
        249 | 250 | 251 => {
            err = validate_floating_point(it, type_0, flags);
            if 0 != err as u64 {
                return err;
            }
        }
        255 => return CborErrorUnknownType,
        246 | 245 | _ => {}
    }
    err = cbor_value_advance_fixed(it);
    return err;
}
unsafe extern "C" fn validate_floating_point(
    mut it: *mut CborValue,
    mut type_0: CborType,
    mut flags: uint32_t,
) -> CborError {
    let mut err: CborError = CborNoError;
    let mut r: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    let mut valf: libc::c_float = 0.;
    let mut valf16: uint16_t = 0;
    if type_0 as libc::c_uint != CborDoubleType as libc::c_int as libc::c_uint {
        if type_0 as libc::c_uint == CborFloatType as libc::c_int as libc::c_uint {
            err = cbor_value_get_float(it, &mut valf);
            val = valf as libc::c_double
        } else {
            err = cbor_value_get_half_float(it, &mut valf16 as *mut uint16_t as *mut libc::c_void);
            val = decode_half(valf16)
        }
    } else {
        err = cbor_value_get_double(it, &mut val)
    }
    /* can't fail */
    r = if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        __fpclassifyf(val as libc::c_float)
    } else if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        __fpclassify(val)
    } else {
        __fpclassifyl(val as libc::c_double)
    };
    if r == 0i32 || r == 1i32 {
        if 0 != flags & CborValidateFiniteFloatingPoint as libc::c_int as libc::c_uint {
            return CborErrorExcludedValue;
        } else if 0 != flags & CborValidateShortestFloatingPoint as libc::c_int as libc::c_uint {
            if type_0 as libc::c_uint == CborDoubleType as libc::c_int as libc::c_uint {
                return CborErrorOverlongEncoding;
            } else if type_0 as libc::c_uint == CborFloatType as libc::c_int as libc::c_uint {
                return CborErrorOverlongEncoding;
            } else if r == 0i32 && valf16 as libc::c_int != 0x7e00i32 {
                return CborErrorImproperValue;
            } else if r == 1i32
                && valf16 as libc::c_int != 0x7c00i32
                && valf16 as libc::c_int != 0xfc00i32
            {
                return CborErrorImproperValue;
            }
        }
    }
    if 0 != flags & CborValidateShortestFloatingPoint as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint > CborHalfFloatType as libc::c_int as libc::c_uint
    {
        if type_0 as libc::c_uint == CborDoubleType as libc::c_int as libc::c_uint {
            valf = val as libc::c_float;
            if valf as libc::c_double == val {
                return CborErrorOverlongEncoding;
            }
        }
        if type_0 as libc::c_uint == CborFloatType as libc::c_int as libc::c_uint {
            valf16 = encode_half(valf as libc::c_double);
            if valf as libc::c_double == decode_half(valf16) {
                return CborErrorOverlongEncoding;
            }
        }
    }
    return CborNoError;
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
/* software implementation of float-to-fp16 conversions */
unsafe extern "C" fn encode_half(mut val: libc::c_double) -> libc::c_ushort {
    let mut v: uint64_t = 0;
    let mut sign: libc::c_int = 0;
    let mut exp: libc::c_int = 0;
    let mut mant: libc::c_int = 0;
    memcpy(
        &mut v as *mut uint64_t as *mut libc::c_void,
        &mut val as *mut libc::c_double as *const libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
    sign = (v >> 63i32 << 15i32) as libc::c_int;
    exp = (v >> 52i32 & 0x7ffi32 as libc::c_ulong) as libc::c_int;
    /* keep only the 11 most significant bits of the mantissa */
    mant = (v << 12i32 >> 12i32 >> 53i32 - 11i32) as libc::c_int;
    exp -= 1023i32;
    if exp == 1024i32 {
        /* infinity or NaN */
        exp = 16i32;
        mant >>= 1i32
    } else if exp >= 16i32 {
        /* overflow, as largest number */
        exp = 15i32;
        mant = 1023i32
    } else if !(exp >= -14i32) {
        /* regular normal */
        if exp >= -24i32 {
            /* subnormal */
            mant |= 1024i32;
            mant >>= -(exp + 14i32);
            exp = -15i32
        } else {
            return 0i32 as libc::c_ushort;
        }
    }
    /* safe cast here as bit operations above guarantee not to overflow */
    return (sign | exp + 15i32 << 10i32 | mant) as libc::c_ushort;
}
unsafe extern "C" fn validate_simple_type(
    mut simple_type: uint8_t,
    mut flags: uint32_t,
) -> CborError {
    /* At current time, all known simple types are those from RFC 7049,
     * which are parsed by the parser into different CBOR types.
     * That means that if we've got here, the type is unknown */
    if (simple_type as libc::c_int) < 32i32 {
        return (if 0 != flags & CborValidateNoUnknownSimpleTypesSA as libc::c_int as libc::c_uint {
            CborErrorUnknownSimpleType as libc::c_int
        } else {
            CborNoError as libc::c_int
        }) as CborError;
    } else {
        return (if flags & CborValidateNoUnknownSimpleTypes as libc::c_int as libc::c_uint
            == CborValidateNoUnknownSimpleTypes as libc::c_int as libc::c_uint
        {
            CborErrorUnknownSimpleType as libc::c_int
        } else {
            CborNoError as libc::c_int
        }) as CborError;
    };
}
unsafe extern "C" fn validate_tag(
    mut it: *mut CborValue,
    mut tag: CborTag,
    mut flags: uint32_t,
    mut recursionLeft: libc::c_int,
) -> CborError {
    let mut type_0: CborType = cbor_value_get_type(it);
    let knownTagCount: size_t = (::std::mem::size_of::<[KnownTagData; 22]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<KnownTagData>() as libc::c_ulong);
    let mut tagData: *const KnownTagData = knownTagData.as_ptr();
    let knownTagDataEnd: *const KnownTagData = knownTagData.as_ptr().offset(knownTagCount as isize);
    if 0 == recursionLeft {
        return CborErrorNestingTooDeep;
    } else if 0 != flags & CborValidateNoTags as libc::c_int as libc::c_uint {
        return CborErrorExcludedType;
    } else {
        /* find the tag data, if any */
        while tagData != knownTagDataEnd {
            if ((*tagData).tag as libc::c_ulong) < tag {
                tagData = tagData.offset(1isize)
            } else {
                if !((*tagData).tag as libc::c_ulong > tag) {
                    break;
                }
                tagData = 0 as *const KnownTagData;
                break;
            }
        }
        if tagData == knownTagDataEnd {
            tagData = 0 as *const KnownTagData
        }
        if 0 != flags & CborValidateNoUnknownTags as libc::c_int as libc::c_uint
            && tagData.is_null()
        {
            /* tag not found */
            if 0 != flags & CborValidateNoUnknownTagsSA as libc::c_int as libc::c_uint
                && tag < 24i32 as libc::c_ulong
            {
                return CborErrorUnknownTag;
            } else if flags & CborValidateNoUnknownTagsSR as libc::c_int as libc::c_uint
                == CborValidateNoUnknownTagsSR as libc::c_int as libc::c_uint
                && tag < 256i32 as libc::c_ulong
            {
                return CborErrorUnknownTag;
            } else if flags & CborValidateNoUnknownTags as libc::c_int as libc::c_uint
                == CborValidateNoUnknownTags as libc::c_int as libc::c_uint
            {
                return CborErrorUnknownTag;
            }
        }
        if 0 != flags & CborValidateTagUse as libc::c_int as libc::c_uint
            && !tagData.is_null()
            && 0 != (*tagData).types
        {
            let mut allowedTypes: uint32_t = (*tagData).types;
            /* correct Integer so it's not zero */
            if type_0 as libc::c_uint == CborIntegerType as libc::c_int as libc::c_uint {
                type_0 = (type_0 as libc::c_uint).wrapping_add(1i32 as libc::c_uint) as CborType
            }
            while 0 != allowedTypes {
                if (allowedTypes & 0xffi32 as libc::c_uint) as uint8_t as libc::c_uint
                    == type_0 as libc::c_uint
                {
                    break;
                }
                allowedTypes >>= 8i32
            }
            if 0 == allowedTypes {
                return CborErrorInappropriateTagForType;
            }
        }
        return validate_value(it, flags, recursionLeft);
    };
}
static mut knownTagData: [KnownTagData; 22] = unsafe {
    [
        KnownTagData {
            tag: 0i32 as uint32_t,
            types: CborTextStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 1i32 as uint32_t,
            types: (CborIntegerType as libc::c_int + 1i32) as uint32_t,
        },
        KnownTagData {
            tag: 2i32 as uint32_t,
            types: CborByteStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 3i32 as uint32_t,
            types: CborByteStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 4i32 as uint32_t,
            types: CborArrayType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 5i32 as uint32_t,
            types: CborArrayType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 16i32 as uint32_t,
            types: CborArrayType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 17i32 as uint32_t,
            types: CborArrayType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 18i32 as uint32_t,
            types: CborArrayType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 21i32 as uint32_t,
            types: CborByteStringType as libc::c_int as uint32_t
                | (CborArrayType as libc::c_int as uint32_t) << 8i32
                | (CborMapType as libc::c_int as uint32_t) << 16i32,
        },
        KnownTagData {
            tag: 22i32 as uint32_t,
            types: CborByteStringType as libc::c_int as uint32_t
                | (CborArrayType as libc::c_int as uint32_t) << 8i32
                | (CborMapType as libc::c_int as uint32_t) << 16i32,
        },
        KnownTagData {
            tag: 23i32 as uint32_t,
            types: CborByteStringType as libc::c_int as uint32_t
                | (CborArrayType as libc::c_int as uint32_t) << 8i32
                | (CborMapType as libc::c_int as uint32_t) << 16i32,
        },
        KnownTagData {
            tag: 24i32 as uint32_t,
            types: CborByteStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 32i32 as uint32_t,
            types: CborTextStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 33i32 as uint32_t,
            types: CborTextStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 34i32 as uint32_t,
            types: CborTextStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 35i32 as uint32_t,
            types: CborTextStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 36i32 as uint32_t,
            types: CborTextStringType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 96i32 as uint32_t,
            types: CborArrayType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 97i32 as uint32_t,
            types: CborArrayType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 98i32 as uint32_t,
            types: CborArrayType as libc::c_int as uint32_t,
        },
        KnownTagData {
            tag: 55799i32 as uint32_t,
            types: 0u32,
        },
    ]
};
unsafe extern "C" fn validate_utf8_string(
    mut ptr: *const libc::c_void,
    mut n: size_t,
) -> CborError {
    let mut buffer: *const uint8_t = ptr as *const uint8_t;
    let end: *const uint8_t = buffer.offset(n as isize);
    while buffer < end {
        let mut uc: uint32_t = get_utf8(&mut buffer, end);
        if !(uc == !0u32) {
            continue;
        }
        return CborErrorInvalidUtf8TextString;
    }
    return CborNoError;
}
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
unsafe extern "C" fn get_utf8(
    mut buffer: *mut *const uint8_t,
    mut end: *const uint8_t,
) -> uint32_t {
    let mut charsNeeded: libc::c_int = 0;
    let mut uc: uint32_t = 0;
    let mut min_uc: uint32_t = 0;
    let mut b: uint8_t = 0;
    let mut n: ptrdiff_t = end.wrapping_offset_from(*buffer) as libc::c_long;
    if n == 0i32 as libc::c_long {
        return !0u32;
    } else {
        let fresh0 = *buffer;
        *buffer = (*buffer).offset(1);
        uc = *fresh0 as uint32_t;
        if uc < 0x80i32 as libc::c_uint {
            /* single-byte UTF-8 */
            return uc;
        } else if 0 != (uc <= 0xc1i32 as libc::c_uint) as libc::c_int as libc::c_long {
            return !0u32;
        } else {
            if uc < 0xe0i32 as libc::c_uint {
                /* two-byte UTF-8 */
                charsNeeded = 2i32;
                min_uc = 0x80i32 as uint32_t;
                uc &= 0x1fi32 as libc::c_uint
            } else if uc < 0xf0i32 as libc::c_uint {
                /* three-byte UTF-8 */
                charsNeeded = 3i32;
                min_uc = 0x800i32 as uint32_t;
                uc &= 0xfi32 as libc::c_uint
            } else if uc < 0xf5i32 as libc::c_uint {
                /* four-byte UTF-8 */
                charsNeeded = 4i32;
                min_uc = 0x10000i32 as uint32_t;
                uc &= 0x7i32 as libc::c_uint
            } else {
                return !0u32;
            }
            if n < charsNeeded as libc::c_long {
                return !0u32;
            } else {
                /* first continuation character */
                let fresh1 = *buffer;
                *buffer = (*buffer).offset(1);
                b = *fresh1;
                if b as libc::c_int & 0xc0i32 != 0x80i32 {
                    return !0u32;
                } else {
                    uc <<= 6i32;
                    uc |= (b as libc::c_int & 0x3fi32) as libc::c_uint;
                    if charsNeeded > 2i32 {
                        /* second continuation character */
                        let fresh2 = *buffer;
                        *buffer = (*buffer).offset(1);
                        b = *fresh2;
                        if b as libc::c_int & 0xc0i32 != 0x80i32 {
                            return !0u32;
                        } else {
                            uc <<= 6i32;
                            uc |= (b as libc::c_int & 0x3fi32) as libc::c_uint;
                            if charsNeeded > 3i32 {
                                /* third continuation character */
                                let fresh3 = *buffer;
                                *buffer = (*buffer).offset(1);
                                b = *fresh3;
                                if b as libc::c_int & 0xc0i32 != 0x80i32 {
                                    return !0u32;
                                } else {
                                    uc <<= 6i32;
                                    uc |= (b as libc::c_int & 0x3fi32) as libc::c_uint
                                }
                            }
                        }
                    }
                    /* overlong sequence? surrogate pair? out or range? */
                    if uc < min_uc
                        || uc.wrapping_sub(0xd800u32) < 2048u32
                        || uc > 0x10ffffi32 as libc::c_uint
                    {
                        return !0u32;
                    } else {
                        return uc;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn validate_number(
    mut it: *const CborValue,
    mut type_0: CborType,
    mut flags: uint32_t,
) -> CborError {
    let mut err: CborError = CborNoError;
    let mut ptr: *const uint8_t = (*it).ptr;
    let mut bytesUsed: size_t = 0;
    let mut bytesNeeded: size_t = 0;
    let mut value: uint64_t = 0;
    if flags & CborValidateShortestIntegrals as libc::c_int as libc::c_uint == 0i32 as libc::c_uint
    {
        return err;
    } else if type_0 as libc::c_uint >= CborHalfFloatType as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint <= CborDoubleType as libc::c_int as libc::c_uint
    {
        /* checked elsewhere */
        return err;
    } else {
        err = _cbor_value_extract_number(&mut ptr, (*(*it).parser).end, &mut value);
        if 0 != err as u64 {
            return err;
        } else {
            bytesUsed = (ptr.wrapping_offset_from((*it).ptr) as libc::c_long - 1i32 as libc::c_long)
                as size_t;
            bytesNeeded = 0i32 as size_t;
            if value >= Value8Bit as libc::c_int as libc::c_ulong {
                bytesNeeded = bytesNeeded.wrapping_add(1)
            }
            if value > 0xffu32 as libc::c_ulong {
                bytesNeeded = bytesNeeded.wrapping_add(1)
            }
            if value > 0xffffu32 as libc::c_ulong {
                bytesNeeded = (bytesNeeded as libc::c_ulong).wrapping_add(2i32 as libc::c_ulong)
                    as size_t as size_t
            }
            if value > 0xffffffffu32 as libc::c_ulong {
                bytesNeeded = (bytesNeeded as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong)
                    as size_t as size_t
            }
            if bytesNeeded < bytesUsed {
                return CborErrorOverlongEncoding;
            } else {
                return CborNoError;
            }
        }
    };
}
unsafe extern "C" fn validate_container(
    mut it: *mut CborValue,
    mut containerType: libc::c_int,
    mut flags: uint32_t,
    mut recursionLeft: libc::c_int,
) -> CborError {
    let mut err: CborError = CborNoError;
    let mut previous: *const uint8_t = 0 as *const uint8_t;
    let mut previous_end: *const uint8_t = 0 as *const uint8_t;
    if 0 == recursionLeft {
        return CborErrorNestingTooDeep;
    } else {
        while !cbor_value_at_end(it) {
            let mut current: *const uint8_t = cbor_value_get_next_byte(it);
            if containerType == CborMapType as libc::c_int {
                if 0 != flags & CborValidateMapKeysAreString as libc::c_int as libc::c_uint {
                    let mut type_0: CborType = cbor_value_get_type(it);
                    if type_0 as libc::c_uint == CborTagType as libc::c_int as libc::c_uint {
                        /* skip the tags */
                        let mut copy: CborValue = *it;
                        err = cbor_value_skip_tag(&mut copy);
                        if 0 != err as u64 {
                            return err;
                        } else {
                            type_0 = cbor_value_get_type(&mut copy)
                        }
                    }
                    if type_0 as libc::c_uint != CborTextStringType as libc::c_int as libc::c_uint {
                        return CborErrorMapKeyNotString;
                    }
                }
            }
            err = validate_value(it, flags, recursionLeft);
            if 0 != err as u64 {
                return err;
            } else {
                if containerType != CborMapType as libc::c_int {
                    continue;
                }
                if 0 != flags & CborValidateMapIsSorted as libc::c_int as libc::c_uint {
                    if !previous.is_null() {
                        let mut len1: uint64_t = 0;
                        let mut len2: uint64_t = 0;
                        let mut ptr: *const uint8_t = 0 as *const uint8_t;
                        /* extract the two lengths */
                        ptr = previous;
                        _cbor_value_extract_number(&mut ptr, (*(*it).parser).end, &mut len1);
                        ptr = current;
                        _cbor_value_extract_number(&mut ptr, (*(*it).parser).end, &mut len2);
                        if len1 > len2 {
                            return CborErrorMapNotSorted;
                        } else if len1 == len2 {
                            let mut bytelen1: size_t = previous_end.wrapping_offset_from(previous)
                                as libc::c_long
                                as size_t;
                            let mut bytelen2: size_t =
                                (*it).ptr.wrapping_offset_from(current) as libc::c_long as size_t;
                            let mut r: libc::c_int = memcmp(
                                previous as *const libc::c_void,
                                current as *const libc::c_void,
                                if bytelen1 <= bytelen2 {
                                    bytelen1
                                } else {
                                    bytelen2
                                },
                            );
                            if r == 0i32 && bytelen1 != bytelen2 {
                                r = if bytelen1 < bytelen2 { -1i32 } else { 1i32 }
                            }
                            if r > 0i32 {
                                return CborErrorMapNotSorted;
                            } else if r == 0i32
                                && flags
                                    & CborValidateMapKeysAreUnique as libc::c_int as libc::c_uint
                                    == CborValidateMapKeysAreUnique as libc::c_int as libc::c_uint
                            {
                                return CborErrorMapKeysNotUnique;
                            }
                        }
                    }
                    previous = current;
                    previous_end = (*it).ptr
                }
                /* map: that was the key, so get the value */
                err = validate_value(it, flags, recursionLeft);
                if !(0 != err as u64) {
                    continue;
                }
                return err;
            }
        }
        return CborNoError;
    };
}
