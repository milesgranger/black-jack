#[cfg(test)]
pub mod series_map_tests {

    use containers::Data;

    #[test]
    fn multiply_by_1() {
        let _data = Data::Int32(vec![1, 2, 3, 4]);
        series_map!(&_data, "lambda v: v * 2");
        //assert_eq!(new_data, Data::Int32(vec![2, 4, 6, 8]));
    }
}


#[cfg(test)]
pub mod opterate_on_vec_by_scalar_tests {

    use containers::Data;

    #[test]
    fn multiply_i32vec_i32scalar_not_inplace() {
        // Test multiplication by creating a copy
        let data = Data::Int32(vec![1, 2, 3]);
        let new_data = operate_on_vec_by_scalar!(!inplace &data, *, 2);
        assert_eq!(&Data::Int32(vec![1, 2, 3]), &data);
        assert_eq!(&Data::Int32(vec![2, 4, 6]), &new_data);
        println!("Original: {:?}, New: {:?}", data, new_data);
    }

    #[test]
    fn subtract_i32vec_i32scalar_not_inplace() {
        // Test multiplication by creating a copy
        let data = Data::Int32(vec![1, 2, 3]);
        let new_data = operate_on_vec_by_scalar!(!inplace &data, -, 1);
        assert_eq!(&Data::Int32(vec![1, 2, 3]), &data);
        assert_eq!(&Data::Int32(vec![0, 1, 2]), &new_data);
        println!("Original: {:?}, New: {:?}", data, new_data);
    }

    #[test]
    fn multiply_i32vec_f64scalar_not_inplace() {
        // Test multiplication by creating a copy
        let data = Data::Int32(vec![1_i32, 2_i32, 3_i32]);
        let orig = data.clone();
        let new_data = operate_on_vec_by_scalar!(!inplace data, *, 2_f64);
        assert_eq!(&Data::Int32(vec![1_i32, 2_i32, 3_i32]), &orig);
        assert_eq!(&Data::Float64(vec![2., 4., 6.]), &new_data);
        println!("Original: {:?}, New: {:?}", orig, new_data);
    }

    #[test]
    fn multiply_i32vec_i32scalar_inplace() {
        // Test multiplication by inplace, all i32
        let mut data = Data::Int32(vec![1_i32, 2_i32, 3_i32]);
        data = operate_on_vec_by_scalar!(inplace data, *, 2_i32);
        assert_eq!(&Data::Int32(vec![2, 4, 6]), &data);
    }

    #[test]
    fn multiply_i32vec_f64scalar_inplace() {
        // Test multiplication by inplace, vec of i32 * f64
        let mut data = Data::Int32(vec![1_i32, 2_i32, 3_i32]);
        data = operate_on_vec_by_scalar!(inplace data, *, 2_f64);
        assert_eq!(&Data::Float64(vec![2., 4., 6.]), &data);
    }

    #[test]
    fn add_i32vec_f64scalar_inplace() {
        // Test multiplication by inplace, vec of i32 * f64
        let mut data = Data::Int32(vec![1_i32, 2_i32, 3_i32]);
        data = operate_on_vec_by_scalar!(inplace data, +, 1_f64);
        assert_eq!(&Data::Float64(vec![2., 3., 4.]), &data);
    }
}