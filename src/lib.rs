#![warn(missing_docs)]
//! Black-Jack is under development, and not meant to be used in any real capacity. 
//! 
//! The (_long term_) goal is to create a lightweight [Pandas](https://pandas.pydata.org/) equivelent 
//! by and for the Rust community. Along with a Python wrapper: [Lumber-Jack](https://github.com/milesgranger/lumber-jack)
//! 
//! # Example use:
//! 
//! ```
//! use blackjack::prelude::*;
//! 
//! // Dataframe; container for series objects.
//! let mut df = DataFrame::new();
//! 
//! // Make some series
//! let series_i32: Series = Series::arange(0, 5);
//! let mut series_f64: Series = Series::from_vec(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
//! 
//! // You can set a series name!
//! series_f64.set_name("my-float-series");
//! 
//! // Or not... 
//! assert_eq!(series_i32.name(), None);
//! 
//! // Add columns (of different types) to a dataframe
//! // .add_column() -> Result<(), String>; `series` must be the same length as `df`
//! assert!(df.add_column(series_i32).is_ok());
//! assert!(df.add_column(series_f64).is_ok());
//! 
//! // Get columns by either method or indexing it
//! let series_ref: Option<&Series> = df.get_column("my-float-series");
//! let series_ref: &Series = &df["my-float-series"];  // panics if series name does not exist in dataframe!
//! ```

extern crate num;
extern crate csv;
extern crate stats;
extern crate flate2;
extern crate rgsl;
extern crate rayon;
extern crate itertools;
extern crate prettytable;
extern crate indexmap;
extern crate ndarray;
extern crate bincode;
#[macro_use] extern crate serde;
#[macro_use] extern crate derive_more;
#[macro_use] extern crate serde_derive;


#[macro_use] pub mod macros;

pub mod series;
pub mod dataframe;
pub mod prelude;
pub mod enums;
pub mod traits;
