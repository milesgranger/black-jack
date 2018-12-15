//! DataFrame object and associated functionality
//!
//!

use std::path::Path;
use std::ffi::OsStr;

use bincode::{serialize_into, deserialize_from};
use csv;
use serde::{Deserialize};

use prelude::*;


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
    IoError(std::io::Error)
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

    /// Read a CSV file into a [`DataFrame`] where each column represents a Series
    /// supports automatic decompression of gzipped files if they end with `.gz`
    ///
    /// ## Example
    ///
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let path = format!("{}/tests/data/basic_csv.csv", env!("CARGO_MANIFEST_DIR"));
    /// let df = DataFrame::read_csv(&path, b',').unwrap();
    ///
    /// let col1: Series<f32> = df.get_column("col1").unwrap();
    /// assert_eq!(col1.sum() as i32, 15);
    ///
    /// ```
    pub fn read_csv<S>(path: S, delimiter: u8) -> Result<Self, BlackJackError>
        where S: AsRef<OsStr> + ToString
    {

        use std::io::prelude::*;
        use std::fs::File;
        use flate2::read::GzDecoder;

        let p = Path::new(&path);
        let file_reader: Box<Read> = if path.to_string().to_lowercase().ends_with(".gz") {
                                            // Return a Gzip reader
                                            Box::new(GzDecoder::new(File::open(p)?))
                                        } else {
                                            // Return plain file reader
                                            Box::new(File::open(p)?)
                                        };

        let mut reader = csv::ReaderBuilder::new()
                                .delimiter(delimiter)
                                .from_reader(file_reader);

        // TODO: Don't fail on non existant headers -> give 'col0', 'col1', etc.
        let headers: Vec<String> = reader.headers()?
                                        .clone()
                                        .into_iter()
                                        .map(|v| v.to_string())
                                        .collect();

        // Containers for storing column data
        let mut vecs: Vec<Vec<String>> = (0..headers.len())
                                            .map(|_| Vec::new())
                                            .collect();

        for record in reader.records() {

            match record {

                Ok(rec) => {
                    for (field, container) in rec.iter().zip(&mut vecs) {
                        container.push(field.into());
                    };
                },

                // TODO: Process for dealing with invalid records.
                Err(err) => println!("Unable to read record: '{}'", err)
            }
        }

        let mut df = DataFrame::new();

        // map headers to vectors containing it's fields in parallel and into
        // Series structs, parsing each field.
        // TODO: Parallelize this operation, parse && serialize columns in parallel, then add them.
        let _ = headers
            .into_iter()
            .zip(vecs)
            .map(|(header, vec)| {
                let mut series = Series::from_vec(vec);
                series.set_name(&header);
                if let Ok(ser) = series.astype::<i32>() {
                    df.add_column(ser).unwrap();
                } else if let Ok(ser) = series.astype::<f32>() {
                    df.add_column(ser).unwrap()
                } else {
                    df.add_column(series).unwrap()
                }
            })
            .collect::<Vec<()>>();
        Ok(df)
    }
}
