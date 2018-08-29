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

    /// Get a new mutable container given type annocation. ie. `df.get_container_mut::<Series<i32>>()` 
    /// yielding a mutable reference to the dataframes's  `Vec<Series<i32>>`
    pub fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> () {

        // Add a storage if it doesn't exist yet
        self.containers
            .entry(series.name().unwrap_or("new-name".to_string()))  // TODO: Pick a name based on number of columns, if no name is provided..
            .or_insert_with(
                || Box::new(series)
            );
    }

    /// Get a reference to a series by name
    pub fn get_column<T: BlackJackData>(&self, name: &str) -> Option<&Series<T>> {
        let name = name.to_string();
        let series = self.containers.get(&name).unwrap();
        match series.downcast_ref::<Series<T>>() {
            Some(series) => Some(series),
            None => None
        }
    }

}