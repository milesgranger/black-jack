//! Traits to be used throughout the crate


use std::fmt::Debug;
use std::any::{Any};
use std::iter::{Sum};
use std::path::Path;
use std::error::Error;

use num::*;
use prelude::*;

/* 
    Traits used throughout crate
*/

/// Trait dictates the supported primitives for use in [`Series`] structs.
pub trait BlackJackData: Copy + Debug + ToPrimitive {

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


/*
    DataFrame Traits
*/


/// Define the behavior for managing columns/series within a dataframe
pub trait ColumnManager {
    /// Add a new series to the dataframe as a column.
    fn add_column(&mut self, series: Series) -> ();

    /// Get a reference to a series by name, **will also have to know the primitive type stored**.
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
    ///
    /// let series_ref: &Series = df.get_column("series1").unwrap();  // Fetch the column back as a reference.
    ///
    /// assert_eq!(*series_ref, series_clone)  // ensure they equal.
    /// ```
    fn get_column(&self, name: &str) -> Option<&Series>;

    /// Same as [`ColumnManager::get_column`] but as a `mut` reference.
    fn get_column_mut(&mut self, name: &str) -> Option<&mut Series>;

    /// Get the number of columns
    fn n_columns(&self) -> usize;
}

/// Represents behavior for [`DataFrame`] io behavior
pub trait DataFrameIO: Sized {

    /// Read a CSV file into a [`DataFrame`] where each column represents a Series
    fn read_csv<S: AsRef<Path>>(path: S) -> Result<Self, Box<Error>>;
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

/// Define the behavior of a Series object.
pub trait SeriesTrait: Debug + Sized + Any {

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
    fn sum<T>(&self) -> T
        where T: Num + Clone + From<DataElement> + Sum;

    /// Average / Mean of a given series - Requires specifying desired float return annotation 
    /// 
    /// ## Example:
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series = Series::arange(0, 5);
    /// let mean = series.mean();
    /// 
    /// match mean {
    ///     Ok(result) => {
    ///         println!("Result is: {}", &result);
    ///         assert_eq!(result, 2.0);
    ///     },
    ///     Err(err) => {
    ///         panic!("Was unable to compute mean, error: {}", err);
    ///     }
    /// }
    /// ```
    fn mean(&self) -> Result<f64, &'static str>;

    /// Find the minimum of the series. If several elements are equally minimum, the first element is returned. 
    /// If it's empty, an Error will be returned
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series: Series = Series::arange(10, 100);
    /// 
    /// assert_eq!(series.min(), Ok(10));
    /// ```
    fn min<T>(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord + BlackJackData + From<DataElement>;

    /// Exibits the same behavior and usage of [`SeriesTrait::min`], only yielding the [`Result`] of a maximum.
    fn max<T>(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord + From<DataElement>;

    /// Determine the length of the Series
    fn len(&self) -> usize;

    /// Determine if series is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the dtype
    fn dtype(&self) -> DType;

    /// As boxed pointer, recoverable by `Box::from_raw(ptr)` or `SeriesTrait::from_raw(*mut Self)`
    fn into_raw(self) -> *mut Self { 
        Box::into_raw(Box::new(self)) 
    }

    /// Create from raw pointer
    fn from_raw(ptr: *mut Self) -> Self { 
        unsafe { *Box::from_raw(ptr) } 
    }
}