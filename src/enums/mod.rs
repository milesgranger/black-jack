//! Enums to be used throughout the crate.

use crate::prelude::*;

/// Possible DType returns, matches [`BlackJackData`]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, PartialOrd)]
pub enum DType {
    /// `f64`
    F64,

    /// `i64`
    I64,

    /// `f32`
    F32,

    /// `i32`
    I32,

    /// `String`
    STRING,
}

/// Container for use with `Row` struct
#[derive(PartialEq)]
pub enum Datum<'a> {
    F64(&'a f64),
    I64(&'a i64),
    F32(&'a f32),
    I32(&'a i32),
    STR(&'a String),
}

pub enum Column {
    F64(Series<f64>),
    I64(Series<i64>),
    F32(Series<f32>),
    I32(Series<i32>),
    STR(Series<String>),
}
