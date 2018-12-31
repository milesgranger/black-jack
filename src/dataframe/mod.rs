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
pub struct DataFrame<I>
    where I: PartialOrd + PartialEq + BlackJackData
{
    index: Series<I>,
    meta: Vec<SeriesMeta>,
    data: Baggie<String>
}

impl<I: PartialOrd + PartialEq + BlackJackData> DataFrame<I> {

    /// Create a new `DataFrame` struct
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut df: DataFrame<i32> = DataFrame::new();  // `i32` indicates index type of DataFrame
    /// ```
    pub fn new() -> Self {
        DataFrame {
            index: Series::default(),
            data: Baggie::new(),
            meta: vec![]
        }
    }


    /// Join a series into the dataframe
    pub fn join_series<T>(&mut self, series: Series<T>, how: Join) -> Result<(), BlackJackError>
        where T: BlackJackData + 'static,
              I: ToPrimitive,
            Vec<I>: std::iter::FromIterator<i32>
    {
        let mut series = series;
        let other_index: &Vec<i32> = series.index().into();

        // TODO: Implement other Joins besides 'Inner'
        // TODO: Remove clones

        match how {
            Join::Inner => {

                // Find unshared indexes
                let unshared = self.index.values
                    .iter()
                    .map(|idx| idx.to_i32().unwrap())
                    .filter(|idx| other_index.contains(&idx))
                    .collect::<Vec<i32>>();

                // Drop these indexes from the series
                series.drop_indexes(unshared.clone());

                // Drop these indexes from the dataframe, also removing from every other owned series
                self.index.drop_indexes(unshared.clone());

                // TODO: DRY
                for meta in &self.meta {
                    match meta.dtype {
                        DType::I64 => {
                            let column: &mut Series<i64> = self.data.get_mut(&meta.name).unwrap();
                            column.drop_indexes(unshared.clone());
                        },
                        DType::F64 => {
                            let column: &mut Series<f64> = self.data.get_mut(&meta.name).unwrap();
                            column.drop_indexes(unshared.clone());
                        },
                        DType::I32 => {
                            let column: &mut Series<i32> = self.data.get_mut(&meta.name).unwrap();
                            column.drop_indexes(unshared.clone());
                        },
                        DType::F32 => {
                            let column: &mut Series<f32> = self.data.get_mut(&meta.name).unwrap();
                            column.drop_indexes(unshared.clone());
                        },
                        DType::STRING => {
                            let column: &mut Series<String> = self.data.get_mut(&meta.name).unwrap();
                            column.drop_indexes(unshared.clone());
                        },
                    }
                }

                // TODO: Sort the series by the current index of the dataframe

                // Add column
                self.add_column(series)?;
            },
            _ => unimplemented!()
        }

        Ok(())
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
        self.index.len()
    }

    /// Quickly identify if the dataframe is empty.
    pub fn is_empty(&self) -> bool {
        if self.len() > 0 { false } else { true }
    }

    /// Add a column to this dataframe.
    pub fn add_column<T: BlackJackData + 'static>(&mut self, series: Series<T>) -> Result<(), BlackJackError>
        where Vec<I>: std::iter::FromIterator<i32>
    {
        let mut series = series;

        // Ensure length is a match if we have columns
        if self.len() > 0 && self.len() != series.len() {
            return Err(
                BlackJackError::LengthMismatch(
                    format!("DataFrame has length: {}, cannot add series of length: {}", self.len(), series.len())))
        } else {
            self.index = Series::from_vec((0..series.len() as i32).collect::<Vec<I>>())
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
