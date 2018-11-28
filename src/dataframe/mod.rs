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
    pub fn decoded_series<'a, T: BlackJackData + Deserialize<'a>>(&'a self) -> Result<Series<T>, bincode::Error>
    {
        bincode::deserialize::<Series<T>>(&self.encoded_data)
    }

}

pub struct DataFrame {
    data: Vec<SerializedSeries>
}