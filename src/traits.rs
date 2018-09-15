//! Traits to be used throughout the crate


use std::fmt::{Debug};
use std::any::{Any};
use std::iter::{Sum};
use std::collections::HashSet;


use num::*;
use prelude::*;

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
    fn add_column(&mut self, series: Series) -> ();

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

/// Define the behavior of a Series object.
pub trait SeriesTrait: Debug + Sized + Any {

    /// Set the name of a series
    fn set_name(&mut self, name: &str) -> ();

    /// Get the name of the series; Series may not be assigned a string, 
    /// so an `Option` is returned.
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

    /// Finds the returns a [`Series`] containing the mode(s) of the current
    /// [`Series`]
    fn mode<T>(&self) -> Result<Self, &'static str>
        where T: BlackJackData + From<DataElement> + PartialOrd + Clone + ToPrimitive;

    /// Calculate the variance of the series  
    /// **NOTE** that whatever type is determined is what the values are cast to
    /// during calculation of the variance. 
    /// 
    /// ie. `series.var::<i32>()` will cast each element into `i32` as input
    /// for calculating the variance, and yield a `i32` value. If you want all
    /// values to be calculated as `f64` then specify that in the type annotation.
    fn var<T>(&self) -> Result<T, &'static str>
        where 
            T: BlackJackData + From<DataElement> + ToPrimitive + Clone;

    /// Sum a given series, yielding the same type as the elements stored in the 
    /// series.
    fn sum<T>(&self) -> T
        where T: Num + Clone + From<DataElement> + Sum + Copy;

    /// Average / Mean of a given series - Requires specifying desired float 
    /// return annotation 
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

    /// Find the minimum of the series. If several elements are equally minimum,
    /// the first element is returned. If it's empty, an Error will be returned.
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

    /// Exibits the same behavior and usage of [`SeriesTrait::min`], only
    /// yielding the [`Result`] of a maximum.
    fn max<T>(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord + From<DataElement>;

    /// Determine the length of the Series
    fn len(&self) -> usize;

    /// Determine if series is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the dtype, returns `None` if series dtype is unknown. 
    /// in such a case, calling `.astype()` to coerce all types to a single
    /// type is needed. 
    fn dtype(&self) -> Option<DType>;

    /// Cast all [`DataElement`]s within a series to a given [`DType`]
    /// Will fail if series contains a string and asking for an integer, 
    /// of an `NaN` and asking for an integer.
    /// 
    /// ie. "Hello" -> .astype([`DType::I64`]) -> **Error!**  
    /// ie. "Hello" -> .astype([`DType::F64`]) -> `NaN`  
    /// ipso-facto... `NaN` -> .astype([`DType::I64`]) -> **Error!**
    fn astype(&mut self, dtype: DType) -> Result<(), &'static str>;

    /// Append a [`BlackJackData`] element to the Series
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let mut series = Series::from_vec(vec![0, 1, 2]);
    /// assert_eq!(series.len(), 3);
    /// 
    /// series.append(3);
    /// assert_eq!(series.len(), 4);
    /// ```
    fn append<V: Into<DataElement>>(&mut self, val: V) -> ();

    /// As boxed pointer, recoverable by `Box::from_raw(ptr)` or 
    /// `SeriesTrait::from_raw(*mut Self)`
    fn into_raw(self) -> *mut Self { 
        Box::into_raw(Box::new(self)) 
    }

    /// Create from raw pointer
    fn from_raw(ptr: *mut Self) -> Self { 
        unsafe { *Box::from_raw(ptr) } 
    }
}