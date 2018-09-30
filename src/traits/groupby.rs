
use std::iter::Sum;

use num::*;
use prelude::*;


/// [`Series::groupby`]  result.
/// Contains the split series by key
pub struct SeriesGroupBy {
    groups: Vec<Series>
}

impl SeriesGroupBy {

    pub fn new(groups: Vec<Series>) -> Self {
        SeriesGroupBy { groups }
    }

    pub fn apply<F, T>(self, agg_func: F) -> Series
        where 
            F: Fn(Series) -> T,
            T: BlackJackData
    {
        let results = self.groups
            .into_iter()
            .map(agg_func)
            .collect();
        Series::from_vec(results)
    }

    /// Apply a `sum` aggregation to each [`Series`] group
    pub fn sum<T>(&self) -> Series 
        where T: Num + From<DataElement> + Sum + Copy + BlackJackData
    {
        let mut results = vec![];
        for group in &self.groups {
            let result = group.sum::<T>();
            results.push(result);
        }
        Series::from_vec(results)
    }
}


/// Trait defining the concept of split -> apply -> combine
pub trait SeriesGroupByBehavior {
    fn groupby(&self, keys: Series) -> SeriesGroupBy;
}
