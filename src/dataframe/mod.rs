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
use std::any::{Any, TypeId};

use series::{SeriesTrait, Container, Series, BlackJackData};

/// Struct for holding [Series](struct.Series.html) or [SeriesTrait](trait.SeriesTrait.html) like objects. 
/// as well as adding some additional functionality by grouping them.
#[derive(Default)]
pub struct DataFrame {
    containers: HashMap<TypeId, Box<Any>>,
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
    fn get_container_mut<T: BlackJackData>(&mut self) -> &mut <Series<T> as SeriesTrait>::Container {
        let type_id = TypeId::of::<T>();

        // Add a storage if it doesn't exist yet
        self.containers
            .entry(type_id)
            .or_insert_with(
                || Box::new(<Series<T> as SeriesTrait>::Container::new())
            );

        // Get the storage for this type
        match self.containers.get_mut(&type_id) {
            Some(probably_container) => {
                // Turn the Any into the storage for that type
                match probably_container.downcast_mut::<<Series<T> as SeriesTrait>::Container>() {
                    Some(container) => container,
                    None => unreachable!(), // <- you may want to do something less explosive here
                }
            }
            None => unreachable!(),
        }
    }

    /// Add a new series to the dataframe. 
    /// 
    /// ## Example:
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let mut df = DataFrame::new();
    /// let series = Series::arange(0, 10);
    /// 
    /// df.add_column(series);
    /// ```
    pub fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> () {
        self.get_container_mut::<T>().insert(series);
    }

        /// Get a reference to a series by name
    pub fn get_column_ref<T: BlackJackData>(&self, name: &str) -> Option<&Series<T>> {
        let name = name.to_string();
        for (_typeid, container) in &self.containers {
            match container.downcast_ref::<<Series<T> as SeriesTrait>::Container>() {
                Some(container) => {
                    match container.get_ref(&name) {
                        Some(series) => return Some(series),
                        None => continue
                    }
                },
                None => continue
            };
        }
        None
    }

}