
extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_index_mut() {
    let mut df = DataFrame::new();
    let mut s1 = Series::arange(0, 5);
    s1.set_name("s1");
    df.add_column(s1);

    let s1 = Series::arange(5, 10);
    let sc = s1.clone();
    df["s1"] = s1;

    assert_eq!(&df["s1"], &sc);
}

#[test]
fn test_column_names() {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    let mut s1 = Series::arange(0, 2);
    let mut s2 = Series::arange(1, 3);

    let mut df = DataFrame::new();

    s1.set_name("s1");
    s2.set_name("s2");
    df.add_column(s1);
    df.add_column(s2);

    assert_eq!(
        df.columns(), 
        HashSet::from_iter(vec![&"s1".to_string(), &"s2".to_string()])
    );
}

#[test]
fn test_pretty_display() {
    let mut df = DataFrame::new();
    let s1 = Series::arange(0, 5);
    let s2 = Series::arange(5, 10);

    df.add_column(s1);
    df.add_column(s2);

    println!("{}", df);
}

#[test]
fn test_read_csv() {
    
    let path = format!("{}/tests/data/basic_csv.csv", env!("CARGO_MANIFEST_DIR"));
    println!("Using path: {}", &path);
    let df = DataFrame::read_csv(&path).expect("Unable to read file!");
    println!("Resulting DataFrame: {}", df);

}

#[test]
#[should_panic(expected = "No column")]
fn test_fail_index_column() {
    let df = DataFrame::new();
    let _series = &df["col doesn't exist!"];
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
    df.add_column(series1);
    df.add_column(series2);
    
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
