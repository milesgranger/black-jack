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
//! let mut df = DataFrame::new();
//! let series_i32: Series<i32> = Series::arange(0, 5);
//! let series_f64: Series<f64> = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
//! 
//! df.add_column(series_i32);
//! df.add_column(series_f64);
//! 
//! ```

extern crate ndarray;
extern crate num;
extern crate libc;
extern crate num_traits;

#[macro_use]
pub mod macros;

pub mod series;
pub mod dataframe;
pub mod prelude;
