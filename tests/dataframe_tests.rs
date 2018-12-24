
extern crate blackjack;
extern crate tempfile;

use tempfile::tempdir;

use blackjack::prelude::*;


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
*/

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

    let grouped = df.groupby(&keys).sum();
    println!("{:?}", grouped);
}


#[test]
fn test_column_names() {

    let mut s1 = Series::arange(0, 2);
    let mut s2 = Series::arange(1, 3);

    let mut df = DataFrame::new();

    s1.set_name("s1");
    s2.set_name("s2");
    df.add_column(s1).unwrap();
    df.add_column(s2).unwrap();

    let mut col_names = df.columns().collect::<Vec<&str>>();
    col_names.sort();

    assert_eq!(
        col_names,
        vec!["s1", "s2"]
    );
}


#[test]
fn test_read_write_basic_csv() {
    let path = format!("{}/tests/data/medium_csv.csv", env!("CARGO_MANIFEST_DIR"));
    println!("Using path: {}", &path);
    let df = Reader::new(&path).read().expect("Unable to read file!");
    //let cols = vec!["col1", "col2", "col3"];
    //assert_eq!(cols, df.columns().collect::<Vec<&str>>());

    {
        let col2: &Series<i32> = df.get_column("col2").unwrap();
        assert_eq!(col2.sum() as i32, 3000);
    }

    {
        let tdir = tempdir().unwrap();
        let out_path = tdir.path().join("out.csv");
        let out_path_str = out_path.to_str().unwrap();
        Writer::new(&out_path_str).write(df).unwrap();
        let new_df = Reader::new(&out_path_str).read().unwrap();
        let col2: &Series<i32> = new_df.get_column("col2").unwrap();
        assert_eq!(col2.sum() as i32, 3000);
    }

}


#[test]
fn test_read_gzipped_basic_csv() {
    
    let path = format!("{}/tests/data/basic_csv.csv.gz", env!("CARGO_MANIFEST_DIR"));
    println!("Using path: {}", &path);
    let df = Reader::new(&path).read().unwrap();
    let cols = vec!["col1", "col2", "col3"];

    let mut col_names = df.columns().collect::<Vec<&str>>();
    col_names.sort();
    assert_eq!(cols, col_names);

}


#[test]
fn test_add_columns() {
    let mut df = DataFrame::new();

    let mut series1: Series<i32> = Series::arange(0, 5);
    series1.set_name("series-1");
    let series1_clone = series1.clone();

    let mut series2: Series<f32> = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    series2.set_name("series-2");
    let series2_clone = series2.clone();

    // Add both columns
    df.add_column(series1).unwrap();
    df.add_column(series2).unwrap();
    
    {
        // Test the columns match
        let series1ref: &Series<i32> = df.get_column("series-1").expect("No column named 'series-1'");
        assert_eq!(series1ref, &series1_clone);

        let series2ref: &Series<f32> = df.get_column("series-2").expect("No column named 'series-2'");
        assert_eq!(series2ref, &series2_clone);

        // Just sanity check two columns are still there.
        assert_eq!(df.n_columns(), 2);
    }


}

#[test]
fn test_get_column_by_name() {
    let mut df = DataFrame::new();
    let mut series: Series<i32> = Series::arange(0, 5);
    series.set_name("test-series");
    let series_clone = series.clone();

    df.add_column(series).unwrap();

    let series_ref: &Series<i32> = df.get_column("test-series").expect("Unable to find column named 'test-series'");
    assert_eq!(series_ref, &series_clone);

}
