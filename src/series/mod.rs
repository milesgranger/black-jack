
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
pub struct Series<I>
    where I: LumberJackData
{
    pub name: Option<String>,
    pub data: Vec<I>
}

pub trait SeriesData<I> 
    where I: LumberJackData
{

    fn arange(start: I, stop: I) -> Self
        where
            I: Integer + LumberJackData, 
            Self: Sized,
            Range<I>: Iterator, 
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
            Vec<I>: From<Vec<I>>,
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>;

    fn len(&self) -> usize;
    fn name(&self) -> Option<String>;
    fn set_name(&mut self, name: String) -> ();
}


// Satisfy an `Series::arange()` Call, converting a stricter type of LumberJackData to LumberJackData base
impl<I> From<Vec<I>> for Data<I> 
    where
        I: Integer + LumberJackData, 
        Self: Sized,
        Range<I>: Iterator, 
        Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
        Vec<I>: From<Vec<I>>
{
    fn from(vec: Vec<I>) -> Data<I> {
        Data::Integer(vec)
    }
}

impl<I: LumberJackData> Series<I> {}

impl<I: LumberJackData> SeriesData<I> for Series<I>
{

    fn arange(start: I, stop: I) -> Self
        where
            I: Integer + LumberJackData, 
            Self: Sized,
            Range<I>: Iterator, 
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>, 
            Vec<I>: From<Vec<I>>,
            Vec<I>: FromIterator<<Range<I> as Iterator>::Item>
    {
        let data: Vec<I> = (start..stop).collect();
        Series { name: None, data}
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn name(&self) -> Option<String> {
        self.name.clone()
    }
    fn set_name(&mut self, name: String) -> () {
        self.name = Some(name);
    }
}
