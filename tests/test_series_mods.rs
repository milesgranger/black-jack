/*
    Test various series modifications
*/

use blackjack::prelude::*;


#[test]
fn test_drop_positions() {
    let mut series = Series::from_vec(vec![0, 1, 2, 3, 4, 5]);

    // change the index values should have no effect.
    assert!(series.set_index(vec![1, 2, 3, 4, 5, 6]).is_ok());

    // before drops
    assert_eq!(series.len(), 6);
    assert_eq!(series.values, vec![0, 1, 2, 3, 4, 5]);

    // Drop and check results
    series.drop_positions(vec![0, 4]);
    assert_eq!(series.len(), 4);
    assert_eq!(series.values, vec![1, 2, 3, 5]);
}