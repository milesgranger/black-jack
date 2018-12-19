extern crate blackjack;
extern crate num;
extern crate float_cmp;

use blackjack::prelude::*;

/*
#[test]
fn test_series_index() {
    let mut series = Series::arange(0, 10);
    let index  = 1..series.len() + 1;

    let expected_index = index
        .clone()
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<DataElement>>();

    assert!(series.set_index(index.into_iter()).is_ok());
    assert_eq!(&expected_index, series.index());
}
*/

/* Series <op> Series tests */
#[test]
fn test_series_op_series_impls() {
    let series1 = Series::from_vec(vec![1, 1, 1, 1, 1]);
    let series2 = Series::from_vec(vec![1, 1, 1, 1, 1, 1]);

    // Adding a series with another series of a different shape should Err
    let result = (series1.clone() + series2).is_ok();
    assert_eq!(result, false);

    // Adding a two series of the same shape should be fine
    let result = series1.clone() + series1;
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().sum(), 10);
}

#[test]
fn test_series_op_series_inplace() {
    let mut series1 = Series::from_vec(vec![1, 1, 1, 1]);
    let series = Series::from_vec(vec![2, 2, 2, 2]);

    series1 *= series;
    assert_eq!(series1.sum(), 8);
}

#[test]
fn test_into_iter() {
    let series: Series<i32> = Series::from_vec(vec![1, 2, 3, 4]);
    let sum: i32 = series.into_iter().sum();
    assert_eq!(sum, 10);
}

#[test]
fn test_isna() {
    let mut series = Series::from_vec(vec![0, 1, 2])
        .astype::<f32>()
        .unwrap();

    assert_eq!(series.isna().collect::<Vec<bool>>(), vec![false, false, false]);

    series[0] = num::Float::nan();
    assert_eq!(series.isna().collect::<Vec<bool>>(), vec![true, false, false]);
}

#[test]
fn test_all() {
    let series = Series::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(series.all(|x| *x > 0), true);
    assert_eq!(series.all(|x| *x > 3), false);
}

#[test]
fn test_any() {
    let series = Series::from_vec(vec![1, 2, 3, 4]);
    assert_eq!(series.any(|x| x > &&3), true);
    assert_eq!(series.any(|x| x < &&1), false);
}

#[test]
fn test_locate() {

    let series = Series::from_vec(vec![1, 2, 1, 2]);
    let ones = series.positions(|x| *x == 1).collect::<Vec<usize>>();
    assert_eq!(ones, vec![0, 2]);
}

#[test]
fn test_map() {
    let series = Series::from_vec(vec![1, 1, 1, 1]);

    // Test single thread map
    let new = series.clone().map(|x| x * 2);
    assert_eq!(series.sum() * 2, new.sum());

    // Test parallel map
    let new = series.clone().map_par(|x| x * 2);
    assert_eq!(series.sum() * 2, new.sum());
}

#[test]
fn test_groupbys() {
    let series = Series::from_vec(vec![1, 2, 3, 1, 2, 3]);
    let keys   = Series::from_vec(vec![4, 5, 6, 4, 5, 6]);

    // Split into groups and sort those groups
    let grouped = series.groupby(keys.clone()).sum();

    // 3 keys == 3 len
    assert_eq!(grouped.len(), 3);

    let vals = grouped.into_vec();
    assert_eq!(vals, vec![2, 4, 6]);

    // Test min
    let grouped = series.groupby(keys.clone()).min().unwrap();
    let vals = grouped.into_vec();
    assert_eq!(vals, vec![1, 2, 3]);

    // Test max
    let grouped = series.groupby(keys.clone()).max().unwrap();
    let vals = grouped.into_vec();
    assert_eq!(vals, vec![1, 2, 3]);

    // Test mean
    let grouped = series.groupby(keys.clone()).mean().unwrap();
    let vals = grouped.into_vec();
    assert_eq!(vals, vec![1_f64, 2_f64, 3_f64]);

    // Test var
    let grouped = series.groupby(keys.clone()).var().unwrap();
    let vals = grouped.into_vec();
    assert_eq!(vals, vec![0_f64, 0_f64, 0_f64]);

}

