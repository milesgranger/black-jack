//! The common Error(s) and associated implementations used in within the crate

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
    ValueError(String),

    /// Length mismatch
    #[fail(display = "LengthMismatch")]
    LengthMismatch(String),
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
