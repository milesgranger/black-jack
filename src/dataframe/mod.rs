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

use std::collections::HashMap;
use std::ops::{Index, IndexMut};

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
        let n_cols = self.n_columns();
        self.series_objects
            .entry(series.name()
                    .unwrap_or_else(|| format!("COL_{}", n_cols) ))
            .or_insert_with(|| series );
    }

    fn get_column(&self, name: &str) -> Option<&Series> {
        let name = name.to_string();
        self.series_objects.get(&name)
    }

    fn get_column_mut(&mut self, name: &str) -> Option<&mut Series>{
        let name: String = name.into();
        self.series_objects.get_mut(&name)
    }

    fn n_columns(&self) -> usize {
        self.series_objects.len() as usize
    }
}

// Support `let series = &DataFrame["some-column-name"]`
impl<S: Into<String>> Index<S> for DataFrame {
    type Output = Series;

    fn index(&self, name: S) -> &Series {
        let name: String = name.into();

        match self.get_column(&name) {
            Some(series) => series,
            None => panic!("No column named: '{}'", name)
        }
    }
}

// Support `DataFrame["some-column-name"] = some_series;`
impl<S: Into<String>> IndexMut<S> for DataFrame {
    fn index_mut(&mut self, name: S) -> &mut Series {
        // TODO: Find a way to error if mismatch in series sizes.
        let name: String = name.into();

        // Create a series and set the name to the name given here
        let mut series = Series::arange(0, 10);
        series.set_name(&name);
        self.add_column(series);

        // Fetch back the column as a mutable reference.
        match self.get_column_mut(&name) {
            Some(series) => series,
            None => panic!("No column named '{}'", name)
        }
    }
}
