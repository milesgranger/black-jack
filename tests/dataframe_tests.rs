
extern crate blackjack;

use std::collections::HashSet;

use blackjack::prelude::*;


#[test]
fn test_series_serializer() {
    let mut series = Series::arange(0, 10);
    series.set_name("col1");
    let serialized = SerializedSeries::from_series(series).unwrap();
    let deserialzed: Series<i32> = serialized.decode().unwrap();
}

/*

#[test]
fn test_change_df_index() {
    let s1 = Series::arange(0, 5);
    let idx = 1..s1.len() + 1;

    let mut df = DataFrame::new();
    df.add_column(s1).unwrap();

    let expected_index = idx
        .clone()
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<DataElement>>();

    assert!(df.set_index(idx.into_iter()).is_ok());
    assert_eq!(&expected_index, df.index());
}

#[test]
fn test_df_column_size_mismatch() {
    let s1 = Series::arange(0, 5);
    let s2 = Series::arange(0, 100);

    let mut df = DataFrame::new();

    // first addition can be any size
    assert!(df.add_column(s1).is_ok());

    // second must be same length as the first.
    assert!(df.add_column(s2).is_err());
}

#[test]
fn test_df_groupby() {
    let mut df = DataFrame::new();
    let series1 = Series::arange(0, 10);
    let series2 = Series::arange(10, 20);

    df.add_column(series1).unwrap();
    df.add_column(series2).unwrap();

    let keys = Series::from_vec(
        vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1]
    );

    let grouped = df.groupby(keys).sum::<i32>();
    println!("{}", grouped);
}

#[test]
fn test_index_mut() {
    let mut df = DataFrame::new();
    let mut s1 = Series::arange(0, 5);
    s1.set_name("s1");
    df.add_column(s1).unwrap();

    let s1 = Series::arange(5, 10);
    let sc = s1.clone();
    df["s1"] = s1;

    assert_eq!(&df["s1"], &sc);
}
*/
#[test]
fn test_column_names() {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    let mut s1 = Series::arange(0, 2);
    let mut s2 = Series::arange(1, 3);

    let mut df = DataFrame::new();

    s1.set_name("s1");
    s2.set_name("s2");
    df.add_column(s1).unwrap();
    df.add_column(s2).unwrap();

    assert_eq!(
        df.columns().collect::<Vec<&str>>(),
        vec!["s1", "s2"]
    );
}
/*
#[test]
fn test_pretty_display() {
    let mut df = DataFrame::new();
    let mut s1 = Series::arange(0, 5);
    s1.set_name("series-1");
    let s2 = Series::arange(5, 10);

    df.add_column(s1).unwrap();
    df.add_column(s2).unwrap();

    println!("{}", df);
}

#[test]
fn test_read_basic_csv() {
    let path = format!("{}/tests/data/basic_csv.csv", env!("CARGO_MANIFEST_DIR"));
    println!("Using path: {}", &path);
    let df = DataFrame::read_csv(&path, b',').expect("Unable to read file!");
    println!("Resulting DataFrame: {}", df);

    let cols = ["col1".to_string(), "col2".to_string(), "col3".to_string()];
    let expected_columns: HashSet<&String> = cols.iter().collect();
    assert_eq!(expected_columns, df.columns());
}

#[test]
fn test_read_gzipped_basic_csv() {
    
    let path = format!("{}/tests/data/basic_csv.csv.gz", env!("CARGO_MANIFEST_DIR"));
    println!("Using path: {}", &path);
    let df = DataFrame::read_csv(&path, b',').unwrap();
    println!("Resulting DataFrame: {}", df);


    let cols = ["col1".to_string(), "col2".to_string(), "col3".to_string()];
    let expected_columns: HashSet<&String> = cols.iter().collect();
    assert_eq!(expected_columns, df.columns());

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

    let mut series2: Series = Series::from_vec(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    series2.set_name("series-2");
    let series2_clone = series2.clone();

    // Add both columns
    df.add_column(series1).unwrap();
    df.add_column(series2).unwrap();
    
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
*/
#[test]
fn test_get_column_by_name() {
    let mut df = DataFrame::new();
    let mut series: Series<i32> = Series::arange(0, 5);
    series.set_name("test-series");
    let series_clone = series.clone();

    df.add_column(series).unwrap();

    let series_ref: Series<i32> = df.get_column("test-series").expect("Unable to find column named 'test-series'");
    assert_eq!(series_ref, series_clone);

}
