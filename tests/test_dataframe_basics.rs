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
fn test_push() {
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
    #[derive(DataFrame)]
    pub struct Row {
        pub col1: usize,
        pub col2: String,
    }
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
fn test_select() {
    #[derive(DataFrame, Clone, PartialEq, Debug)]
    pub struct Row {
        pub col1: usize,
        pub col2: String,
    }
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
    #[derive(DataFrame)]
    pub struct Row {
        pub col1: usize,
        pub col2: String,
    }
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
