//! Enums to be used throughout the crate.

use prelude::*;

/// Possible DType returns, matches [BlackJackData](trait.BlackJackData.html)
#[derive(Debug, PartialEq)]
pub enum DType {

    /// `f64`
    F64,

    /// `i64`
    I64,

    /// `f32`
    F32,

    /// `i32`
    I32
}

/// Enum of all possible series types.
// TODO: Rename to 'SeriesEnum' and include mutable refrences as well as owned Series.
pub enum SeriesEnumRef<'a> {

    /// `&Series<f64>` type
    F64(&'a Series<f64>),

    /// `&Series<i64>` type
    I64(&'a Series<i64>),

    /// `&Series<f32>` type
    F32(&'a Series<f32>),

    /// `&Series<i32>` type
    I32(&'a Series<i32>),
}
