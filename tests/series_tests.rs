extern crate blackjack;
extern crate num;
extern crate float_cmp;

use num::*;
use float_cmp::*;
use blackjack::prelude::*;


#[test]
fn test_groupby_sum() {
    let series = Series::from_vec(vec![1, 2, 3, 1, 2, 3]);
    let keys   = Series::from_vec(vec![4, 5, 6, 4, 5, 6]);

    // Split into groups and sort those groups
    let grouped = series.groupby(keys).sum::<i32>();

    println!("{}", &grouped);

    // 3 keys == 3 len
    assert_eq!(grouped.len(), 3);

    let mut vals = grouped.into_vec::<i32>();
    vals.sort();
    assert_eq!(vals, vec![2, 4, 6]);
}

#[test]
fn test_unique() {
    let series = Series::from_vec(vec![1, 2, 1, 0, 1, 0, 1, 1]);
    let unique = series.unique::<i32>();
    assert_eq!(unique, Series::from_vec(vec![0, 1, 2]));
}

#[test]
fn test_series_ops_inplace() {

    let base_series = Series::arange(0, 5);

    // Test MulAssign - all i64
    let mut series = base_series.clone();
    series.astype(DType::I64).unwrap();
    series *= 2_i64;
    assert_eq!(series.sum::<i32>(), 20);

    // Test MulAssign - series_f64 *= 2_i64
    let mut series = base_series.clone();
    series.astype(DType::F64).unwrap();
    series *= 2_i64;
    assert_eq!(series.sum::<i32>(), 20);
    
    // Test AddAssign - all i64
    let mut series = base_series.clone();
    series.astype(DType::I64).unwrap();
    series += 2_i64;
    assert_eq!(series.sum::<i32>(), 20);

    // Test MulAssign - series_f64 += 2_i64
    let mut series = base_series.clone();
    series.astype(DType::F64).unwrap();
    series += 2_i64;
    assert_eq!(series.sum::<i32>(), 20);

    // Test SubAssign - all i64
    let mut series = base_series.clone();
    series.astype(DType::I64).unwrap();
    series -= 2_i64;
    assert_eq!(series.sum::<i32>(), 0);

    // Test SublAssign - series_f64 -= 2_i64
    let mut series = base_series.clone();
    series.astype(DType::F64).unwrap();
    series -= 2_i64;
    assert_eq!(series.sum::<i32>(), 0);
    
    // Test DivAssign - all i64
    let mut series = base_series.clone();
    series.astype(DType::I64).unwrap();
    series /= 2_i64;
    assert_eq!(series.sum::<i32>() as i32, 4);

    // Test DivAssign - series_f64 * 2_i64
    let mut series = base_series.clone();
    series.astype(DType::F64).unwrap();
    series /= 2_i64;
    assert_eq!(series.sum::<f32>() as i32, 4);
    
}

