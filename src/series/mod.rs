//! Series represents a single column within a `DataFrame`
//!
//! ## Example use:
//!
//! ```
//! use blackjack::prelude::*;
//!
//! let mut series = Series::arange(0, 5);
//!
//! // Index and change elements
//! series[0] = 1;
//! series[1] = 0;
//!
//! assert_eq!(series[0], 1);
//! assert_eq!(series.sum(), 10);
//! assert_eq!(series.len(), 5);
//! ```

use std::convert::From;
use std::fmt;
use std::iter::{FromIterator, Sum};
use std::marker::{Send, Sync};
use std::ops::{Index, IndexMut, Range};
use std::str::FromStr;
use std::vec::IntoIter;

use itertools::Itertools;

use num::*;
use rayon::prelude::*;
use stats;

pub mod overloaders;
pub mod rolling;
pub mod series_groupby;
pub mod variants;

pub use self::rolling::*;
pub use self::series_groupby::*;
pub use self::variants::*;

use crate::funcs;
use crate::prelude::*;

// Allow series.into_iter()
impl_series_into_iter!(String);
impl_series_into_iter!(f64);
impl_series_into_iter!(i64);
impl_series_into_iter!(f32);
impl_series_into_iter!(i32);

/// Series struct for containing underlying Array and other meta data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, PartialOrd)]
pub struct Series<T>
where
    T: BlackJackData,
{
    /// Name of the series, if added to a dataframe without a name, it will be assigned
    /// a default name equalling the count of columns in the dataframe.
    pub name: Option<String>,

    /// The underlying values of the Series
    pub values: Vec<T>,

    dtype: Option<DType>,
}

impl<I> Default for Series<I>
where
    I: PartialOrd + PartialEq + BlackJackData,
{
    fn default() -> Self {
        Series::from_vec(vec![])
    }
}

