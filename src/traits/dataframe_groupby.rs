
use std::iter::Sum;
use num::*;

use prelude::*;

/// [`DataFrame::groupby`]  result.
/// Contains the split series by key
pub struct DataFrameGroupBy {
    groups: Vec<SeriesGroupBy>
}

impl DataFrameGroupBy {

    /// Construct a new [`DataFrameGroupBy`] from a collection of [`SeiresGroupBy`]
    /// structs; shouldn't be needed to be used directly.
    pub fn new(groups: Vec<SeriesGroupBy>) -> Self {
        DataFrameGroupBy{ groups }
    }

    /// Sum this grouped dataframe object. 
    /// basically calls `sum` in parallel on each grouped series collected.
    pub fn sum<T>(&self) -> DataFrame 
        where T: BlackJackData + Copy + Sum + From<DataElement> + Num + Send
    {
        // TODO: Pandas casts columns to matching dtypes, and so do we.
        // but should we? Could keep the current (if known) dtype of each column?

        let mut df = DataFrame::new();

        let _ = self.groups
            .iter()
            .map(|series_groupby| series_groupby.sum::<T>())
            .map(|series| {println!("{}", &series); df.add_column(series)})
            .collect::<Vec<()>>();

        df
    }
}


/// The intended behavior of a grouped DataFrame.
pub trait DataFrameGroupByBehavior {

    /// Group by method for grouping [`Series`] in a [`DataFrame`]
    /// by key.
    // TODO: Change keys to an iterable producing items of DataElement & same length
    fn groupby(&self, keys: Series) -> DataFrameGroupBy;
}

impl DataFrameGroupByBehavior for DataFrame {

    fn groupby(&self, keys: Series) -> DataFrameGroupBy {

        // TODO: More efficient impl without so much cloning...
        let groups = self.columns()
            .into_iter()
            .map(|col_name| {
                let series = &self[col_name.as_str()].clone();
                series.groupby(keys.clone())
            })
            .collect::<Vec<SeriesGroupBy>>();

        DataFrameGroupBy::new(groups)
    }
}