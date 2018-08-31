extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn supported_dtypes_check() {
    assert_eq!(0_f64.dtype(), DType::F64);
    assert_eq!(0_i64.dtype(), DType::I64);
    assert_eq!(0_f32.dtype(), DType::F32);
    assert_eq!(0_i32.dtype(), DType::I32);
}