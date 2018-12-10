
use std::iter::Sum;

use num::*;
use prelude::*;


/// [`Series::groupby`]  result.
/// Contains the split series by key
#[derive(Clone)]
pub struct SeriesGroupBy<T: BlackJackData> {
    groups: Vec<Series<T>>
}

impl<T> SeriesGroupBy<T>
    where T: BlackJackData
{

    /// Create a new [`SeriesGroupBy`] from a `Vec<Series>`
    pub fn new(groups: Vec<Series<T>>) -> Self {
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
    /// let grouped: SeriesGroupBy<i32> = series.groupby(keys);
    /// let series = grouped.apply(|s: Series<i32>| s.min().unwrap());
    /// 
    /// assert_eq!(series.max(), Ok(3));  // by key, 3 is the max.
    /// ```
    pub fn apply<F>(self, agg_func: F) -> Series<T>
        where 
            F: Fn(Series<T>) -> T + Sync + Send,
            T: Send
    {
        let results = self.groups
            .into_iter()
            .map(agg_func)
            .collect::<Vec<T>>();
        Series::from_vec(results)
    }

    /// Apply a `sum` aggregation to each [`Series`] group
    pub fn sum<'a>(&'a self) -> Series<T>
        where T: Send + Num + Sum<&'a T> + Copy + Sum,
    {
        let results = self.groups
            .iter()
            .map(|series| series.sum())
            .collect::<Vec<T>>();
        Series::from_vec(results)
    }
}
