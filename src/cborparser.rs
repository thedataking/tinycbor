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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(
        __dst: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type int64_t = libc::c_longlong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type uintptr_t = libc::c_ulong;
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
pub struct CborValue<'a> {
    pub idx: usize,
    pub vec: &'a[uint8_t],
    pub remaining: uint32_t,
    pub extra: uint16_t,
    pub type_0: uint8_t,
    pub flags: uint8_t,
}
impl<'a> CborValue<'a> {
    pub fn new() -> CborValue<'a> {
        CborValue {
            idx: 0 as usize,
            vec: &[],
            remaining: 0,
            extra: 0,
            type_0: 0,
            flags: 0,
        }
    }

    /// True iff `idx` is one element beyond end of `vec`.
    pub fn at_end(&self) -> bool {
        self.idx == self.vec.len()
    }

    /// Returns number of remaining bytes until value is exhausted.
    pub fn remaining(&self) -> usize {
        self.vec.len() - self.idx
    }

    /// Returns byte at current index
    pub fn get8(&self) -> uint8_t {
        self.vec[self.idx]
    }

    /// Returns byte at `offset` from current index
    pub fn get8_at_offset(&self, offset: usize) -> uint8_t {
        self.vec[self.idx + offset]
    }

    /// Returns pointer to byte at `offset` from current index
    pub fn get_ptr(&self) -> *const uint8_t {
        self.get_ptr_at_offset(0)
    }

    /// Returns pointer one past last element
    pub unsafe fn get_end_ptr(&self) -> *const uint8_t {
        // NOTE: we're computing pointer one past end of allocation
        self.vec.as_ptr().offset(self.vec.len() as isize)
    }

    pub fn get_ptr_at_offset(&self, offset: usize) -> *const uint8_t {
        &self.vec[self.idx + offset]
    }

    pub fn advance_pos(&mut self, offset: usize) {
        self.idx = self.idx + offset;
    }

    pub fn set_pos(&mut self, other: &CborValue) {
        self.idx = other.idx;
    }

    pub fn offset_from(&self, other: *const uint8_t) -> isize {
        panic!("untested");
        let ptr: *const uint8_t = &self.vec[self.idx];
        ptr.wrapping_offset_from(other)
    }

}
pub const Value16Bit: unnamed = 25;
pub const Value8Bit: unnamed = 24;
pub const Break: CborSimpleTypes = 31;
/* not really a simple type */
pub const SimpleTypeInNextByte: CborSimpleTypes = 24;
/* ditto */
pub const HalfPrecisionFloat: CborSimpleTypes = 25;
pub const UndefinedValue: CborSimpleTypes = 23;
pub const NullValue: CborSimpleTypes = 22;
pub const TrueValue: CborSimpleTypes = 21;
/* ditto */
pub const DoublePrecisionFloat: CborSimpleTypes = 27;
/* ditto */
pub const SinglePrecisionFloat: CborSimpleTypes = 26;
pub const FalseValue: CborSimpleTypes = 20;
pub const SimpleTypesType: CborMajorTypes = 7;
pub const MajorTypeShift: unnamed = 5;
pub const MajorTypeMask: unnamed = -32;
pub const NegativeIntegerType: CborMajorTypes = 1;
pub const IndefiniteLength: unnamed = 31;
pub const Value64Bit: unnamed = 27;
/* 31 */
pub const SmallValueMask: unnamed = 31;
pub const BreakByte: unnamed = 255;
/* We return uintptr_t so that we can pass memcpy directly as the iteration
 * function. The choice is to optimize for memcpy, which is used in the base
 * parser API (cbor_value_copy_string), while memcmp is used in convenience API
 * only. */
pub type IterateFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *const uint8_t, _: size_t) -> uintptr_t>;
pub const Value32Bit: unnamed = 26;
/* CBOR_NO_HALF_FLOAT_TYPE */
/*
 * CBOR Major types
 * Encoded in the high 3 bits of the descriptor byte
 * See http://tools.ietf.org/html/rfc7049#section-2.1
 */
pub type CborMajorTypes = libc::c_uint;
pub const TagType: CborMajorTypes = 6;
/* a.k.a. object */
pub const MapType: CborMajorTypes = 5;
pub const ArrayType: CborMajorTypes = 4;
pub const TextStringType: CborMajorTypes = 3;
pub const ByteStringType: CborMajorTypes = 2;
pub const UnsignedIntegerType: CborMajorTypes = 0;
/*
 * CBOR simple and floating point types
 * Encoded in the low 8 bits of the descriptor byte when the
 * Major Type is 7.
 */
