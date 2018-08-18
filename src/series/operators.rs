/*

This module consists of vector operations; sum, multiply, divide, etc.

*/

use std::iter::Sum;
use std::mem;
use containers::{DataPtr, Data, into_data_ptr, from_data_ptr};

pub trait LumberJackData {}
impl LumberJackData for f64 {}
impl LumberJackData for i32 {}

/// Sum a vector which consists of values allowed to be summed and return a Vec of size one
/// which plays well with DataPtr
pub fn sum_vec<'a, T>(vec: &'a Vec<T>) -> Vec<T>
    where T: Sum<&'a T>
{
    let mut result = Vec::with_capacity(1_usize);
    result.push(vec.iter().sum());
    result
}

#[no_mangle]
pub extern "C" fn multiply_by_scalar(data_ptr: DataPtr, scalar: f64, inplace: bool) -> DataPtr {
    let data = from_data_ptr(data_ptr);
    let result = if inplace {
        operate_on_vec_by_scalar!(inplace data, *, scalar)
    } else {
        let r = operate_on_vec_by_scalar!(!inplace &data, *, scalar);
        mem::forget(data);
        r
    };

    into_data_ptr(result)
}

#[no_mangle]
pub extern "C" fn add_by_scalar(data_ptr: DataPtr, scalar: f64, inplace: bool) -> DataPtr {
    let data = from_data_ptr(data_ptr);
    let result = if inplace {
        operate_on_vec_by_scalar!(inplace data, +, scalar)
    } else {
        let r = operate_on_vec_by_scalar!(!inplace &data, +, scalar);
        mem::forget(data);
        r
    };

    into_data_ptr(result)
}

#[no_mangle]
pub extern "C" fn sum(data_ptr: DataPtr) -> f64 {

    let data = from_data_ptr(data_ptr);

    let result = match data {
        Data::Float64(ref vec) => {
            vec.into_iter().sum::<f64>() as f64
        },
        Data::Int32(ref vec) => {
            vec.into_iter().sum::<i32>() as f64
        }
    };
    mem::forget(data);
    result
}

#[no_mangle]
pub extern "C" fn cumsum(data_ptr: DataPtr) -> DataPtr {
    let data = from_data_ptr(data_ptr);

    match data {
        Data::Float64(vec) => {
            let mut result= Vec::with_capacity(vec.len());
            let mut cumsum = 0_f64;
            for val in vec.iter() {
                cumsum += val;
                result.push(cumsum);
            }
            let ptr = into_data_ptr(Data::Float64(result));
            mem::forget(vec);
            ptr
        },
        Data::Int32(vec) => {
            let mut result = Vec::with_capacity(vec.len());
            let mut cumsum = 0_i32;
            for val in vec.iter() {
                cumsum += val;
                result.push(cumsum);
            }
            let ptr = into_data_ptr(Data::Int32(result));
            mem::forget(vec);
            ptr
        }
    }
}


#[no_mangle]
pub extern "C" fn mean(data_ptr: DataPtr) -> f64 {
    let data = from_data_ptr(data_ptr);

    match data {
        Data::Float64(vec) => {
            let val = vec.iter().sum::<f64>() / vec.len() as f64;
            mem::forget(vec);
            val
        },
        Data::Int32(vec) => {
            let val = vec.iter().sum::<i32>() as f64 / vec.len() as f64;
            mem::forget(vec);
            val
        }
    }
}