#[test]
fn test_series_scalar_ops() {
    
    let base_series = Series::arange(0, 5);

    // Test Mul
    let series = base_series.clone();
    let series = series * 2;
    assert_eq!(series.sum::<i32>(), 20);

    // Test Add
    let series = base_series.clone();
    let series = series + 2;
    assert_eq!(series.sum::<i32>(), 20);

    // Test Sub
    let series = base_series.clone();
    let series = series - 2;
    assert_eq!(series.sum::<i32>(), 0);

    // Test Div, convert to f32 so floats don't get rounded during
    // sum operations, where each DataElement would be cast as an integer.
    let series = base_series.clone();
    let series = series / 2_f64;
    assert_eq!(series.sum::<f32>() as i32, 5);
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
fn test_actions_on_various_element_series() {

    let mut series = Series::from_data_elements(vec![
        DataElement::I32(1),
        DataElement::F32(1.0),
        DataElement::I64(1),
        DataElement::F64(1.0),
        DataElement::STRING("hi".to_string())
    ]);

    // Test that trying to convert series to I32, containing a String, 
    // results in an Error
    match series.astype(DType::I32) {
        Ok(()) => {
            panic!("Should not have been able to convert a String to NaN integer")
        },
        Err(_) => println!("Unable to convert String value to Integer! TEST PASSED!")
    };

    // Test that trying to convert series to I64, containing a String, 
    // results in an Error
    match series.astype(DType::I64) {
        Ok(()) => {
            panic!("Should not have been able to convert a String to NaN integer")
        },
        Err(_) => println!("Unable to convert String value to Integer! TEST PASSED!")
    };

    // Conversion to a Float type does work, because NaN is, itself a Float
    // But converting a NaN to an integer resulting in the primitive's MIN val
    // Which we think goes against the principle of least surprise; so 
    // we raise an error instead.
    series.astype(DType::F32).unwrap();

    // Now check that converting a series with an NaN results in an error
    match series.astype(DType::I32) {
        Ok(()) => panic!("Was able to convert a series w/ NaN into Integer!"),
        Err(_) => println!("Could not convert NaN to Integer -> TEST PASSED")
    }

    // Summing a series should skip any String or NaN values...
    assert_eq!(series.sum::<i32>(), 4);
}

#[test]
fn test_astype_conversions() {
    let series_base = Series::from_data_elements(vec![
        DataElement::I64(1_i64),
        DataElement::F32(1_f32),
        DataElement::STRING("Hello".to_string())
    ]);
    let nan: f64 = Float::nan();  // Can't make NaN an integer directly

    // Test conversion to f64, special float comparison needed...
    let mut series = series_base.clone();
    series.astype(DType::F64).unwrap();
    let vec = series.into_vec::<f64>();
    for (a, b) in vec.into_iter().zip(vec![1_f64, 1_f64, nan]) {
        assert!(a.approx_eq(&b, 0.000001, 1));
    }

    // Test conversion to string
    let mut series = series_base.clone();
    series.astype(DType::STRING).unwrap();
    assert_eq!(
        series.into_vec::<String>(), 
        vec![1_i64.to_string(), 1_i64.to_string(), "Hello".to_string()]
    );
}

#[test]
fn test_display_series() {
    let mut series = Series::arange(0, 10);
    series.set_name("test-column");
    println!("{}", series);
}

#[test]
fn test_series_arange() {
    let series: Series = Series::arange(0, 10);
    assert_eq!(series.len(), 10);
    assert_eq!(series.dtype(), Some(DType::I32));
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
    let series: Series = Series::arange(0, 5);

    // Test sum
    assert_eq!(series.sum::<f64>(), 10_f64);

    // Test mean
    assert_eq!(series.mean(), Ok(2.0));

    // Test min
    assert_eq!(series.min(), Ok(0_i32));

    // Test max
    assert_eq!(series.max(), Ok(4_i32));

    // Test mode - both single mode and multiple modes
    let series = Series::from_vec(vec![0, 0, 0, 1, 2, 3]);
    assert_eq!(series.mode::<i32>(), Ok(Series::from_vec(vec![0])));

    let series = Series::from_vec(vec![0, 0, 0, 1, 1, 1, 2]);
    assert_eq!(series.mode::<i32>(), Ok(Series::from_vec(vec![0, 1])));

    // Test variance
    let series = Series::arange(0, 5);
    assert_eq!(series.var::<f32>().unwrap() as i32, 2);

    // Test standard deviation
    let series = Series::arange(0, 10);
    let std = series.std::<f32>().unwrap();
    assert!(std > 2.87);
    assert!(std < 2.88);

    // Test median, both float and integer
    let series = Series::arange(0, 10);
    let median = series.median::<f32>().unwrap();
    assert!(median < 4.51);
    assert!(median > 4.49);
    let series = Series::arange(0, 3);
    assert_eq!(series.median::<i32>().unwrap(), 1);


    // Test quantile
    let series = Series::arange(0, 101);
    assert_eq!(series.quantile::<i32>(0.5), Ok(50));
    let series = Series::arange(0, 100);
    let qtl = series.quantile::<f32>(0.5).unwrap();
    assert!(qtl < 49.51);
    assert!(qtl > 49.49);

}

#[test]
fn test_into_from_raw() {
    let series: Series = Series::arange(0, 5);
    let series_clone = series.clone();

    let ptr = series.into_raw();
    let recovered_series = Series::from_raw(ptr);
    assert_eq!(recovered_series, series_clone)


}
