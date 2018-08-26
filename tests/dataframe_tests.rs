
extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_new_dataframe() {
    println!("Working!");
}

#[test]
fn test_add_columns_same_length() {
    let mut df = DataFrame::new();

    let series1: Series<i32> = Series::arange(0, 5);
    let series2: Series<f64> = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0]);

    df.add_column(series1);
    df.add_column(series2);
}
