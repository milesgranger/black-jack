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

use prelude::*;


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


impl DataFrameBehavior for DataFrame {}


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

    fn get_column_unknown_type(&self, name: &str) -> Option<SeriesEnum> {
        /*
            Only way (AFAIK) how to return a Series without requiring the user to specify the type in the call. 
        */
        let name = name.to_string();

        let series_ref: &Box<Any> = self.series_objects.get(&name).unwrap();

        // TODO: Better way?
        match series_ref.downcast_ref::<Series<f64>>() {
            Some(series) => Some(SeriesEnum::F64(series)),

            None => match series_ref.downcast_ref::<Series<i64>>() {
                Some(series) => Some(SeriesEnum::I64(series)),
            
                None => match series_ref.downcast_ref::<Series<f32>>() {
                    Some(series) => Some(SeriesEnum::F32(series)),
            
                    None => match series_ref.downcast_ref::<Series<i32>>() {
                        Some(series) => Some(SeriesEnum::I32(series)),
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
