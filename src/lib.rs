#![warn(missing_docs)]
//! BlackJack strives to be a full featured crate for general data processing.
//! 
//! _Long term_ goal is to create a lightweight [Pandas](https://pandas.pydata.org/) equivalent
//! by and for the Rust community, but with slight differences in focus...
//!
//! The project strives for a few key principles. When any implementation decisions are to be made,
//! they are made with these principles in mind, and in this order:
//! 1. **Memory efficiency**
//!     - Minimize memory use at every opportunity.
//! 2. **Usability**
//!     - Strive for ergonomics; often done by modeling the `Pandas` API where possible.
//! 3. **Speedy**
//!     - It comes naturally most times with Rust. :)
//!
//! Eventually, a Python wrapper: [Lumber-Jack](https://github.com/milesgranger/lumber-jack)
//! associated with this crate, but that time will come.
//! 
//! # Example use:
//! 
//! ```
//! use blackjack::prelude::*;
//!
//! // We have a dataframe, of course...
//! let mut df = DataFrame::new();
//!
//! // Make some series, of different types
//! let series_i32: Series<i32> = Series::arange(0, 5);
//! let mut series_f64: Series<f64> = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
//! 
//! // You can set a series name!
//! series_f64.set_name("my-series");
//! 
//! // Or not... 
//! assert_eq!(series_i32.name(), None);
//!
//! // And add them to the dataframe
//! df.add_column(series_f64).unwrap();
//! df.add_column(series_i32).unwrap();
//!
//! // And then get a reference to a Series
//! let series_f64_ref: &Series<f64> = df.get_column("my-series").unwrap();
//!
//! // and a lot more...
//! ```

extern crate num;
extern crate csv;
extern crate stats;
extern crate flate2;
extern crate rgsl;
extern crate snap;
extern crate rayon;
extern crate itertools;
extern crate prettytable;
extern crate indexmap;
extern crate ndarray;
extern crate bincode;
extern crate serde;
extern crate derive_more;
extern crate baggie;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;


#[macro_use] pub mod macros;

pub mod series;
pub mod dataframe;
pub mod prelude;
pub mod enums;
pub mod traits;
pub mod error;
mod funcs;
