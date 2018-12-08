///! DataFrame object and associated functionality

use std::fmt;

use bincode;
use serde::{Serialize, Deserialize};
use failure::Error;

use prelude::*;


#[derive(Debug, Fail)]
pub enum BlackJackError {
    #[fail(display = "No series name present!")]
    NoSeriesName,
    #[fail(display = "Unable to decode series")]
    SerializationDecodeError(Box<bincode::ErrorKind>),
}

impl From<Box<bincode::ErrorKind>> for BlackJackError {
    fn from(error: Box<bincode::ErrorKind>) -> BlackJackError {
        BlackJackError::SerializationDecodeError(error)
    }
}

#[derive(Debug)]
pub struct SerializedSeries {
    name: String,
    len: usize,
    dtype: DType,
    encoded_data: Vec<u8>
}

impl SerializedSeries {

    /// Serialize a [`Series`] into a [`SerializedSeries`]
    /// used for storing various Series types into a container, typically, you will not use
    /// this directly.
    pub fn from_series<T: BlackJackData + Serialize>(series: Series<T>) -> Result<Self, BlackJackError> {
        match series.name() {
            Some(name) => {
                let encoded_data = bincode::serialize(&series)?;
                let dtype = series.dtype();
                let len = series.len();
                Ok(SerializedSeries { name, dtype, len, encoded_data, })
            },
            None => Err(BlackJackError::NoSeriesName)
        }
    }
    /// Deserialize this into a series
    pub fn decode<'a, T>(&'a self) -> Result<Series<T>, bincode::Error>
        where T: BlackJackData + Deserialize<'a>
    {
        bincode::deserialize::<Series<T>>(&self.encoded_data)
    }

}

#[derive(Default, Debug)]
pub struct DataFrame {
    data: Vec<SerializedSeries>
}

impl DataFrame {

    /// Create a new `DataFrame` struct
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut df = DataFrame::new();
    /// ```
    pub fn new() -> Self {
        DataFrame {
            data: vec![]
        }
    }

    /// Length of the dataframe
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut df = DataFrame::new();
    /// assert_eq!(df.len(), None);
    ///
    /// let series: Series<i32> = Series::arange(0, 10);
    /// df.add_column(series).unwrap();
    ///
    /// assert_eq!(df.len(), Some(10));
    /// ```
    pub fn len(&self) -> Option<usize> {
        if self.data.len() > 0 {
            Some(self.data[0].len)
        } else {
            None
        }
    }

    /// Quickly identify if the dataframe is empty.
    pub fn is_empty(&self) -> bool {
        self.len().is_none()
    }

    /// Add a column to this dataframe.
    pub fn add_column<T: BlackJackData>(&mut self, mut series: Series<T>) -> Result<(), BlackJackError> {
        // Set series name if it wasn't set already.
        if let None = series.name() {
            series.set_name(&format!("col_{}", self.data.len()))
        }
        let serialized = SerializedSeries::from_series(series)?;
        self.data.push(serialized);
        Ok(())
    }

    /// Get a reference to a column
    pub fn get_column<'a, T>(&'a self, name: impl Into<&'a str>) -> Result<Series<T>, BlackJackError>
        where T: BlackJackData + Deserialize<'a>
    {
        let name = name.into();
        for encoded_series in &self.data {
            if encoded_series.name == name {
                return Ok(encoded_series.decode::<T>()?)
            }
        }
        Err(BlackJackError::NoSeriesName)
    }

    /// Get a list of column names in this dataframe as an iterator
    pub fn columns(&self) -> impl Iterator<Item=&str> {
        self.data
            .iter()
            .map(|c| c.name.as_str())
    }

    /// Get the number of columns for this dataframe
    pub fn n_columns(&self) -> usize {
        self.data.len()
    }
}

