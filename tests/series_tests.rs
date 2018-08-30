extern crate blackjack;
extern crate num;

use blackjack::prelude::*;


#[test]
fn test_series_arange() {
    let series = Series::arange(0, 10);
    assert_eq!(series.len(), 10)
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


}
