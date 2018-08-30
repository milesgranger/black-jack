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
//!

use std::any::Any;
use std::collections::HashMap;

use series::{BlackJackData, Series, SeriesTrait, SeriesEnumRef};

/// Struct for holding [Series](struct.Series.html) or [SeriesTrait](trait.SeriesTrait.html) like objects.
/// as well as adding some additional functionality by grouping them.
#[derive(Default)]
pub struct DataFrame {
    series_objects: HashMap<String, Box<Any>>,
}

impl DataFrame {
    /// Create a new `DataFrame` struct
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut df = DataFrame::new();
    /// ```
    pub fn new() -> Self {
        Self {
            series_objects: HashMap::new(),
        }
    }
}

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
    fn get_column_unknown_type(&self, name: &str) -> Option<SeriesEnumRef>;

    /// Get the number of columns
    fn n_columns(&self) -> usize;
}

impl ColumnManager for DataFrame {

    fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> () {
        self.series_objects
            .entry(series.name().unwrap_or("new-name".to_string()))  // TODO: Pick a name based on number of columns, if no name is provided..
            .or_insert_with(
                || Box::new(series)
            );
    }

    fn get_column<T: BlackJackData>(&self, name: &str) -> Option<&Series<T>> {
        let name = name.to_string();
        let series = self.series_objects.get(&name).unwrap();
        match series.downcast_ref::<Series<T>>() {
            Some(series) => Some(series),
            None => None
        }
    }

    fn get_column_unknown_type(&self, name: &str) -> Option<SeriesEnumRef> {
        /*
            Only way (AFAIK) how to return a Series without requiring the user to specify the type in the call. 
        */
        let name = name.to_string();

        let series_ref: &Box<Any> = self.series_objects.get(&name).unwrap();

        // TODO: Better way?
        match series_ref.downcast_ref::<Series<f64>>() {
            Some(series) => Some(SeriesEnumRef::F64(series)),

            None => match series_ref.downcast_ref::<Series<i64>>() {
                Some(series) => Some(SeriesEnumRef::I64(series)),
            
                None => match series_ref.downcast_ref::<Series<f32>>() {
                    Some(series) => Some(SeriesEnumRef::F32(series)),
            
                    None => match series_ref.downcast_ref::<Series<i32>>() {
                        Some(series) => Some(SeriesEnumRef::I32(series)),
                        None => None,
                    },
                },
            },
        }
    }

    fn n_columns(&self) -> usize {
        self.series_objects.len() as usize
    }
}
