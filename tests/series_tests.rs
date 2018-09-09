extern crate blackjack;
extern crate num;
extern crate float_cmp;

use num::*;
use float_cmp::*;
use blackjack::prelude::*;


#[test]
fn test_astype_conversions() {
    let series_base = Series::from_data_elements(vec![
        DataElement::I64(1_i64),
        DataElement::F32(1_f32),
        DataElement::STRING("Hello".to_string())
    ]);
    let nan: f64 = Float::nan();  // Can't make NaN an integer directly

    // Test conversion to i64
    let mut series = series_base.clone();
    series.astype(DType::I64);
    assert_eq!(series, Series::from_vec(vec![1_i64, 1_i64, nan as i64]));

    // Test conversion to f64, special float comparison needed...
    let mut series = series_base.clone();
    series.astype(DType::F64);
    let vec = series.to_vec::<f64>();
    for (a, b) in vec.into_iter().zip(vec![1_f64, 1_f64, nan]) {
        assert!(a.approx_eq(&b, 0.000001, 1));
    }

    // Test conversion to string
    let mut series = series_base.clone();
    series.astype(DType::STRING);
    assert_eq!(
        series.to_vec::<String>(), 
        vec![1_i64.to_string(), 1_i64.to_string(), "Hello".to_string()]
    );
}

#[test]
fn test_display_series() {
    let mut series = Series::arange(0, 10);
    series.set_name("test-column");
    println!("{}", series);
}

#[test]
fn test_series_arange() {
    let series: Series = Series::arange(0, 10);
    assert_eq!(series.len(), 10);
    assert_eq!(series.dtype(), Some(DType::I32));
}

#[test]
fn test_series_from_vec() {
    let series = Series::from_vec(vec![1.0, 2.0, 3.0]);
    assert_eq!(series.len(), 3);
}

#[test]
fn test_series_naming() {
    let mut series = Series::from_vec(vec![1, 2, 3]);
    assert_eq!(series.name(), None);
    series.set_name("new-series");
    assert_eq!(series.name().unwrap(), "new-series".to_string());
}

#[test]
fn test_series_ops() {
    let series: Series = Series::arange(0, 5);

    // Test sum
    assert_eq!(series.sum::<f64>(), 10_f64);

    // Test mean
    assert_eq!(series.mean(), Ok(2.0));

    // Test min
    assert_eq!(series.min(), Ok(0_i32));

    // Test max
    assert_eq!(series.max(), Ok(4_i32));

}

#[test]
fn test_into_from_raw() {
    let series: Series = Series::arange(0, 5);
    let series_clone = series.clone();

    let ptr = series.into_raw();
    let recovered_series = Series::from_raw(ptr);
    assert_eq!(recovered_series, series_clone)


}
