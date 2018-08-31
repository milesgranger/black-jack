#![warn(missing_docs)]
//! This black-jack crate is meant as a Rust backend for the Python package "lumber-jack"
//! 
//! Its main intention is to have a Python wrapper, but feel free
//! to make use of it in other settings!
//! 
//! # Examples
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
