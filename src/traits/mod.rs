//! Traits to be used throughout the crate


use std::fmt::{Debug};
use std::collections::HashSet;

use prelude::*;
mod series_groupby;
mod dataframe_groupby;
pub use self::series_groupby::*;
pub use self::dataframe_groupby::*;

/* 
    Traits used throughout crate
*/

/// Trait dictates the supported primitives for use in [`Series`] structs.
pub trait BlackJackData: Debug + ToString {

    /// Return the current [`DType`] for this type. 
    fn dtype(&self) -> DType;
}
impl BlackJackData for f64 {
    fn dtype(&self) -> DType { DType::F64 }
}
impl BlackJackData for i64 {
    fn dtype(&self) -> DType { DType::I64 }
}
impl BlackJackData for f32 {
    fn dtype(&self) -> DType { DType::F32 }
}
impl BlackJackData for i32 {
    fn dtype(&self) -> DType { DType::I32 }
}
impl BlackJackData for String {
    fn dtype(&self) -> DType { DType::STRING }
}


/*
    DataFrame Traits
*/


/// Define the behavior for managing columns/series within a dataframe
pub trait ColumnManager {
    /// Add a new series to the dataframe as a column.
    fn add_column(&mut self, series: Series) -> Result<(), String>;

    /// Get a reference to a series by name, **will also have to know the 
    /// primitive type stored**.
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut df = DataFrame::new();
    /// let mut series = Series::from_vec(vec![1, 2, 3]);
    /// series.set_name("series1");
    ///
    /// let series_clone = series.clone(); // Create a clone to compare later
    ///
    /// df.add_column(series);  // Add the column to dataframe
    /// // Fetch the column back as a reference.
    /// let series_ref: &Series = df.get_column("series1").unwrap();
    ///
    /// assert_eq!(*series_ref, series_clone)  // ensure they equal.
    /// ```
    fn get_column(&self, name: &str) -> Option<&Series>;

    /// Same as [`ColumnManager::get_column`] but as a `mut` reference.
    fn get_column_mut(&mut self, name: &str) -> Option<&mut Series>;

    /// Get the number of columns
    fn n_columns(&self) -> usize;

    /// Return the current dataframe's columns
    fn columns(&self) -> HashSet<&String>;
}

/// DataFrame behavior
pub trait DataFrameBehavior: Sized {

    /// Transform into a raw pointer
    fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    /// Get a DataFrame from raw pointer
    fn from_raw(ptr: *mut Self) -> Self {
        unsafe { *Box::from_raw(ptr) }
    }
}


/* 
    Series traits
*/
