
use std::iter::Sum;

use num::*;
use rayon::prelude::*;
use prelude::*;


/// [`Series::groupby`]  result.
/// Contains the split series by key
pub struct SeriesGroupBy {
    groups: Vec<Series>
}

impl SeriesGroupBy {

    /// Create a new [`SeriesGroupBy`] from a `Vec<Series>`
    pub fn new(groups: Vec<Series>) -> Self {
        SeriesGroupBy { groups }
    }

    /// Apply an **aggregation** function to each [`Series`] 
    /// in [`SeriesGroupBy`] yielding a grouped [`Series`]
    /// 
    /// The passed function should return type `T` when given
    /// a [`Series`] where [`BlackJackData`] has been 
    /// implemented for `T`
    /// 
    /// ## Example
    /// 
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series = Series::from_vec(vec![9, 9, 3, 1, 1, 9]);
    /// let keys   = Series::from_vec(vec![4, 5, 6, 4, 5, 6]);
    /// 
    /// let grouped: SeriesGroupBy = series.groupby(keys);
    /// let series = grouped.apply(|s: Series| s.min::<i32>().unwrap());
    /// 
    /// assert_eq!(series.max::<i32>(), Ok(3));  // by key, 3 is the max.
    /// ```
    pub fn apply<F, T>(self, agg_func: F) -> Series
        where 
            F: Fn(Series) -> T + Sync + Send,
            T: BlackJackData + Send
    {
        let results = self.groups
            .into_par_iter()
            .map(agg_func)
            .collect();
        Series::from_vec(results)
    }

    /// Apply a `sum` aggregation to each [`Series`] group
    pub fn sum<T>(&self) -> Series 
        where T: Send + Num + From<DataElement> + Sum + Copy + BlackJackData
    {
        let results = self.groups
            .par_iter()
            .map(|s| s.sum::<T>())
            .collect();
        Series::from_vec(results)
    }
}


/// Trait defining the concept of split -> apply -> combine
pub trait SeriesGroupByBehavior {

    /// Group by method for grouping elements in a [`Series`]
    /// by key.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series = Series::from_vec(vec![1, 2, 3, 1, 2, 3]);
    /// let keys   = Series::from_vec(vec![4, 5, 6, 4, 5, 6]);
    /// 
    /// let grouped: Series = series.groupby(keys).sum::<i32>();
    /// assert_eq!(grouped.len(), 3);
    /// 
    /// let mut vals = grouped.into_vec::<i32>();
    /// vals.sort();
    /// assert_eq!(vals, vec![2, 4, 6]);
    /// ```
    fn groupby(&self, keys: Series) -> SeriesGroupBy;
}
