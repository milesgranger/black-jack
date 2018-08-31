extern crate blackjack;
extern crate num;

use blackjack::prelude::*;


#[test]
fn test_series_arange() {
    let series: Series<i32> = Series::arange(0, 10);
    assert_eq!(series.len(), 10);
    assert_eq!(series.dtype(), DType::I32);
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
    let series: Series<i32> = Series::arange(0, 5);

    // Test sum
    assert_eq!(series.sum(), 10_i32);

    // Test mean
    assert_eq!(series.mean::<f32>().unwrap(), 2.0);

}

#[test]
fn test_into_from_raw() {
    let series: Series<i64> = Series::arange(0, 5);
    let series_clone = series.clone();

    let ptr = series.into_raw();
    let recovered_series = Series::from_raw(ptr);
    assert_eq!(recovered_series, series_clone)


}
