
extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_new_dataframe() {
    let df: DataFrame = DataFrame::new();
}

#[test]
fn test_len() {
    let series = Series::arange(0, 10);
}
