
extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_new_dataframe() {
    println!("Working!");
}

#[test]
fn test_add_columns() {
    let mut df = DataFrame::new();

    let mut series1: Series<i32> = Series::arange(0, 5);
    series1.set_name("series-1");

    let mut series2: Series<f64> = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    series2.set_name("series-2");

    df.add_column(series1);
    df.add_column(series2);

    let _series1ref: &Series<i32> = df.get_column("series-1").expect("No column named 'series-1'");
    let _series2ref: &Series<f64> = df.get_column("series-2").expect("No column named 'series-2'");

    assert_eq!(df.n_columns(), 2);

}

#[test]
fn test_get_column_by_name() {
    let mut df = DataFrame::new();
    let mut series: Series<i32> = Series::arange(0, 5);
    series.set_name("test-series");
    let series_clone = series.clone();

    df.add_column(series);

    let series_ref = df.get_column("test-series").expect("Unable to find column named 'test-series'");
    assert_eq!(*series_ref, series_clone);
}
