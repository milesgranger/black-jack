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