
use std::ops::Range;
use std::iter::{Iterator, FromIterator};
use containers::{Data, DType};
use num::*;
use num_traits::*;


pub trait LumberJackData {
    fn dtype(&self) -> DType;
}
impl LumberJackData for f64 {
    fn dtype(&self) -> DType {
        DType::Float64
    }
}
impl LumberJackData for i32 {
    fn dtype(&self) -> DType {
        DType::Int32
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Series<I, F>
    where 
        I: Integer,
        F: Float
{
    pub name: Option<String>,
    pub data: Data<I, F>
}

impl<I, F> Series<I, F>
    where 
        I: Integer + NumCast,
        F: Float + NumCast
{
    pub fn arange(start: I, stop: I) -> Self
        where I:
            Integer,
            Range<I>: Iterator, 
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
            Vec<I>: From<Vec<I>>
    {
        let data: Vec<I> = (start..stop).collect::<Vec<I>>().into();
        Series { name: None, data: Data::Integer(data)}
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn from_vec<T>(vec: Vec<T>) -> Self 
        where T: LumberJackData, Data<I, F>: FromIterator<T>
    {
        let data = Data::from_iter(vec.into_iter());
        Series { name: None, data }
    }
}