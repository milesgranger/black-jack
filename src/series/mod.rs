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
//! // Index and change elements, call `.into()` to easily convert to `DataElement`
//! series[0] = 1.into();              // `into()` on `BlackJackData`
//! series[1] = DataElement::I32(0);   // ...or more explicitly set the value
//! 
//! assert_eq!(series[0], DataElement::I32(1));
//! assert_eq!(series.sum::<i32>(), 10);
//! assert_eq!(series.len(), 5);
//! ```

use std::ops::{Range, Index, IndexMut};
use std::iter::{FromIterator, Sum};
use std::convert::From;
use std::fmt;

use num::*;
use stats;
use rayon::prelude::*;

pub mod overloaders;
use prelude::*;


/// Series struct for containing underlying Array and other meta data.
#[derive(Debug, Clone, PartialEq)]
pub struct Series<T>
    where
        T: BlackJackData
{
    
    /// Name of the series, if added to a dataframe without a name, it will be assigned
    /// a default name equalling the cound of columns in the dataframe.
    pub name: Option<String>,

    /// The underlying values of the Series
    pub values: Vec<T>,

    /// The index of the Series
    index: Vec<DataElement>,

    // Only set if called by `.astype()` or parsing or raw data was able to
    // confirm all `DataElement`s are of the same type.
    dtype: Option<DType>
}

impl<T> Index<usize> for Series<T>
    where T: BlackJackData
{
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        &self.values[idx]
    }
}

impl<T: BlackJackData> IndexMut<usize> for Series<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.values[idx]
    }
}

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
    /// let series: Series = Series::arange(0, 10);
    /// ```
    pub fn arange(start: T, stop: T) -> Self
        where
            T: Integer + BlackJackData + ToPrimitive,
            Range<T>: Iterator, 
            Vec<T>: FromIterator<<Range<T> as Iterator>::Item>
    {
        let dtype = Some(start.dtype());
        let values: Vec<T> = (start..stop).collect();
        Series { 
            name: None,
            dtype,
            values
        }
    }

    /// Get a series of the unique elements held in this series
    /// 
    /// ## Example
    /// 
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series: Series = Series::from_vec(vec![1.0, 2.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0]);
    /// let unique: Series = series.unique::<i32>();
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
    /// let series: Series = Series::from_vec(vec![1, 2, 3]);
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Self
    {
        let dtype = if vec.len() > 0 { Some(vec[0].dtype()) } else  { None };
        Series { 
            name: None,
            dtype,
            index,
            values
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
    ///     series.clone().into_vec::<i32>(), 
    ///     vec![1_i32, 2_i32, 3_i32]
    /// );
    /// assert_eq!(
    ///     series.into_vec::<String>(), 
    ///     vec![1_f64.to_string(), 2_f64.to_string(), 3_f64.to_string()]
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
        let mut modes = Series::from_vec(modes);
        Ok(modes)
    }

    /// Calculate the variance of the series  
    /// **NOTE** that whatever type is determined is what the values are cast to
    /// during calculation of the variance. 
    /// 
    /// ie. `series.var::<i32>()` will cast each element into `i32` as input
    /// for calculating the variance, and yield a `i32` value. If you want all
    /// values to be calculated as `f64` then specify that in the type annotation.
    pub fn var(&self) -> Result<f64, &'static str>
        where 
            T: BlackJackData + ToPrimitive + Copy
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
    /// let series = Series::arange(0, 10);
    /// 
    /// let std = series.std::<f32>().unwrap(); // Ok(2.8722...)
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
    /// let series = Series::arange(0, 100);
    /// let qtl = series.quantile::<f32>(0.5).unwrap(); // `49.5_f32`
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
    /// let series: Series = Series::arange(10, 100);
    /// 
    /// assert_eq!(series.min(), Ok(10));
    /// ```
    pub fn min(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord + BlackJackData
    {
        let min = self.values.iter().min();
        match min {
            Some(m) => Ok(m.clone()),
            None => Err("Unable to find minimum of values, perhaps values is empty?")
        }
    }

    /// Exibits the same behavior and usage of [`SeriesTrait::min`], only
    /// yielding the [`Result`] of a maximum.
    pub fn max(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord
    {
        let max = self.values.iter().max();
        match max {
            Some(m) => Ok(m.clone()),
            None => Err("Unable to find maximum of values, perhaps values is empty?")
        }
    }

    /// Determine the length of the Series
    pub fn len(&self) -> usize { self.values.len() }

    /// Determine if series is empty.
    pub fn is_empty(&self) -> bool { self.len() == 0 }

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
        self.values.push(val.into());
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
    /// let grouped: Series = series.groupby(keys).sum::<i32>();
    /// assert_eq!(grouped.len(), 3);
    ///
    /// let mut vals = grouped.into_vec::<i32>();
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
