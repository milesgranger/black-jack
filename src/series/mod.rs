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
//! assert_eq!(series.len(), 5);
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


/// Series struct for containing underlying Array and other meta data.
#[derive(Debug, Clone, PartialEq)]
pub struct Series<T: BlackJackData> {
    
    /// Name of the series, if added to a dataframe without a name, it will be assigned
    /// a default name equalling the cound of columns in the dataframe.
    pub name: Option<String>,

    /// ndarray attribute; the underlying values of the Series
    pub values: Array<T>
}

/// Constructor methods for `Series<T>`
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
            Vec<T>: FromIterator<<Range<T> as Iterator>::Item>
    {
        let data: Vec<T> = (start..stop).collect();
        Series { 
            name: None,
            values: Array::from_vec(data), 
        }
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
        Series { 
            name: None,
            values: Array::from_vec(vec),
        }
    }
}


/// Define the behavior of a Series object.
pub trait SeriesTrait: Debug + Sized + Any {

    /// The primitive associated with this Series; ie. `f64`
    type Item;

    /// Set the name of a series
    fn set_name(&mut self, name: &str) -> ();

    /// Get the name of the series; Series may not be assigned a string, so an `Option` is returned.
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let mut series = Series::from_vec(vec![1, 2, 3]);
    /// series.set_name("my-series");
    /// 
    /// assert_eq!(series.name(), Some("my-series".to_string()));
    /// ```
    fn name(&self) -> Option<String>;

    /// Sum a given series, yielding the same type as the elements stored in the series.
    fn sum(&self) -> Self::Item where Self::Item: Num + Clone;

    /// Determine the length of the Series
    fn len(&self) -> usize;

    /// Determine if series is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }
}

impl<T: BlackJackData> SeriesTrait for Series<T> {
    type Item = T;

    fn set_name(&mut self, name: &str) -> () {
        self.name = Some(name.to_string());
    }

    fn name(&self) -> Option<String> {
        match self.name {
            Some(ref name) => Some(name.clone()),
            None => None
        }
    }

    fn sum(&self) -> T  where T: Num + Clone {
        self.values.scalar_sum()
    }

    fn len(&self) -> usize { self.values.len() }
}



