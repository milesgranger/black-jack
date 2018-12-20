//! DataFrame object and associated functionality
//!
//!

use bincode::{serialize_into, deserialize_from};
use csv;
use serde::{Deserialize};

use prelude::*;

pub mod io;
pub use self::io::*;


/// Common error enum for the crate
#[derive(Debug, Fail)]
pub enum BlackJackError {

    /// A failure of not having the `Series` name set, where one was expected
    #[fail(display = "No series name present!")]
    NoSeriesName,

    /// A failure to decode a `Series<T>` which was previously encoded to `SerializedSeries`
    #[fail(display = "Unable to decode series")]
    SerializationDecodeError(Box<bincode::ErrorKind>),

    /// Failure to parse the header of a CSV file.
    #[fail(display = "Unable to read headers!")]
    HeaderParseError(csv::Error),

    /// Failure of a general `std::io::Error`
    #[fail(display = "IO error")]
    IoError(std::io::Error),

    /// Failure due to mismatched sizes
    #[fail(display = "ValueError")]
    ValueError(String)
}

impl From<&str> for BlackJackError {
    fn from(error: &str) -> BlackJackError {
        BlackJackError::ValueError(error.to_owned())
    }
}

impl From<std::io::Error> for BlackJackError {
    fn from(error: std::io::Error) -> BlackJackError {
        BlackJackError::IoError(error)
    }
}

impl From<csv::Error> for BlackJackError {
    fn from(error: csv::Error) -> BlackJackError {
        BlackJackError::HeaderParseError(error)
    }
}

impl From<Box<bincode::ErrorKind>> for BlackJackError {
    fn from(error: Box<bincode::ErrorKind>) -> BlackJackError {
        BlackJackError::SerializationDecodeError(error)
    }
}


/// Enum for holding valid Series types
pub enum GenericSeriesContainer {

    /// Hold `i64` type series
    I64(Series<i64>),
    /// Hold `f64` type series
    F64(Series<f64>),
    /// Hold `i32` type series
    I32(Series<i32>),
    /// Hold `f32` type series
    F32(Series<f32>),
    /// Hold `String` type series
    STRING(Series<String>)
}

impl GenericSeriesContainer {

    fn into_string_vec(self) -> Vec<String> {
        // TODO: `.unwrap()` is pretty safe here, but should avoid it anyhow.
        match self {
            GenericSeriesContainer::I64(series) => series.into_type::<String>().unwrap().into_vec(),
            GenericSeriesContainer::F64(series) => series.into_type::<String>().unwrap().into_vec(),
            GenericSeriesContainer::I32(series) => series.into_type::<String>().unwrap().into_vec(),
            GenericSeriesContainer::F32(series) => series.into_type::<String>().unwrap().into_vec(),
            GenericSeriesContainer::STRING(series) => series.into_vec()
        }
    }
}

/// Serialized version of `Series<T>`, enabling storage inside a homogeneous container
/// where metadata is stored and data is stored in byte/compressed format.
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
    pub fn from_series<T>(series: Series<T>) -> Result<Self, BlackJackError>
        where T: BlackJackData
    {
        match series.name() {
            Some(name) => {
                let dtype = series.dtype();
                let len = series.len();
                let mut encoded_data = vec![];
                serialize_into(&mut encoded_data, &series.values)?;
                Ok(SerializedSeries { name, dtype, len, encoded_data, })
            },
            None => Err(BlackJackError::NoSeriesName)
        }
    }
    /// Deserialize this into a series
    pub fn decode<T>(&self) -> Result<Series<T>, bincode::Error>
        where for<'de> T: BlackJackData + Deserialize<'de>,
    {
        let data = deserialize_from(&self.encoded_data[..])?;
        let mut series = Series::from_vec(data);
        series.set_name(&self.name);
        Ok(series)
    }

    /// Decode the series into a `GenericSeriesContainer`; useful if you don't know the
    /// resulting type of the `Series` you're after
    pub fn decode_infer(&self) -> Result<GenericSeriesContainer, BlackJackError> {
        let container = match self.dtype {
            DType::I64 => GenericSeriesContainer::I64(self.decode::<i64>()?),
            DType::F64 => GenericSeriesContainer::F64(self.decode::<f64>()?),
            DType::I32 => GenericSeriesContainer::I32(self.decode::<i32>()?),
            DType::F32 => GenericSeriesContainer::F32(self.decode::<f32>()?),
            DType::STRING => GenericSeriesContainer::STRING(self.decode::<String>()?),
            _ => return Err(BlackJackError::ValueError("Series dtype 'None' invalid here!".to_string()))
        };

        Ok(container)
    }

}

/// The container for `Series<T>` objects, allowing for additional functionality
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

    /// Retrieves a column from the dataframe as an owned representation of it.
    pub fn get_column<'a, T>(&self, name: impl Into<&'a str>) -> Result<Series<T>, BlackJackError>
        where for<'de> T: BlackJackData + Deserialize<'de>
    {
        let name = name.into();
        for encoded_series in &self.data {
            if encoded_series.name == name {
                let series = encoded_series.decode::<T>()?;
                return Ok(series)
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
