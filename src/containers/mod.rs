
use std::iter::FromIterator;

/// Define which data types can be requested or cast to.
/// to serve as flags between Cython and Rust for data type conversions / creations
#[derive(Debug, PartialEq)]
pub enum DType {
    Float64,
    Int32
}

/// Container for various supported data types
#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Float64(Vec<f64>),
    Int32(Vec<i32>)
}

impl Data {
    pub fn len(&self) -> usize {
        match self {
            Data::Float64(ref vec) => vec.len(),
            Data::Int32(ref vec) => vec.len()
        }
    }

    pub fn get_dtype(&self) -> DType {
        match self {
            Data::Float64(ref _vec) => DType::Float64,
            Data::Int32(ref _vec) => DType::Int32
        }
    }

    pub fn astype(&self, dtype: DType) -> Self {
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

    pub fn astype_consume(self, dtype: DType) -> Self {
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