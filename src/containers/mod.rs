
use std::iter::FromIterator;
use num::*;
use num::integer::Integer;
use series::LumberJackData;


/// Define which data types can be requested or cast to.
/// to serve as flags between Cython and Rust for data type conversions / creations
#[derive(Debug, PartialEq)]
pub enum DType {
    Float,
    Integer
}

/// Container for various supported data types
#[derive(Debug, PartialEq, Clone)]
pub enum Data<T>
    where 
        T: LumberJackData

{
    Integer(Vec<T>),
    Float(Vec<T>)
}

impl<T> Data<T>
    where 
        T: LumberJackData
{
    pub fn len(&self) -> usize {
        match self {
            Data::Integer(ref vec) => vec.len(),
            Data::Float(ref vec) => vec.len()
        }
    }

    pub fn get_dtype(&self) -> DType {
        match self {
            Data::Integer(ref _vec) => DType::Integer,
            Data::Float(ref _vec) => DType::Float
        }
    }

    pub fn astype(self, dtype: DType) -> Self 
        where T: NumCast
    {
        match self {
            Data::Integer(vec) => {
                match dtype {
                    DType::Float => Data::Float(vec.into_iter().map(|v| NumCast::from(v).expect("Cannot convert integer to float!")).collect()),
                    DType::Integer => Data::Integer(vec.into_iter().map(|v| v).collect())
                }
            },
            Data::Float(vec) => {
                match dtype {
                    DType::Float => Data::Float(vec.into_iter().map(|v| v).collect()),
                    DType::Integer => Data::Integer(vec.into_iter().map(|v| NumCast::from(v).expect("Cannot convert float to integer!")).collect())
                }
            }
        }
    }
}

impl<T> FromIterator<T> for Data<T> 
    where
        T: LumberJackData
{
    fn from_iter<A: IntoIterator<Item=T>>(iter: A) -> Data<T> {
        let mut vec = Vec::new();
        for v in iter {
            vec.push(v)
        }
        Data::Integer(vec)
    }
}