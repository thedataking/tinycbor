#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
#![feature(libc, ptr_wrapping_offset_from)]
extern crate libc;
use cbor_h::{
    unnamed, CborArrayType, CborBooleanType, CborByteStringType, CborDoubleType, CborEncoder,
    CborError, CborErrorAdvancePastEOF, CborErrorDataTooLarge, CborErrorDuplicateObjectKeys,
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
    CborIteratorFlag_UnknownLength, CborMapType, CborNoError, CborNullType,
    CborParserIteratorFlags, CborSimpleType, CborTag, CborTagType, CborTextStringType, CborType,
    CborUndefinedType, CborUnknownError,
};
use cborinternal_p_h::{
    unnamed_0, ArrayType, Break, BreakByte, ByteStringType, CborMajorTypes, CborSimpleTypes,
    DoublePrecisionFloat, FalseValue, HalfPrecisionFloat, IndefiniteLength, MajorTypeMask,
    MajorTypeShift, MapType, NegativeIntegerType, NullValue, SimpleTypeInNextByte, SimpleTypesType,
    SinglePrecisionFloat, SmallValueBitLength, SmallValueMask, TagType, TextStringType, TrueValue,
    UndefinedValue, UnsignedIntegerType, Value16Bit, Value32Bit, Value64Bit, Value8Bit,
};
use stdlib::{
    __int64_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t, int64_t, memcpy, ptrdiff_t, size_t,
    uint16_t, uint32_t, uint64_t, uint8_t,
};

static mut CborIndefiniteLength: size_t = unsafe { 18446744073709551615u64 };
#[no_mangle]
pub unsafe extern "C" fn cbor_encoder_init(
    mut encoder: *mut CborEncoder,
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
    mut encoder: *mut CborEncoder,
    mut value: uint64_t,
) -> CborError {
    return encode_number(
        encoder,
        value,
        ((UnsignedIntegerType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
    );
}
unsafe extern "C" fn encode_number(
    mut encoder: *mut CborEncoder,
    mut ui: uint64_t,
    mut shiftedMajorType: uint8_t,
) -> CborError {
    saturated_decrement(encoder);
    return encode_number_no_update(encoder, ui, shiftedMajorType);
}
unsafe extern "C" fn encode_number_no_update(
    mut encoder: *mut CborEncoder,
    mut ui: uint64_t,
    mut shiftedMajorType: uint8_t,
) -> CborError {
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
    mut encoder: *mut CborEncoder,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> CborError {
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
unsafe extern "C" fn advance_ptr(mut encoder: *mut CborEncoder, mut n: size_t) -> () {
    if !(*encoder).end.is_null() {
        (*encoder).data.ptr = (*encoder).data.ptr.offset(n as isize)
    } else {
        (*encoder).data.bytes_needed = ((*encoder).data.bytes_needed as libc::c_ulong)
            .wrapping_add(n) as ptrdiff_t as ptrdiff_t
    };
}
unsafe extern "C" fn would_overflow(mut encoder: *mut CborEncoder, mut len: size_t) -> bool {
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
unsafe extern "C" fn saturated_decrement(mut encoder: *mut CborEncoder) -> () {
    if 0 != (*encoder).remaining {
        (*encoder).remaining = (*encoder).remaining.wrapping_sub(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_int(
    mut encoder: *mut CborEncoder,
    mut value: int64_t,
) -> CborError {
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
    mut encoder: *mut CborEncoder,
    mut absolute_value: uint64_t,
) -> CborError {
    return encode_number(
        encoder,
        absolute_value.wrapping_sub(1i32 as libc::c_ulong),
        ((NegativeIntegerType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_simple_value(
    mut encoder: *mut CborEncoder,
    mut value: uint8_t,
) -> CborError {
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
    mut encoder: *mut CborEncoder,
    mut tag: CborTag,
) -> CborError {
    /* tags don't count towards the number of elements in an array or map */
    return encode_number_no_update(
        encoder,
        tag,
        ((TagType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_text_string(
    mut encoder: *mut CborEncoder,
    mut string: *const libc::c_char,
    mut length: size_t,
) -> CborError {
    return encode_string(
        encoder,
        length,
        ((TextStringType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
        string as *const libc::c_void,
    );
}
unsafe extern "C" fn encode_string(
    mut encoder: *mut CborEncoder,
    mut length: size_t,
    mut shiftedMajorType: uint8_t,
    mut string: *const libc::c_void,
) -> CborError {
    let mut err: CborError = encode_number(encoder, length, shiftedMajorType);
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
unsafe extern "C" fn isOomError(mut err: CborError) -> bool {
    return 0 != 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_byte_string(
    mut encoder: *mut CborEncoder,
    mut string: *const uint8_t,
    mut length: size_t,
) -> CborError {
    return encode_string(
        encoder,
        length,
        ((ByteStringType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
        string as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encode_floating_point(
    mut encoder: *mut CborEncoder,
    mut fpType: CborType,
    mut value: *const libc::c_void,
) -> CborError {
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
    mut encoder: *mut CborEncoder,
    mut arrayEncoder: *mut CborEncoder,
    mut length: size_t,
) -> CborError {
    return create_container(
        encoder,
        arrayEncoder,
        length,
        ((ArrayType as libc::c_int) << MajorTypeShift as libc::c_int) as uint8_t,
    );
}
unsafe extern "C" fn create_container(
    mut encoder: *mut CborEncoder,
    mut container: *mut CborEncoder,
    mut length: size_t,
    mut shiftedMajorType: uint8_t,
) -> CborError {
    let mut err: CborError = CborNoError;
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
    mut encoder: *mut CborEncoder,
    mut byte: uint8_t,
) -> CborError {
    return append_to_buffer(
        encoder,
        &mut byte as *mut uint8_t as *const libc::c_void,
        1i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cbor_encoder_create_map(
    mut encoder: *mut CborEncoder,
    mut mapEncoder: *mut CborEncoder,
    mut length: size_t,
) -> CborError {
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
    mut encoder: *mut CborEncoder,
    mut containerEncoder: *const CborEncoder,
) -> CborError {
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
        }) as CborError;
    } else if (*encoder).end.is_null() {
        /* keep the state */
        return CborErrorOutOfMemory;
    } else {
        return CborNoError;
    };
}
