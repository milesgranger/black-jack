use blackjack::prelude::*;

#[test]
fn test_set_index() {
    let mut series = Series::from_vec(vec![1, 2, 3, 4, 5]);

    // default, 0 indexed, index
    let index: &Vec<i32> = series.index();
    assert_eq!(index, &vec![1, 2, 3, 4, 5]);

    // Change index
    assert!(series.set_index(vec![1, 2, 3, 4, 5]).is_ok());

    // verify the change
    let index: &Vec<i32> = series.index().into();
    assert_eq!(index, &vec![1, 2, 3, 4, 5]);

    // Change index with wrong length.
    assert!(series.set_index(vec![0, 1]).is_err());
}

#[test]
fn test_sort_by_another_index() {
    let series = Series::from_vec(vec![1, 2, 3, 4, 5]);
    {
        let index: &Vec<i32> = series.index();
        assert_eq!(index, &vec![1, 2, 3, 4, 5]);
    }

}

#[test]
fn test_loc_selects_index() {
    let series = Series::from_vec(vec![1, 2, 3, 4, 5]);

    // Select values based on matching index values
    let vals = series.loc(vec![0, 1, 0, 4]);
    assert_eq!(vals, vec![&1, &4]);
}

#[test]
fn test_iloc_selects_index() {
    let series = Series::from_vec(vec![1, 2, 3, 4, 5]);

    // Select values based on matching index values
    let vals = series.iloc(&vec![0, 1, 0, 4]);
    assert_eq!(vals, vec![&1, &2, &1, &5]);

}