pub type CborSimpleTypes = libc::c_uint;
pub type unnamed = libc::c_int;
pub const SmallValueBitLength: unnamed = 5;
#[no_mangle]
pub unsafe extern "C" fn cbor_parser_init(
    mut buffer: &mut Vec<uint8_t>,
    mut size: size_t,
    mut flags: uint32_t,
) -> (CborValue, CborError) {
    let mut it: CborValue = CborValue {
        idx: 0,
        vec: buffer.as_slice(),
        /* there's one type altogether, usually an array or map */
        remaining: 1i32 as uint32_t,
        /* may be initalized in `preparse_value` in C version */
        extra: 0,
        type_0: 0,
        flags: 0,
    };
    let err = preparse_value(&mut it);
    return (it, err)
}
unsafe extern "C" fn preparse_value(mut it: &mut CborValue) -> CborError {
    let mut current_block: u64;
    (*it).type_0 = CborInvalidType as libc::c_int as uint8_t;
    /* are we at the end? */
    if it.at_end() {
        return CborErrorUnexpectedEOF;
    } else {
        let mut descriptor: uint8_t = it.vec[it.idx];
        let mut type_0: uint8_t =
            (descriptor as libc::c_int & MajorTypeMask as libc::c_int) as uint8_t;
        (*it).type_0 = type_0;
        (*it).flags = 0i32 as uint8_t;
        descriptor = (descriptor as libc::c_int & SmallValueMask as libc::c_int) as uint8_t;
        (*it).extra = descriptor as uint16_t;
        if descriptor as libc::c_int > Value64Bit as libc::c_int {
            if 0 != (descriptor as libc::c_int != IndefiniteLength as libc::c_int) as libc::c_int
                as libc::c_long
            {
                return (if type_0 as libc::c_int == CborSimpleType as libc::c_int {
                    CborErrorUnknownType as libc::c_int
                } else {
                    CborErrorIllegalNumber as libc::c_int
                }) as CborError;
            } else if 0 != !is_fixed_type(type_0) as libc::c_int as libc::c_long {
                /* special case */
                (*it).flags = ((*it).flags as libc::c_int
                    | CborIteratorFlag_UnknownLength as libc::c_int)
                    as uint8_t;
                (*it).type_0 = type_0;
                return CborNoError;
            } else {
                return (if type_0 as libc::c_int == CborSimpleType as libc::c_int {
                    CborErrorUnexpectedBreak as libc::c_int
                } else {
                    CborErrorIllegalNumber as libc::c_int
                }) as CborError;
            }
        } else {
            let mut bytesNeeded: size_t = (if (descriptor as libc::c_int) < Value8Bit as libc::c_int
            {
                0i32
            } else {
                1i32 << descriptor as libc::c_int - Value8Bit as libc::c_int
            }) as size_t;
            if bytesNeeded.wrapping_add(1i32 as libc::c_ulong)
                > (*it).remaining() as size_t
            {
                return CborErrorUnexpectedEOF;
            } else {
                let mut majortype: uint8_t =
                    (type_0 as libc::c_int >> MajorTypeShift as libc::c_int) as uint8_t;
                if majortype as libc::c_int == NegativeIntegerType as libc::c_int {
                    (*it).flags = ((*it).flags as libc::c_int
                        | CborIteratorFlag_NegativeInteger as libc::c_int)
                        as uint8_t;
                    (*it).type_0 = CborIntegerType as libc::c_int as uint8_t
                } else if majortype as libc::c_int == SimpleTypesType as libc::c_int {
                    match descriptor as libc::c_int {
                        20 => {
                            (*it).extra = 0i32 as uint16_t;
                            (*it).type_0 = CborBooleanType as libc::c_int as uint8_t;
                            current_block = 8515828400728868193;
                        }
                        26 | 27 => {
                            (*it).flags = ((*it).flags as libc::c_int
                                | CborIteratorFlag_IntegerValueTooLarge as libc::c_int)
                                as uint8_t;
                            /* fall through */
                            current_block = 4301060678865021963;
                        }
                        21 | 22 | 23 | 25 => {
                            current_block = 4301060678865021963;
                        }
                        24 => {
                            (*it).extra = (*it).get8_at_offset(1) as uint16_t;
                            if 0 != (((*it).extra as libc::c_int) < 32i32) as libc::c_int
                                as libc::c_long
                            {
                                (*it).type_0 = CborInvalidType as libc::c_int as uint8_t;
                                return CborErrorIllegalSimpleType;
                            } else {
                                current_block = 8515828400728868193;
                            }
                        }
                        28 | 29 | 30 | 31 => {
                            /* these conditions can't be reached */
                            if 0 != (0 == 0i32) as libc::c_int as libc::c_long {
                                __assert_rtn(
                                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                        b"preparse_value\x00",
                                    )).as_ptr(),
                                    b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
                                    282i32,
                                    b"0\x00" as *const u8 as *const libc::c_char,
                                );
                            } else {
                            };
                            return CborErrorUnexpectedBreak;
                        }
                        _ => {
                            current_block = 8515828400728868193;
                        }
                    }
                    match current_block {
                        4301060678865021963 => (*it).type_0 = (*it).get8(),
                        _ => {}
                    }
                    return CborNoError;
                }
                /* try to decode up to 16 bits */
                if (descriptor as libc::c_int) < Value8Bit as libc::c_int {
                    return CborNoError;
                } else {
                    if descriptor as libc::c_int == Value8Bit as libc::c_int {
                        (*it).extra = (*it).get8_at_offset(1) as uint16_t
                    } else if descriptor as libc::c_int == Value16Bit as libc::c_int {
                        (*it).extra = get16(&(*it).vec[(*it).idx + 1]);
                    } else {
                        (*it).flags = ((*it).flags as libc::c_int
                            | CborIteratorFlag_IntegerValueTooLarge as libc::c_int)
                            as uint8_t
                    }
                    return CborNoError;
                }
            }
        }
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
/* *
 * \defgroup CborParsing Parsing CBOR streams
 * \brief Group of functions used to parse CBOR streams.
 *
 * TinyCBOR provides functions for pull-based stream parsing of a CBOR-encoded
 * payload. The main data type for the parsing is a CborValue, which behaves
 * like an iterator and can be used to extract the encoded data. It is first
 * initialized with a call to cbor_parser_init() and is usually used to extract
 * exactly one item, most often an array or map.
 *
 * Nested CborValue objects can be parsed using cbor_value_enter_container().
 * Each call to cbor_value_enter_container() must be matched by a call to
 * cbor_value_leave_container(), with the exact same parameters.
 *
 * The example below initializes a CborParser object, begins the parsing with a
 * CborValue and decodes a single integer:
 *
 * \code
 * int extract_int(const uint8_t *buffer, size_t len)
 * {
 *     CborParser parser;
 *     CborValue value;
 *     int result;
 *     cbor_parser_init(buffer, len, 0, &parser, &value);
 *     cbor_value_get_int(&value, &result);
 *     return result;
 * }
 * \endcode
 *
 * The code above does no error checking, which means it assumes the data comes
 * from a source trusted to send one properly-encoded integer. The following
 * example does the exact same operation, but includes error checking and
 * returns 0 on parsing failure:
 *
 * \code
 * int extract_int(const uint8_t *buffer, size_t len)
 * {
 *     CborParser parser;
 *     CborValue value;
 *     int result;
 *     if (cbor_parser_init(buffer, len, 0, &parser, &value) != CborNoError)
 *         return 0;
 *     if (!cbor_value_is_integer(&value) ||
 *             cbor_value_get_int(&value, &result) != CborNoError)
 *         return 0;
 *     return result;
 * }
 * \endcode
 *
 * Note, in the example above, that one can't distinguish a parsing failure
 * from an encoded value of zero. Reporting a parsing error is left as an
 * exercise to the reader.
 *
 * The code above does not execute a range-check either: it is possible that
 * the value decoded from the CBOR stream encodes a number larger than what can
 * be represented in a variable of type \c{int}. If detecting that case is
 * important, the code should call cbor_value_get_int_checked() instead.
 *
 * <h3 class="groupheader">Memory and parsing constraints</h3>
 *
 * TinyCBOR is designed to run with little memory and with minimal overhead.
 * Except where otherwise noted, the parser functions always run on constant
 * time (O(1)), do not recurse and never allocate memory (thus, stack usage is
 * bounded and is O(1)).
 *
 * <h3 class="groupheader">Error handling and preconditions</h3>
 *
 * All functions operating on a CborValue return a CborError condition, with
 * CborNoError standing for the normal situation in which no parsing error
 * occurred. All functions may return parsing errors in case the stream cannot
 * be decoded properly, be it due to corrupted data or due to reaching the end
 * of the input buffer.
 *
 * Error conditions must not be ignored. All decoder functions have undefined
 * behavior if called after an error has been reported, and may crash.
 *
 * Some functions are also documented to have preconditions, like
 * cbor_value_get_int() requiring that the input be an integral value.
 * Violation of preconditions also results in undefined behavior and the
 * program may crash.
 */
