//! Series represents a single column within a dataframe and wraps many `Array` like
//! functionality.
//! 
//! For methods implemented for a `Series`, please check out the trait [SeriesObj](trait.SeriesObj.html)
//! 
//! ## Example use:
//! 
//! ```
//! use blackjack::prelude::*;
//! 
//! let series = Series::arange(0, 5);
//! 
//! assert_eq!(series.sum(), 10);
//! ```

use num::*;
use std::ops::Range;
use std::collections::HashMap;
use std::iter::{FromIterator};
use std::fmt::Debug;
use std::any::{Any, TypeId};

use ndarray::Array1 as Array;

pub trait BlackJackData: Debug + 'static {}
impl BlackJackData for f64 {}
impl BlackJackData for i64 {}
impl BlackJackData for f32 {}
impl BlackJackData for i32 {}

#[derive(Debug)]
pub struct VecStorage<T: Debug + 'static> {
    internal: Vec<T>,
}

#[derive(Debug)]
pub struct Series<T: BlackJackData> {
    pub data: Array<T>
}

impl<T: BlackJackData> Series<T> {

    pub fn arange(start: T, stop: T) -> Self 
        where
            T: Integer, 
            Self: Sized,
            Range<T>: Iterator, 
            Vec<T>: FromIterator<<Range<T> as Iterator>::Item>, 
            Vec<T>: From<Vec<T>>
    {
        let data: Vec<T> = (start..stop).collect();
        Series { data: Array::from_vec(data) }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Series { data: Array::from_vec(vec) }
    }
}

pub trait SeriesTrait: Debug + Sized + Any {

    type Container: Container<Self>;
    type Item;

    fn sum(&self) -> Self::Item where Self::Item: Num + Clone;

}

impl<T: BlackJackData> SeriesTrait for Series<T> {
    type Container = VecStorage<Self>;
    type Item = T;

    fn sum(&self) -> T  where T: Num + Clone {
        self.data.scalar_sum()
    }
}


impl<T: Debug> Container<T> for VecStorage<T> {
    fn new() -> Self {
        Self { internal: Vec::new() }
    }
    fn insert(&mut self, value: T) {
        self.internal.push(value);
    }
}



pub trait Container<T: Debug>: Debug + Any {
    fn new() -> Self where Self: Sized;
    fn insert(&mut self, value: T);
}



