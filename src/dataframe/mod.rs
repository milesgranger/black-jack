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
use std::ffi::OsStr;
use std::error::Error;
use std::fmt;

use rayon::prelude::*;
use csv;
use prelude::*;


/// Struct for holding [`Series`] or [`SeriesTrait`] like objects.
/// as well as adding some additional functionality by grouping them.
#[derive(Default, Debug)]
pub struct DataFrame {
    series_objects: HashMap<String, Series>,
    index: Vec<DataElement>
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
            index: vec![]
        }
    }

    /// Length of the dataframe
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Get a reference to the current index
    pub fn index(&self) -> &Vec<DataElement> {
        &self.index
    }

    /// Set the index of this dataframe from an iterator producing elements
    /// which can be transformed into [`DataElement`]s; ie, any [`BlackJackData`]
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let s1 = Series::arange(0, 5);
    /// let idx = 1..s1.len() + 1;
    /// 
    /// let mut df = DataFrame::new();
    /// df.add_column(s1).unwrap();
    /// 
    /// // Need to create a clone of `Vec<DataElement>` to ensure index was changed
    /// let idx_clone = idx
    ///     .clone()
    ///     .into_iter()
    ///     .map(|v| v.into())
    ///     .collect::<Vec<DataElement>>();
    /// 
    /// assert!(df.set_index(idx.into_iter()).is_ok());  // Set index
    /// assert_eq!(&idx_clone, df.index());              // Ensure is't what we gave
    /// 
    /// ```
    pub fn set_index<'a, I, T>(&mut self, index: I) -> Result<(), String> 
        where 
            I: ExactSizeIterator + Iterator<Item=T>,
            T: Into<DataElement>
    {
        if index.len() != self.len() {
            let err = format!(
                "Length of dataframe is {} but index passed is {}.", 
                self.len(), index.len()
            );
            Err(err)
        } else {
            self.index = index.into_iter().map(|v| v.into()).collect();
            Ok(())
        }
    }

    /// Read a CSV file into a [`DataFrame`] where each column represents a Series
    /// supports automatic decompression of gzipped files if they end with `.gz`
    /// 
    /// ## Example
    /// 
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let path = format!("{}/tests/data/basic_csv.csv.gz", env!("CARGO_MANIFEST_DIR"));
    /// let df = DataFrame::read_csv(&path, b',').unwrap();
    /// 
    /// assert_eq!(df["col1"].sum::<i32>(), 15);
    /// 
    /// ```
    pub fn read_csv<S>(path: S, delimiter: u8) -> Result<Self, Box<Error>> 
        where S: AsRef<OsStr> + ToString
    {

        use std::io::prelude::*;
        use std::fs::File;
        use flate2::read::GzDecoder;


        let p = Path::new(&path);
        let file_reader: Box<Read> = if path.to_string().ends_with(".gz") {

                                            // Return a Gzip reader
                                            Box::new(
                                                GzDecoder::new(File::open(p)?)
                                            )
                                        } else {
                                            
                                            // Return plain file reader
                                            Box::new(
                                                File::open(p)?
                                            )
                                        };

        let mut reader = csv::ReaderBuilder::new()
                                .delimiter(delimiter)
                                .from_reader(file_reader);

        // TODO: Don't fail on non existant headers -> give 'col0', 'col1', etc.
        let headers: Vec<String> = reader.headers()?
                                        .clone()
                                        .into_iter()
                                        .map(|v| v.to_string())
                                        .collect();  

        // Containers for storing column data
        let mut vecs: Vec<Vec<String>> = (0..headers.len())
                                            .map(|_| Vec::new())
                                            .collect();

        for record in reader.records() {

            match record {

                Ok(rec) => { 
                    for (field, container) in rec.iter().zip(&mut vecs) {
                        container.push(field.into());
                    };
                },

                // TODO: Process for dealing with invalid records.
                Err(err) => println!("Unable to read record: '{}'", err)
            }
        }

        let mut df = DataFrame::new();

        // map headers to vectors containing it's fields in parallel and into
        // Series structs, parsing each field.
        let sc: Vec<Series> = headers.into_par_iter()
                                    .zip(vecs)
                                    .map(|(header, vec)| {
                                        let de = vec.into_par_iter()
                                                    .map(|s| DataElement::from_str_parse(s))
                                                    .collect();
                                        let mut series = Series::from_data_elements(de);
                                        series.set_name(&header);
                                        series
                                    })
                                    .collect();
        for series in sc {
            df.add_column(series)?;
        }

        Ok(df)
    }
}


impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        use prettytable::{Table, Row, Cell};

        let mut table = Table::new();

        // Vec of series refs in df.
        let series_refs = self.columns()
            .into_iter()
            .map(|col_name| (col_name, &self[col_name.as_str()]) )
            .collect::<Vec<(&String, &Series)>>();

        // Create header row
        table.add_row(
            Row::new(
                series_refs
                    .iter()
                    .map(|(name, _series)| {
                        Cell::new(&name)
                    })
                    .collect::<Vec<Cell>>()
            )
        );

        // TODO: Impl a better len, ie DataFrame::len()
        // Build rows.
        for i in 0..series_refs[0].1.len() {
            let row = series_refs
                .iter()
                .map(|(_name, series)| {
                    let val: String = series[i].clone().into();
                    Cell::new(&format!("{}", val))
                })
                .collect::<Vec<Cell>>();

            table.add_row(Row::new(row));
        }
        
        // Build header
        write!(f, "{}", table)
    }
}


impl DataFrameBehavior for DataFrame {}

impl ColumnManager for DataFrame {

    fn add_column(&mut self, series: Series) -> Result<(), String> {

        // Set column index if this is the first column being added
        if self.index.is_empty() {
            self.index = (0..series.len())
                .map(|v| DataElement::from(v as i64))
                .collect();
        }

        if self.len() != series.len() {
            let err = format!(
                "DataFrame is of size {}, but passed series is {}",
                self.len(), series.len()
            );
            Err(err)

        } else {
            let n_cols = self.n_columns();
            self.series_objects
                .entry(series.name()
                        .unwrap_or_else(|| format!("COL_{}", n_cols) ))
                .or_insert_with(|| series );
            Ok(())
        }

        
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
