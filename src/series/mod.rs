
use std::ops::Range;
use std::iter::{Iterator, FromIterator};
use containers::{Data, DType};
use num::Integer;
use num_traits::real::Real;


pub trait LumberJackData {}
impl LumberJackData for f64 {}
impl LumberJackData for i64 {}

#[derive(Clone, Debug, PartialEq)]
pub struct Series<'a>
{
    pub name: &'a str,
    pub data: Data
}

impl<'a> Series<'a>
{
    pub fn arange<I>(start: I, stop: I) -> ()
        where I: Integer, Range<I>: Iterator, Vec<I>: FromIterator<<Range<I> as Iterator>::Item>
    {
        let range = (start..stop).collect::<Vec<I>>();
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}