#[test]
fn test_unique() {
    let series = Series::from_vec(vec![1, 2, 1, 0, 1, 0, 1, 1]);
    let unique = series.unique();
    assert_eq!(unique, Series::from_vec(vec![0, 1, 2]));
}

#[test]
fn test_series_scalar_ops() {
    
    let base_series = Series::arange(0, 5);

    // Test Mul
    let series = base_series.clone();
    let series = series * 2;
    assert_eq!(series.sum(), 20);

    // Test Add
    let series = base_series.clone();
    let series = series + 2;
    assert_eq!(series.sum(), 20);

    // Test Sub
    let series = base_series.clone();
    let series = series - 2;
    assert_eq!(series.sum(), 0);

    // Test Div, convert to f32 so floats don't get rounded during
    // sum operations, where each DataElement would be cast as an integer.
    let series = base_series.clone();
    let series = series / 2_i32;
    assert_eq!(series.sum() as i32, 4);
}

#[test]
fn test_series_indexing() {
    let mut series = Series::from_vec(vec![0, 1, 2, 3]);
    series[0] = 1.into();
    assert_eq!(series[0], 1.into());
}

#[test]
fn test_series_append() {
    let mut series = Series::from_vec(vec![0, 1, 2]);
    assert_eq!(series.len(), 3);

    series.append(3);
    assert_eq!(series.len(), 4);
    assert_eq!(series[3], 3.into());
}

#[test]
fn test_display_series() {
    let mut series = Series::arange(0, 10);
    series.set_name("test-column");
    println!("{:#?}", series);
}

#[test]
fn test_series_arange() {
    let series = Series::arange(0, 10);
    assert_eq!(series.len(), 10);
    assert_eq!(series.dtype(), DType::I32);
}

#[test]
fn test_series_from_vec() {
    let series = Series::from_vec(vec![1.0, 2.0, 3.0]);
    assert_eq!(series.len(), 3);
}

#[test]
fn test_series_naming() {
    let mut series = Series::from_vec(vec![1, 2, 3]);
    assert_eq!(series.name(), None);
    series.set_name("new-series");
    assert_eq!(series.name().unwrap(), "new-series".to_string());
}

#[test]
fn test_series_aggregation_ops() {
    let series = Series::arange(0, 5);

    // Test sum
    assert_eq!(series.sum(), 10_i32);

    // Test mean
    assert_eq!(series.mean(), Ok(2.0));

    // Test min
    assert_eq!(series.min().unwrap(), 0_i32);

    // Test max
    assert_eq!(series.max().unwrap(), 4_i32);

    // Test mode - both single mode and multiple modes
    let series = Series::from_vec(vec![0, 0, 0, 1, 2, 3]);
    assert_eq!(series.mode(), Ok(Series::from_vec(vec![0])));

    let series = Series::from_vec(vec![0, 0, 0, 1, 1, 1, 2]);
    assert_eq!(series.mode(), Ok(Series::from_vec(vec![0, 1])));

    // Test variance
    let series = Series::arange(0, 5);
    assert_eq!(series.var().unwrap() as i32, 2);

    // Test standard deviation
    let series = Series::arange(0, 10);
    let std = series.std().unwrap();
    assert!(std > 2.87);
    assert!(std < 2.88);

    // Test median, both float and integer
    let series = Series::arange(0, 10);
    let median = series.median().unwrap();
    assert!(median < 4.51);
    assert!(median > 4.49);
    let series = Series::arange(0, 3);
    assert_eq!(series.median(), Ok(1.0));


    // Test quantile
    let series = Series::arange(0, 101);
    assert_eq!(series.quantile(0.5), Ok(50.0));
    let series = Series::arange(0, 100);
    let qtl = series.quantile(0.5).unwrap();
    assert!(qtl < 49.51);
    assert!(qtl > 49.49);

}

#[test]
fn test_into_from_raw() {
    let series = Series::arange(0, 5);
    let series_clone = series.clone();

    let ptr = series.into_raw();
    let recovered_series = Series::from_raw(ptr);
    assert_eq!(recovered_series, series_clone)


}
