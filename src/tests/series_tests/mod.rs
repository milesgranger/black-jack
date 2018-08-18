use series;
use containers::{Data, into_data_ptr, from_data_ptr, DataPtr, DType};


#[test]
fn cumsum() {
    let vec = vec![0, 1, 2, 3, 4];
    let ptr = into_data_ptr(Data::Int32(vec));
    let result = series::cumsum(ptr);
    let vec = match result {
        DataPtr::Int32 { data_ptr, len } => {
            unsafe { Vec::from_raw_parts(data_ptr, len, len)}
        },
        _ => panic!("Expected to get DataPtr::Int32!")
        };
    println!("Got vec: {:?}", &vec);
    assert_eq!(vec.last().expect("Vector was empty!"), &10_i32);
}

#[test]
fn sum() {
    let vec = vec![1, 2, 3, 4];
    let ptr = into_data_ptr(Data::Int32(vec));
    let result = series::sum(ptr);
    assert_eq!(result, 10_f64);

}

#[test]
fn mean() {
    let vec = vec![1, 1, 1, 1, 1];
    let ptr = into_data_ptr(Data::Int32(vec));
    let result = series::mean(ptr);
    assert_eq!(result, 1_f64);
}

#[test]
fn arange() {
    let v = series::arange(0, 5, DType::Int32);
    let data = from_data_ptr(v);
    if let Data::Int32(vec) = data {
        assert_eq!(vec.iter().sum::<i32>(), 10);
    } else {
        panic!("Expected Data::Int32 but got {:?} instead!", data);
    }
}