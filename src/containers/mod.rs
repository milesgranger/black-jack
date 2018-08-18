
use std::mem;
use std::iter::FromIterator;

/// This enum is what Cython will use to read the data created from Rust
#[derive(Clone)]
#[repr(C)]
pub enum DataPtr {
    Float64 {
        data_ptr: *mut f64,
        len: usize
    },
    Int32 {
        data_ptr: *mut i32,
        len: usize
    }
}


/// Container for various supported data types
#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Float64(Vec<f64>),
    Int32(Vec<i32>)
}

/// Container for individual item
#[derive(Debug)]
#[repr(C)]
pub enum DataElement {
    Float64(f64),
    Int32(i32)
}

pub trait Length {
    fn len(&self) -> usize;
}

impl Length for Data {
    fn len(&self) -> usize {
        match self {
            Data::Float64(ref vec) => vec.len(),
            Data::Int32(ref vec) => vec.len()
        }
    }
}

pub trait GetDType {
    fn get_dtype(&self) -> DType;
}

impl GetDType for Data {
    fn get_dtype(&self) -> DType {
        match self {
            Data::Float64(ref _vec) => DType::Float64,
            Data::Int32(ref _vec) => DType::Int32
        }
    }
}

pub trait AsType {

    // Create another Data enum as another type
    fn astype(&self, dtype: DType) -> Self;

    // Convert Data to another type _while_ consuming the original one.
    fn astype_consume(self, dtype: DType) -> Self;
}

impl AsType for Data {

    fn astype(&self, dtype: DType) -> Self {
        match self {
            Data::Int32(ref vec) => {
                match dtype {
                    DType::Float64 => Data::Float64(vec.iter().map(|v| *v as f64).collect()),
                    DType::Int32 => Data::Int32(vec.iter().map(|v| *v).collect())
                }
            },
            Data::Float64(ref vec) => {
                match dtype {
                    DType::Float64 => Data::Float64(vec.iter().map(|v| *v).collect()),
                    DType::Int32 => Data::Float64(vec.iter().map(|v| *v as f64).collect())
                }
            }
        }
    }

    fn astype_consume(self, dtype: DType) -> Self {
        match self {
            Data::Int32(vec) => {
                match dtype {
                    DType::Float64 => Data::Float64(vec.into_iter().map(|v| v as f64).collect()),
                    DType::Int32 => Data::Int32(vec.into_iter().map(|v| v).collect())
                }
            },
            Data::Float64(vec) => {
                match dtype {
                    DType::Float64 => Data::Float64(vec.into_iter().map(|v| v).collect()),
                    DType::Int32 => Data::Int32(vec.into_iter().map(|v| v as i32).collect())
                }
            }
        }
    }
}


/// Define which data types can be requested or cast to.
/// to serve as flags between Cython and Rust for data type conversions / creations
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum DType {
    Float64,
    Int32
}

impl FromIterator<i32> for Data {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Data {
        let mut vec = Vec::new();
        for v in iter {
            vec.push(v)
        }
        Data::Int32(vec)
    }
}

impl FromIterator<f64> for Data {
    fn from_iter<I: IntoIterator<Item=f64>>(iter: I) -> Data {
        let mut vec = Vec::new();
        for v in iter {
            vec.push(v)
        }
        Data::Float64(vec)
    }
}

/// Return a vector from a pointer
pub unsafe fn vec_from_raw<T>(ptr: *mut T, n_elements: usize) -> Vec<T> {
    Vec::from_raw_parts(ptr, n_elements, n_elements)
}



/// Return a Data enum from DataPtr
pub fn from_data_ptr(ptr: DataPtr) -> Data {
    match ptr {
        DataPtr::Float64 { data_ptr, len } => {
            Data::Float64(unsafe { vec_from_raw(data_ptr, len)})
        },
        DataPtr::Int32 { data_ptr, len } => {
            Data::Int32(unsafe { vec_from_raw(data_ptr, len)})
        }
    }
}

/// Build a DataPtr from the Data enum
pub fn into_data_ptr(data: Data) -> DataPtr {

    // Create a pointer which has the raw vector pointer and does not let it fall out of
    // scope by forgetting it, as it will be used later, and 'self' will be dropped.
    let data_ptr = match data {

        Data::Float64(mut vec) => {
            vec.shrink_to_fit();
            let ptr = DataPtr::Float64 {
                data_ptr: vec.as_mut_ptr(),
                len: vec.len()
            };
            mem::forget(vec);
            ptr
        },

        Data::Int32(mut vec) => {
            vec.shrink_to_fit();
            let ptr = DataPtr::Int32 {
                data_ptr: vec.as_mut_ptr(),
                len: vec.len()
            };
            mem::forget(vec);
            ptr
        }
    };

    data_ptr
}