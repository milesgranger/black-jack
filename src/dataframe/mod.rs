///! DataFrame object and associated functionality

use std::fmt;
use std::path::Path;
use std::ffi::OsStr;

use bincode;
use serde::{Serialize, Deserialize};
use failure::Error;
use rayon::prelude::*;
use csv;
use snap;

use prelude::*;


#[derive(Debug, Fail)]
pub enum BlackJackError {

    #[fail(display = "No series name present!")]
    NoSeriesName,

    #[fail(display = "Unable to decode series")]
    SerializationDecodeError(Box<bincode::ErrorKind>),

    #[fail(display = "Unable to read headers!")]
    HeaderParseError(csv::Error),

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
                let encoded_data = bincode::serialize(&series.clone())?;
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

    /// Retrieves a column from the dataframe as an owned representation of it.
    pub fn get_column<'a, T>(&'a self, name: impl Into<&'a str>) -> Result<Series<T>, BlackJackError>
        where T: BlackJackData + Deserialize<'a>
    {
        let name = name.into();
        for encoded_series in &self.data {
            if encoded_series.name == name {
                let series = encoded_series.decode::<T>()?;
                println!("Decoded series: {:?}", &series);
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
    /// let col1: Series<String> = df.get_column("col1").unwrap();
    /// assert_eq!(col1.len(), 15);
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
        let _ = headers
            .into_iter()
            .zip(vecs)
            .map(|(header, vec)| {
                let mut series = Series::from_vec(vec);
                series.set_name(&header);
                if let Ok(ser) = series.astype::<f32>() {

                    df.add_column(ser.clone()).unwrap();
                    println!("Added series i32: {:?}", &ser);
                } else {
                    df.add_column(series).unwrap()
                }
            })
            .collect::<Vec<()>>();
        Ok(df)
    }
}

