
use std::ops::Range;
use std::iter::{Iterator, FromIterator};
use containers::{Data, DType};
use num::Integer;
use num_traits::real::Real;


pub trait LumberJackData {}
impl LumberJackData for f64 {}
impl LumberJackData for i64 {}

#[derive(Clone, Debug, PartialEq)]
pub struct Series<'a, T> 
    where T: LumberJackData
{
    pub name: &'a str,
    pub data: Vec<T>
}

impl<'a, T> Series<'a, T>
    where T: LumberJackData
{
    pub fn arange<I>(start: I, stop: I) -> ()
        where I: Integer, Range<I>: Iterator, Vec<T>: FromIterator<<Range<I> as Iterator>::Item>
    {
        let range = (start..stop).collect::<Vec<T>>();
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}