/* *
 * \addtogroup CborParsing
 * @{
 */
/* *
 * \struct CborValue
 *
 * This type contains one value parsed from the CBOR stream. Each CborValue
 * behaves as an iterator in a StAX-style parser.
 *
 * \if privatedocs
 * Implementation details: the CborValue contains these fields:
 * \list
 *   \li ptr: pointer to the actual data
 *   \li flags: flags from the decoder
 *   \li extra: partially decoded integer value (0, 1 or 2 bytes)
 *   \li remaining: remaining items in this collection after this item or UINT32_MAX if length is unknown
 * \endlist
 * \endif
 */
unsafe extern "C" fn get16(ptr: *const uint8_t) -> uint16_t {
    let mut result: uint16_t = 0;
    memcpy(
        &mut result as *mut uint16_t as *mut libc::c_void,
        ptr as *const libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    return result.swap_bytes();
}
unsafe extern "C" fn is_fixed_type(type_0: uint8_t) -> bool {
    return type_0 as libc::c_int != CborTextStringType as libc::c_int
        && type_0 as libc::c_int != CborByteStringType as libc::c_int
        && type_0 as libc::c_int != CborArrayType as libc::c_int
        && type_0 as libc::c_int != CborMapType as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_validate_basic(mut it: &CborValue) -> CborError {
    let mut value: CborValue = *it;
    return cbor_value_advance(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_advance(mut it: &mut CborValue) -> CborError {
    if 0 != !((*it).type_0 as libc::c_int != CborInvalidType as libc::c_int) as libc::c_int
        as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"cbor_value_advance\x00"))
                .as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            526i32,
            b"it->type != CborInvalidType\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 == (*it).remaining {
        return CborErrorAdvancePastEOF;
    } else {
        return advance_recursive(it, 1024i32);
    };
}
unsafe extern "C" fn advance_recursive(
    mut it: &mut CborValue,
    mut nestingLevel: libc::c_int,
) -> CborError {
    let mut err: CborError = CborNoError;
    let mut recursed = CborValue::new();
    if is_fixed_type((*it).type_0) {
        return advance_internal(it);
    } else if !cbor_value_is_container(it) {
        let mut len: size_t = 18446744073709551615u64;
        return _cbor_value_copy_string(
            it,
            0 as *mut libc::c_void,
            &mut len,
            None); // was `Some(it)`
    } else if nestingLevel == 0i32 {
        return CborErrorNestingTooDeep;
    } else {
        err = cbor_value_enter_container(it, &mut recursed);
        if 0 != err as u64 {
            return err;
        } else {
            while !cbor_value_at_end(&mut recursed) {
                err = advance_recursive(&mut recursed, nestingLevel - 1i32);
                if !(0 != err as u64) {
                    continue;
                }
                return err;
            }
            return cbor_value_leave_container(it, &mut recursed);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_leave_container(
    mut it: &mut CborValue,
    mut recursed: &CborValue,
) -> CborError {
    if 0 != !cbor_value_is_container(it) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"cbor_value_leave_container\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            638i32,
            b"cbor_value_is_container(it)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 != !((*recursed).type_0 as libc::c_int == CborInvalidType as libc::c_int) as libc::c_int
        as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"cbor_value_leave_container\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            639i32,
            b"recursed->type == CborInvalidType\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    (*it).set_pos(recursed);
    return preparse_next_value(it);
}
unsafe extern "C" fn preparse_next_value(mut it: &mut CborValue) -> CborError {
    if (*it).remaining != 4294967295u32 {
        /* don't decrement the item count if the current item is tag: they don't count */
        if (*it).type_0 as libc::c_int != CborTagType as libc::c_int && {
            (*it).remaining = (*it).remaining.wrapping_sub(1);
            (*it).remaining == 0i32 as libc::c_uint
        } {
            (*it).type_0 = CborInvalidType as libc::c_int as uint8_t;
            return CborNoError;
        }
    }
    return preparse_next_value_nodecrement(it);
}
unsafe extern "C" fn preparse_next_value_nodecrement(mut it: &mut CborValue) -> CborError {
    if (*it).remaining == 4294967295u32
        && !(*it).at_end()
        && (*it).get8() as libc::c_int == BreakByte as libc::c_int as uint8_t as libc::c_int
    {
        /* end of map or array */
        (*it).advance_pos(1);
        (*it).type_0 = CborInvalidType as libc::c_int as uint8_t;
        (*it).remaining = 0i32 as uint32_t;
        return CborNoError;
    } else {
        return preparse_value(it);
    };
}
unsafe extern "C" fn cbor_value_is_container(mut it: *const CborValue) -> bool {
    return (*it).type_0 as libc::c_int == CborArrayType as libc::c_int
        || (*it).type_0 as libc::c_int == CborMapType as libc::c_int;
}
unsafe extern "C" fn cbor_value_at_end(mut it: *const CborValue) -> bool {
    return (*it).remaining == 0i32 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_enter_container<'a>(
    mut it: &mut CborValue<'a>,
    mut recursed: & mut CborValue<'a>,
) -> CborError {
    if 0 != !cbor_value_is_container(it) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"cbor_value_enter_container\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            589i32,
            b"cbor_value_is_container(it)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    *recursed = *it;
    if 0 != (*it).flags as libc::c_int & CborIteratorFlag_UnknownLength as libc::c_int {
        (*recursed).remaining = 4294967295u32;
        (*recursed).advance_pos(1);
    } else {
        let mut len: uint64_t = 0;
        let mut err: CborError =
            _cbor_value_extract_number(
                recursed,
                &mut len);
        if 0 != !(err as libc::c_int == CborNoError as libc::c_int) as libc::c_int as libc::c_long {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                    b"cbor_value_enter_container\x00",
                )).as_ptr(),
                b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
                598i32,
                b"err == CborNoError\x00" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        (*recursed).remaining = len as uint32_t;
        if (*recursed).remaining as libc::c_ulonglong != len
            || len == 4294967295u32 as libc::c_ulonglong
        {
            /* back track the pointer to indicate where the error occurred */
            (*recursed).set_pos(it);
            return CborErrorDataTooLarge;
        } else {
            if (*recursed).type_0 as libc::c_int == CborMapType as libc::c_int {
                /* maps have keys and values, so we need to multiply by 2 */
                if (*recursed).remaining > 4294967295u32.wrapping_div(2i32 as libc::c_uint) {
                    /* back track the pointer to indicate where the error occurred */
                    (*recursed).set_pos(it);

                    return CborErrorDataTooLarge;
                } else {
                    (*recursed).remaining = ((*recursed).remaining as libc::c_uint)
                        .wrapping_mul(2i32 as libc::c_uint)
                        as uint32_t as uint32_t
                }
            }
            if len == 0i32 as libc::c_ulonglong {
                /* the case of the empty container */
                (*recursed).type_0 = CborInvalidType as libc::c_int as uint8_t;
                return CborNoError;
            }
        }
    }
    return preparse_next_value_nodecrement(recursed);
}
#[no_mangle]
pub unsafe extern "C" fn _cbor_value_extract_number(
    mut it: *mut CborValue,
    mut len: *mut uint64_t,
) -> CborError {
    let mut end: *const uint8_t = (*it).get_end_ptr();
    let mut idx: *mut usize = &mut (*it).idx;
    let mut bytesNeeded: size_t = 0;
    let mut additional_information: uint8_t =
        ((*it).get8() as libc::c_int & SmallValueMask as libc::c_int) as uint8_t;
    (*it).advance_pos(1);
    if (additional_information as libc::c_int) < Value8Bit as libc::c_int {
        *len = additional_information as uint64_t;
        return CborNoError;
    } else if 0 != (additional_information as libc::c_int > Value64Bit as libc::c_int)
        as libc::c_int as libc::c_long
    {
        return CborErrorIllegalNumber;
    } else {
        bytesNeeded =
            (1i32 << additional_information as libc::c_int - Value8Bit as libc::c_int) as size_t;
        if 0 != (bytesNeeded > (*it).remaining() as libc::c_long as size_t)
            as libc::c_int as libc::c_long
        {
            return CborErrorUnexpectedEOF;
        } else {
            if bytesNeeded == 1i32 as libc::c_ulong {
                *len = (*it).get8() as uint64_t
            } else if bytesNeeded == 2i32 as libc::c_ulong {
                *len = get16((*it).get_ptr()) as uint64_t
            } else if bytesNeeded == 4i32 as libc::c_ulong {
                *len = get32((*it).get_ptr()) as uint64_t
            } else {
                *len = get64((*it).get_ptr())
            }
            (*it).advance_pos(bytesNeeded as usize);
            return CborNoError;
        }
    };
}
unsafe extern "C" fn get64(mut ptr: *const uint8_t) -> uint64_t {
    let mut result: uint64_t = 0;
    memcpy(
        &mut result as *mut uint64_t as *mut libc::c_void,
        ptr as *const libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
    return result.swap_bytes();
}
unsafe extern "C" fn get32(mut ptr: *const uint8_t) -> uint32_t {
    let mut result: uint32_t = 0;
    memcpy(
        &mut result as *mut uint32_t as *mut libc::c_void,
        ptr as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    return result.swap_bytes();
}
#[no_mangle]
pub unsafe extern "C" fn _cbor_value_copy_string<'a>(
    value: &CborValue<'a>,
    mut buffer: *mut libc::c_void,
    mut buflen: *mut size_t,
    mut next: Option<&mut CborValue<'a>>,
) -> CborError {
    let mut copied_all: bool = false;
    let mut err: CborError = iterate_string_chunks(
        value,
        buffer as *mut libc::c_char,
        buflen,
        &mut copied_all,
        next,
        if !buffer.is_null() {
            Some(iterate_memcpy)
        } else {
            Some(iterate_noop)
        },
    );
    return (if 0 != err as libc::c_int {
        err as libc::c_int
    } else if 0 != copied_all as libc::c_int {
        CborNoError as libc::c_int
    } else {
        CborErrorOutOfMemory as libc::c_int
    }) as CborError;
}
unsafe extern "C" fn iterate_noop(
    mut dest: *mut libc::c_char,
    mut src: *const uint8_t,
    mut len: size_t,
) -> uintptr_t {
    return 1i32 as uintptr_t;
}
unsafe extern "C" fn iterate_memcpy(
    mut dest: *mut libc::c_char,
    mut src: *const uint8_t,
    mut len: size_t,
) -> uintptr_t {
    return memcpy(dest as *mut libc::c_void, src as *const libc::c_void, len) as uintptr_t;
}
unsafe extern "C" fn iterate_string_chunks<'a>(
    value: &CborValue<'a>,
    mut buffer: *mut libc::c_char,
    mut buflen: *mut size_t,
    mut result: *mut bool,
    mut next: Option<&mut CborValue<'a>>,
    mut func: IterateFunction,
) -> CborError {
    let mut nul: [uint8_t; 1] = [0; 1];
    let mut err: CborError = CborNoError;
    let mut total: size_t = 0i32 as size_t;
    let mut ptr: *const libc::c_void = 0 as *const libc::c_void;
    if 0 != !(0 != cbor_value_is_byte_string(value) as libc::c_int
        || 0 != cbor_value_is_text_string(value) as libc::c_int) as libc::c_int
        as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"iterate_string_chunks\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            1088i32,
            b"cbor_value_is_byte_string(value) || cbor_value_is_text_string(value)\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
    };

    let mut tmp = CborValue::new();
    let mut next = match next {
        Some(n) => n,
        None => &mut tmp,
    };
    *next = *value;
    *result = 0 != 1i32;
    loop {
        let mut newTotal: size_t = 0;
        let mut chunkLen: size_t = 0;
        err = get_string_chunk(next, &mut ptr, &mut chunkLen);
        if 0 != err as u64 {
            return err;
        } else {
            if ptr.is_null() {
                break;
            }
            if 0
                != add_check_overflow(total, chunkLen, &mut newTotal) as libc::c_int as libc::c_long
            {
                return CborErrorDataTooLarge;
            } else {
                if 0 != *result as libc::c_int && *buflen >= newTotal {
                    *result = 0 != func.expect("non-null function pointer")(
                        buffer.offset(total as isize),
                        ptr as *const uint8_t,
                        chunkLen,
                    )
                } else {
                    *result = 0 != 0i32
                }
                total = newTotal
            }
        }
    }
    /* is there enough room for the ending NUL byte? */
    if 0 != *result as libc::c_int && *buflen > total {
        nul = [0i32 as uint8_t];
        *result = 0 != func.expect("non-null function pointer")(
            buffer.offset(total as isize),
            nul.as_mut_ptr(),
            1i32 as size_t,
        )
    }
    *buflen = total;
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
/* inline is a keyword */
/* DBL_DECIMAL_DIG is C11 */
/* C-style const_cast without triggering a warning with -Wcast-qual */
unsafe extern "C" fn add_check_overflow(
    mut v1: size_t,
    mut v2: size_t,
    mut r: *mut size_t,
) -> bool {
    let (fresh0, fresh1) = v1.overflowing_add(v2);
    *r = fresh0;
    return fresh1;
}
unsafe extern "C" fn get_string_chunk(
    mut it: &mut CborValue,
    mut bufferptr: *mut *const libc::c_void,
    mut len: *mut size_t,
) -> CborError {
    let mut current_block: u64;
    let mut err: CborError = CborNoError;
    /* Possible states:
     * length known | iterating | meaning
     *     no       |    no     | before the first chunk of a chunked string
     *     yes      |    no     | at a non-chunked string
     *     no       |    yes    | second or later chunk
     *     yes      |    yes    | after a non-chunked string
     */
    if 0 != (*it).flags as libc::c_int & CborIteratorFlag_IteratingStringChunks as libc::c_int {
        /* already iterating */
        if cbor_value_is_length_known(it) {
            /* if the length was known, it wasn't chunked, so finish iteration */
            current_block = 16990311520665930296;
        } else {
            current_block = 792017965103506125;
        }
    } else {
        prepare_string_iteration(it);
        current_block = 792017965103506125;
    }
    match current_block {
        792017965103506125 => {
            /* are we at the end? */
            if (*it).at_end() {
                return CborErrorUnexpectedEOF;
            } else if (*it).get8() as libc::c_int == BreakByte as libc::c_int {
                /* last chunk */
                (*it).advance_pos(1);
            } else if ((*it).get8() as libc::c_int & MajorTypeMask as libc::c_int) as uint8_t
                as libc::c_int == (*it).type_0 as libc::c_int
            {
                err = extract_length(
                    it,
                    len);
                if 0 != err as u64 {
                    return err;
                } else if *len > (*it).remaining() as size_t
                {
                    return CborErrorUnexpectedEOF;
                } else {
                    *bufferptr = (*it).get_ptr() as *const libc::c_void;
                    (*it).advance_pos(*len as usize);
                    (*it).flags = ((*it).flags as libc::c_int
                        | CborIteratorFlag_IteratingStringChunks as libc::c_int)
                        as uint8_t;
                    return CborNoError;
                }
            } else {
                return CborErrorIllegalType;
            }
        }
        _ => {}
    }
    *bufferptr = 0 as *const libc::c_void;
    *len = 0i32 as size_t;
    return preparse_next_value(it);
}
unsafe extern "C" fn extract_length(
    mut it: *mut CborValue,
    mut len: *mut size_t,
) -> CborError {
    let mut v: uint64_t = 0;
    let mut err: CborError = _cbor_value_extract_number(
        it,
        &mut v);
    if 0 != err as u64 {
        *len = 0i32 as size_t;
        return err;
    } else {
        *len = v as size_t;
        if v != *len as libc::c_ulonglong {
            return CborErrorDataTooLarge;
        } else {
            return CborNoError;
        }
    };
}
unsafe extern "C" fn prepare_string_iteration(mut it: *mut CborValue) -> () {
    if !cbor_value_is_length_known(it) {
        /* chunked string: we're before the first chunk;
         * advance to the first chunk */
        (*it).advance_pos(1);
        (*it).flags = ((*it).flags as libc::c_int
            | CborIteratorFlag_IteratingStringChunks as libc::c_int)
            as uint8_t
    };
}
unsafe extern "C" fn cbor_value_is_length_known(mut value: *const CborValue) -> bool {
    return (*value).flags as libc::c_int & CborIteratorFlag_UnknownLength as libc::c_int == 0i32;
}
unsafe extern "C" fn cbor_value_is_text_string(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborTextStringType as libc::c_int;
}
/* Strings */
unsafe extern "C" fn cbor_value_is_byte_string(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborByteStringType as libc::c_int;
}
unsafe extern "C" fn advance_internal(mut it: &mut CborValue) -> CborError {
    let mut length: uint64_t = 0;
    let mut err: CborError =
        _cbor_value_extract_number(
            it,
            &mut length);
    if 0 != !(err as libc::c_int == CborNoError as libc::c_int) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"advance_internal\x00"))
                .as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            330i32,
            b"err == CborNoError\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if (*it).type_0 as libc::c_int == CborByteStringType as libc::c_int
        || (*it).type_0 as libc::c_int == CborTextStringType as libc::c_int
    {
        if 0 != !(length == length as size_t as libc::c_ulonglong) as libc::c_int as libc::c_long {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"advance_internal\x00"))
                    .as_ptr(),
                b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
                333i32,
                b"length == (size_t)length\x00" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        if 0 != !((*it).flags as libc::c_int & CborIteratorFlag_UnknownLength as libc::c_int
            == 0i32) as libc::c_int as libc::c_long
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"advance_internal\x00"))
                    .as_ptr(),
                b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
                334i32,
                b"(it->flags & CborIteratorFlag_UnknownLength) == 0\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
        };
        (*it).advance_pos(length as usize);
    }
    return preparse_next_value(it);
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_advance_fixed(mut it: &mut CborValue) -> CborError {
    if 0 != !((*it).type_0 as libc::c_int != CborInvalidType as libc::c_int) as libc::c_int
        as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"cbor_value_advance_fixed\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            474i32,
            b"it->type != CborInvalidType\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 != !is_fixed_type((*it).type_0) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"cbor_value_advance_fixed\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            475i32,
            b"is_fixed_type(it->type)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 == (*it).remaining {
        return CborErrorAdvancePastEOF;
    } else {
        return advance_internal(it);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cbor_value_decode_int64_internal(
    mut value: *const CborValue,
) -> uint64_t {
    if 0 != !(0
        != (*value).flags as libc::c_int & CborIteratorFlag_IntegerValueTooLarge as libc::c_int
        || (*value).type_0 as libc::c_int == CborFloatType as libc::c_int
        || (*value).type_0 as libc::c_int == CborDoubleType as libc::c_int)
        as libc::c_int as libc::c_long
    {
        __assert_rtn((*::std::mem::transmute::<&[u8; 34],
                                               &[libc::c_char; 34]>(b"_cbor_value_decode_int64_internal\x00")).as_ptr(),
                     b"src/cborparser.c\x00" as *const u8 as
                         *const libc::c_char, 354i32,
                     b"value->flags & CborIteratorFlag_IntegerValueTooLarge || value->type == CborFloatType || value->type == CborDoubleType\x00"
                         as *const u8 as *const libc::c_char);
    } else {
    };
    /* since the additional information can only be Value32Bit or Value64Bit,
     * we just need to test for the one bit those two options differ */
    if 0 != !((*value).get8() as libc::c_int & SmallValueMask as libc::c_int
        == Value32Bit as libc::c_int
        || (*value).get8() as libc::c_int & SmallValueMask as libc::c_int
            == Value64Bit as libc::c_int) as libc::c_int as libc::c_long
    {
        __assert_rtn((*::std::mem::transmute::<&[u8; 34],
                                               &[libc::c_char; 34]>(b"_cbor_value_decode_int64_internal\x00")).as_ptr(),
                     b"src/cborparser.c\x00" as *const u8 as
                         *const libc::c_char, 358i32,
                     b"(*value->ptr & SmallValueMask) == Value32Bit || (*value->ptr & SmallValueMask) == Value64Bit\x00"
                         as *const u8 as *const libc::c_char);
    } else {
    };
    if (*value).get8() as libc::c_int & 1i32 == Value32Bit as libc::c_int & 1i32 {
        return get32((*value).get_ptr_at_offset(1)) as uint64_t;
    } else {
        if 0 != !((*value).get8() as libc::c_int & SmallValueMask as libc::c_int
            == Value64Bit as libc::c_int) as libc::c_int as libc::c_long
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"_cbor_value_decode_int64_internal\x00",
                )).as_ptr(),
                b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
                362i32,
                b"(*value->ptr & SmallValueMask) == Value64Bit\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
        };
        return get64((*value).get_ptr_at_offset(1));
    };
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
/* Integers */
unsafe extern "C" fn cbor_value_is_integer(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborIntegerType as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_get_int64_checked(
    mut value: *const CborValue,
    mut result: *mut int64_t,
) -> CborError {
    let mut v: uint64_t = 0;
    if 0 != !cbor_value_is_integer(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"cbor_value_get_int64_checked\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            821i32,
            b"cbor_value_is_integer(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    v = _cbor_value_extract_int64_helper(value);
    /* Check before converting, as the standard says (C11 6.3.1.3 paragraph 3):
     * "[if] the new type is signed and the value cannot be represented in it; either the
     *  result is implementation-defined or an implementation-defined signal is raised."
     *
     * The range for int64_t is -2^63 to 2^63-1 (int64_t is required to be
     * two's complement, C11 7.20.1.1 paragraph 3), which in CBOR is
     * represented the same way, differing only on the "sign bit" (the major
     * type).
     */
    if 0 != (v > 9223372036854775807i64 as uint64_t) as libc::c_int as libc::c_long {
        return CborErrorDataTooLarge;
    } else {
        *result = v as int64_t;
        if 0 != (*value).flags as libc::c_int & CborIteratorFlag_NegativeInteger as libc::c_int {
            *result = -*result - 1i32 as libc::c_longlong
        }
        return CborNoError;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_get_int_checked(
    mut value: *const CborValue,
    mut result: *mut libc::c_int,
) -> CborError {
    let mut v: uint64_t = 0;
    if 0 != !cbor_value_is_integer(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"cbor_value_get_int_checked\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            861i32,
            b"cbor_value_is_integer(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    v = _cbor_value_extract_int64_helper(value);
    /* Check before converting, as the standard says (C11 6.3.1.3 paragraph 3):
     * "[if] the new type is signed and the value cannot be represented in it; either the
     *  result is implementation-defined or an implementation-defined signal is raised."
     *
     * But we can convert from signed to unsigned without fault (paragraph 2).
     *
     * The range for int is implementation-defined and int is not guaranteed to use
     * two's complement representation (although int32_t is).
     */
    if 0 != (*value).flags as libc::c_int & CborIteratorFlag_NegativeInteger as libc::c_int {
        if 0 != (v > -(-2147483647i32 - 1i32 + 1i32) as libc::c_uint as libc::c_ulonglong)
            as libc::c_int as libc::c_long
        {
            return CborErrorDataTooLarge;
        } else {
            *result = v as libc::c_int;
            *result = -*result - 1i32
        }
    } else if 0 != (v > 2147483647i32 as uint64_t) as libc::c_int as libc::c_long {
        return CborErrorDataTooLarge;
    } else {
        *result = v as libc::c_int
    }
    return CborNoError;
}
/* Tags */
unsafe extern "C" fn cbor_value_is_tag(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborTagType as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_skip_tag(mut it: &mut CborValue) -> CborError {
    while cbor_value_is_tag(it) {
        let mut err: CborError = cbor_value_advance_fixed(it);
        if !(0 != err as u64) {
            continue;
        }
        return err;
    }
    return CborNoError;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_calculate_string_length(
    mut value: &CborValue,
    mut len: *mut size_t,
) -> CborError {
    *len = 18446744073709551615u64;
    return _cbor_value_copy_string(value, 0 as *mut libc::c_void, len, None);
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_text_string_equals(
    mut value: *const CborValue,
    mut string: *const libc::c_char,
    mut result: *mut bool,
) -> CborError {
    let mut len: size_t = 0;
    let mut copy: CborValue = *value;
    let mut err: CborError = cbor_value_skip_tag(&mut copy);
    if 0 != err as u64 {
        return err;
    } else if !cbor_value_is_text_string(&mut copy) {
        *result = 0 != 0i32;
        return CborNoError;
    } else {
        len = strlen(string);
        return iterate_string_chunks(
            &mut copy,
            string as uintptr_t as *mut libc::c_char,
            &mut len,
            result,
            None,
            Some(iterate_memcmp),
        );
    };
}
unsafe extern "C" fn iterate_memcmp(
    mut s1: *mut libc::c_char,
    mut s2: *const uint8_t,
    mut len: size_t,
) -> uintptr_t {
    return (memcmp(
        s1 as *const libc::c_void,
        s2 as *const libc::c_char as *const libc::c_void,
        len,
    ) == 0i32) as libc::c_int as uintptr_t;
}
unsafe extern "C" fn cbor_value_is_map(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborMapType as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_map_find_value<'a>(
    mut map: &mut CborValue<'a>,
    mut string: *const libc::c_char,
    mut element: &mut CborValue<'a>,
) -> CborError {
    let mut current_block: u64;
    let mut err: CborError = CborNoError;
    let mut len: size_t = strlen(string);
    if 0 != !cbor_value_is_map(map) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"cbor_value_map_find_value\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            1311i32,
            b"cbor_value_is_map(map)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    err = cbor_value_enter_container(map, element);
    if !(0 != err as u64) {
        loop {
            if cbor_value_at_end(element) {
                current_block = 8258075665625361029;
                break;
            }
            /* find the non-tag so we can compare */
            err = cbor_value_skip_tag(element);
            if 0 != err as u64 {
                current_block = 4866892619474992625;
                break;
            }
            if cbor_value_is_text_string(element) {
                let mut equals: bool = false;
                let mut dummyLen: size_t = len;
                err = iterate_string_chunks(
                    element,
                    string as uintptr_t as *mut libc::c_char,
                    &mut dummyLen,
                    &mut equals,
                    None, // was `Some(element)`
                    Some(iterate_memcmp),
                );
                if 0 != err as u64 {
                    current_block = 4866892619474992625;
                    break;
                }
                if equals {
                    return preparse_value(element);
                }
            } else {
                /* skip this key */
                err = cbor_value_advance(element);
                if 0 != err as u64 {
                    current_block = 4866892619474992625;
                    break;
                }
            }
            /* skip this value */
            err = cbor_value_skip_tag(element);
            if 0 != err as u64 {
                current_block = 4866892619474992625;
                break;
            }
            err = cbor_value_advance(element);
            if 0 != err as u64 {
                current_block = 4866892619474992625;
                break;
            }
        }
        match current_block {
            4866892619474992625 => {}
            _ => {
                /* not found */
                (*element).type_0 = CborInvalidType as libc::c_int as uint8_t;
                return CborNoError;
            }
        }
    }
    (*element).type_0 = CborInvalidType as libc::c_int as uint8_t;
    return err;
}
/* Floating point */
unsafe extern "C" fn cbor_value_is_half_float(mut value: *const CborValue) -> bool {
    return (*value).type_0 as libc::c_int == CborHalfFloatType as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_value_get_half_float(
    mut value: *const CborValue,
    mut result: *mut libc::c_void,
) -> CborError {
    let mut v: uint16_t = 0;
    if 0 != !cbor_value_is_half_float(value) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"cbor_value_get_half_float\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            1422i32,
            b"cbor_value_is_half_float(value)\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    /* size has been computed already */
    v = get16((*value).get_ptr_at_offset(1));
    memcpy(
        result,
        &mut v as *mut uint16_t as *const libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    return CborNoError;
}
#[no_mangle]
pub unsafe extern "C" fn _cbor_value_prepare_string_iteration(
    mut it: *mut CborValue,
) -> CborError {
    if 0 != !((*it).flags as libc::c_int & CborIteratorFlag_IteratingStringChunks as libc::c_int
        == 0i32) as libc::c_int as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"_cbor_value_prepare_string_iteration\x00",
            )).as_ptr(),
            b"src/cborparser.c\x00" as *const u8 as *const libc::c_char,
            987i32,
            b"(it->flags & CborIteratorFlag_IteratingStringChunks) == 0\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
    };
    prepare_string_iteration(it);
    /* are we at the end? */
    if (*it).at_end() {
        return CborErrorUnexpectedEOF;
    } else {
        return CborNoError;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cbor_value_get_string_chunk<'a>(
    mut value: &CborValue<'a>,
    mut bufferptr: *mut *const libc::c_void,
    mut len: *mut size_t,
    mut next: Option<&mut CborValue<'a>>,
) -> CborError {
    let mut tmp = CborValue::new();
    let mut next = match next {
        Some(n) => n,
        None => &mut tmp,
    };
    *next = *value;
    return get_string_chunk(next, bufferptr, len);
}