/// Constructor methods for `Series<T>`
impl<T> Series<T>
where
    T: BlackJackData,
{
    /// Create a new Series struct from an integer range with one step increments.
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series: Series<i32> = Series::arange(0, 10);
    /// ```
    pub fn arange(start: T, stop: T) -> Self
    where
        T: Integer + BlackJackData + ToPrimitive,
        Range<T>: Iterator,
        Vec<T>: FromIterator<<Range<T> as Iterator>::Item>,
    {
        let dtype = Some(start.dtype());
        let values: Vec<T> = (start..stop).collect();
        Series {
            name: None,
            dtype,
            values,
        }
    }

    /// Drop positions of the Series
    pub fn drop_positions<I>(&mut self, positions: I) -> ()
    where
        I: IntoIterator<Item = usize>,
    {
        // TODO: refactor to avoid duplicating data

        let positions = positions.into_iter().collect::<Vec<usize>>();

        // Create a collection of the new values and indexes
        self.values = self
            .values
            .iter()
            .enumerate()
            .filter_map(|(position, val)| {
                if positions.contains(&position) {
                    None
                } else {
                    Some(val.clone())
                }
            })
            .collect::<Vec<T>>();
    }

    /// Fetch values from the series by matching index _positions_, _not_ by index value.
    ///
    /// _No data copies are made_, and currently this is _not_ done in parallel. As by currently
    /// single threaded exceeds parallel execution up to ~10m elements. As the _majority_ of use cases
    /// have less than this amount, we've opted for single threading. If you need concurrent execution,
    /// please file an issue at our github. :-)
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut series = Series::arange(0, 10000);  // Index values end up being 0-10000 by default here
    ///
    /// let vals = series.iloc(&vec![250, 500, 1000, 2000, 4000, 5000]);  // ~300ns, ~28x faster than Pandas
    /// assert_eq!(vals, vec![&250, &500, &1000, &2000, &4000, &5000]);
    /// ```
    pub fn iloc<'b, I>(&self, idx_vals: I) -> Vec<&T>
    where
        I: IntoIterator<Item = &'b usize>,
    {
        idx_vals
            .into_iter()
            .map(|idx_val| &self.values[*idx_val])
            .collect::<Vec<&T>>()
    }

    /// Calculate a predefined rolling aggregation
    ///
    /// See [`Rolling`] for additional functionality.
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// use float_cmp::ApproxEq;
    ///
    /// let series = Series::from_vec(vec![0, 1, 2, 3, 4, 5]);
    ///
    /// let rolled: Series<f64> = series.rolling(4).mean().unwrap();
    /// assert_eq!(rolled.len(), 6);
    ///
    /// // vals in indexes 0 thru 2 should be NaN as they are inside the window
    /// assert_eq!(rolled[0..2].iter().all(|v| v.is_nan()), true);
    ///
    /// assert_eq!(rolled[3], 1.5);
    /// assert_eq!(rolled[4], 2.5);
    /// assert_eq!(rolled[5], 3.5);
    /// ```
    pub fn rolling(&self, window: usize) -> Rolling<T>
    where
        T: Send + Sync,
    {
        Rolling::new(window, &self)
    }

    /// Return an iterable of booleans determining if any element is NaN
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut series = Series::from_vec(vec![0, 1, 2])
    ///     .astype::<f32>()
    ///     .unwrap();
    ///
    /// // No NaNs currently
    /// assert_eq!(series.isna().collect::<Vec<bool>>(), vec![false, false, false]);
    ///
    /// // Insert a NaN at index zero
    /// series[0] = num::Float::nan();
    /// assert_eq!(series.isna().collect::<Vec<bool>>(), vec![true, false, false]);
    /// ```
    pub fn isna<'a>(&'a self) -> impl Iterator<Item = bool> + 'a
    where
        T: Float,
    {
        self.values.iter().map(|v| v.is_nan())
    }

    /// Determine if _all_ elements in the Series meet a given condition
    ///
    /// This will stop iteration after encountering the first element which breaks
    /// the condition.
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series = Series::from_vec(vec![1, 2, 3, 4]);
    ///
    /// assert_eq!(series.all(|x| *x > 0), true);
    /// assert_eq!(series.all(|x| *x > 2), false);
    /// ```
    pub fn all<F>(&self, condition: F) -> bool
    where
        for<'r> F: Fn(&'r T) -> bool,
    {
        self.values.iter().all(condition)
    }

    /// Check if all elements with the Series are equal
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series = Series::from_vec(vec![1, 1, 1, 1, 1]);
    /// assert!(series.all_equal());
    /// ```
    pub fn all_equal(&self) -> bool
    where
        T: PartialEq,
    {
        self.values.iter().all_equal()
    }

    /// Determine if _any_ element in the Series meets a given condition
    ///
    /// This will stop iteration after encountering the first element which meets
    /// conditions supplied.
    pub fn any<F>(&self, condition: F) -> bool
    where
        for<'r> F: FnMut(&'r &T) -> bool,
    {
        let first_match = self.values.iter().find(condition);
        match first_match {
            Some(_) => true,
            None => false,
        }
    }

    /// Create a cartesian product of this series and another, returns a pair of
    /// `Series` representing the cartesian product
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series1 = Series::from_vec(vec![0, 1]);
    /// let series2 = Series::from_vec(vec![1, 2]);
    ///
    /// let (cart_prod1, cart_prod2) = series1.cartesian_product(&series2);
    ///
    /// assert_eq!(cart_prod1.values, vec![0, 0, 1, 1]);
    /// assert_eq!(cart_prod2.values, vec![1, 2, 1, 2]);
    /// ```
    pub fn cartesian_product<O>(&self, other: &Series<O>) -> (Series<T>, Series<O>)
    where
        O: BlackJackData,
    {
        let mut left = vec![];
        let mut right = vec![];
        let _ = self
            .values
            .clone()
            .into_iter()
            .cartesian_product(other.values.clone().into_iter())
            .map(|(l, r)| {
                left.push(l);
                right.push(r);
            })
            .collect::<Vec<()>>();
        (Series::from_vec(left), Series::from_vec(right))
    }

    /// Return the positions of where a given condition evaluates to `true`
    ///
    /// This is somewhat akin to the pandas `where` method.
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series = Series::from_vec(vec![1, 2, 1, 2]);
    ///
    /// let indexes_of_ones = series.positions(|x| *x == 1).collect::<Vec<usize>>();
    /// assert_eq!(indexes_of_ones, vec![0, 2]);
    /// ```
    pub fn positions<'a, F>(&'a self, condition: F) -> impl Iterator<Item = usize> + 'a
    where
        F: 'a + Fn(&T) -> bool,
    {
        self.values.iter().positions(condition)
    }

    /// Map a function over a series _in parallel_
    /// Function takes some type `T` and returns some type `B` which
    /// has `BlackJackData` implemented.
    ///
    /// ## Example
    ///
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series = Series::from_vec(vec![1, 1, 1, 1]);
    ///
    /// let new_series = series.map_par(|x| x * 2);
    /// assert_eq!(new_series.sum(), 8);
    /// ```
    pub fn map_par<B, F>(self, func: F) -> Series<B>
    where
        B: BlackJackData,
        F: Fn(T) -> B + Send + Sync,
    {
        let new_data = self.values.into_par_iter().map(func).collect();
        Series::from_vec(new_data)
    }

    /// Map a function over a series in a single thread
    /// Function takes some type `T` and returns some type `B` which
    /// has `BlackJackData` implemented.
    pub fn map<B, F>(self, func: F) -> Series<B>
    where
        B: BlackJackData,
        F: Fn(T) -> B,
    {
        let new_data = self.values.into_iter().map(func).collect();
        Series::from_vec(new_data)
    }

    /// Convert the series into another [`DType`] (creates a new series)
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series: Series<i32> = Series::arange(0, 10);
    /// assert_eq!(series[0].dtype(), DType::I32);
    /// let new_series = series.astype::<f64>().unwrap();
    /// assert_eq!(new_series[0].dtype(), DType::F64);
    /// ```
    pub fn astype<A>(&self) -> Result<Series<A>, &'static str>
    where
        A: BlackJackData + FromStr,
    {
        let values = self
            .values
            .clone()
            .into_iter()
            .map(|v| v.to_string())
            .map(|v| v.parse::<A>().map_err(|_| "Cannot cast into type"))
            .collect::<Result<Vec<A>, _>>()?;
        let series = Series {
            name: self.name.clone(),
            dtype: Some(values[0].dtype()),
            values,
        };
        Ok(series)
    }

    /// Convert this series into another [`DType`] (consumes current series)
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series: Series<i32> = Series::arange(0, 10);
    /// assert_eq!(series[0].dtype(), DType::I32);
    /// let new_series = series.into_type::<f64>().unwrap();
    /// assert_eq!(new_series[0].dtype(), DType::F64);
    /// ```
    pub fn into_type<A>(self) -> Result<Series<A>, &'static str>
    where
        A: BlackJackData + FromStr,
    {
        let values = self
            .values
            .into_iter()
            .map(|v| v.to_string())
            .map(|v| v.parse::<A>().map_err(|_| "Cannot cast into type"))
            .collect::<Result<Vec<A>, _>>()?;
        let series = Series {
            name: self.name.clone(),
            dtype: Some(values[0].dtype()),
            values,
        };
        Ok(series)
    }

    /// Get a series of the unique elements held in this series
    ///
    /// ## Example
    ///
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series: Series<i32> = Series::from_vec(vec![1, 2, 1, 0, 1, 0, 1, 1]);
    /// let unique: Series<i32> = series.unique();
    /// assert_eq!(unique, Series::from_vec(vec![0, 1, 2]));
    /// ```
    pub fn unique(&self) -> Series<T>
    where
        T: PartialOrd + Copy,
    {
        // Cannot use `HashSet` as f32 & f64 don't implement Hash
        let mut unique: Vec<T> = vec![];
        let mut values = self.values.clone();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for val in values {
            if unique.len() > 0 {
                if val == unique[unique.len() - 1] {
                    continue;
                } else {
                    unique.push(val)
                }
            } else {
                unique.push(val)
            }
        }

        Series::from_vec(unique)
    }

    /// Create a new Series struct from a vector, where T is supported by [`BlackJackData`].
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series: Series<i32> = Series::from_vec(vec![1, 2, 3]);
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Self {
        let dtype = if vec.len() == 0 {
            None
        } else {
            Some(vec[0].dtype())
        };
        Series {
            name: None,
            dtype,
            values: vec,
        }
    }

    /// Convert the series to a [`Vec`]
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series = Series::from_vec(vec![1_f64, 2_f64, 3_f64]);
    ///
    /// assert_eq!(
    ///     series.clone().into_vec(),
    ///     vec![1_f64, 2_f64, 3_f64]
    /// );
    /// ```
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }

    /// Set the name of a series
    pub fn set_name(&mut self, name: &str) -> () {
        self.name = Some(name.to_string());
    }

    /// Get the name of the series; Series may not be assigned a string,
    /// so an `Option` is returned.
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut series = Series::from_vec(vec![1, 2, 3]);
    /// series.set_name("my-series");
    ///
    /// assert_eq!(series.name(), Some("my-series".to_string()));
    /// ```
    pub fn name(&self) -> Option<String> {
        match self.name {
            Some(ref name) => Some(name.clone()),
            None => None,
        }
    }

    /// Finds the returns a [`Series`] containing the mode(s) of the current
    /// [`Series`]
    pub fn mode(&self) -> Result<Self, BlackJackError>
    where
        T: BlackJackData + PartialOrd + Copy + ToPrimitive,
    {
        if self.len() == 0 {
            return Err(BlackJackError::from(
                "Cannot compute mode of an empty series!",
            ));
        }

        let modes = stats::modes(self.values.iter().map(|v| *v));
        let modes = Series::from_vec(modes);
        Ok(modes)
    }

    /// Calculate the variance of the series, using either population or sample variance
    /// > Population: `ddof` == 0_f64
    /// > Sample: `ddof` == 1_f64
    pub fn var(&self, ddof: f64) -> Result<f64, BlackJackError>
    where
        T: ToPrimitive + Num,
    {
        if self.len() == 0 {
            return Err(BlackJackError::ValueError(
                "Cannot compute variance of an empty series!".to_owned(),
            ));
        }
        funcs::variance(self.values.as_slice(), ddof)
            .ok_or_else(|| BlackJackError::from("Failed to calculate variance of series."))
    }

    /// Calculate the standard deviation of the series
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// use float_cmp::ApproxEq;
    ///
    /// let series = Series::arange(0, 10).astype::<f32>().unwrap();
    ///
    /// let std = series.std(1_f64).unwrap(); // using population ddof (sample variance == 0_f64)
    /// assert_eq!(std, 3.0276503540974917);
    /// ```
    pub fn std(&self, ddof: f64) -> Result<f64, BlackJackError>
    where
        T: BlackJackData + ToPrimitive + Copy + Num,
    {
        if self.len() == 0 {
            return Err(BlackJackError::ValueError(
                "Cannot compute standard deviation of an empty series!".to_owned(),
            ));
        }
        funcs::std(self.values.as_slice(), ddof)
            .ok_or_else(|| BlackJackError::from("Failed to calculate stddev of series."))
    }

    /// Sum a given series, yielding the same type as the elements stored in the
    /// series.
    pub fn sum(&self) -> T
    where
        T: Num + Copy + Sum,
    {
        funcs::sum(self.values.as_slice())
    }

    /// Average / Mean of a given series - Requires specifying desired float
    /// return annotation
    ///
    /// ## Example:
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series = Series::arange(0, 5);
    /// let mean = series.mean();
    ///
    /// match mean {
    ///     Ok(result) => {
    ///         println!("Result is: {}", &result);
    ///         assert_eq!(result, 2.0);
    ///     },
    ///     Err(err) => {
    ///         panic!("Was unable to compute mean, error: {}", err);
    ///     }
    /// }
    /// ```
    pub fn mean(&self) -> Result<f64, BlackJackError>
    where
        T: ToPrimitive + Copy + Num + Sum,
    {
        funcs::mean(self.values.as_slice())
            .ok_or_else(|| BlackJackError::from("Failed to calculate mean!"))
    }

    /// Calculate the quantile of the series
    ///
    /// ## Example:
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series = Series::arange(0, 100).astype::<f32>().unwrap();
    /// let qtl = series.quantile(0.5).unwrap(); // `49.5_f32`
    ///
    /// assert!(qtl < 49.51);
    /// assert!(qtl > 49.49);
    /// ```
    pub fn quantile(&self, quantile: f64) -> Result<f64, BlackJackError>
    where
        T: ToPrimitive + BlackJackData,
    {
        use rgsl::statistics::quantile_from_sorted_data;
        use std::cmp::Ordering;

        let mut vec = self
            .clone()
            .into_vec()
            .into_iter()
            .map(|v| v.to_f64().unwrap())
            .collect::<Vec<f64>>();

        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let qtl = quantile_from_sorted_data(&vec[..], 1, vec.len(), quantile);
        Ok(qtl)
    }

    /// Calculate the median of a series
    pub fn median(&self) -> Result<f64, BlackJackError>
    where
        T: ToPrimitive + Copy + PartialOrd,
    {
        if self.len() == 0 {
            return Err(BlackJackError::from(
                "Cannot calculate median of an empty series.",
            ));
        }
        stats::median(self.values.iter().map(|v| v.to_f64().unwrap())).ok_or_else(|| {
            BlackJackError::from(
                r#"Unable to calculate median, please create an issue!
                           as this wasn't expected to ever happen on a non-empty
                           series. :("#,
            )
        })
    }

    /// Find the minimum of the series. If several elements are equally minimum,
    /// the first element is returned. If it's empty, an Error will be returned.
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let series: Series<i32> = Series::arange(10, 100);
    ///
    /// assert_eq!(series.min().unwrap(), 10);
    /// ```
    pub fn min(&self) -> Result<T, BlackJackError>
    where
        T: Num + PartialOrd + BlackJackData + Copy,
    {
        funcs::min(self.values.as_slice())
            .map(|v| *v)
            .ok_or_else(|| BlackJackError::from("Failed to calculate min of series."))
    }

    /// Exibits the same behavior and usage of [`Series::min`], only
    /// yielding the [`Result`] of a maximum.
    pub fn max(&self) -> Result<T, BlackJackError>
    where
        T: Num + PartialOrd + BlackJackData + Copy,
    {
        funcs::max(self.values.as_slice())
            .map(|v| *v)
            .ok_or_else(|| BlackJackError::from("Failed to calculate max of series."))
    }

    /// Determine the length of the Series
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Determine if series is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the dtype, returns `None` if series dtype is unknown.
    /// in such a case, calling `.astype()` to coerce all types to a single
    /// type is needed.
    pub fn dtype(&self) -> Option<DType> {
        self.dtype.clone()
    }

    /// Append a [`BlackJackData`] element to the Series
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut series = Series::from_vec(vec![0, 1, 2]);
    /// assert_eq!(series.len(), 3);
    ///
    /// series.append(3);
    /// assert_eq!(series.len(), 4);
    /// ```
    pub fn append<V: Into<T>>(&mut self, val: V) -> () {
        let v = val.into();
        self.values.push(v);
    }

    /// As boxed pointer, recoverable by `Box::from_raw(ptr)` or
    /// `Series::from_raw(*mut Self)`
    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    /// Create from raw pointer
    pub fn from_raw(ptr: *mut Self) -> Self {
        unsafe { *Box::from_raw(ptr) }
    }

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
    /// let grouped: Series<i32> = series.groupby(&keys).sum();
    /// assert_eq!(grouped.len(), 3);
    ///
    /// let mut vals = grouped.into_vec();
    /// vals.sort();
    /// assert_eq!(vals, vec![2, 4, 6]);
    /// ```
    pub fn groupby(&self, keys: &Series<T>) -> SeriesGroupBy<T>
    where
        T: ToPrimitive,
    {
        /* TODO: Revisit this to avoid the clones. Needs to keep the groups
           in order based on key order; match pandas. ie:

            >>> pd.Series([1, 2, 3, 1, 2, 3]).groupby([4, 5, 6, 4, 5, 6]).sum()
            4    2
            5    4
            6    6
            dtype: int64
        */
        use indexmap::IndexMap;

        let values = self.values.clone();

        let mut map: IndexMap<String, Vec<T>> = IndexMap::new();

        // Group values by their keys
        for (k, v) in keys.values.iter().zip(values.iter()) {
            let key = k.to_string();
            let mr = map.entry(key).or_insert(vec![]);
            mr.push(v.clone());
        }

        // Create new series from the previous mapping.
        let groups = map
            .iter()
            .map(|(name, values)| {
                let mut series = Series::from_vec(values.clone());
                series.set_name(name.as_str());
                series
            })
            .collect();

        SeriesGroupBy::new(groups)
    }

    /// Find the _positions_ where a condition is true
    ///
    /// ## Example
    /// ```
    /// # use blackjack::prelude::*;
    ///
    /// let series = Series::from(0..10);
    /// let positions = series.find(|v| v % 2 == 0);
    ///
    /// assert_eq!(positions, vec![0, 2, 4, 6, 8]);
    /// ```
    pub fn find<F: Fn(&T) -> bool>(&self, condition: F) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_idx, val)| condition(val))
            .map(|(idx, _val)| idx)
            .collect()
    }
}

