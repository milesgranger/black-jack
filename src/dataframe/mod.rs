use std::collections::HashMap;
use num::{Integer, Float};
use num_traits::*;
use series::{Series, LumberJackData, SeriesData};




pub struct DataFrame
{
    data: Vec<Box<SeriesData>>
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
        let vec: Vec<Box<SeriesData>> = Vec::new();
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

    /// Attempt to get a reference to a series in the dataframe by name
    /// 
    /// # Example
    /// 
    /// ```
    /// use blackjack::dataframe::DataFrame;
    /// use blackjack::series::Series;
    /// 
    /// ```
    pub fn add_column<T: LumberJackData + 'static>(&mut self, mut series: Series<T>) -> Result<(), &'static str> {
        // Can only add column if series length matches or this is an empty dataframe
        if (series.len() != self.len()) & (self.len() > 0){
            Err("Length of new column does not match length of index!")
        } else {
            series.name = match series.name {
                Some(name) => Some(name),
                None => {
                    let name = format!("{}", self.len());
                    Some(name)
                }
            };
            self.data.push(Box::new(series));
            Ok(())
        }
    }
}


