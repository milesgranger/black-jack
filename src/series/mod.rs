
use std::ops::Range;
use std::iter::{Iterator, FromIterator};
use containers::{Data, DType};
use num::*;
use std::fmt::Debug;


pub trait LumberJackData: Debug + Copy + Clone  {
    fn dtype(&self) -> DType;
}

impl LumberJackData for i32 {
    fn dtype(&self) -> DType {
        DType::Integer
    }
}

impl LumberJackData for f64 {
    fn dtype(&self) -> DType {
        DType::Float
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Series<T>
    where 
        T: LumberJackData
{
    pub name: Option<String>,
    pub data: Data<T>
}

pub trait SeriesData {

    fn arange<I: Integer + LumberJackData>(start: I, stop: I) -> Series<I>
        where 
            Self: Sized,
            I:
                Integer,
                Range<I>: Iterator, 
                Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
                Vec<I>: From<Vec<I>>;

    fn len(&self) -> usize;
}

impl<A: LumberJackData> SeriesData for Series<A> 
{

    fn arange<I: Integer + LumberJackData>(start: I, stop: I) -> Series<I>
        where I:
            Integer,
            Range<I>: Iterator, 
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
            Vec<I>: From<Vec<I>>
    {
        let data: Vec<I> = (start..stop).collect();
        Series { name: None, data: Data::Integer(data)}
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
