
extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_new_dataframe() {
    println!("Working!");
}

#[test]
fn test_add_columns() {
    let mut df = DataFrame::new();

    let series1: Series<i32> = Series::arange(0, 5);
    let series2: Series<f64> = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0]);

    df.add_column(series1);
    df.add_column(series2);
}

#[test]
fn test_get_column_by_name() {
    let mut df = DataFrame::new();
    let mut series: Series<i32> = Series::arange(0, 5);
    series.set_name("test-series");
    let series_clone = series.clone();

    df.add_column(series);

    let series_ref = df.get_column_ref("test-series").expect("Unable to find column named 'test-series'");
    assert_eq!(*series_ref, series_clone);
}
