use blackjack::prelude::*;
use float_cmp::ApproxEq;

#[test]
fn dataframe_loc() {
    let mut df = DataFrame::new();
    let s1 = Series::from_vec(vec![0, 1, 2, 3]);
    let s2 = Series::from_vec(vec![1, 2, 3, 4]);

    assert!(df.add_column(s1).is_ok());
    assert!(df.add_column(s2).is_ok());

    let rows = df.loc(vec![1]).collect::<Vec<Row>>();

    // First column is s1, second element is 1
    if let Datum::I32(val) = rows[0].data[0].data {
        assert_eq!(val, &1);
    }

    // second column is s2, second element is 2
    if let Datum::I32(val) = rows[0].data[1].data {
        assert_eq!(val, &2);
    }
}
