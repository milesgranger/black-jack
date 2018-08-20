
extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_new_dataframe() {
    let _df: DataFrame = DataFrame::new();
}

#[test]
fn test_add_columns_same_length() {
    let series_int = Series::arange(0, 5);
    let series_flt = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let mut df = DataFrame::new();
    df.add_column(series_int).unwrap();
    df.add_column(series_flt).unwrap();
}
