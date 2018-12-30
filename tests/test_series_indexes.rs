use blackjack::prelude::*;

#[test]
fn test_set_index() {
    let mut series = Series::from_vec(vec![1, 2, 3, 4, 5]);

    // default, 0 indexed, index
    let index: &Vec<i32> = series.index().into();
    assert_eq!(index, &vec![0, 1, 2, 3, 4]);

    // Change index
    assert!(series.set_index(vec![1, 2, 3, 4, 5]).is_ok());

    // verify the change
    let index: &Vec<i32> = series.index().into();
    assert_eq!(index, &vec![1, 2, 3, 4, 5]);

    // Change index with wrong length.
    assert!(series.set_index(vec![0, 1]).is_err());

    // Reset the index
    assert!(series.reset_index().is_ok());
}

#[test]
fn test_selects_using_index() {
    let series = Series::from_vec(vec![1, 2, 3, 4, 5]);

    // Select values based on matching index values
    let vals = series.loc(&vec![0, 1, 0, 4]);
    assert_eq!(vals, vec![&1, &2, &1, &5]);
}