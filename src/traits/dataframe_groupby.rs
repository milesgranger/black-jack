use std::iter::Sum;
use num::*;
use serde::{Deserialize};

use crate::prelude::*;

/// [`DataFrame::groupby`]  result.
/// Contains the split series by key
pub struct DataFrameGroupBy<T>
    where T: BlackJackData
{
    groups: Vec<SeriesGroupBy<T>>
}

impl<T> DataFrameGroupBy<T>
    where T: BlackJackData + 'static
{

    /// Construct a new [`DataFrameGroupBy`] from a collection of [`SeiresGroupBy`]
    /// structs; shouldn't be needed to be used directly.
    pub fn new(groups: Vec<SeriesGroupBy<T>>) -> Self {
        DataFrameGroupBy{ groups }
    }

    /// Sum this grouped dataframe object.
    /// basically calls `sum` in parallel on each grouped series collected.
    pub fn sum(&self) -> DataFrame<i32>  // TODO:
        where T: BlackJackData + Copy + Sum + Num + Send + Ord
    {
        // TODO: Return result

        let mut df = DataFrame::new();

        let _ = self.groups
            .iter()
            .map(|series_groupby| series_groupby.sum())
            .map(|series| df.add_column(series).unwrap())
            .collect::<Vec<()>>();
        df
    }
}


/// The intended behavior of a grouped DataFrame.
pub trait DataFrameGroupByBehavior
{

    /// Group by method for grouping [`Series`] in a [`DataFrame`]
    /// by key.
    fn groupby<T>(&self, keys: &Series<T>) -> DataFrameGroupBy<T>
        where for<'de> T: BlackJackData + Deserialize<'de> + ToPrimitive + 'static;
}

impl<I> DataFrameGroupByBehavior for DataFrame<I>
    where I: BlackJackData + PartialOrd + PartialEq
{
    fn groupby<T>(&self, keys: &Series<T>) -> DataFrameGroupBy<T>
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