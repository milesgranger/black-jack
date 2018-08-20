
use std::iter::FromIterator;
use num::*;
use num::integer::Integer;


/// Define which data types can be requested or cast to.
/// to serve as flags between Cython and Rust for data type conversions / creations
#[derive(Debug, PartialEq)]
pub enum DType {
    Float64,
    Int32
}

/// Container for various supported data types
#[derive(Debug, PartialEq, Clone)]
pub enum Data<I, F>
    where 
        I: Integer, 
        F: Float

{
    Integer(Vec<I>),
    Float(Vec<F>)
}

impl<I, F> Data<I, F>
    where 
        I: Integer + ToPrimitive + NumCast,
        F: Float + ToPrimitive + NumCast
{
    pub fn len(&self) -> usize {
        match self {
            Data::Integer(ref vec) => vec.len(),
            Data::Float(ref vec) => vec.len()
        }
    }

    pub fn get_dtype(&self) -> DType {
        match self {
            Data::Integer(ref _vec) => DType::Int32,
            Data::Float(ref _vec) => DType::Int32
        }
    }

    pub fn astype(self, dtype: DType) -> Self {
        match self {
            Data::Integer(vec) => {
                match dtype {
                    DType::Float64 => Data::Float(vec.into_iter().map(|v| NumCast::from(v).expect("Cannot convert integer to float!")).collect()),
                    DType::Int32 => Data::Integer(vec.into_iter().map(|v| v).collect())
                }
            },
            Data::Float(vec) => {
                match dtype {
                    DType::Float64 => Data::Float(vec.into_iter().map(|v| v).collect()),
                    DType::Int32 => Data::Integer(vec.into_iter().map(|v| NumCast::from(v).expect("Cannot convert float to integer!")).collect())
                }
            }
        }
    }
}

impl<I, F> FromIterator<I> for Data<I, F> 
    where 
        I: Integer,
        F: Float,
{
    fn from_iter<T: IntoIterator<Item=I>>(iter: T) -> Data<I, F> {
        let mut vec = Vec::new();
        for v in iter {
            vec.push(v)
        }
        Data::Integer(vec)
    }
}