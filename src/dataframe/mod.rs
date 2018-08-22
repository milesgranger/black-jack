use std::collections::HashMap;
use num::{Integer, Float};
use num_traits::*;
use series::{Series, LumberJackData, SeriesData};


type SeriesItem = SeriesData<T = LumberJackData>;

pub struct DataFrame
{
    data: Vec<Box<SeriesItem>>
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
        let vec: Vec<Box<SeriesItem>> = Vec::new();
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
    pub fn add_column<A: LumberJackData>(&mut self, mut series: Series<A>) -> Result<(), &'static str> 
    {
        // Can only add column if series length matches or this is an empty dataframe
        if (series.len() != self.len()) & (self.len() > 0){
            Err("Length of new column does not match length of index!")
        } else {
            let name = match series.name() {
                Some(name) => name,
                None => { format!("{}", self.len()) }
            };
            series.set_name(name);

            self.data.push(Box::new(series) as Box<SeriesItem>);
            Ok(())
        }
    }
}


