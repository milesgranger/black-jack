#![allow(dead_code)]

use std::mem;
mod operators;

use containers::{DataPtr, Length, DType, Data, into_data_ptr, from_data_ptr, AsType};
pub use series::operators::*;


#[derive(Clone, Debug, PartialEq)]
pub struct Series<'a> {
    pub name: &'a str,
    pub data: Data
}

impl<'a> Series<'a> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/*
    Functions exposed to C which create lumberjack series or frees one.
*/

#[no_mangle]
pub extern "C" fn from_numpy_ptr(ptr: &mut f64, len: u32) -> DataPtr {
    let vec = unsafe { Vec::from_raw_parts(ptr, len as usize, len as usize) };
    let data = Data::Float64(vec);
    into_data_ptr(data)
}

#[no_mangle]
pub extern "C" fn copy_ptr(ptr: &mut DataPtr) -> DataPtr {
    ptr.clone()
}

/// Create Series from arange and pass back as DataPtr
#[no_mangle]
pub extern "C" fn arange(start: i32, stop: i32, dtype: DType) -> DataPtr {
    let ptr = match dtype {
            DType::Float64 => {
                let mut data = (start..stop).map(|v| v as f64).collect::<Vec<f64>>();
                let ptr = into_data_ptr(Data::Float64(data));
                ptr
            }

            DType::Int32 => {
                let mut data = (start..stop).map(|v| v as i32).collect::<Vec<i32>>();
                let ptr = into_data_ptr(Data::Int32(data));
                ptr
            }
        };
    ptr
}

/// Set some value at the ith index
#[no_mangle]
pub extern "C" fn set_item(ptr: DataPtr, idx: u32, value: f64) {

    let mut data = from_data_ptr(ptr);
    match data {
        Data::Float64(ref mut vec) => vec[idx as usize] = value,
        Data::Int32(ref mut vec) => vec[idx as usize] = value as i32
    }
    mem::forget(data);
}

///
#[no_mangle]
pub extern "C" fn astype(ptr: DataPtr, dtype: DType) -> DataPtr {
    let data = from_data_ptr(ptr);
    let new_data = data.astype(dtype);
    let ptr = into_data_ptr(new_data);
    mem::forget(data);
    ptr
}

/// Set an individual item on an existing vec
#[no_mangle]
pub extern "C" fn verify(data_ptr: DataPtr) {
    let data = from_data_ptr(data_ptr.clone());
    println!("Got element: {:?} - {:p}", &data, &data);
    mem::forget(data);
}

/// Reconstruct Series from DataPtr and let it fall out of scope to clear from memory.
#[no_mangle]
pub extern "C" fn free_data(data_ptr: DataPtr) {
    // TODO: Replace this with dropping a pointer instead of passing the entire DataPtr struct back
    let _data = from_data_ptr(data_ptr);
    //println!("Got data letting it fall out of scope!");
}
