use num::*;
use std::ops::Range;
use std::iter::FromIterator;

/// Trait which is implemented for all supported data types (i32, f64, ect)
pub trait BlackJackData {}
impl BlackJackData for i32 {}
impl BlackJackData for f64 {}


/// Series struct, meta data surrounding the underlying Vec<BlackJackData>
pub struct Series<T: BlackJackData> {
    data: Vec<T>
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
        Series { data }
    }
}


/// Trait defining functionality of a Series object.
pub trait SeriesObj {
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
}


impl<T: BlackJackData> SeriesObj for Series<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
}


