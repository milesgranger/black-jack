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

use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::error::Error;
use std::fmt;

use csv;
use prelude::*;


/// Struct for holding [`Series`] or [`SeriesTrait`] like objects.
/// as well as adding some additional functionality by grouping them.
#[derive(Default, Debug)]
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


impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printed_series: Vec<Vec<String>> = Vec::new();
        for series in self.series_objects.values() {
            let stdout: Vec<String> = format!("{}", series).split("\n").map(|v| v.to_string()).collect();
            printed_series.push(stdout);
        }
        let mut output: String = "\n".to_string();
        for i in 0..printed_series[0].len() {
            for ii in 0..printed_series.len() {
                output.push_str(&printed_series[ii][i]);
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}


impl DataFrameBehavior for DataFrame {}

impl DataFrameIO for DataFrame {

    fn read_csv<S: AsRef<Path>>(path: S) -> Result<Self, Box<Error>> {
        let mut reader = csv::Reader::from_path(path)?;

        let headers = reader.headers()?.clone();  // TODO: Don't fail on non existant headers -> give 'col0', 'col1', etc.
        println!("Header list: {:?}", headers);

        // Containers for storing column data
        let mut vecs: Vec<Vec<DataElement>> = (0..headers.len()).map(|_| Vec::new()).collect();

        for record in reader.records() {

            match record {

                Ok(rec) => { 
                    println!("Record: {:?}", &rec);
                    for (field, container) in rec.iter().zip(&mut vecs) {
                        container.push(
                            DataElement::from_parse(field)
                        );
                    };
                },

                // TODO: Process for dealing with invalid records.
                Err(err) => println!("Unable to read record: '{}'", err)
            }
        }

        // TODO: Place into Series and start converting and comparing against primitives.. 
        // for example, convert to f64 and see if that is partially equal to i64, if so, keep i64
        // if all numeric conversion trials fail, assume strings.
        let mut df = DataFrame::new();
        for (header, vec) in headers.into_iter().zip(vecs) {
            let mut series = Series::from_data_elements(vec);
            series.set_name(header);
            df.add_column(series);
        }

        Ok(df)
    }
}

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

    fn columns(&self) -> HashSet<&String> {
        let columns: HashSet<&String> = self.series_objects.keys().collect();
        columns
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
        let name: String = name.into();

        match self.get_column_mut(&name) {
            Some(series) => series,
            None => panic!("No column named: '{}'", name)
        }
    }
}
