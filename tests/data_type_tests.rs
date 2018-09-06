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