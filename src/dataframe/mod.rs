use std::collections::HashMap;
use num::{Integer, Float};
use num_traits::*;
use series::{Series, LumberJackData, SeriesData};


type DataFrameData = Vec<Box<SeriesData<LumberJackData>>>;

pub struct DataFrame
{
    data: DataFrameData
}

impl DataFrame
{

    /// Constructs a new `DataFrame<'a>`
    /// 
    /// # Example
    /// 
    /// ```
    /// use blackjack::dataframe::DataFrame;
    /// 
    /// let df: DataFrame = DataFrame::new();
    /// ```
    pub fn new() -> Self {
        let vec: DataFrameData = Vec::new();
        DataFrame { data: vec }
    }

    /// Return length of the dataframe
    pub fn len(&self) -> usize {
        if self.data.len() > 0 {
            let first_series = &self.data[0];
            1
        } else {
            0
        }
    }
}
