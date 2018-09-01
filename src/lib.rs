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
//! let series_i32: Series<i32> = Series::arange(0, 5);
//! let mut series_f64: Series<f64> = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
//! 
//! // You can set a series name!
//! series_f64.set_name("my-float-series");
//! 
//! // Or not... 
//! assert_eq!(series_i32.name(), None);
//! 
//! // Add columns (of different types) to a dataframe
//! df.add_column(series_i32);
//! df.add_column(series_f64);
//! 
//! ```

extern crate ndarray;
extern crate num;

#[macro_use] pub mod macros;

pub mod series;
pub mod dataframe;
pub mod prelude;
pub mod enums;
pub mod traits;
