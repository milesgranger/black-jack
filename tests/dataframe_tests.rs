
extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_read_csv() {
    
    let path = format!("{}/tests/data/basic_csv.csv", env!("CARGO_MANIFEST_DIR"));
    println!("Using path: {}", &path);
    let df = DataFrame::read_csv(&path).expect("Unable to read file!");

}

#[test]
fn test_add_columns() {
    let mut df = DataFrame::new();

    let mut series1: Series = Series::arange(0, 5);
    series1.set_name("series-1");
    let series1_clone = series1.clone();

    let mut series2: Series = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    series2.set_name("series-2");
    let series2_clone = series2.clone();

    // Add both columns
    df.add_column(series1);   // Add by method
    df["series-2"] = series2; // Add by IndexMut
    
    {
        // Test the columns match
        let series1ref: &Series = df.get_column("series-1").expect("No column named 'series-1'");
        assert_eq!(*series1ref, series1_clone);

        let series2ref: &Series = df.get_column("series-2").expect("No column named 'series-2'");
        assert_eq!(*series2ref, series2_clone);

        // Just sanity check two columns are still there.
        assert_eq!(df.n_columns(), 2);
    }

    // Test into and from raw pointer 
    let ptr = df.into_raw();
    let df  = DataFrame::from_raw(ptr);
    assert_eq!(df.n_columns(), 2);


}

#[test]
fn test_get_column_by_name() {
    let mut df = DataFrame::new();
    let mut series: Series = Series::arange(0, 5);
    series.set_name("test-series");
    let series_clone = series.clone();

    df.add_column(series);

    let series_ref: &Series = df.get_column("test-series").expect("Unable to find column named 'test-series'");
    assert_eq!(*series_ref, series_clone);

    let series_ref: &Series = &df["test-series"];
    assert_eq!(*series_ref, series_clone);
}
