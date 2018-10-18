use libc;
use cborparser::CborValue;
use cborparser::_cbor_value_extract_number;
extern "C" {
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
    fn cbor_value_get_half_float(
        value: *const CborValue,
        result: *mut libc::c_void,
    ) -> CborError_0;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __fpclassifyl(_: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn __fpclassifyd(_: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn __fpclassifyf(_: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn _cbor_value_get_string_chunk(
        value: *const CborValue,
        bufferptr: *mut *const libc::c_void,
        len: *mut size_t,
        next: *mut CborValue,
    ) -> CborError;
    #[no_mangle]
    fn _cbor_value_prepare_string_iteration(it: *mut CborValue) -> CborError_0;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
pub const Value8Bit: unnamed = 24;
pub const IndefiniteLength: unnamed = 31;
/* 31 */
pub const SmallValueMask: unnamed = 31;
pub type unnamed = libc::c_int;
pub const BreakByte: unnamed = 255;
pub const MajorTypeMask: unnamed = -32;
pub const MajorTypeShift: unnamed = 5;
pub const Value64Bit: unnamed = 27;
pub const Value32Bit: unnamed = 26;
pub const Value16Bit: unnamed = 25;
pub const SmallValueBitLength: unnamed = 5;
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
unsafe extern "C" fn cbor_value_is_unsigned_integer(mut value: *const CborValue) -> bool {
    return 0 != cbor_value_is_integer(value) as libc::c_int
        && (*value).flags as libc::c_int & CborIteratorFlag_NegativeInteger as libc::c_int == 0i32;
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
unsafe extern "C" fn cbor_value_is_length_known(mut value: *const CborValue) -> bool {
    return (*value).flags as libc::c_int & CborIteratorFlag_UnknownLength as libc::c_int == 0i32;
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
pub unsafe extern "C" fn cbor_value_to_pretty_stream(
    mut streamFunction: CborStreamFunction,
    mut token: *mut libc::c_void,
    mut value: *mut CborValue,
    mut flags: libc::c_int,
) -> CborError_0 {
    return value_to_pretty(streamFunction, token, value, flags, 1024i32);
}
unsafe extern "C" fn value_to_pretty(
    mut stream: CborStreamFunction,
    mut out: *mut libc::c_void,
    mut it: *mut CborValue,
    mut flags: libc::c_int,
    mut recursionsLeft: libc::c_int,
) -> CborError_0 {
    let mut ival: uint64_t = 0;
    let mut r: libc::c_int = 0;
    let mut val_1: libc::c_double = 0.;
    let mut suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut f16: uint16_t = 0;
    let mut f: libc::c_float = 0.;
    let mut current_block: u64;
    let mut err: CborError_0 = CborNoError;
    let mut type_0: CborType_0 = cbor_value_get_type(it);
    match type_0 as libc::c_uint {
        128 | 160 => {
            /* recursive type */
            let mut recursed: CborValue = CborValue::new();
            let mut indicator: *const libc::c_char = get_indicator(it, flags);
            let mut space: *const libc::c_char = if 0 != *indicator as libc::c_int {
                b" \x00" as *const u8 as *const libc::c_char
            } else {
                indicator
            };
            err = stream.expect("non-null function pointer")(
                out,
                b"%c%s%s\x00" as *const u8 as *const libc::c_char,
                if type_0 as libc::c_uint == CborArrayType as libc::c_int as libc::c_uint {
                    '[' as i32
                } else {
                    '{' as i32
                },
                indicator,
                space,
            );
            if 0 != err as u64 {
                return err;
            } else {
                err = cbor_value_enter_container(it, &mut recursed);
                if 0 != err as u64 {
                    (*it).ptr = recursed.ptr;
                    (*it).idx = recursed.idx;
                    /* parse error */
                    return err;
                } else {
                    err = container_to_pretty(
                        stream,
                        out,
                        &mut recursed,
                        type_0,
                        flags,
                        recursionsLeft - 1i32,
                    );
                    if 0 != err as u64 {
                        (*it).ptr = recursed.ptr;
                        (*it).idx= recursed.idx;
                        /* parse error */
                        return err;
                    } else {
                        err = cbor_value_leave_container(it, &mut recursed);
                        if 0 != err as u64 {
                            /* parse error */
                            return err;
                        } else {
                            return stream.expect("non-null function pointer")(
                                out,
                                if type_0 as libc::c_uint
                                    == CborArrayType as libc::c_int as libc::c_uint
                                {
                                    b"]\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"}\x00" as *const u8 as *const libc::c_char
                                },
                            );
                        }
                    }
                }
            }
        }
        0 => {
            let mut val: uint64_t = 0;
            /* can't fail */
            cbor_value_get_raw_integer(it, &mut val);
            if cbor_value_is_unsigned_integer(it) {
                err = stream.expect("non-null function pointer")(
                    out,
                    b"%llu\x00" as *const u8 as *const libc::c_char,
                    val,
                )
            } else {
                /* CBOR stores the negative number X as -1 - X
                 * (that is, -1 is stored as 0, -2 as 1 and so forth) */
                val = val.wrapping_add(1);
                if 0 != val {
                    /* unsigned overflow may happen */
                    err = stream.expect("non-null function pointer")(
                        out,
                        b"-%llu\x00" as *const u8 as *const libc::c_char,
                        val,
                    )
                } else {
                    err = stream.expect("non-null function pointer")(
                        out,
                        b"-18446744073709551616\x00" as *const u8 as *const libc::c_char,
                    )
                }
            }
            if 0 == err as u64 {
                err = stream.expect("non-null function pointer")(
                    out,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    get_indicator(it, flags),
                );
                current_block = 9386390421034826751;
            } else {
                current_block = 9386390421034826751;
            }
        }
        64 | 96 => {
            let mut n: size_t = 0i32 as size_t;
            let mut ptr: *const libc::c_void = 0 as *const libc::c_void;
            let mut showingFragments: bool = 0
                != flags & CborPrettyShowStringFragments as libc::c_int
                && !cbor_value_is_length_known(it);
            let mut separator: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
            let mut close: libc::c_char = '\'' as i32 as libc::c_char;
            let mut open: [libc::c_char; 3] =
                *::std::mem::transmute::<&[u8; 3], &mut [libc::c_char; 3]>(b"h\'\x00");
            let mut indicator_0: *const libc::c_char = 0 as *const libc::c_char;
            if type_0 as libc::c_uint == CborTextStringType as libc::c_int as libc::c_uint {
                open[0usize] = '\"' as i32 as libc::c_char;
                close = open[0usize];
                open[1usize] = '\u{0}' as i32 as libc::c_char
            }
            if showingFragments {
                err = stream.expect("non-null function pointer")(
                    out,
                    b"(_ \x00" as *const u8 as *const libc::c_char,
                );
                if 0 == err as u64 {
                    err = _cbor_value_prepare_string_iteration(it)
                }
            } else {
                err = stream.expect("non-null function pointer")(
                    out,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    open.as_mut_ptr(),
                )
            }
            while 0 == err as u64 {
                if 0 != showingFragments as libc::c_int || indicator_0.is_null() {
                    /* any iteration, except the second for a non-chunked string */
                    indicator_0 = resolve_indicator(it, flags)
                }
                err = _cbor_value_get_string_chunk(it, &mut ptr, &mut n, it);
                if ptr.is_null() {
                    break;
                }
                if 0 == err as u64 && 0 != showingFragments as libc::c_int {
                    err = stream.expect("non-null function pointer")(
                        out,
                        b"%s%s\x00" as *const u8 as *const libc::c_char,
                        separator,
                        open.as_mut_ptr(),
                    )
                }
                if 0 == err as u64 {
                    err = (if type_0 as libc::c_uint
                        == CborByteStringType as libc::c_int as libc::c_uint
                    {
                        hexDump(stream, out, ptr, n) as libc::c_int
                    } else {
                        utf8EscapedDump(stream, out, ptr, n) as libc::c_int
                    }) as CborError_0
                }
                if !(0 == err as u64 && 0 != showingFragments as libc::c_int) {
                    continue;
                }
                err = stream.expect("non-null function pointer")(
                    out,
                    b"%c%s\x00" as *const u8 as *const libc::c_char,
                    close as libc::c_int,
                    indicator_0,
                );
                separator = b", \x00" as *const u8 as *const libc::c_char
            }
            if 0 == err as u64 {
                if showingFragments {
                    err = stream.expect("non-null function pointer")(
                        out,
                        b")\x00" as *const u8 as *const libc::c_char,
                    )
                } else {
                    err = stream.expect("non-null function pointer")(
                        out,
                        b"%c%s\x00" as *const u8 as *const libc::c_char,
                        close as libc::c_int,
                        indicator_0,
                    )
                }
            }
            return err;
        }
        192 => {
            let mut tag: CborTag = 0;
            /* can't fail */
            cbor_value_get_tag(it, &mut tag);
            err = stream.expect("non-null function pointer")(
                out,
                b"%llu%s(\x00" as *const u8 as *const libc::c_char,
                tag,
                get_indicator(it, flags),
            );
            if 0 == err as u64 {
                err = cbor_value_advance_fixed(it)
            }
            if 0 == err as u64 && 0 != recursionsLeft {
                err = value_to_pretty(stream, out, it, flags, recursionsLeft - 1i32)
            } else if 0 == err as u64 {
                printRecursionLimit(stream, out);
            }
            if 0 == err as u64 {
                err = stream.expect("non-null function pointer")(
                    out,
                    b")\x00" as *const u8 as *const libc::c_char,
                )
            }
            return err;
        }
        224 => {
            /* simple types can't fail and can't have overlong encoding */
            let mut simple_type: uint8_t = 0;
            cbor_value_get_simple_type(it, &mut simple_type);
            err = stream.expect("non-null function pointer")(
                out,
                b"simple(%hhu)\x00" as *const u8 as *const libc::c_char,
                simple_type as libc::c_int,
            );
            current_block = 9386390421034826751;
        }
        246 => {
            err = stream.expect("non-null function pointer")(
                out,
                b"null\x00" as *const u8 as *const libc::c_char,
            );
            current_block = 9386390421034826751;
        }
        247 => {
            err = stream.expect("non-null function pointer")(
                out,
                b"undefined\x00" as *const u8 as *const libc::c_char,
            );
            current_block = 9386390421034826751;
        }
        245 => {
            let mut val_0: bool = false;
            /* can't fail */
            cbor_value_get_boolean(it, &mut val_0);
            err = stream.expect("non-null function pointer")(
                out,
                if 0 != val_0 as libc::c_int {
                    b"true\x00" as *const u8 as *const libc::c_char
                } else {
                    b"false\x00" as *const u8 as *const libc::c_char
                },
            );
            current_block = 9386390421034826751;
        }
        251 => {
            suffix = 0 as *const libc::c_char;
            val_1 = 0.;
            r = 0;
            ival = 0;
            cbor_value_get_double(it, &mut val_1);
            suffix = b"\x00" as *const u8 as *const libc::c_char;
            current_block = 18153031941552419006;
        }
        250 => {
            cbor_value_get_float(it, &mut f);
            val_1 = f as libc::c_double;
            suffix = if 0 != flags & CborPrettyNumericEncodingIndicators as libc::c_int {
                b"_2\x00" as *const u8 as *const libc::c_char
            } else {
                b"f\x00" as *const u8 as *const libc::c_char
            };
            current_block = 18153031941552419006;
        }
        249 => {
            cbor_value_get_half_float(it, &mut f16 as *mut uint16_t as *mut libc::c_void);
            val_1 = decode_half(f16);
            suffix = if 0 != flags & CborPrettyNumericEncodingIndicators as libc::c_int {
                b"_1\x00" as *const u8 as *const libc::c_char
            } else {
                b"f16\x00" as *const u8 as *const libc::c_char
            };
            current_block = 18153031941552419006;
        }
        255 => {
            err = stream.expect("non-null function pointer")(
                out,
                b"invalid\x00" as *const u8 as *const libc::c_char,
            );
            if 0 != err as u64 {
                return err;
            } else {
                return CborErrorUnknownType;
            }
        }
        _ => {
            current_block = 9386390421034826751;
        }
    }
    match current_block {
        18153031941552419006 => {
            if flags & CborPrettyNumericEncodingIndicators as libc::c_int == 0i32 {
                r = if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
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
                    suffix = b"\x00" as *const u8 as *const libc::c_char
                }
            }
            if convertToUint64(val_1, &mut ival) {
                /* this double value fits in a 64-bit integer, so show it as such
                 * (followed by a floating point suffix, to disambiguate) */
                err = stream.expect("non-null function pointer")(
                    out,
                    b"%s%llu.%s\x00" as *const u8 as *const libc::c_char,
                    if val_1 < 0i32 as libc::c_double {
                        b"-\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"\x00" as *const u8 as *const libc::c_char
                    },
                    ival,
                    suffix,
                )
            } else {
                err = stream.expect("non-null function pointer")(
                    out,
                    b"%.17g%s\x00" as *const u8 as *const libc::c_char,
                    val_1,
                    suffix,
                )
            }
        }
        _ => {}
    }
    if 0 == err as u64 {
        err = cbor_value_advance_fixed(it)
    }
    return err;
}
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
 * \defgroup CborPretty Converting CBOR to text
 * \brief Group of functions used to convert CBOR to text form.
 *
 * This group contains two functions that can be used to convert a \ref
 * CborValue object to a text representation. This module attempts to follow
 * the recommendations from RFC 7049 section 6 "Diagnostic Notation", though it
 * has a few differences. They are noted below.
 *
 * TinyCBOR does not provide a way to convert from the text representation back
 * to encoded form. To produce a text form meant to be parsed, CborToJson is
 * recommended instead.
 *
 * Either of the functions in this section will attempt to convert exactly one
 * CborValue object to text. Those functions may return any error documented
 * for the functions for CborParsing. In addition, if the C standard library
 * stream functions return with error, the text conversion will return with
 * error CborErrorIO.
 *
 * These functions also perform UTF-8 validation in CBOR text strings. If they
 * encounter a sequence of bytes that is not permitted in UTF-8, they will return
 * CborErrorInvalidUtf8TextString. That includes encoding of surrogate points
 * in UTF-8.
 *
 * \warning The output type produced by these functions is not guaranteed to
 * remain stable. A future update of TinyCBOR may produce different output for
 * the same input and parsers may be unable to handle it.
 *
 * \sa CborParsing, CborToJson, cbor_parser_init()
 */
/* *
 * \addtogroup CborPretty
 * @{
 * <h2 class="groupheader">Text format</h2>
 *
 * As described in RFC 7049 section 6 "Diagnostic Notation", the format is
 * largely borrowed from JSON, but modified to suit CBOR's different data
 * types. TinyCBOR makes further modifications to distinguish different, but
 * similar values.
 *
 * CBOR values are currently encoded as follows:
 * \par Integrals (unsigned and negative)
 *      Base-10 (decimal) text representation of the value
 * \par Byte strings:
 *      <tt>"h'"</tt> followed by the Base16 (hex) representation of the binary data, followed by an ending quote (')
 * \par Text strings:
 *      C-style escaped string in quotes, with C11/C++11 escaping of Unicode codepoints above U+007F.
 * \par Tags:
 *      Tag value, with the tagged value in parentheses. No special encoding of the tagged value is performed.
 * \par Simple types:
 *      <tt>"simple(nn)"</tt> where \c nn is the simple value
 * \par Null:
 *      \c null
 * \par Undefined:
 *      \c undefined
 * \par Booleans:
 *      \c true or \c false
 * \par Floating point:
 *      If NaN or infinite, the actual words \c NaN or \c infinite.
 *      Otherwise, the decimal representation with as many digits as necessary to ensure no loss of information.
 *      By default, float values are suffixed by "f" and half-float values suffixed by "f16" (doubles have no suffix).
 *      If the CborPrettyNumericEncodingIndicators flag is active, the values instead are encoded following the
 *      Section 6 recommended encoding indicators: float values are suffixed with "_2" and half-float with "_1".
 *      A decimal point is always present.
 * \par Arrays:
 *      Comma-separated list of elements, enclosed in square brackets ("[" and "]").
 * \par Maps:
 *      Comma-separated list of key-value pairs, with the key and value separated
 *      by a colon (":"), enclosed in curly braces ("{" and "}").
 *
 * The CborPrettyFlags enumerator contains flags to control some aspects of the
 * encoding:
 * \par String fragmentation
 *      When the CborPrettyShowStringFragments option is active, text and byte
 *      strings that are transmitted in fragments are shown instead inside
 *      parentheses ("(" and ")") with no preceding number and each fragment is
 *      displayed individually. If a tag precedes the string, then the output
 *      will contain a double set of parentheses. If the option is not active,
 *      the fragments are merged together and the display will not show any
 *      difference from a string transmitted with determinate length.
 * \par Encoding indicators
 *      Numbers and lengths in CBOR can be encoded in multiple representations.
 *      If the CborPrettyIndicateOverlongNumbers option is active, numbers
 *      and lengths that are transmitted in a longer encoding than necessary
 *      will be indicated, by appending an underscore ("_") to either the
 *      number or the opening bracket or brace, followed by a number
 *      indicating the CBOR additional information: 0 for 1 byte, 1 for 2
 *      bytes, 2 for 4 bytes and 3 for 8 bytes.
 *      If the CborPrettyIndicateIndeterminateLength option is active, maps,
 *      arrays and strings encoded with indeterminate length will be marked by
 *      an underscore after the opening bracket or brace or the string (if not
 *      showing fragments), without a number after it.
 */
/* *
 * \enum CborPrettyFlags
 * The CborPrettyFlags enum contains flags that control the conversion of CBOR to text format.
 *
 * \value CborPrettyNumericEncodingIndicators   Use numeric encoding indicators instead of textual for float and half-float.
 * \value CborPrettyTextualEncodingIndicators   Use textual encoding indicators for float ("f") and half-float ("f16").
 * \value CborPrettyIndicateIndeterminateLength (default) Indicate when a map or array has indeterminate length.
 * \value CborPrettyIndicateOverlongNumbers     Indicate when a number or length was encoded with more bytes than needed.
 * \value CborPrettyShowStringFragments         If the byte or text string is transmitted in chunks, show each individually.
 * \value CborPrettyMergeStringFragment         Merge all chunked byte or text strings and display them in a single entry.
 * \value CborPrettyDefaultFlags                Default conversion flags.
 */
unsafe extern "C" fn convertToUint64(mut v: libc::c_double, mut absolute: *mut uint64_t) -> bool {
    let mut supremum: libc::c_double = 0.;
    v = fabs(v);
    /* C11 standard section 6.3.1.4 "Real floating and integer" says:
     *
     *  1 When a finite value of real floating type is converted to an integer
     *    type other than _Bool, the fractional part is discarded (i.e., the
     *    value is truncated toward zero). If the value of the integral part
     *    cannot be represented by the integer type, the behavior is undefined.
     *
     * So we must perform a range check that v <= UINT64_MAX, but we can't use
     * UINT64_MAX + 1.0 because the standard continues:
     *
     *  2 When a value of integer type is converted to a real floating type, if
     *    the value being converted can be represented exactly in the new type,
     *    it is unchanged. If the value being converted is in the range of
     *    values that can be represented but cannot be represented exactly, the
     *    result is either the nearest higher or nearest lower representable
     *    value, chosen in an implementation-defined manner.
     */
    /* -2 * (- 2^63) == 2^64 */
    supremum = -2.0f64 * (-9223372036854775807i64 - 1i32 as libc::c_longlong) as libc::c_double;
    if v >= supremum {
        return 0 != 0i32;
    } else {
        /* Now we can convert, these two conversions cannot be UB */
        *absolute = v as uint64_t;
        return *absolute as libc::c_double == v;
    };
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
unsafe extern "C" fn printRecursionLimit(
    mut stream: CborStreamFunction,
    mut out: *mut libc::c_void,
) -> () {
    stream.expect("non-null function pointer")(
        out,
        b"<nesting too deep, recursion stopped>\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn get_indicator(
    mut it: *mut CborValue,
    mut flags: libc::c_int,
) -> *const libc::c_char {
    return resolve_indicator(it, flags);
}
unsafe extern "C" fn resolve_indicator(
    mut it: *mut CborValue,
    mut flags: libc::c_int,
) -> *const libc::c_char {
    let mut ptr: *const uint8_t = (*it).ptr;
    let mut end: *const uint8_t = (*(*it).parser).end;
    let mut idx: *mut usize = &mut (*it).idx;
    static mut indicators: [[libc::c_char; 3]; 8] = [
        [95, 48, 0],
        [95, 49, 0],
        [95, 50, 0],
        [95, 51, 0],
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
        [95, 0, 0],
    ];
    /* these are not possible */
    /* empty string */
    let mut no_indicator: *const libc::c_char = indicators[5usize].as_ptr();
    let mut additional_information: uint8_t = 0;
    let mut expected_information: uint8_t = 0;
    let mut value: uint64_t = 0;
    let mut err: CborError_0 = CborNoError;
    if ptr == end {
        /* CborErrorUnexpectedEOF */
        return 0 as *const libc::c_char;
    } else {
        additional_information = (*ptr as libc::c_int & SmallValueMask as libc::c_int) as uint8_t;
        if (additional_information as libc::c_int) < Value8Bit as libc::c_int {
            return no_indicator;
        } else if 0 != flags & CborPrettyIndicateIndeterminateLength as libc::c_int
            && additional_information as libc::c_int == IndefiniteLength as libc::c_int
        {
            return indicators[(IndefiniteLength as libc::c_int - Value8Bit as libc::c_int) as usize]
                .as_ptr();
        } else if flags & CborPrettyIndicateOverlongNumbers as libc::c_int == 0i32 {
            return no_indicator;
        } else {
            err = _cbor_value_extract_number(it, &mut value);
            if 0 != err as u64 {
                /* CborErrorUnexpectedEOF */
                return 0 as *const libc::c_char;
            } else {
                expected_information = (Value8Bit as libc::c_int - 1i32) as uint8_t;
                if value >= Value8Bit as libc::c_int as libc::c_ulonglong {
                    expected_information = expected_information.wrapping_add(1)
                }
                if value > 0xffu32 as libc::c_ulonglong {
                    expected_information = expected_information.wrapping_add(1)
                }
                if value > 0xffffu32 as libc::c_ulonglong {
                    expected_information = expected_information.wrapping_add(1)
                }
                if value > 0xffffffffu32 as libc::c_ulonglong {
                    expected_information = expected_information.wrapping_add(1)
                }
                return if expected_information as libc::c_int
                    == additional_information as libc::c_int
                {
                    no_indicator
                } else {
                    indicators[(additional_information as libc::c_int - Value8Bit as libc::c_int)
                                   as usize]
                        .as_ptr()
                };
            }
        }
    };
}
/* This function decodes buffer as UTF-8 and prints as escaped UTF-16.
 * On UTF-8 decoding error, it returns CborErrorInvalidUtf8TextString */
unsafe extern "C" fn utf8EscapedDump(
    mut stream: CborStreamFunction,
    mut out: *mut libc::c_void,
    mut ptr: *const libc::c_void,
    mut n: size_t,
) -> CborError_0 {
    let mut current_block: u64;
    let mut buffer: *const uint8_t = ptr as *const uint8_t;
    let end: *const uint8_t = buffer.offset(n as isize);
    let mut err: CborError_0 = CborNoError;
    while buffer < end && 0 == err as u64 {
        let mut uc: uint32_t = get_utf8(&mut buffer, end);
        if uc == !0u32 {
            return CborErrorInvalidUtf8TextString;
        } else {
            if uc < 0x80i32 as libc::c_uint {
                /* single-byte UTF-8 */
                let mut escaped: libc::c_uchar = uc as libc::c_uchar;
                if uc < 0x7fi32 as libc::c_uint
                    && uc >= 0x20i32 as libc::c_uint
                    && uc != '\\' as i32 as libc::c_uint
                    && uc != '\"' as i32 as libc::c_uint
                {
                    err = stream.expect("non-null function pointer")(
                        out,
                        b"%c\x00" as *const u8 as *const libc::c_char,
                        uc as libc::c_char as libc::c_int,
                    );
                    continue;
                } else {
                    match uc {
                        34 | 92 => {
                            current_block = 820271813250567934;
                            match current_block {
                                2039210764951876080 => escaped = 'f' as i32 as libc::c_uchar,
                                6578035323546625521 => escaped = 'n' as i32 as libc::c_uchar,
                                5430767077051480867 => escaped = 'b' as i32 as libc::c_uchar,
                                10029446268360178412 => escaped = 't' as i32 as libc::c_uchar,
                                5575675750184812876 => escaped = 'r' as i32 as libc::c_uchar,
                                _ => {}
                            }
                            err = stream.expect("non-null function pointer")(
                                out,
                                b"\\%c\x00" as *const u8 as *const libc::c_char,
                                escaped as libc::c_int,
                            );
                            continue;
                        }
                        8 => {
                            current_block = 5430767077051480867;
                            match current_block {
                                2039210764951876080 => escaped = 'f' as i32 as libc::c_uchar,
                                6578035323546625521 => escaped = 'n' as i32 as libc::c_uchar,
                                5430767077051480867 => escaped = 'b' as i32 as libc::c_uchar,
                                10029446268360178412 => escaped = 't' as i32 as libc::c_uchar,
                                5575675750184812876 => escaped = 'r' as i32 as libc::c_uchar,
                                _ => {}
                            }
                            err = stream.expect("non-null function pointer")(
                                out,
                                b"\\%c\x00" as *const u8 as *const libc::c_char,
                                escaped as libc::c_int,
                            );
                            continue;
                        }
                        12 => {
                            current_block = 2039210764951876080;
                            match current_block {
                                2039210764951876080 => escaped = 'f' as i32 as libc::c_uchar,
                                6578035323546625521 => escaped = 'n' as i32 as libc::c_uchar,
                                5430767077051480867 => escaped = 'b' as i32 as libc::c_uchar,
                                10029446268360178412 => escaped = 't' as i32 as libc::c_uchar,
                                5575675750184812876 => escaped = 'r' as i32 as libc::c_uchar,
                                _ => {}
                            }
                            err = stream.expect("non-null function pointer")(
                                out,
                                b"\\%c\x00" as *const u8 as *const libc::c_char,
                                escaped as libc::c_int,
                            );
                            continue;
                        }
                        10 => {
                            current_block = 6578035323546625521;
                            match current_block {
                                2039210764951876080 => escaped = 'f' as i32 as libc::c_uchar,
                                6578035323546625521 => escaped = 'n' as i32 as libc::c_uchar,
                                5430767077051480867 => escaped = 'b' as i32 as libc::c_uchar,
                                10029446268360178412 => escaped = 't' as i32 as libc::c_uchar,
                                5575675750184812876 => escaped = 'r' as i32 as libc::c_uchar,
                                _ => {}
                            }
                            err = stream.expect("non-null function pointer")(
                                out,
                                b"\\%c\x00" as *const u8 as *const libc::c_char,
                                escaped as libc::c_int,
                            );
                            continue;
                        }
                        13 => {
                            current_block = 5575675750184812876;
                            match current_block {
                                2039210764951876080 => escaped = 'f' as i32 as libc::c_uchar,
                                6578035323546625521 => escaped = 'n' as i32 as libc::c_uchar,
                                5430767077051480867 => escaped = 'b' as i32 as libc::c_uchar,
                                10029446268360178412 => escaped = 't' as i32 as libc::c_uchar,
                                5575675750184812876 => escaped = 'r' as i32 as libc::c_uchar,
                                _ => {}
                            }
                            err = stream.expect("non-null function pointer")(
                                out,
                                b"\\%c\x00" as *const u8 as *const libc::c_char,
                                escaped as libc::c_int,
                            );
                            continue;
                        }
                        9 => {
                            current_block = 10029446268360178412;
                            match current_block {
                                2039210764951876080 => escaped = 'f' as i32 as libc::c_uchar,
                                6578035323546625521 => escaped = 'n' as i32 as libc::c_uchar,
                                5430767077051480867 => escaped = 'b' as i32 as libc::c_uchar,
                                10029446268360178412 => escaped = 't' as i32 as libc::c_uchar,
                                5575675750184812876 => escaped = 'r' as i32 as libc::c_uchar,
                                _ => {}
                            }
                            err = stream.expect("non-null function pointer")(
                                out,
                                b"\\%c\x00" as *const u8 as *const libc::c_char,
                                escaped as libc::c_int,
                            );
                            continue;
                        }
                        _ => {}
                    }
                }
            } else if uc > 0xffffu32 {
                /* needs surrogate pairs */
                err = stream.expect("non-null function pointer")(
                    out,
                    b"\\u%04X\\u%04X\x00" as *const u8 as *const libc::c_char,
                    (uc >> 10i32).wrapping_add(0xd7c0i32 as libc::c_uint),
                    uc.wrapping_rem(0x400i32 as libc::c_uint)
                        .wrapping_add(0xdc00i32 as libc::c_uint),
                );
                continue;
            }
            /* high surrogate */
            /* no surrogate pair needed */
            err = stream.expect("non-null function pointer")(
                out,
                b"\\u%04X\x00" as *const u8 as *const libc::c_char,
                uc,
            )
        }
    }
    return err;
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
unsafe extern "C" fn hexDump(
    mut stream: CborStreamFunction,
    mut out: *mut libc::c_void,
    mut ptr: *const libc::c_void,
    mut n: size_t,
) -> CborError_0 {
    let mut buffer: *const uint8_t = ptr as *const uint8_t;
    let mut err: CborError_0 = CborNoError;
    loop {
        let fresh4 = n;
        n = n.wrapping_sub(1);
        if !(0 != fresh4 && 0 == err as u64) {
            break;
        }
        let fresh5 = buffer;
        buffer = buffer.offset(1);
        err = stream.expect("non-null function pointer")(
            out,
            b"%02hhx\x00" as *const u8 as *const libc::c_char,
            *fresh5 as libc::c_int,
        )
    }
    return err;
}
unsafe extern "C" fn container_to_pretty(
    mut stream: CborStreamFunction,
    mut out: *mut libc::c_void,
    mut it: *mut CborValue,
    mut containerType: CborType_0,
    mut flags: libc::c_int,
    mut recursionsLeft: libc::c_int,
) -> CborError_0 {
    let mut comma: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut err: CborError_0 = CborNoError;
    if 0 == recursionsLeft {
        printRecursionLimit(stream, out);
        /* do allow the dumping to continue */
        return err;
    } else {
        while !cbor_value_at_end(it) && 0 == err as u64 {
            err = stream.expect("non-null function pointer")(
                out,
                b"%s\x00" as *const u8 as *const libc::c_char,
                comma,
            );
            comma = b", \x00" as *const u8 as *const libc::c_char;
            if 0 == err as u64 {
                err = value_to_pretty(stream, out, it, flags, recursionsLeft)
            }
            if containerType as libc::c_uint == CborArrayType as libc::c_int as libc::c_uint {
                continue;
            }
            /* map: that was the key, so get the value */
            if 0 == err as u64 {
                err = stream.expect("non-null function pointer")(
                    out,
                    b": \x00" as *const u8 as *const libc::c_char,
                )
            }
            if !(0 == err as u64) {
                continue;
            }
            err = value_to_pretty(stream, out, it, flags, recursionsLeft)
        }
        return err;
    };
}
