
use std::ops::Range;
use std::iter::{Iterator, FromIterator};
use containers::{Data, DType};
use num::Integer;


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
pub struct Series
{
    pub name: Option<String>,
    pub data: Data
}

impl Series
{
    pub fn arange<I>(start: I, stop: I) -> Self
        where I: 
            Integer, 
            Range<I>: Iterator, 
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
            Vec<i32>: From<Vec<I>> 
    {
        let data: Vec<i32> = (start..stop).collect::<Vec<I>>().into();
        Series { name: None, data: Data::Int32(data)}
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn from_vec<T>(vec: Vec<T>) -> Self 
        where T: LumberJackData, Data: FromIterator<T>
    {
        let data = Data::from_iter(vec.into_iter());
        Series { name: None, data }
    }
}