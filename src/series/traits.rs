//! Define traits to be used by [Series](struct.Series.html) struct.
//! 
//! 

use std::any::{Any};
use std::fmt::Debug;

use num::*;
use prelude::*;

/// Define the behavior of a Series object.
pub trait SeriesTrait: Debug + Sized + Any {

    /// The primitive associated with this Series; ie. `f64`
    type Item: BlackJackData;

    /// Set the name of a series
    fn set_name(&mut self, name: &str) -> ();

    /// Get the name of the series; Series may not be assigned a string, so an `Option` is returned.
    /// 
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    /// 
    /// let mut series = Series::from_vec(vec![1, 2, 3]);
    /// series.set_name("my-series");
    /// 
    /// assert_eq!(series.name(), Some("my-series".to_string()));
    /// ```
    fn name(&self) -> Option<String>;

    /// Sum a given series, yielding the same type as the elements stored in the series.
    fn sum(&self) -> Self::Item where Self::Item: Num + Clone;

    /// Determine the length of the Series
    fn len(&self) -> usize;

    /// Determine if series is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the dtype
    fn dtype(&self) -> DType;

    /// As boxed pointer, recoverable by `Box::from_raw(ptr)` or `SeriesTrait::from_raw(*mut Self)`
    fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    /// Create from raw pointer
    fn from_raw(ptr: *mut Self) -> Self {
        let obj = unsafe { Box::from_raw(ptr) };
        *obj
    }
}