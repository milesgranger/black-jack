extern crate blackjack;
extern crate num;

use blackjack::prelude::*;
use num::*;


#[test]
fn test_arange() {
    let _series: Series<i32> = Series::arange(0_i32, 5_i32);
}


#[test]
fn test_len() {
    let _series = Series::arange(0, 5);
    assert_eq!(_series.len(), 5);
}

#[test]
fn test_sum() {
    let series = Series::arange(0, 5);
    assert_eq!(series.sum(), 10);
}

#[test]
fn test_from_vec() {
    let series = Series::from_vec(vec![1.0, 2.0, 3.0]);
    assert_eq!(series.sum(), 6.0);
}