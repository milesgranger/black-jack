//! Enums to be used throughout the crate.

use std::string::ToString;
use num::*;
use prelude::*;
pub mod overloaders;

/// Possible DType returns, matches [`BlackJackData`]
#[derive(Debug, PartialEq, Clone)]
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

    /// `None`
    None
}
