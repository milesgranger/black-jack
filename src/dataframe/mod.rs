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

use std::collections::HashMap;
use std::any::{Any};

use series::{SeriesTrait, Series, BlackJackData};

/// Struct for holding [Series](struct.Series.html) or [SeriesTrait](trait.SeriesTrait.html) like objects. 
/// as well as adding some additional functionality by grouping them.
#[derive(Default)]
pub struct DataFrame {
    containers: HashMap<String, Box<Any>>,
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
        Self { containers: HashMap::new() }
    }
}


/// Define the behavior for managing columns/series within a dataframe
pub trait ColumnManager {

    /// Add a new series to the dataframe as a column.
    fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> ();

    /// Get a reference to a series by name, will also have to know the primitive type stored.
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

    /// Get the number of columns
    fn n_columns(&self) -> usize;

}

impl ColumnManager for DataFrame {

    fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> () {

        self.containers
            .entry(series.name().unwrap_or("new-name".to_string()))  // TODO: Pick a name based on number of columns, if no name is provided..
            .or_insert_with(
                || Box::new(series)
            );
    }

    fn get_column<T: BlackJackData>(&self, name: &str) -> Option<&Series<T>> {
        let name = name.to_string();

        let series = self.containers.get(&name).unwrap();
        match series.downcast_ref::<Series<T>>() {
            Some(series) => Some(series),
            None => None
        }
    }

    fn n_columns(&self) -> usize {
        self.containers.len() as usize
    }
}