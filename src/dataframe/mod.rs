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

    pub fn drop_indexes<N: IntoIterator<Item=i32> + Clone>(&mut self, indexes: N) {
        for meta in self.meta.clone() {
            match meta.dtype {
                DType::I64 => {
                    let series: &mut Series<i64> = self.get_column_mut(meta.name.as_str()).unwrap();
                    series.drop_indexes(indexes.clone());
                },
                DType::I32 => {
                    let series: &mut Series<i32> = self.get_column_mut(meta.name.as_str()).unwrap();
                    series.drop_indexes(indexes.clone());
                },
                DType::F64 => {
                    let series: &mut Series<f64> = self.get_column_mut(meta.name.as_str()).unwrap();
                    series.drop_indexes(indexes.clone());
                },
                DType::F32 => {
                    let series: &mut Series<f64> = self.get_column_mut(meta.name.as_str()).unwrap();
                    series.drop_indexes(indexes.clone());
                },
                DType::STRING => {
                    let series: &mut Series<String> = self.get_column_mut(meta.name.as_str()).unwrap();
                    series.drop_indexes(indexes.clone());
                },
            };
        };

        self.index.drop_indexes(indexes);
    }

    /// Retrieves a mutable reference to the column
    pub fn get_column_mut<'a, T>(&mut self, name: impl Into<&'a str>) -> Option<&mut Series<T>>
        where T: BlackJackData + 'static
    {
        let name = name.into();
        for meta in &self.meta {
            if meta.name == name {
                let series: Option<&mut Series<T>> = self.data.get_mut(&meta.name);
                return series
            }
        }
        None
    }

    /// Retrieves a reference to a column
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
