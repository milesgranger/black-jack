//! Series represents a single column within a dataframe and wraps many `Array` like
//! functionality.
//! 
//! For methods implemented for a `Series`, please check out the trait [SeriesTrait](trait.SeriesTrait.html)
//! 
//! ## Example use:
//! 
//! ```
//! use blackjack::prelude::*;
//! 
//! let series = Series::arange(0, 5);
//! 
//! assert_eq!(series.sum(), 10);
//! assert_eq!(series.len(), 5);
//! ```

use num::*;
use std::ops::{Range};
use std::iter::{FromIterator, Sum};

use ndarray::Array1 as Array;
use prelude::*;


#[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
pub enum DataElement {
    I64(i64),
    F64(f64),
    I32(i32),
    F32(f32)
}

impl<T: BlackJackData + ToPrimitive> From<T> for DataElement {
    fn from(val: T) -> Self {
        match val.dtype() {
            DType::I64 => DataElement::I64(val.to_i64().unwrap_or_else(|| panic!("Unable to convert value to i64"))),
            DType::F64 => DataElement::F64(val.to_f64().unwrap_or_else(|| panic!("Unable to convert value to f64"))),
            DType::I32 => DataElement::I32(val.to_i32().unwrap_or_else(|| panic!("Unable to convert value to i32"))),
            DType::F32 => DataElement::F32(val.to_f32().unwrap_or_else(|| panic!("Unable to convert value to f32"))),
        }
    }
}

/// Series struct for containing underlying Array and other meta data.
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    
    /// Name of the series, if added to a dataframe without a name, it will be assigned
    /// a default name equalling the cound of columns in the dataframe.
    pub name: Option<String>,

    /// ndarray attribute; the underlying values of the Series
    pub values: Array<DataElement>
}

/// Constructor methods for `Series<T>`
impl Series {

    /// Create a new Series struct from an integer range with one step increments. 
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series: Series<i32> = Series::arange(0, 10);
    /// ```
    pub fn arange<T>(start: T, stop: T) -> Self 
        where
            T: Integer, 
            Self: Sized,
            Range<T>: Iterator, 
            Vec<DataElement>: FromIterator<<Range<T> as Iterator>::Item>
    {
        let data: Vec<DataElement> = (start..stop).collect();
        Series { 
            name: None,
            values: Array::from_vec(data), 
        }
    }

    /// Create a new Series struct from a vector, where T is supported by [`BlackJackData`]. 
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series: Series<i32> = Series::from_vec(vec![1, 2, 3]);
    /// ```
    pub fn from_vec<T>(vec: Vec<T>) -> Self 
        where 
            T: BlackJackData + ToPrimitive,
            DataElement: From<T>
    {
        let vec: Vec<DataElement> = vec.into_iter().map(|v| DataElement::from(v)).collect();
        Series { 
            name: None,
            values: Array::from_vec(vec),
        }
    }
}




impl<T: BlackJackData> SeriesTrait<T> for Series {

    fn set_name(&mut self, name: &str) -> () {
        self.name = Some(name.to_string());
    }

    fn name(&self) -> Option<String> {
        match self.name {
            Some(ref name) => Some(name.clone()),
            None => None
        }
    }

    fn sum(&self) -> T
        where 
            T: Num + Clone + From<DataElement> + Sum
            
    {
        self.values.iter().map(|v| T::from(*v)).sum()
    }

    fn mean(&self) -> Result<f64, &'static str>
        where 
            T: Num + Clone + ToPrimitive + From<DataElement> + Sum,
            f64: Sum<T>
    {
        let mean = self.values.iter().map(|v| T::from(*v)).sum::<f64>() / self.values.len() as f64;
        Ok(mean)
    }

    fn min(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord,
            T: From<DataElement>
    {
        let min = self.values.iter().map(|v| T::from(*v)).min();
        match min {
            Some(m) => Ok(m),
            None => Err("Unable to find minimum of values, perhaps values is empty?")
        }
    }

    fn max(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord,
            T: From<DataElement>
    {
        let max = self.values.iter().map(|v| T::from(*v)).max();
        match max {
            Some(m) => Ok(m),
            None => Err("Unable to find maximum of values, perhaps values is empty?")
        }
    }

    fn len(&self) -> usize { self.values.len() }

    fn dtype(&self) -> DType { 
        // TODO: Add len check, return Option instead.
        match self.values[0] {
            DataElement::I64(_) => DType::I64,
            DataElement::F64(_) => DType::F64,
            DataElement::I32(_) => DType::I32,
            DataElement::F32(_) => DType::F32
        }
     }

}
