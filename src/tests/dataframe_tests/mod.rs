use dataframe::{DataFrame};
use series::Series;
use containers::{Data, DType, GetDType};

#[test]
fn test_create_dataframe() {
    let _df = DataFrame::new();
}

#[test]
fn test_assign_column() {
    let data   = Data::Int32(vec![1, 2, 3]);
    let series = Series { name: "col1", data };
    let mut df = DataFrame::new();
    df.add_column(series.clone()).unwrap();

    // Ensure the underlying data is equal.
    assert_eq!(df.get_column_by_name("col1").unwrap(), &series);
}

#[test]
fn test_assign_uneven_length_cols() {
    let data1 = Series {
        name: "data1", 
        data: Data::Int32(vec![1, 2, 3, 4])
    };
    let data2 = Series {
        name: "data2",
        data: Data::Float64(vec![1.0, 2.0, 3.0, 4.0])
    };
    let data3 = Series {
        name: "data3",
        data: Data::Int32(vec![1, 2, 3, 4, 5])
    };
    
    let mut df = DataFrame::new();

    // Ok, no data in dataframe, any length wil do
    assert_eq!(
        df.add_column(data1), 
        Ok(())
    );

    // Ok, data2 same length as data1
    assert_eq!(
        df.add_column(data2), 
        Ok(())
    );

    // Err, data3 is length 5, not 4!
    assert_eq!(
        df.add_column(data3), 
        Err("Length of new column does not match length of index!")
    );
}

