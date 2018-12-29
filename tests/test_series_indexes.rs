use blackjack::prelude::*;

#[test]
fn test_set_index() {
    let mut series = Series::from_vec(vec![1, 2, 3, 4, 5]);

    // default, 0 indexed, index
    let index: &Vec<i32> = series.index().into();
    assert_eq!(index, &vec![0, 1, 2, 3, 4]);

    // Change index
    series.set_index(vec![1, 2, 3, 4, 5]).unwrap();

    // verify the change
    let index: &Vec<i32> = series.index().into();
    assert_eq!(index, &vec![1, 2, 3, 4, 5]);

    // Change index with wrong length.
    assert!(series.set_index(vec![0, 1]).is_err());
}