extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_data_element_ops() {

    // Test multiplication
    assert_eq!(DataElement::I32(2)   * 2_i32, DataElement::I32(4_i32));
    assert_eq!(DataElement::F32(2.0) * 2_f32, DataElement::F32(4_f32));
    assert_eq!(DataElement::I64(2)   * 2_i64, DataElement::I64(4_i64));
    assert_eq!(DataElement::F64(2.0) * 2_f64, DataElement::F64(4_f64));

    // Test division
    assert_eq!(DataElement::I32(4)   / 2_i32, DataElement::I32(2_i32));
    assert_eq!(DataElement::F32(4.0) / 2_f64, DataElement::F64(2_f64));
    assert_eq!(DataElement::I64(4)   / 2_i64, DataElement::I64(2_i64));
    assert_eq!(DataElement::F64(4.0) / 2_f32, DataElement::F32(2_f32));


    // Test addition
    assert_eq!(DataElement::I32(2)   + 2.5_f32, DataElement::F32(4.5));
    assert_eq!(DataElement::F32(2.0) + 2.0_f32, DataElement::F32(4.0));
    assert_eq!(DataElement::I64(2)   + 2_i64,   DataElement::I64(4));
    assert_eq!(DataElement::F64(2.0) + 2_f64,   DataElement::F64(4.0));


    // Test subtraction
    assert_eq!(DataElement::I32(2)   - 2.5_f32, DataElement::F32(-0.5));
    assert_eq!(DataElement::F32(2.0) - 2_f32,   DataElement::F32(0.0));
    assert_eq!(DataElement::I64(2)   - 2_i64,   DataElement::I64(0));
    assert_eq!(DataElement::F64(2.0) - 2_f32,   DataElement::F32(0.0));
}

#[test]
fn supported_dtypes_check() {
    assert_eq!(0_f64.dtype(), DType::F64);
    assert_eq!(0_i64.dtype(), DType::I64);
    assert_eq!(0_f32.dtype(), DType::F32);
    assert_eq!(0_i32.dtype(), DType::I32);
    assert_eq!("value".to_string().dtype(), DType::STRING);
}

#[test]
fn test_data_element_parsing() {


    let elem = DataElement::from_str_parse("5.0");
    assert_eq!(elem, DataElement::F64(5.0));

    let elem = DataElement::from_str_parse("5");
    assert_eq!(elem, DataElement::I64(5));

    let elem = DataElement::from_str_parse("STRING VALUE");
    assert_eq!(elem, DataElement::STRING("STRING VALUE".to_string()));

}

#[test]
fn test_data_element_froms() {

    // Test from/into DataElement from primitives
    let _: DataElement = "value".to_string().into();
    let _: DataElement = 1_i64.into();
    let _: DataElement = 1_f64.into();
    let _: DataElement = 1_i32.into();
    let _: DataElement = 1_f64.into();

    // Test from/into primitives from DataElement
    let _: String = DataElement::STRING("val".to_string()).into();
    let _: i64 = DataElement::I64(1_i64).into();
    let _: f64 = DataElement::F64(1_f64).into();
    let _: i32 = DataElement::I32(1_i32).into();
    let _: f32 = DataElement::F32(1_f32).into();
}