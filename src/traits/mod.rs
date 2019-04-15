//! Traits to be used throughout the crate

use std::fmt::{Debug, Display};

use serde::Serialize;

use crate::prelude::*;

/// Trait dictates the supported primitives for use in [`Series`] structs.
pub trait BlackJackData: Serialize + Debug + ToString + Clone + Send + Display {
    /// Return the current [`DType`] for this type.
    fn dtype(&self) -> DType;
}
impl BlackJackData for f64 {
    fn dtype(&self) -> DType {
        DType::F64
    }
}
impl BlackJackData for i64 {
    fn dtype(&self) -> DType {
        DType::I64
    }
}
impl BlackJackData for f32 {
    fn dtype(&self) -> DType {
        DType::F32
    }
}
impl BlackJackData for i32 {
    fn dtype(&self) -> DType {
        DType::I32
    }
}
impl BlackJackData for String {
    fn dtype(&self) -> DType {
        DType::STRING
    }
}
