extern crate blackjack;

use blackjack::prelude::*;


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


    let elem = DataElement::from_parse("5.0");
    assert_eq!(elem, DataElement::F64(5.0));

    let elem = DataElement::from_parse("5");
    assert_eq!(elem, DataElement::I64(5));

    let elem = DataElement::from_parse("STRING VALUE");
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