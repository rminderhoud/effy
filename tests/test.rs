use std::default::Default;

use effy::FFI;

#[derive(Default, FFI)]
pub struct TestStruct {
    uint8: u8,
    uint16: u16,
    uint32: u32,
    uint64: u64,
    uint128: u128,
    int8: i8,
    int16: i16,
    int32: i32,
    int64: i64,
    int128: i128,
    float32: f32,
    float64: f64,
    boolean: bool,
}

#[test]
fn test() {
    use FFI::*;

    unsafe {
        let test = test_struct_new();

        // -- Primitives
        assert_eq!(test_struct_uint8(test), 0);
        assert_eq!(test_struct_uint16(test), 0);
        assert_eq!(test_struct_uint32(test), 0);
        assert_eq!(test_struct_uint64(test), 0);
        assert_eq!(test_struct_uint128(test), 0);
        assert_eq!(test_struct_int8(test), 0);
        assert_eq!(test_struct_int16(test), 0);
        assert_eq!(test_struct_int32(test), 0);
        assert_eq!(test_struct_int64(test), 0);
        assert_eq!(test_struct_int128(test), 0);
        assert_eq!(test_struct_float32(test), 0.0);
        assert_eq!(test_struct_float64(test), 0.0);
        assert_eq!(test_struct_boolean(test), false);

        test_struct_set_uint8(test, 13);
        test_struct_set_uint16(test, 13);
        test_struct_set_uint32(test, 13);
        test_struct_set_uint64(test, 13);
        test_struct_set_uint128(test, 13);
        test_struct_set_int8(test, 13);
        test_struct_set_int16(test, 13);
        test_struct_set_int32(test, 13);
        test_struct_set_int64(test, 13);
        test_struct_set_int128(test, 13);
        test_struct_set_float32(test, 13.13);
        test_struct_set_float64(test, 13.13);
        test_struct_set_boolean(test, true);

        assert_eq!(test_struct_uint8(test), 13);
        assert_eq!(test_struct_uint16(test), 13);
        assert_eq!(test_struct_uint32(test), 13);
        assert_eq!(test_struct_uint64(test), 13);
        assert_eq!(test_struct_uint128(test), 13);
        assert_eq!(test_struct_int8(test), 13);
        assert_eq!(test_struct_int16(test), 13);
        assert_eq!(test_struct_int32(test), 13);
        assert_eq!(test_struct_int64(test), 13);
        assert_eq!(test_struct_int128(test), 13);
        assert_eq!(test_struct_float32(test), 13.13);
        assert_eq!(test_struct_float64(test), 13.13);
        assert_eq!(test_struct_boolean(test), true);

        test_struct_free(test);
    }
}
