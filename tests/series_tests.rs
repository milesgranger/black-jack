extern crate blackjack;
extern crate num;

use blackjack::prelude::*;
use num::*;


#[test]
fn test_arange() {
    let _series: Series<i32> = Series::arange(0_i32, 5_i32);
}


#[test]
fn test_sum() {
    let _series = Series::arange(0, 5);
    
}