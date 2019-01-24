//! DataFrame object and associated functionality
//!
//!

use num::*;
use serde::{Deserialize};
use baggie::Baggie;

use crate::prelude::*;

pub mod io;
pub mod dataframe_groupby;
pub use self::io::*;
pub use self::dataframe_groupby::*;

/// The container for `Series<T>` objects, allowing for additional functionality
#[derive(Default, Debug)]
pub struct DataFrame {
    index: Option<String>,
    meta: Vec<SeriesMeta>,
    data: Baggie<String>
}

impl DataFrame {

    /// Create a new `DataFrame` struct
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut df: DataFrame = DataFrame::new();
    /// ```
    pub fn new() -> Self {
        DataFrame {
            index: None,
            meta: vec![],
            data: Baggie::new(),
        }
    }

    /// Drop rows by position within the dataframe
    pub fn drop_positions<I: IntoIterator<Item=usize> + Clone>(&mut self, positions: I) -> () {
        let new_lens = self.meta
            .clone()
            .iter()
            .map(|meta| {
                match meta.dtype {
                    DType::I64 => {
                        let series: &mut Series<i64> = self.data.get_mut(&meta.name).unwrap();
                        series.drop_positions(positions.clone());
                        series.len()
                    },
                    DType::F64 => {
                        let series: &mut Series<f64> = self.data.get_mut(&meta.name).unwrap();
                        series.drop_positions(positions.clone());
                        series.len()
                    },
                    DType::I32 => {
                        let series: &mut Series<i32> = self.data.get_mut(&meta.name).unwrap();
                        series.drop_positions(positions.clone());
                        series.len()
                    },
                    DType::F32 => {
                        let series: &mut Series<f32> = self.data.get_mut(&meta.name).unwrap();
                        series.drop_positions(positions.clone());
                        series.len()
                    },
                    DType::STRING => {
                        let series: &mut Series<String> = self.data.get_mut(&meta.name).unwrap();
                        series.drop_positions(positions.clone());
                        series.len()
                    }
                }
            })
            .collect::<Vec<usize>>();

        // Update the new lens in metadata
        self.meta
            .iter_mut()
            .zip(new_lens)
            .map(|(ref mut meta, len)| meta.len = len)
            .last();
    }

    /// Join a series into the dataframe
    pub fn join_series<T>(&mut self, series: Series<T>, how: Join, on: Option<&str>) -> Result<(), BlackJackError>
        where T: BlackJackData + 'static
    {

        // Prep the series
        let mut series = series;
        if let None = series.name() {
            series.set_name(&format!("col_{}", self.n_columns()))
        }


        match on {
            Some(col) => {

                // Determine if the name to match on exists
                match self.meta.iter().filter(|meta| meta.name == col).next() {

                    // If so we can check they are the same type.
                    Some(meta) => {
                        if meta.dtype != series.dtype().ok_or(BlackJackError::from("Series does not have DType defined"))? {
                            Err(BlackJackError::from("Mismatch in series to join and index dtypes!"))
                        } else {

                            // We are cleared to do a join now.
                            Ok(())
                        }
                    },

                    // Otherwise we can't join on non-existent column.
                    None => {
                        Err(BlackJackError::from(format!("No column named: {}", col).as_str()))
                    }
                }

            },

            None => {

                // If dataframe is empty, no column to join on is ok, we can add it directly
                if self.is_empty() {
                    self.add_column(series)?;
                    Ok(())
                // Otherwise we must have a column name to join on if using `join_series`
                } else {
                    Err(BlackJackError::from("Dataframe has existing columns, must provide one to join on. Did you mean to use `add_column`?"))
                }
            }
        }
    }

    /// Length of the dataframe
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut df = DataFrame::new();
    /// assert_eq!(df.len(), 0);
    ///
    /// let series: Series<i32> = Series::arange(0, 10);
    /// df.add_column(series).unwrap();
    ///
    /// assert_eq!(df.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        if self.n_columns() > 0 {
            self.meta[0].len
        } else {
            0
        }
    }

    /// Quickly identify if the dataframe is empty.
    pub fn is_empty(&self) -> bool {
        if self.len() > 0 { false } else { true }
    }

    /// Add a column to this dataframe.
    pub fn add_column<T: BlackJackData + 'static>(&mut self, series: Series<T>) -> Result<(), BlackJackError>
        where Vec<T>: std::iter::FromIterator<T>
    {
        let mut series = series;

        // Ensure length is a match if we have columns
        if self.len() > 0 && self.len() != series.len() {
            return Err(
                BlackJackError::LengthMismatch(
                    format!("DataFrame has length: {}, cannot add series of length: {}", self.len(), series.len())))
        }
        if let None = series.name() {
            series.set_name(&format!("col_{}", self.n_columns()))
        }

        let meta = SeriesMeta::from(&series);
        self.data.insert(meta.name.clone(), series);
        self.meta.push(meta);

        Ok(())
    }

    /// Retrieves a column from the dataframe as an owned representation of it.
    pub fn get_column<'a, T>(&self, name: impl Into<&'a str>) -> Option<&Series<T>>
        where T: BlackJackData + 'static
    {
        let name = name.into();
        for meta in &self.meta {
            if meta.name == name {
                let series: Option<&Series<T>> = self.data.get(&meta.name);
                return series
            }
        }
        None
    }

    /// Get column, infer
    pub fn get_column_infer<'a>(&self, name: impl Into<&'a str>) -> Option<GenericSeriesContainer> {
        let name = name.into();
        if self.data.contains_key(name) {
            let meta: &SeriesMeta = self.meta.iter().filter(|m| m.name == name).last()?;
            let container = match meta.dtype {
                DType::I64 => GenericSeriesContainer::I64(self.data.get::<Series<i64>, _>(name)?.clone()),
                DType::F64 => GenericSeriesContainer::F64(self.data.get::<Series<f64>, _>(name)?.clone()),
                DType::I32 => GenericSeriesContainer::I32(self.data.get::<Series<i32>, _>(name)?.clone()),
                DType::F32 => GenericSeriesContainer::F32(self.data.get::<Series<f32>, _>(name)?.clone()),
                DType::STRING => GenericSeriesContainer::STRING(self.data.get::<Series<String>, _>(name).unwrap().clone())
            };
            Some(container)
        } else {
            None
        }
    }

    /// Get a list of column names in this dataframe as an iterator
    pub fn columns(&self) -> impl Iterator<Item=&str> {
        self.data
            .keys()
            .map(|c| c.as_str())
    }

    /// Get the number of columns for this dataframe
    pub fn n_columns(&self) -> usize {
        self.data.len()
    }

    /// Group by method for grouping [`Series`] in a [`DataFrame`]
    /// by key.
    pub fn groupby<T>(&self, keys: &Series<T>) -> DataFrameGroupBy<T>
        where for<'de>
              T: BlackJackData + Deserialize<'de> + ToPrimitive + 'static
    {

        let groups = self
            .columns()
            .map(|col_name| {
                let series = self.get_column(col_name).unwrap();
                series.groupby(keys)
            })
            .collect::<Vec<SeriesGroupBy<T>>>();

        DataFrameGroupBy::new(groups)
    }
}
