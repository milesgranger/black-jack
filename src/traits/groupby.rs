
use std::iter::Sum;

use num::*;
use prelude::*;


/// [`Series::groupby`]  result.
/// Contains the split series by key
pub struct SeriesGroupBy {
    groups: Vec<Series>
}

impl SeriesGroupBy {

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
pub trait GroupByBehavior {

    fn groupby(&self, keys: Series) -> SeriesGroupBy {
        let groups = self.split(keys);
        SeriesGroupBy { groups }
    }
    fn split(&self, keys: Series) -> Vec<Series>;
    fn apply<F, T>(&self, agg_func: F) -> T
        where 
            F: Fn(&Series) -> T,
            T: BlackJackData;

    fn combine<T: BlackJackData>(vec: Vec<T>) -> Series {
        Series::from_vec(vec)
    }
}
