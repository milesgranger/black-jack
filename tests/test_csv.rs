use blackjack::{blackjack_init, DataFrame};
use serde::{Deserialize};
use std::iter::FromIterator;

blackjack_init![];

#[test]
fn test_read_csv() {
    let data: Vec<u8> = r#"
        col1,col2,col3
        1,3.0,foo
        2,2.0,bar"#
        .replace(' ', "")
        .into();

    #[derive(DataFrame, Default, Deserialize, Debug)]
    pub struct Row {
        col1: usize,
        col2: f32,
        col3: String,
    }

    let mut csv_rdr = csv::Reader::from_reader(data.as_slice());

    let df =
        <DataFrame<Row>>::from_iter(csv_rdr.deserialize().into_iter().filter_map(|row| row.ok()));
    assert_eq!(df.col1().sum::<usize>(), 3);
    assert_eq!(df.col2().sum::<f32>() as usize, 5);
    assert_eq!(df.col3().collect::<Vec<&String>>(), vec!["foo", "bar"]);
}
