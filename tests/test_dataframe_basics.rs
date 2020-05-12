use blackjack::DataFrame;

#[test]
fn test_derive() {
    #[derive(DataFrame)]
    #[allow(dead_code)]
    pub struct Row {
        pub col1: usize,
    }

    let _df = RowDataFrame::new();
}

#[test]
fn test_basics() {
    #[derive(DataFrame)]
    pub struct Row {
        pub col1: usize,
        pub col2: String,
    }

    let row = Row {
        col1: 1,
        col2: "Hello".to_string(),
    };

    let mut df = RowDataFrame::new();

    // Pushing
    assert_eq!(df.col1.len(), 0);
    assert_eq!(df.col2.len(), 0);
    df.push(row);
    assert_eq!(df.col1.len(), 1);
    assert_eq!(df.col2.len(), 1);
}
