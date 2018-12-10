use std::iter::Sum;
use num::*;

use prelude::*;

/// [`DataFrame::groupby`]  result.
/// Contains the split series by key
pub struct DataFrameGroupBy<T>
    where T: BlackJackData
{
    groups: Vec<SeriesGroupBy<T>>
}

impl<T> DataFrameGroupBy<T>
    where T: BlackJackData
{

    /// Construct a new [`DataFrameGroupBy`] from a collection of [`SeiresGroupBy`]
    /// structs; shouldn't be needed to be used directly.
    pub fn new(groups: Vec<SeriesGroupBy<T>>) -> Self {
        DataFrameGroupBy{ groups }
    }

    /// Sum this grouped dataframe object.
    /// basically calls `sum` in parallel on each grouped series collected.
    pub fn sum<'a>(&'a self) -> DataFrame
        where T: BlackJackData + Copy + Sum<&'a T> + Sum + Num + Send
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
    fn groupby<T>(&self, keys: Series<T>) -> DataFrameGroupBy<T>
        where T: BlackJackData + ToPrimitive;
}

impl DataFrameGroupByBehavior for DataFrame
{
    fn groupby<T>(&self, keys: Series<T>) -> DataFrameGroupBy<T>
        where T: BlackJackData + ToPrimitive
    {

        let groups = self
            .columns()
            .map(|col_name| {
                let series = self.get_column(col_name).unwrap();
                series.groupby(keys.clone())
            })
            .collect::<Vec<SeriesGroupBy<T>>>();

        DataFrameGroupBy::new(groups)
    }
}