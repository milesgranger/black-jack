//! A `DataFrame` represents a collection of varying types of `Series` objects.
//! 
//! ## Example use:
//! 
//! ```
//! use blackjack::prelude::*;
//! 
//! let mut df = DataFrame::new();
//! let series = Series::arange(0, 5);
//! 
//! df.add_column(series);
//! ```
use series::{SeriesObj};


/// Holds a collection of `blackjack::series::SeriesObj` traits
/// additing additional functionality that comes with additional series combined in one place. 
pub struct DataFrame<S>
    where S: SeriesObj
{
    data: Vec<Box<S>>
}

impl<S: SeriesObj> DataFrame<S> {

    /// Create a new, empty dataframe
    pub fn new() -> DataFrame<S> {
        let data: Vec<Box<S>> = Vec::new();
        DataFrame { data }
    }

    /// Add a series to the dataframe
    /// 
    /// ## Example
    /// 
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let mut df = DataFrame::new();
    /// let series = Series::arange(0, 5);
    /// df.add_column(series);
    /// ```
    pub fn add_column(&mut self, series: S) -> () {
        self.data.push(Box::new(series));
    }
}