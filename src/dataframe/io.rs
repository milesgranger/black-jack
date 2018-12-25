//!
//! This module contains the io operators for dealing with DataFrames reading and writing.
//!

use std::path::Path;
use std::ffi::OsStr;

use crate::prelude::*;


/// DataFrame reading struct
///
/// ## Example
///
/// ```
/// use blackjack::prelude::*;
///
/// let path = format!("{}/tests/data/basic_csv.csv", env!("CARGO_MANIFEST_DIR"));
/// let df = Reader::new(&path).delimiter(b',').read().unwrap();
///
/// let col1: &Series<f32> = df.get_column("col1").unwrap();
/// assert_eq!(col1.sum() as i32, 15);
///
/// ```
#[derive(Clone)]
pub struct Reader {
    path: String,
    delimiter: u8,
    terminator: csv::Terminator,
    quote: u8,
    has_headers: bool,
    header: Option<Vec<String>>
}

/// DataFrame reading struct
///
/// ## Example
/// ```
/// use blackjack::prelude::*;
///
/// let mut df = DataFrame::new();
///
/// df.add_column(Series::arange(0, 10));
/// df.add_column(Series::arange(0, 10));
///
/// let result = Writer::new(&"/tmp/test.csv.gz").delimiter(b',').write(df).is_ok(); // Gzip compression inferred.
/// assert_eq!(result, true);
/// ```
#[derive(Clone)]
pub struct Writer {
    path: String,
    delimiter: u8,
    terminator: csv::Terminator,
    quote: u8,
    has_headers: bool,
}

impl Reader {

    /// Create a new instance of `Reader` with defaults CSV params
    pub fn new<S: AsRef<OsStr> + ToString>(path: &S) -> Self {
        Reader {
            path: path.to_string(),
            delimiter: b',',
            terminator: csv::Terminator::CRLF,
            quote: b'"',
            has_headers: true,
            header: None
        }
    }

    /// Set header, must be set if `has_headers` is false, and ignore if it is true
    pub fn headers(self, header: Vec<String>) -> Self {
        let mut rdr = self;
        rdr.header = Some(header);
        rdr
    }

    /// Whether to expect headers in the file or not.
    pub fn has_headers(self, yes: bool) -> Self {
        let mut rdr = self;
        rdr.has_headers = yes;
        rdr
    }

    /// Set the CSV quote character, default is `b'"'`
    pub fn quote(self, quote: u8) -> Self {
        let mut rdr = self;
        rdr.quote = quote;
        rdr
    }

    /// Set the CSV delimiter, default is `b','` (comma delimited)
    pub fn delimiter(self, delimiter: u8) -> Self {
        let mut rdr = self;
        rdr.delimiter = delimiter;
        rdr
    }

    /// Set the CSV line terminator, default treats any of `\r`, `\n` or `\r\n` as a line terminator
    pub fn terminator(self, terminator: u8) -> Self {
        let mut rdr = self;
        rdr.terminator = csv::Terminator::Any(terminator);
        rdr
    }

    /// Read a CSV file into a [`DataFrame`] where each column represents a Series
    /// supports automatic decompression of gzipped files if they end with `.gz`
    pub fn read(&self) -> Result<DataFrame<i32>, BlackJackError>
    {

        use std::io::prelude::*;
        use std::fs::File;
        use flate2::read::GzDecoder;

        let p = Path::new(&self.path);
        let file_reader: Box<Read> = if self.path.to_string().to_lowercase().ends_with(".gz") {
                                            // Return a Gzip reader
                                            Box::new(GzDecoder::new(File::open(p)?))
                                        } else {
                                            // Return plain file reader
                                            Box::new(File::open(p)?)
                                        };

        let mut reader = csv::ReaderBuilder::new()
            .quote(self.quote)
            .has_headers(self.has_headers)
            .delimiter(self.delimiter)
            .terminator(self.terminator)
            .from_reader(file_reader);

        let headers: Vec<String> = if self.has_headers {
            reader.headers()?
                .clone()
                .into_iter()
                .map(|v| v.to_string())
                .collect()
        } else {
            match &self.header {
                Some(header) => header.to_owned(),
                None => {
                    return Err(
                        BlackJackError::ValueError(r#"Reader specifies file does not have headers,
                        but no headers were supplied with Reader::header()"#
                            .to_owned()
                        )
                    )
                }
            }
        };

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

impl Writer {

    /// Create a new instance of `Reader` with defaults CSV params
    pub fn new<S: AsRef<OsStr> + ToString>(path: &S) -> Self {
        Writer {
            path: path.to_string(),
            delimiter: b',',
            terminator: csv::Terminator::CRLF,
            quote: b'"',
            has_headers: true
        }
    }

    /// Whether to write headers in the file or not with the dataframe output.
    pub fn has_headers(self, yes: bool) -> Self {
        let mut wtr = self;
        wtr.has_headers = yes;
        wtr
    }

    /// Set the CSV quote character, default is `b'"'`
    pub fn quote(self, quote: u8) -> Self {
        let mut wtr = self;
        wtr.quote = quote;
        wtr
    }

    /// Set the CSV delimiter, default is `b','` (comma delimited)
    pub fn delimiter(self, delimiter: u8) -> Self {
        let mut wtr = self;
        wtr.delimiter = delimiter;
        wtr
    }

    /// Set the CSV line terminator, default treats any of `\r`, `\n` or `\r\n` as a line terminator
    pub fn terminator(self, terminator: u8) -> Self {
        let mut wtr = self;
        wtr.terminator = csv::Terminator::Any(terminator);
        wtr
    }

    /// Write a dataframe to CSV, consumes self, and thus will not double memory whilst
    /// writing to CSV.
    pub fn write<I: PartialEq + PartialOrd + BlackJackData>(&self, df: DataFrame<I>) -> Result<(), BlackJackError>
    {
        use std::io::prelude::*;
        use std::fs::File;
        use flate2::read::GzEncoder;
        use flate2::Compression;

        let p = Path::new(&self.path);

        let file_writer: Box<Write> = if self.path.to_string().to_lowercase().ends_with(".gz") {
                                            // Return a Gzip reader
                                            Box::new(GzEncoder::new(File::create(p)?, Compression::default()))
                                        } else {
                                            // Return plain file reader
                                            Box::new(File::create(p)?)
                                        };

        let mut writer = csv::WriterBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(self.has_headers)
            .quote(self.quote)
            .terminator(self.terminator)
            .from_writer(file_writer);

        let header = df.columns().map(|v| v.to_string()).collect::<Vec<String>>();

        // Deserialize all series into string vecs
        let mut data = vec![];
        for col_name in df.data.keys() {
            let series_container = df.get_column_infer(col_name.as_str()).unwrap();
            let string_vec = series_container.into_string_vec();
            data.push(string_vec);
        }

        // User might not want to write out headers
        if self.has_headers {
            // Write out records
            writer.write_record(header.as_slice())?;
        };

        // TODO: Probably a better way to do this?
        for row_idx in 0..data[0].len() {
            let mut row = vec![];
            for column_idx in 0..data.len() {
                row.push(&data[column_idx][row_idx]);
            }
            writer.write_record(row.as_slice())?;
        }

        Ok(())
    }
}