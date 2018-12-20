//! Series represents a single column within a dataframe and wraps many `Array` like
//! functionality.
//! 
//! For methods implemented for a [`Series`], please check out the trait [`SeriesTrait`]
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

use std::ops::{Range, Index, IndexMut};
use std::iter::{FromIterator, Sum};
use std::vec::IntoIter;
use std::convert::From;
use std::fmt;
use std::str::FromStr;
use std::marker::{Send, Sync};

use itertools::Itertools;

use rayon::prelude::*;
use num::*;
use stats;

pub mod overloaders;
use prelude::*;


// Allow series.into_iter()
impl_series_into_iter!(String);
impl_series_into_iter!(f64);
impl_series_into_iter!(i64);
impl_series_into_iter!(f32);
impl_series_into_iter!(i32);


/// Series struct for containing underlying Array and other meta data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Series<T>
    where
        T: BlackJackData
{
    
    /// Name of the series, if added to a dataframe without a name, it will be assigned
    /// a default name equalling the count of columns in the dataframe.
    pub name: Option<String>,

    /// The underlying values of the Series
    pub values: Vec<T>,

    dtype: DType
}

/// Constructor methods for `Series<T>`
impl<T> Series<T>
    where
        T: BlackJackData
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
            Vec<T>: FromIterator<<Range<T> as Iterator>::Item>
    {
        let dtype = start.dtype();
        let values: Vec<T> = (start..stop).collect();
        Series { 
            name: None,
            dtype,
            values
        }
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
    pub fn isna<'a>(&'a self) -> impl Iterator<Item=bool> + 'a
        where T: Float
    {
        self.values
            .iter()
            .map(|v| v.is_nan())
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
        where for<'r> F: Fn(&'r T) -> bool
    {
        self.values
            .iter()
            .all(condition)
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
        where T: PartialEq
    {
        self.values.iter().all_equal()
    }

    /// Determine if _any_ element in the Series meets a given condition
    ///
    /// This will stop iteration after encountering the first element which meets
    /// conditions supplied.
    pub fn any<F>(&self, condition: F) -> bool
        where for<'r> F: FnMut(&'r &T,) -> bool
    {
        let first_match = self.values
            .iter()
            .find(condition);
        match first_match {
            Some(_) => true,
            None => false
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
        where O: BlackJackData
    {
        let mut left = vec![];
        let mut right = vec![];
        let _ = self.values
            .clone()
            .into_iter()
            .cartesian_product(other.values.clone().into_iter())
            .map(|(l, r)| { left.push(l); right.push(r); })
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
    pub fn positions<'a, F>(&'a self, condition: F) -> impl Iterator<Item=usize> + 'a
        where F: 'a + Fn(&T) -> bool
    {
        self.values
            .iter()
            .positions(condition)
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
            F: Fn(T) -> B + Send + Sync
    {
        let new_data = self.values
            .into_par_iter()
            .map(func)
            .collect();
        Series::from_vec(new_data)
    }

    /// Map a function over a series in a single thread
    /// Function takes some type `T` and returns some type `B` which
    /// has `BlackJackData` implemented.
    pub fn map<B, F>(self, func: F) -> Series<B>
        where
            B: BlackJackData,
            F: Fn(T) -> B
    {
        let new_data = self.values
            .into_iter()
            .map(func)
            .collect();
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
        where A: BlackJackData + FromStr
    {
        let values = self.values
                .clone()
                .into_iter()
                .map(|v| v.to_string())
                .map(|v| v.parse::<A>().map_err(|_| "Cannot cast into type"))
                .collect::<Result<Vec<A>, _>>()?;
        let series = Series {
            name: self.name.clone(),
            dtype: values[0].dtype(),
            values
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
        where A: BlackJackData + FromStr
    {
        let values = self.values
                .into_iter()
                .map(|v| v.to_string())
                .map(|v| v.parse::<A>().map_err(|_| "Cannot cast into type"))
                .collect::<Result<Vec<A>, _>>()?;
        let series = Series {
            name: self.name.clone(),
            dtype: values[0].dtype(),
            values
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
        where T: PartialOrd + Copy
    {
        // Cannot use `HashSet` as f32 & f64 don't implement Hash
        let mut unique: Vec<T> = vec![];
        let mut values = self.values.clone();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for val in values
        {
            if unique.len() > 0 {
                if val == unique[unique.len() - 1] {
                    continue
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
    pub fn from_vec(vec: Vec<T>) -> Self
    {
        let dtype = vec[0].dtype();  // TODO: Do something better.
        Series { 
            name: None,
            dtype,
            values: vec
        }
    }

    /// Convert the series to a [`Vec`]  
    /// **Type Annotations required**
    /// Will coerce elements into the desired [`DType`] primitive, just as
    /// [`SeriesTrait::astype()`]. 
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
            None => None
        }
    }

    /// Finds the returns a [`Series`] containing the mode(s) of the current
    /// [`Series`]
    pub fn mode(&self) -> Result<Self, &'static str>
        where T: BlackJackData + PartialOrd + Copy + ToPrimitive
    {
        if self.len() == 0 {
            return Err("Cannot compute mode of an empty series!")
        }

        let modes = stats::modes(self.values.iter().map(|v| *v));
        let modes = Series::from_vec(modes);
        Ok(modes)
    }

    /// Calculate the variance of the series
    pub fn var(&self) -> Result<f64, &'static str>
        where T: BlackJackData + ToPrimitive + Copy
    {
        if self.len() == 0  {
            return Err("Cannot compute variance of an empty series!");
        }
        let var = stats::variance(self.values.iter().map(|v| *v));
        Ok(var)
    }

    /// Calculate the standard deviation of the series
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series = Series::arange(0, 10).astype::<f32>().unwrap();
    /// 
    /// let std = series.std().unwrap(); // Ok(2.8722...)
    /// assert!(std > 2.87);
    /// assert!(std < 2.88);
    /// ```
    pub fn std(&self) -> Result<f64, &'static str>
        where T: BlackJackData + ToPrimitive + Copy
    {
        if self.len() == 0 {
            return Err("Cannot compute standard deviation of an empty series!")
        }
        let std = stats::stddev(self.values.iter().map(|v| *v));
        Ok(std)
    }

    /// Sum a given series, yielding the same type as the elements stored in the 
    /// series.
    pub fn sum(&self) -> T
        where
            T: Num + Copy + Sum
    {
        self.values
            .iter()
            .map(|v| *v)
            .sum()
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
    pub fn mean<'a>(&'a self) -> Result<f64, &'static str>
        where
            T: ToPrimitive + Copy + Sum<&'a T> + Num + Sum
    {
        let total = self.sum().to_f64().unwrap();
        let count = self.len() as f64;
        Ok(total / count)
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
    pub fn quantile(&self, quantile: f64) -> Result<f64, &'static str>
        where 
            T: ToPrimitive + BlackJackData
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
    pub fn median<'a>(&'a self) -> Result<f64, &'static str>
        where T: ToPrimitive + Copy + PartialOrd
    {
        if self.len() == 0 {
            return Err("Cannot calculate median of an empty series.")
        }
        let med_opt = stats::median(self.values.iter().map(|v| v.to_f64().unwrap()));
        match med_opt {
            Some(med) => Ok(med),
            None => Err(r#"Unable to calculate median, please create an issue!
                           as this wasn't expected to ever happen on a non-empty
                           series. :("#)
        }
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
            T: Num + PartialOrd + BlackJackData + Copy
    {
        match self.values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
            Some(min) => Ok(min.to_owned()),
            None => Err(BlackJackError::ValueError("Cannot find min of empty series".into()))
        }
    }

    /// Exibits the same behavior and usage of [`SeriesTrait::min`], only
    /// yielding the [`Result`] of a maximum.
    pub fn max(&self) -> Result<T, BlackJackError>
        where 
            T: Num + PartialOrd + BlackJackData + Copy
    {
        match self.values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
            Some(max) => Ok(max.to_owned()),
            None => Err(BlackJackError::ValueError("Cannot find max of empty series".into()))
        }
    }

    /// Determine the length of the Series
    pub fn len(&self) -> usize { self.values.len() }

    /// Determine if series is empty.
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the dtype, returns `None` if series dtype is unknown. 
    /// in such a case, calling `.astype()` to coerce all types to a single
    /// type is needed. 
    pub fn dtype(&self) -> DType {
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
    /// `SeriesTrait::from_raw(*mut Self)`
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
    /// let grouped: Series<i32> = series.groupby(keys).sum();
    /// assert_eq!(grouped.len(), 3);
    ///
    /// let mut vals = grouped.into_vec();
    /// vals.sort();
    /// assert_eq!(vals, vec![2, 4, 6]);
    /// ```
    pub fn groupby(&self, keys: Series<T>) -> SeriesGroupBy<T>
        where T: ToPrimitive
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
        for (k, v) in keys.values.into_iter().zip(values.into_iter()) {
            let key = k.to_string();
            let mr = map.entry(key).or_insert(vec![]);
            mr.push(v);
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
}


// Support ref indexing
impl<T> Index<usize> for Series<T>
    where T: BlackJackData
{
    type Output = T;
    fn index(&self, idx: usize) -> &T {
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
        String: From<T>
{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        use prettytable::{Table, Row, Cell};

        let mut table = Table::new();

        // Title (column name)
        table.add_row(
            Row::new(
                vec![
                    Cell::new(&self.name().unwrap_or("<NA>".to_string()))
                ]
            )
        );

        // Build remaining values.
        // TODO: Limit how many are actually printed.
        let _ = self.values
            .iter()
            .map(|v| {
                let v: String = v.clone().into();
                table.add_row(
                    Row::new(vec![
                        Cell::new(&format!("{}", v))
                    ])
                );
            })
            .collect::<Vec<()>>();

        write!(f, "{}\n", table)
    }
}
