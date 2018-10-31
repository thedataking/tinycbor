#![feature(libc)]
#![feature(extern_types)]
#![feature(asm)]
#![feature(ptr_wrapping_offset_from)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]

extern crate libc;

pub mod cbordump;

pub mod cborencoder;

pub mod cborencoder_close_container_checked;

pub mod cborerrorstrings;

pub mod cborparser;

pub mod cborparser_dup_string;

pub mod cborpretty;

pub mod cborpretty_stdio;

pub mod cbortojson;

pub mod cborvalidation;

pub fn main() {
    cbordump::main()
}
pub mod cborjson_h {
    pub const CborConvertIgnoreTags: CborToJsonFlags = 0;
    pub const CborConvertRequireMapStringKeys: CborToJsonFlags = 0;
    use libc;
    pub type CborToJsonFlags = libc::c_uint;
    extern "C" {
        #[no_mangle]
        pub fn cbor_value_to_json_advance(
            out: *mut FILE,
            value: *mut CborValue,
            flags: libc::c_int,
        ) -> CborError;
    }
    pub const CborConvertAddMetadata: CborToJsonFlags = 1;
    pub const CborConvertStringifyMapKeys: CborToJsonFlags = 8;
    pub const CborConvertObeyByteStringTags: CborToJsonFlags = 0;
    pub const CborConvertByteStringsToBase64Url: CborToJsonFlags = 4;
    pub const CborConvertTagsToObjects: CborToJsonFlags = 2;
    pub const CborConvertDefaultFlags: CborToJsonFlags = 0;
    use cbor_h::{CborError, CborValue};
    use stdlib::FILE;
}
pub mod stdarg_h {
    pub type va_list = __builtin_va_list;
    use stdlib::__builtin_va_list;
}
pub mod stdlib {
    extern "C" {}
    pub type __builtin_va_list = [__va_list_tag; 1];
    extern "C" {}
    extern "C" {
        #[no_mangle]
        pub fn __errno_location() -> *mut libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
    extern "C" {

        #[no_mangle]
        pub fn open_memstream(
            __bufloc: *mut *mut libc::c_char,
            __sizeloc: *mut size_t,
        ) -> *mut FILE;

        #[no_mangle]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    }
    extern "C" {}
    extern "C" {}
    extern "C" {
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;

    }
    extern "C" {}
    extern "C" {
        #[no_mangle]
        pub fn __fpclassifyl(__value: libc::c_double) -> libc::c_int;
        #[no_mangle]
        pub fn __fpclassify(__value: libc::c_double) -> libc::c_int;
        #[no_mangle]
        pub fn __fpclassifyf(__value: libc::c_float) -> libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        pub static mut optopt: libc::c_int;
        #[no_mangle]
        pub fn getopt(
            ___argc: libc::c_int,
            ___argv: *const *mut libc::c_char,
            __shortopts: *const libc::c_char,
        ) -> libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: *mut __va_list_tag)
            -> libc::c_int;
    }
    extern "C" {}
    extern "C" {}

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct _IO_marker {
        pub _next: *mut _IO_marker,
        pub _sbuf: *mut _IO_FILE,
        pub _pos: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
    pub type FILE = _IO_FILE;
    extern "C" {
        #[no_mangle]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
    }
    pub type __off64_t = libc::c_long;
    extern "C" {}
    pub type uintptr_t = libc::c_ulong;
    extern "C" {
        #[no_mangle]
        pub static mut stdin: *mut _IO_FILE;
        #[no_mangle]
        pub static mut stdout: *mut _IO_FILE;
        #[no_mangle]
        pub static mut stderr: *mut _IO_FILE;
        #[no_mangle]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        pub fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
        #[no_mangle]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
        #[no_mangle]
        pub fn puts(__s: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn fread(
            __ptr: *mut libc::c_void,
            __size: size_t,
            __n: size_t,
            __stream: *mut FILE,
        ) -> size_t;
        #[no_mangle]
        pub fn feof(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        pub fn ferror(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        pub fn perror(__s: *const libc::c_char) -> ();
    }
    pub type __int64_t = libc::c_long;
    pub type __off_t = libc::c_long;
    extern "C" {
        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void) -> ();
        #[no_mangle]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
    pub type uint8_t = __uint8_t;
    pub type __uint32_t = libc::c_uint;
    pub type int64_t = __int64_t;
    pub type uint_least32_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
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
    pub type __uint8_t = libc::c_uchar;
    pub type uint32_t = __uint32_t;
    pub type __uint16_t = libc::c_ushort;
    pub type __uint64_t = libc::c_ulong;
    pub type uint16_t = __uint16_t;
    pub type ptrdiff_t = libc::c_long;
    pub type size_t = libc::c_ulong;
    extern "C" {
        #[no_mangle]
        pub fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    }
    use libc;
    extern "C" {
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memcmp(
            _: *const libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
    pub type uint64_t = __uint64_t;
}
pub mod cbor_h {
    pub const CborUnixTime_tTag: CborKnownTags = 1;
    extern "C" {}
    pub const CborDateTimeStringTag: CborKnownTags = 0;
    pub const CborValidateNoUnknownTagsSA: CborValidationFlags = 268435456;
    pub const CborValidateUtf8: CborValidationFlags = 16384;
    pub const CborExpectedBase64Tag: CborKnownTags = 22;
    pub const CborValidateNoUnknownSimpleTypes: CborValidationFlags = 201326592;
    pub const CborNegativeBignumTag: CborKnownTags = 3;
    pub const CborValidateNoUnknownSimpleTypesSA: CborValidationFlags = 67108864;
    pub const CborValidateStrictMode: CborValidationFlags = 1048320;
    pub const CborValidateFiniteFloatingPoint: CborValidationFlags = 8388608;
    pub const CborValidateShortestNumbers: CborValidationFlags = 3;
    pub const CborBase64Tag: CborKnownTags = 34;
    pub const CborValidateStrictest: CborValidationFlags = -1;
    pub const CborCOSE_EncryptTag: CborKnownTags = 96;
    pub type CborStreamFunction = Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, ...) -> CborError,
    >;
    pub const CborValidateNoUnknownTags: CborValidationFlags = 1879048192;
    pub const CborValidateNoUnknownTagsSR: CborValidationFlags = 805306368;
    pub const CborCOSE_Sign1Tag: CborKnownTags = 18;
    pub const CborValidateCanonicalFormat: CborValidationFlags = 4095;
    pub const CborRegularExpressionTag: CborKnownTags = 35;
    pub const CborPrettyIndicateIndeterminateLength: CborPrettyFlags = 2;
    pub const CborCOSE_Encrypt0Tag: CborKnownTags = 16;
    pub const CborCOSE_Mac0Tag: CborKnownTags = 17;
    extern "C" {
        #[no_mangle]
        pub fn cbor_value_to_pretty_stream(
            streamFunction: CborStreamFunction,
            token: *mut libc::c_void,
            value: *mut CborValue,
            flags: libc::c_int,
        ) -> CborError;
    }
    pub const CborValidateNoIndeterminateLength: CborValidationFlags = 256;
    extern "C" {
        #[no_mangle]
        pub fn cbor_encoder_close_container(
            encoder: *mut CborEncoder,
            containerEncoder: *const CborEncoder,
        ) -> CborError;
    }
    pub type CborPrettyFlags = libc::c_uint;
    pub const CborSignatureTag: CborKnownTags = 55799;
    pub const CborByteStringType: CborType = 64;
    pub const CborErrorDataTooLarge: CborError = 1024;
    pub const CborPrettyDefaultFlags: CborPrettyFlags = 2;
    pub const CborCOSE_MacTag: CborKnownTags = 97;
    pub const CborTagType: CborType = 192;
    extern "C" {
        #[no_mangle]
        pub fn cbor_error_string(error: CborError) -> *const libc::c_char;
        #[no_mangle]
        pub fn cbor_parser_init(
            buffer: *const uint8_t,
            size: size_t,
            flags: uint32_t,
            parser: *mut CborParser,
            it: *mut CborValue,
        ) -> CborError;
        /* The following API requires a hosted C implementation (uses FILE*) */
        #[no_mangle]
        pub fn cbor_value_to_pretty_advance_flags(
            out: *mut FILE,
            value: *mut CborValue,
            flags: libc::c_int,
        ) -> CborError;
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CborEncoder {
        pub data: unnamed,
        pub end: *const uint8_t,
        pub remaining: size_t,
        pub flags: libc::c_int,
    }
    pub const CborPositiveBignumTag: CborKnownTags = 2;
    pub const CborEncodedCborTag: CborKnownTags = 24;
    pub const CborPrettyMergeStringFragments: CborPrettyFlags = 0;
    pub const CborValidateNoUndefined: CborValidationFlags = 2097152;
    pub const CborTextStringType: CborType = 96;
    pub type CborValidationFlags = libc::c_int;
    pub const CborValidateMapKeysAreString: CborValidationFlags = 1048576;
    pub const CborMimeMessageTag: CborKnownTags = 36;
    pub const CborErrorUnknownTag: CborError = 513;
    pub const CborBigfloatTag: CborKnownTags = 5;
    pub const CborCOSE_SignTag: CborKnownTags = 98;
    pub const CborIteratorFlag_UnknownLength: CborParserIteratorFlags = 4;
    pub const CborValidateBasic: CborValidationFlags = 0;
    pub const CborInvalidType: CborType = 255;
    pub const CborBase64urlTag: CborKnownTags = 33;
    pub const CborExpectedBase64urlTag: CborKnownTags = 21;
    pub const CborValidateNoTags: CborValidationFlags = 4194304;
    pub const CborUrlTag: CborKnownTags = 32;
    pub const CborErrorOverlongEncoding: CborError = 520;
    pub const CborValidateMapKeysAreUnique: CborValidationFlags = 4864;
    pub const CborIteratorFlag_ContainerIsMap: CborParserIteratorFlags = 32;
    pub const CborPrettyTextualEncodingIndicators: CborPrettyFlags = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CborParser {
        pub end: *const uint8_t,
        pub flags: uint32_t,
    }
    pub const CborIteratorFlag_IntegerValueTooLarge: CborParserIteratorFlags = 1;
    pub const CborErrorIllegalNumber: CborError = 261;
    pub const CborExpectedBase16Tag: CborKnownTags = 23;
    pub const CborIteratorFlag_NegativeInteger: CborParserIteratorFlags = 2;
    pub type CborTag = uint64_t;
    pub const CborPrettyIndicateIndetermineLength: CborPrettyFlags = 2;
    extern "C" {

        #[no_mangle]
        pub fn _cbor_value_copy_string(
            value: *const CborValue,
            buffer: *mut libc::c_void,
            buflen: *mut size_t,
            next: *mut CborValue,
        ) -> CborError;
        #[no_mangle]
        pub fn _cbor_value_dup_string(
            value: *const CborValue,
            buffer: *mut *mut libc::c_void,
            buflen: *mut size_t,
            next: *mut CborValue,
        ) -> CborError;
        #[no_mangle]
        pub fn cbor_value_calculate_string_length(
            value: *const CborValue,
            length: *mut size_t,
        ) -> CborError;

        #[no_mangle]
        pub fn cbor_value_to_pretty_advance(out: *mut FILE, value: *mut CborValue) -> CborError;
    }
    pub const CborValidateMapIsSorted: CborValidationFlags = 768;
    pub const CborErrorUnknownLength: CborError = 2;
    pub type CborKnownTags = libc::c_uint;
    pub const CborFloatType: CborType = 250;
    pub const CborDoubleType: CborType = 251;
    pub const CborPrettyShowStringFragments: CborPrettyFlags = 256;
    pub const CborErrorInternalError: CborError = 2147483647;
    pub const CborErrorNestingTooDeep: CborError = 1025;
    pub const CborValidateShortestIntegrals: CborValidationFlags = 1;
    extern "C" {}
    pub const CborPrettyNumericEncodingIndicators: CborPrettyFlags = 1;
    pub const CborValidateCompleteData: CborValidationFlags = -2147483648;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union unnamed {
        pub ptr: *mut uint8_t,
        pub bytes_needed: ptrdiff_t,
    }
    pub const CborErrorGarbageAtEnd: CborError = 256;
    pub const CborErrorUnexpectedEOF: CborError = 257;
    pub type CborType = libc::c_uint;
    pub const CborErrorOutOfMemory: CborError = -2147483648;
    pub const CborUndefinedType: CborType = 247;
    pub const CborIteratorFlag_IteratingStringChunks: CborParserIteratorFlags = 2;
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
    pub const CborIntegerType: CborType = 0;
    pub const CborErrorExcludedValue: CborError = 518;
    pub const CborErrorUnsupportedType: CborError = 1026;
    pub const CborDecimalTag: CborKnownTags = 4;
    pub const CborSimpleType: CborType = 224;
    pub const CborErrorMapKeysNotUnique: CborError = 523;
    pub const CborErrorAdvancePastEOF: CborError = 3;
    pub const CborValidateTagUse: CborValidationFlags = 8192;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CborValue {
        pub parser: *const CborParser,
        pub ptr: *const uint8_t,
        pub remaining: uint32_t,
        pub extra: uint16_t,
        pub type_0: uint8_t,
        pub flags: uint8_t,
    }
    pub const CborMapType: CborType = 160;
    pub const CborErrorUnknownType: CborError = 259;
    pub const CborErrorTooManyItems: CborError = 768;
    pub const CborErrorUnexpectedBreak: CborError = 258;
    pub const CborUnknownError: CborError = 1;
    pub type CborParserIteratorFlags = libc::c_uint;
    pub const CborBooleanType: CborType = 245;
    pub const CborErrorIllegalType: CborError = 260;
    pub const CborHalfFloatType: CborType = 249;
    pub const CborErrorUnknownSimpleType: CborError = 512;
    pub const CborErrorIllegalSimpleType: CborError = 262;
    pub const CborErrorImproperValue: CborError = 519;
    pub const CborErrorExcludedType: CborError = 517;
    pub const CborErrorDuplicateObjectKeys: CborError = 515;
    pub const CborErrorInappropriateTagForType: CborError = 514;
    pub const CborValidateShortestFloatingPoint: CborValidationFlags = 2;
    pub const CborPrettyIndicateOverlongNumbers: CborPrettyFlags = 4;
    pub const CborErrorMapKeyNotString: CborError = 521;
    pub const CborErrorIO: CborError = 4;
    pub const CborErrorJsonObjectKeyIsAggregate: CborError = 1280;
    pub type CborError = libc::c_int;
    pub const CborErrorMapNotSorted: CborError = 522;
    pub const CborNullType: CborType = 246;
    pub const CborNoError: CborError = 0;
    pub const CborErrorInvalidUtf8TextString: CborError = 516;
    pub const CborErrorTooFewItems: CborError = 769;
    use libc;
    pub const CborErrorJsonObjectKeyNotString: CborError = 1281;
    pub const CborArrayType: CborType = 128;
    pub const CborErrorJsonNotImplemented: CborError = 1282;
    use stdlib::{int64_t, ptrdiff_t, size_t, uint16_t, uint32_t, uint64_t, uint8_t, FILE};
}
pub mod cborinternal_p_h {
    pub const UndefinedValue: CborSimpleTypes = 23;
    pub const SinglePrecisionFloat: CborSimpleTypes = 26;
    pub const TagType: CborMajorTypes = 6;
    extern "C" {}
    pub const DoublePrecisionFloat: CborSimpleTypes = 27;
    pub const FalseValue: CborSimpleTypes = 20;
    pub const MajorTypeMask: unnamed = -32;
    pub const MapType: CborMajorTypes = 5;
    pub const Value16Bit: unnamed = 25;
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
    pub const ArrayType: CborMajorTypes = 4;
    pub const TextStringType: CborMajorTypes = 3;
    pub const SmallValueBitLength: unnamed_0 = 5;
    pub const SimpleTypeInNextByte: CborSimpleTypes = 24;
    pub type unnamed_0 = libc::c_int;
    pub type CborMajorTypes = libc::c_uint;
    pub const Value32Bit: unnamed = 26;
    use libc;
    pub const UnsignedIntegerType: CborMajorTypes = 0;
    pub const MajorTypeShift: unnamed = 5;
    pub const NullValue: CborSimpleTypes = 22;
    pub const ByteStringType: CborMajorTypes = 2;
    pub const SimpleTypesType: CborMajorTypes = 7;
    pub const Break: CborSimpleTypes = 31;
    pub const NegativeIntegerType: CborMajorTypes = 1;
    pub const Value8Bit: unnamed = 24;
    pub const SmallValueMask: unnamed_0 = 31;
    pub const TrueValue: CborSimpleTypes = 21;
    pub type unnamed = libc::c_int;
    pub type CborSimpleTypes = libc::c_uint;
    pub const BreakByte: unnamed_0 = 255;
    pub const IndefiniteLength: unnamed = 31;
    pub const Value64Bit: unnamed = 27;
    pub const HalfPrecisionFloat: CborSimpleTypes = 25;
    use cbor_h::{CborError, CborValue};
    use stdlib::{size_t, uint64_t, uint8_t};
}
