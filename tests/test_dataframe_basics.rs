use blackjack::DataFrame;

#[derive(DataFrame, PartialEq, Clone, Debug)]
pub struct Row {
    pub col1: usize,
    pub col2: String,
}

#[test]
fn test_derive() {
    let _df = RowDataFrame::new();
}

#[test]
fn test_push() {
    let row = Row {
        col1: 1,
        col2: "Hello".to_string(),
    };

    let mut df = RowDataFrame::new();

    // Pushing
    assert_eq!(df.len(), 0);
    assert_eq!(df.col1().len(), 0);
    assert_eq!(df.col2().len(), 0);
    df.push(row);
    assert_eq!(df.len(), 1);
    assert_eq!(df.col1().len(), 1);
    assert_eq!(df.col2().len(), 1);
}

#[test]
fn test_filter() {
    let mut df = RowDataFrame::new();
    df.push(Row {
        col1: 1,
        col2: "Hello".to_string(),
    });
    df.push(Row {
        col1: 2,
        col2: "World".to_string(),
    });

    assert_eq!(df.len(), 2);
    let filtered = df.filter(|row| row.col1 == 1);
    assert_eq!(filtered.len(), 1);
}

#[test]
fn test_filter_inplace() {
    let mut df = RowDataFrame::new();
    df.push(Row {
        col1: 1,
        col2: "Hello".to_string(),
    });
    df.push(Row {
        col1: 2,
        col2: "World".to_string(),
    });
    df.push(Row {
        col1: 3,
        col2: "!".to_string(),
    });

    assert_eq!(df.len(), 3);
    df.filter_inplace(|row| row.col1 == 1 || row.col1 == 3);
    assert_eq!(df.len(), 1);
}

#[test]
fn test_remove() {
    let mut df = RowDataFrame::new();
    df.push(Row {
        col1: 1,
        col2: "Hello".to_string(),
    });
    let row = Row {
        col1: 2,
        col2: "World".to_string(),
    };
    df.push(row.clone());
    df.push(Row {
        col1: 3,
        col2: "!".to_string(),
    });

    assert_eq!(df.len(), 3);
    let row_out = df.remove(1);
    assert_eq!(row_out.col1, row.col1);
    assert_eq!(df.len(), 2);
}

#[test]
fn test_select() {
    let mut df = RowDataFrame::new();
    df.push(Row {
        col1: 1,
        col2: "Hello".to_string(),
    });
    let row = Row {
        col1: 2,
        col2: "World".to_string(),
    };
    df.push(row.clone());

    let selected_row = df.select(1);
    assert_eq!(row, selected_row);
}

#[test]
fn test_column_accessors() {
    let mut df = RowDataFrame::new();
    df.push(Row {
        col1: 1,
        col2: "Hello".to_string(),
    });

    assert_eq!(df.col1()[0], 1);
    let col: &mut [usize] = df.col1_mut();
    col[0] *= 2;
    assert_eq!(df.col1()[0], 2);

    assert_eq!(&df.col2()[0], "Hello");
    df.col2_mut()[0] = "Hey-ya".to_string();
    assert_eq!(&df.col2()[0], "Hey-ya");
}

#[test]
fn test_into_iter() {
    let mut df = RowDataFrame::new();
    df.push(Row {
        col1: 1,
        col2: "Hello".to_string(),
    });
    df.push(Row {
        col1: 2,
        col2: "World".to_string(),
    });
    df.push(Row {
        col1: 3,
        col2: "!".to_string(),
    });

    assert_eq!(df.into_iter().count(), 3);
}
