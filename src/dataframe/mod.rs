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


pub struct SerializedSeries {
    name: String,
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
                Ok(SerializedSeries { name, dtype, encoded_data, })
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

pub struct DataFrame {
    data: Vec<SerializedSeries>
}

impl DataFrame {

    /// Get a new and empty dataframe
    pub fn new() -> Self {
        DataFrame {
            data: vec![]
        }
    }

    /// Add a column to this dataframe.
    pub fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> Result<(), BlackJackError> {
        let serialized = SerializedSeries::from_series(series)?;
        self.data.push(serialized);
        Ok(())
    }

    /// Get a list of column names in this dataframe
    pub fn columns(&self) -> impl Iterator<Item=&str> {
        self.data
            .iter()
            .map(|c| c.name.as_str())
    }
}

