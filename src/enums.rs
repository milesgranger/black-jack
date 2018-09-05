//! Enums to be used throughout the crate.

use prelude::*;

/// Possible DType returns, matches [`BlackJackData`]
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

// Implement `From<DataElement>` for each supported primitive
impl_DataElement_for_primitive!(f64);
impl_DataElement_for_primitive!(i64);
impl_DataElement_for_primitive!(i32);
impl_DataElement_for_primitive!(f32);

/// Enum to represent all supported data elements, 
/// and should match [`BlackJackData`] elements.
#[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
pub enum DataElement {

    /// i64 support
    I64(i64),

    /// f64 support
    F64(f64),

    /// i32 support
    I32(i32),

    /// f32 support
    F32(f32)
}

impl<T: BlackJackData> From<T> for DataElement {
    fn from(val: T) -> Self {
        match val.dtype() {
            DType::I64 => DataElement::I64(val.to_i64().unwrap_or_else(|| panic!("Unable to convert value to i64"))),
            DType::F64 => DataElement::F64(val.to_f64().unwrap_or_else(|| panic!("Unable to convert value to f64"))),
            DType::I32 => DataElement::I32(val.to_i32().unwrap_or_else(|| panic!("Unable to convert value to i32"))),
            DType::F32 => DataElement::F32(val.to_f32().unwrap_or_else(|| panic!("Unable to convert value to f32"))),
        }
    }
}