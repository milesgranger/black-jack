use blackjack::prelude::*;
use itertools::Itertools;

#[test]
fn dataframe_iter_rows() {
    let mut df = DataFrame::new();
    let s1 = Series::from_vec(vec![0, 1, 2, 3]);
    let s2 = Series::from_vec(vec![1, 2, 3, 4]);

    assert!(df.add_column(s1).is_ok());
    assert!(df.add_column(s2).is_ok());

    let rows = df.iter_rows().collect_vec();
    assert_eq!(rows.len(), 4); // Four rows
    assert!(rows.iter().all(|r| r.data.len() == 2)); // Each row has two elements
}
