//! A [`DataFrame`] represents a collection of varying types of [`Series`] objects.
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

use prelude::*;


/// Struct for holding [`Series`] or [`SeriesTrait`] like objects.
/// as well as adding some additional functionality by grouping them.
#[derive(Default)]
pub struct DataFrame {
    series_objects: HashMap<String, Series>,
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


impl DataFrameBehavior for DataFrame {}


impl ColumnManager for DataFrame {

    fn add_column(&mut self, series: Series) -> () {
        self.series_objects
            .entry(series.name().unwrap_or("new-name".to_string()))  // TODO: Pick a name based on number of columns, if no name is provided..
            .or_insert_with(
                || series
            );
    }

    fn get_column(&self, name: &str) -> Option<&Series> {
        let name = name.to_string();
        self.series_objects.get(&name)
    }

    fn n_columns(&self) -> usize {
        self.series_objects.len() as usize
    }
}
