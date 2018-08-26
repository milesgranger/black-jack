//! Series represents a single column within a dataframe and wraps many `Array` like
//! functionality.
//! 
//! For methods implemented for a `Series`, please check out the trait [SeriesObj](trait.SeriesObj.html)
//! 
//! ## Example use:
//! 
//! ```
//! use blackjack::prelude::*;
//! 
//! let series = Series::arange(0, 5);
//! 
//! assert_eq!(series.sum(), 10);
//! ```

use num::*;
use std::ops::Range;
use std::collections::HashMap;
use std::iter::{FromIterator};
use std::fmt::Debug;
use std::any::{Any, TypeId};

use ndarray::Array1 as Array;

pub trait Series: Debug + Sized + Any {
    type Container: Container<Self>;
}

pub trait Container<T: Debug>: Debug + Any {
    fn new() -> Self
    where
        Self: Sized;

    fn insert(&mut self, value: T);
}