// Support Series creation from Range
impl<T> From<std::ops::Range<T>> for Series<T>
where
    T: BlackJackData,
    std::ops::Range<T>: Iterator,
    Vec<T>: FromIterator<<std::ops::Range<T> as Iterator>::Item>,
{
    fn from(range: std::ops::Range<T>) -> Series<T> {
        let vec = range.collect();
        Series::from_vec(vec)
    }
}

// Support ref indexing
impl<T> Index<usize> for Series<T>
where
    T: BlackJackData,
{
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        &self.values[idx]
    }
}

// Support ref indexing
impl<T> Index<Range<usize>> for Series<T>
where
    T: BlackJackData,
{
    type Output = [T];
    fn index(&self, idx: Range<usize>) -> &[T] {
        &self.values[idx]
    }
}

// Support ref mut indexing
impl<T: BlackJackData> IndexMut<usize> for Series<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.values[idx]
    }
}

// Support Display for Series
impl<T> fmt::Display for Series<T>
where
    T: BlackJackData,
    String: From<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use prettytable::{Cell, Row, Table};

        let mut table = Table::new();

        // Title (column name)
        table.add_row(Row::new(vec![Cell::new(
            &self.name().unwrap_or("<NA>".to_string()),
        )]));

        // Build remaining values.
        // TODO: Limit how many are actually printed.
        let _ = self
            .values
            .iter()
            .map(|v| {
                let v: String = v.clone().into();
                table.add_row(Row::new(vec![Cell::new(&format!("{}", v))]));
            })
            .collect::<Vec<()>>();

        write!(f, "{}\n", table)
    }
}
