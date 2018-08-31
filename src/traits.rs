//! Traits to be used throughout the crate


use std::fmt::Debug;
use std::any::{Any};

use num::*;
use prelude::*;

/* 
    Traits used throughout crate
*/

/// Trait dictates the supported primitives for use in [Series](struct.Series.html) structs.
pub trait BlackJackData: Debug + 'static {

    /// Return the current [DType](enum.DType.html) for this type. 
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
    fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> ();

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
    /// let series_ref: &Series<i32> = df.get_column("series1").unwrap();  // Fetch the column back as a reference.
    ///
    /// assert_eq!(*series_ref, series_clone)  // ensure they equal.
    /// ```
    fn get_column<T: BlackJackData>(&self, name: &str) -> Option<&Series<T>>;

    /// Get a column of which the type is unknown
    /// Returns a [SeriesEnum](enum.SeriesEnum.html) of which will need to `match` the 
    /// resulting series type and deal with accordingly.
    fn get_column_unknown_type(&self, name: &str) -> Option<SeriesEnum>;

    /// Get the number of columns
    fn n_columns(&self) -> usize;
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

    /// The primitive associated with this Series; ie. `f64`
    type Item: BlackJackData;

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
    fn sum(&self) -> Self::Item 
        where Self::Item: Num + Clone;

    /// Average / Mean of a given series - Requires specifying desired float return annotation 
    /// 
    /// ## Example:
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series = Series::arange(0, 5);
    /// let mean = series.mean::<f64>();
    /// 
    /// match mean {
    ///     Ok(result) => println!("Result is: {}", result),
    ///     Err(err) => println!("Was unable to compute mean, error: {}", err)
    /// }
    /// ```
    fn mean<A>(&self) -> Result<A, &'static str> 
        where 
            A: Float, 
            Self::Item: Num + Clone + ToPrimitive;

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