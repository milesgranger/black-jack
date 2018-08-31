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

use ndarray::Array1 as Array;

pub mod traits;
use prelude::*;



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

    fn dtype(&self) -> DType { 
        // TODO: Add len check, return Option instead.
        self.values[0].dtype()
     }

}
