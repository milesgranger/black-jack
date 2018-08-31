//! Traits to be used throughout the crate

use prelude::*;
use std::fmt::Debug;

/// Trait dictates the supported primitives for use in [Series](struct.Series.html) structs.
pub trait BlackJackData: Debug + 'static {

    /// Return the current [DType](enum.DType.html) for this type. 
    fn dtype(&self) -> DType;
}
impl BlackJackData for f64 {
    fn dtype(&self) -> DType { DType::F64 }
}
impl BlackJackData for i64 {
    fn dtype(&self) -> DType { DType::I64 }
}
impl BlackJackData for f32 {
    fn dtype(&self) -> DType { DType::F32 }
}
impl BlackJackData for i32 {
    fn dtype(&self) -> DType { DType::I32 }
}
