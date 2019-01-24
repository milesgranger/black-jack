use blackjack::prelude::*;

#[test]
fn test_basic_join() {

    let series1 = Series::from_vec(vec![0, 1, 2, 3, 4]);
    let series2 = Series::from_vec(vec![1, 2, 3, 4, 5]);

    let mut df = DataFrame::new();
    assert!(df.join_series(series1, Join::Inner, None).is_ok());
    assert!(df.join_series(series2, Join::Inner, Some("col0")).is_ok());

}