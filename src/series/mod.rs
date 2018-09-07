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
//! assert_eq!(series.sum::<i32>(), 10);
//! assert_eq!(series.len(), 5);
//! ```

use num::*;
use std::ops::{Range};
use std::iter::{FromIterator, Sum};
use std::convert::From;
use std::fmt;

use ndarray::Array1 as Array;
use prelude::*;




/// Series struct for containing underlying Array and other meta data.
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    
    /// Name of the series, if added to a dataframe without a name, it will be assigned
    /// a default name equalling the cound of columns in the dataframe.
    pub name: Option<String>,

    /// ndarray attribute; the underlying values of the Series
    pub values: Array<DataElement>
}

impl fmt::Display for Series {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut string = "".to_string();
        let name = self.name().unwrap_or("None".to_string());
        
        // Write name inside column
        let header = format!("| {} |\n", &name);
        string.push_str(&header);

        // Start writing rows... 
        for val in &self.values {

            let mut row_string = "|".to_string();
            let val: String = val.clone().into();

            while row_string.len() < (header.len() / 2) - val.len() as usize {
                row_string.push_str(" ");
            }

            row_string.push_str(&val);
            
            while row_string.len() < header.len() - 2 {
                row_string.push_str(" ");
            }

            row_string.push_str("|\n");
            string.push_str(&row_string);
        }

        write!(f, "{}\n", string)
    }
}

/// Constructor methods for `Series<T>`
impl Series {

    /// Create a new Series struct from an integer range with one step increments. 
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series: Series = Series::arange(0, 10);
    /// ```
    pub fn arange<T>(start: T, stop: T) -> Self 
        where
            T: Integer + BlackJackData + ToPrimitive,
            Range<T>: Iterator, 
            Vec<T>: FromIterator<<Range<T> as Iterator>::Item>
    {
        let data: Vec<T> = (start..stop).collect();
        let vec: Vec<DataElement> = data.into_iter().map(|v| DataElement::from(v)).collect();
        Series { 
            name: None,
            values: Array::from_vec(vec), 
        }
    }

    /// Create a new Series struct from a vector, where T is supported by [`BlackJackData`]. 
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let series: Series = Series::from_vec(vec![1, 2, 3]);
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




impl SeriesTrait for Series {

    fn set_name(&mut self, name: &str) -> () {
        self.name = Some(name.to_string());
    }

    fn name(&self) -> Option<String> {
        match self.name {
            Some(ref name) => Some(name.clone()),
            None => None
        }
    }

    fn sum<T>(&self) -> T
        where 
            T: Num + Clone + From<DataElement> + Sum + Copy
            
    {
        self.values.iter().map(|v| T::from(v.clone())).sum()
    }

    fn mean(&self) -> Result<f64, &'static str>
    {
        let total: f64 = self.sum();
        let count: f64 = self.len() as f64;
        Ok(total / count)
    }

    fn min<T>(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord + BlackJackData + From<DataElement>
    {
        let min = self.values.iter().map(|v| T::from(v.clone())).min();
        match min {
            Some(m) => Ok(m),
            None => Err("Unable to find minimum of values, perhaps values is empty?")
        }
    }

    fn max<T>(&self) -> Result<T, &'static str>
        where 
            T: Num + Clone + Ord,
            T: From<DataElement>
    {
        let max = self.values.iter().map(|v| T::from(v.clone())).max();
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
            DataElement::F32(_) => DType::F32,
            DataElement::STRING(_) => DType::STRING
        }
     }

}
