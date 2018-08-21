
use std::ops::Range;
use std::iter::{Iterator, FromIterator};
use containers::{Data, DType};
use num::*;
use std::fmt::Debug;


pub trait LumberJackData {
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
    where T: LumberJackData
{
    pub name: Option<String>,
    pub data: Data<T>
}

pub trait SeriesData {

    type T: LumberJackData;

    fn arange<I>(start: I, stop: I) -> Series<Self::T>
        where
            Self::T: Integer,
            I: Integer + LumberJackData, 
            Self: Sized,
            Range<I>: Iterator, 
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
            Vec<Self::T>: From<Vec<I>>,
            Vec<Self::T>: FromIterator<<Range<I> as Iterator>::Item>;

    fn len(&self) -> usize;
}

impl<A: LumberJackData> SeriesData for Series<A> 
{

    type T = A;

    fn arange<I>(start: I, stop: I) -> Series<Self::T>
        where
            Self::T: Integer,
            I: Integer + LumberJackData, 
            Self: Sized,
            Range<I>: Iterator, 
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
            Vec<Self::T>: From<Vec<I>>,
            Vec<Self::T>: FromIterator<<Range<I> as Iterator>::Item>
    {
        let data: Vec<Self::T> = (start..stop).collect();
        Series { name: None, data: Data::Integer(data)}
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
