use blackjack::prelude::*;

#[test]
fn test_iloc_selects_index() {
    let mut series = Series::from_vec(vec![1, 2, 3, 4, 5]);

    // Select values based on matching index values
    let vals = series.iloc(&vec![0, 1, 0, 4]);
    assert_eq!(vals, vec![&1, &2, &1, &5]);

    // Select values again, which should be the same even that the index changed.
    let vals = series.iloc(&vec![0, 1, 0, 4]);
    assert_eq!(vals, vec![&1, &2, &1, &5]);
}
