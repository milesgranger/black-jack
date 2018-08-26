//! Series represents a single column within a dataframe and wraps many `Array` like
//! functionality.
//! 
//! For methods implemented for a `Series`, please check out the trait [SeriesTrait](trait.SeriesTrait.html)
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
use std::iter::{FromIterator};
use std::fmt::Debug;
use std::any::{Any};

use ndarray::Array1 as Array;

/// Trait dictates the supported primitives for use in [Series](struct.Series.html) structs.
pub trait BlackJackData: Debug + 'static {}
impl BlackJackData for f64 {}
impl BlackJackData for i64 {}
impl BlackJackData for f32 {}
impl BlackJackData for i32 {}


/// Container for storing Series objects of the same type
#[derive(Debug)]
pub struct VecStorage<T: Debug + 'static> {
    internal: Vec<T>,
}


/// Series struct for containing underlying Array and other meta data.
#[derive(Debug)]
pub struct Series<T: BlackJackData> {
    
    /// ndarray attribute; the underlying values of the Series
    pub data: Array<T>
}

impl<T: BlackJackData> Series<T> {

    /// Create a new Series struct from an integer range with one step increments. 
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series: Series<i32> = Series::arange(0, 10);
    /// ```
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

    /// Create a new Series struct from a vector, where T is supported by [BlackJackData](trait.BlackJackData.html). 
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series: Series<i32> = Series::from_vec(vec![1, 2, 3]);
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Self {
        Series { data: Array::from_vec(vec) }
    }
}


/// Define the behavior of a Series object.
pub trait SeriesTrait: Debug + Sized + Any {

    /// The container storage for which any series objects will be stored into; used by the [DataFrame](struct.DataFrame.html)
    /// to dynamically create new containers if a [Series](struct.Series.html) is added and needs a proper `Vec<T>` for storage
    type Container: Container<Self>;

    /// The primitive associated with this Series; ie. `f64`
    type Item;

    /// Sum a given series, yielding the same type as the elements stored in the series.
    fn sum(&self) -> Self::Item where Self::Item: Num + Clone;

    /// Determine the length of the Series
    fn len(&self) -> usize;
}

impl<T: BlackJackData> SeriesTrait for Series<T> {
    type Container = VecStorage<Self>;
    type Item = T;

    fn sum(&self) -> T  where T: Num + Clone {
        self.data.scalar_sum()
    }

    fn len(&self) -> usize { self.data.len() }
}


impl<T: Debug> Container<T> for VecStorage<T> {
    fn new() -> Self {
        Self { internal: Vec::new() }
    }
    fn insert(&mut self, value: T) {
        self.internal.push(value);
    }
}

/// Container behavior for creating and inserting new storage containers. 
pub trait Container<T: Debug>: Debug + Any {

    /// Create a new container
    fn new() -> Self where Self: Sized;

    /// Insert a new value into this container
    fn insert(&mut self, value: T);
}



