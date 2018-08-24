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
use std::iter::{FromIterator};

use ndarray::Array1 as Array;

/// Trait which is implemented for all supported data types (i32, f64, ect)
pub trait BlackJackData {}  // TODO: Implement an enum to get the high-level type (Integer, Float, String)
impl BlackJackData for i32 {}
impl BlackJackData for f32 {}
impl BlackJackData for i64 {}
impl BlackJackData for f64 {}



/// Series struct, meta data surrounding the underlying Vec<BlackJackData>
pub struct Series<T: BlackJackData> {
    data: Array<T>
}

/// Implement functions capable of creating a series.
impl<T: BlackJackData> Series<T> {

    /// Create a new series via a range, with one step increments. 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// let series: Series<i32> = Series::arange(0_i32, 10_i32);
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
}


/// Trait defining functionality of a Series object.
pub trait SeriesObj {

    /// The trait of the element output
    type OutputElement;

    /// Fetch the length of the current series
    /// 
    /// ## Example
    /// 
    /// ```
    /// use blackjack::prelude::*;
    /// let series = Series::arange(0, 5);
    /// assert_eq!(series.len(), 5);
    /// ```
    fn len(&self) -> usize;

    /// Sum a series, where the datatype meets the conditions of `Clone` and `Num`
    /// 
    /// ## Example
    /// 
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series = Series::arange(0, 5);
    /// assert_eq!(series.sum(), 10);
    /// ```
    fn sum(&self) -> Self::OutputElement where Self::OutputElement: Num + Clone;
}


impl<T: BlackJackData> SeriesObj for Series<T> {

    type OutputElement = T;

    fn len(&self) -> usize {
        self.data.len()
    }

    fn sum(&self) -> Self::OutputElement
        where Self::OutputElement: Num + Clone
    {
        self.data.scalar_sum()
    }
}


