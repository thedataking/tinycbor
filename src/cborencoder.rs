use libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type CborType_0 = CborType;
pub type CborTag = uint64_t;
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
/* Encoder API */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CborEncoder {
    pub data: unnamed,
    pub end: *const uint8_t,
    pub remaining: size_t,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed {
    ptr: *mut uint8_t,
    bytes_needed: ptrdiff_t,
}
pub type CborEncoder_0 = CborEncoder;
pub const MajorTypeShift: unnamed_0 = 5;
pub const UnsignedIntegerType: CborMajorTypes = 0;
pub const Value8Bit: unnamed_0 = 24;
pub const NegativeIntegerType: CborMajorTypes = 1;
pub const SimpleTypesType: CborMajorTypes = 7;
pub const Break: CborSimpleTypes = 31;
/* ditto */
pub const HalfPrecisionFloat: CborSimpleTypes = 25;
pub const TagType: CborMajorTypes = 6;
pub const TextStringType: CborMajorTypes = 3;
pub const ByteStringType: CborMajorTypes = 2;
pub const ArrayType: CborMajorTypes = 4;
pub const CborIteratorFlag_ContainerIsMap: CborParserIteratorFlags = 32;
pub const IndefiniteLength: unnamed_0 = 31;
pub const CborIteratorFlag_UnknownLength: CborParserIteratorFlags = 4;
/* a.k.a. object */
pub const MapType: CborMajorTypes = 5;
pub const BreakByte: unnamed_0 = 255;
/* Parser API */
pub type CborParserIteratorFlags = libc::c_uint;
pub const CborIteratorFlag_IteratingStringChunks: CborParserIteratorFlags = 2;
pub const CborIteratorFlag_NegativeInteger: CborParserIteratorFlags = 2;
pub const CborIteratorFlag_IntegerValueTooLarge: CborParserIteratorFlags = 1;
pub type CborMajorTypes = libc::c_uint;
pub type CborSimpleTypes = libc::c_uint;
/* ditto */
pub const DoublePrecisionFloat: CborSimpleTypes = 27;
/* ditto */
pub const SinglePrecisionFloat: CborSimpleTypes = 26;
/* not really a simple type */
pub const SimpleTypeInNextByte: CborSimpleTypes = 24;
pub const UndefinedValue: CborSimpleTypes = 23;
pub const NullValue: CborSimpleTypes = 22;
pub const TrueValue: CborSimpleTypes = 21;
pub const FalseValue: CborSimpleTypes = 20;
pub type unnamed_0 = libc::c_int;
pub const MajorTypeMask: unnamed_0 = -32;
pub const Value64Bit: unnamed_0 = 27;
pub const Value32Bit: unnamed_0 = 26;
pub const Value16Bit: unnamed_0 = 25;
/* 31 */
pub const SmallValueMask: unnamed_0 = 31;
pub const SmallValueBitLength: unnamed_0 = 5;
static mut CborIndefiniteLength: size_t = unsafe { 18446744073709551615u64 };
#[no_mangle]
pub unsafe extern "C" fn cbor_encoder_init(
    mut encoder: *mut CborEncoder_0,
    mut buffer: *mut uint8_t,
    mut size: size_t,
    mut flags: libc::c_int,
) -> () {
    (*encoder).data.ptr = buffer;
    (*encoder).end = buffer.offset(size as isize);
    (*encoder).remaining = 2i32 as size_t;
    (*encoder).flags = flags;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_uint(
    mut encoder: *mut CborEncoder_0,
    mut value: uint64_t,
) -> CborError_0 {
    return encode_number(
        encoder,
        value,
        ((UnsignedIntegerType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
    );
}
unsafe extern "C" fn encode_number(
    mut encoder: *mut CborEncoder_0,
    mut ui: uint64_t,
    mut shiftedMajorType: uint8_t,
) -> CborError_0 {
    saturated_decrement(encoder);
    return encode_number_no_update(encoder, ui, shiftedMajorType);
}
unsafe extern "C" fn encode_number_no_update(
    mut encoder: *mut CborEncoder_0,
    mut ui: uint64_t,
    mut shiftedMajorType: uint8_t,
) -> CborError_0 {
    /* Little-endian would have been so much more convenient here:
     * We could just write at the beginning of buf but append_to_buffer
     * only the necessary bytes.
     * Since it has to be big endian, do it the other way around:
     * write from the end. */
    let mut buf: [uint64_t; 2] = [0; 2];
    let bufend: *mut uint8_t = (buf.as_mut_ptr() as *mut uint8_t)
        .offset(::std::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong as isize);
    let mut bufstart: *mut uint8_t = bufend.offset(-1isize);
    /* we probably have a bunch of zeros in the beginning */
    put64(buf.as_mut_ptr().offset(1isize) as *mut libc::c_void, ui);
    if ui < Value8Bit as libc::c_int as libc::c_ulong {
        *bufstart = (*bufstart as libc::c_int + shiftedMajorType as libc::c_int) as uint8_t
    } else {
        let mut more: uint8_t = 0i32 as uint8_t;
        if ui > 0xffu32 as libc::c_ulong {
            more = more.wrapping_add(1)
        }
        if ui > 0xffffu32 as libc::c_ulong {
            more = more.wrapping_add(1)
        }
        if ui > 0xffffffffu32 as libc::c_ulong {
            more = more.wrapping_add(1)
        }
        bufstart = bufstart.offset(-(((1i32 as size_t) << more as libc::c_int) as isize));
        *bufstart = (shiftedMajorType as libc::c_int
            + Value8Bit as libc::c_int
            + more as libc::c_int) as uint8_t
    }
    return append_to_buffer(
        encoder,
        bufstart as *const libc::c_void,
        bufend.wrapping_offset_from(bufstart) as libc::c_long as size_t,
    );
}
unsafe extern "C" fn append_to_buffer(
    mut encoder: *mut CborEncoder_0,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> CborError_0 {
    if would_overflow(encoder, len) {
        if !(*encoder).end.is_null() {
            len = (len as libc::c_ulong)
                .wrapping_sub((*encoder).end.wrapping_offset_from((*encoder).data.ptr)
                    as libc::c_long as libc::c_ulong) as size_t as size_t;
            (*encoder).end = 0 as *const uint8_t;
            (*encoder).data.bytes_needed = 0i32 as ptrdiff_t
        }
        advance_ptr(encoder, len);
        return CborErrorOutOfMemory;
    } else {
        memcpy((*encoder).data.ptr as *mut libc::c_void, data, len);
        (*encoder).data.ptr = (*encoder).data.ptr.offset(len as isize);
        return CborNoError;
    };
}
unsafe extern "C" fn advance_ptr(mut encoder: *mut CborEncoder_0, mut n: size_t) -> () {
    if !(*encoder).end.is_null() {
        (*encoder).data.ptr = (*encoder).data.ptr.offset(n as isize)
    } else {
        (*encoder).data.bytes_needed = ((*encoder).data.bytes_needed as libc::c_ulong)
            .wrapping_add(n) as ptrdiff_t as ptrdiff_t
    };
}
unsafe extern "C" fn would_overflow(mut encoder: *mut CborEncoder_0, mut len: size_t) -> bool {
    let mut remaining: ptrdiff_t = (*encoder).end as ptrdiff_t;
    remaining -= if 0 != remaining {
        (*encoder).data.ptr as ptrdiff_t
    } else {
        (*encoder).data.bytes_needed
    };
    remaining -= len as ptrdiff_t;
    return 0 != (remaining < 0i32 as libc::c_long) as libc::c_int as libc::c_long;
}
unsafe extern "C" fn put64(mut where_0: *mut libc::c_void, mut v: uint64_t) -> () {
    v = (v as libc::c_ulonglong).swap_bytes() as uint64_t;
    memcpy(
        where_0,
        &mut v as *mut uint64_t as *const libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn saturated_decrement(mut encoder: *mut CborEncoder_0) -> () {
    if 0 != (*encoder).remaining {
        (*encoder).remaining = (*encoder).remaining.wrapping_sub(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_int(
    mut encoder: *mut CborEncoder_0,
    mut value: int64_t,
) -> CborError_0 {
    /* adapted from code in RFC 7049 appendix C (pseudocode) */
    /* extend sign to whole length */
    let mut ui: uint64_t = (value >> 63i32) as uint64_t;
    /* extract major type */
    let mut majorType: uint8_t = (ui & 0x20i32 as libc::c_ulong) as uint8_t;
    /* complement negatives */
    ui ^= value as libc::c_ulong;
    return encode_number(encoder, ui, majorType);
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_negative_int(
    mut encoder: *mut CborEncoder_0,
    mut absolute_value: uint64_t,
) -> CborError_0 {
    return encode_number(
        encoder,
        absolute_value.wrapping_sub(1i32 as libc::c_ulong),
        ((NegativeIntegerType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_simple_value(
    mut encoder: *mut CborEncoder_0,
    mut value: uint8_t,
) -> CborError_0 {
    /* check if this is a valid simple type */
    if value as libc::c_int >= HalfPrecisionFloat as libc::c_int
        && value as libc::c_int <= Break as libc::c_int
    {
        return CborErrorIllegalSimpleType;
    } else {
        return encode_number(
            encoder,
            value as uint64_t,
            ((SimpleTypesType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_tag(
    mut encoder: *mut CborEncoder_0,
    mut tag: CborTag,
) -> CborError_0 {
    /* tags don't count towards the number of elements in an array or map */
    return encode_number_no_update(
        encoder,
        tag,
        ((TagType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_text_string(
    mut encoder: *mut CborEncoder_0,
    mut string: *const libc::c_char,
    mut length: size_t,
) -> CborError_0 {
    return encode_string(
        encoder,
        length,
        ((TextStringType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
        string as *const libc::c_void,
    );
}
unsafe extern "C" fn encode_string(
    mut encoder: *mut CborEncoder_0,
    mut length: size_t,
    mut shiftedMajorType: uint8_t,
    mut string: *const libc::c_void,
) -> CborError_0 {
    let mut err: CborError_0 = encode_number(encoder, length, shiftedMajorType);
    if 0 != err as libc::c_int && !isOomError(err) {
        return err;
    } else {
        return append_to_buffer(encoder, string, length);
    };
}
/* Note: Since this is currently only used in situations where OOM is the only
 * valid error, we KNOW this to be true.  Thus, this function now returns just 'true',
 * but if in the future, any function starts returning a non-OOM error, this will need
 * to be changed to the test.  At the moment, this is done to prevent more branches
 * being created in the tinycbor output */
unsafe extern "C" fn isOomError(mut err: CborError_0) -> bool {
    return 0 != 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_byte_string(
    mut encoder: *mut CborEncoder_0,
    mut string: *const uint8_t,
    mut length: size_t,
) -> CborError_0 {
    return encode_string(
        encoder,
        length,
        ((ByteStringType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
        string as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_floating_point(
    mut encoder: *mut CborEncoder_0,
    mut fpType: CborType_0,
    mut value: *const libc::c_void,
) -> CborError_0 {
    let mut size: libc::c_uint = 0;
    let mut buf: [uint8_t; 9] = [0; 9];
    buf[0usize] = fpType as uint8_t;
    size = 2u32
        << (fpType as libc::c_uint).wrapping_sub(CborHalfFloatType as libc::c_int as libc::c_uint);
    if size == 8i32 as libc::c_uint {
        put64(
            buf.as_mut_ptr().offset(1isize) as *mut libc::c_void,
            *(value as *const uint64_t),
        );
    } else if size == 4i32 as libc::c_uint {
        put32(
            buf.as_mut_ptr().offset(1isize) as *mut libc::c_void,
            *(value as *const uint32_t),
        );
    } else {
        put16(
            buf.as_mut_ptr().offset(1isize) as *mut libc::c_void,
            *(value as *const uint16_t),
        );
    }
    saturated_decrement(encoder);
    return append_to_buffer(
        encoder,
        buf.as_mut_ptr() as *const libc::c_void,
        size.wrapping_add(1i32 as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn put16(mut where_0: *mut libc::c_void, mut v: uint16_t) -> () {
    v = v.swap_bytes();
    memcpy(
        where_0,
        &mut v as *mut uint16_t as *const libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn put32(mut where_0: *mut libc::c_void, mut v: uint32_t) -> () {
    v = v.swap_bytes();
    memcpy(
        where_0,
        &mut v as *mut uint32_t as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encoder_create_array(
    mut encoder: *mut CborEncoder_0,
    mut arrayEncoder: *mut CborEncoder_0,
    mut length: size_t,
) -> CborError_0 {
    return create_container(
        encoder,
        arrayEncoder,
        length,
        ((ArrayType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
    );
}
unsafe extern "C" fn create_container(
    mut encoder: *mut CborEncoder_0,
    mut container: *mut CborEncoder_0,
    mut length: size_t,
    mut shiftedMajorType: uint8_t,
) -> CborError_0 {
    let mut err: CborError_0 = CborNoError;
    (*container).data.ptr = (*encoder).data.ptr;
    (*container).end = (*encoder).end;
    saturated_decrement(encoder);
    /* overflow ok on CborIndefiniteLength */
    (*container).remaining = length.wrapping_add(1i32 as libc::c_ulong);
    (*container).flags =
        shiftedMajorType as libc::c_int & CborIteratorFlag_ContainerIsMap as libc::c_int;
    if length == CborIndefiniteLength {
        (*container).flags |= CborIteratorFlag_UnknownLength as libc::c_int;
        err = append_byte_to_buffer(
            container,
            (shiftedMajorType as libc::c_int + IndefiniteLength as libc::c_int) as uint8_t,
        )
    } else {
        if 0 != shiftedMajorType as libc::c_int & CborIteratorFlag_ContainerIsMap as libc::c_int {
            (*container).remaining =
                ((*container).remaining as libc::c_ulong).wrapping_add(length) as size_t as size_t
        }
        err = encode_number_no_update(container, length, shiftedMajorType)
    }
    return err;
}
unsafe extern "C" fn append_byte_to_buffer(
    mut encoder: *mut CborEncoder_0,
    mut byte: uint8_t,
) -> CborError_0 {
    return append_to_buffer(
        encoder,
        &mut byte as *mut uint8_t as *const libc::c_void,
        1i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encoder_create_map(
    mut encoder: *mut CborEncoder_0,
    mut mapEncoder: *mut CborEncoder_0,
    mut length: size_t,
) -> CborError_0 {
    if length != CborIndefiniteLength
        && length > 18446744073709551615u64.wrapping_div(2i32 as libc::c_ulong)
    {
        return CborErrorDataTooLarge;
    } else {
        return create_container(
            encoder,
            mapEncoder,
            length,
            ((MapType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encoder_close_container(
    mut encoder: *mut CborEncoder_0,
    mut containerEncoder: *const CborEncoder_0,
) -> CborError_0 {
    if !(*encoder).end.is_null() {
        (*encoder).data.ptr = (*containerEncoder).data.ptr
    } else {
        (*encoder).data.bytes_needed = (*containerEncoder).data.bytes_needed
    }
    (*encoder).end = (*containerEncoder).end;
    if 0 != (*containerEncoder).flags & CborIteratorFlag_UnknownLength as libc::c_int {
        return append_byte_to_buffer(encoder, BreakByte as libc::c_int as uint8_t);
    } else if (*containerEncoder).remaining != 1i32 as libc::c_ulong {
        return (if (*containerEncoder).remaining == 0i32 as libc::c_ulong {
            CborErrorTooManyItems as libc::c_int
        } else {
            CborErrorTooFewItems as libc::c_int
        }) as CborError_0;
    } else if (*encoder).end.is_null() {
        /* keep the state */
        return CborErrorOutOfMemory;
    } else {
        return CborNoError;
    };
}
