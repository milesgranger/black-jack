//! Enums to be used throughout the crate.

use std::string::ToString;
use num::*;
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
    I32,

    /// `String`
    STRING
}

// Implement `From<DataElement>` for each supported primitive
impl_FROM_DataElement_for_primitive!(f64);
impl_FROM_DataElement_for_primitive!(i64);
impl_FROM_DataElement_for_primitive!(i32);
impl_FROM_DataElement_for_primitive!(f32);

impl From<DataElement> for String {
    fn from(val: DataElement) -> Self {
        if let DataElement::STRING(string) = val {
            string
        } else {
            panic!("Value is not a string!")
        }
    }
}

/// Enum to represent all supported data elements, 
/// and should match [`BlackJackData`] elements.
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum DataElement {

    /// i64 support
    I64(i64),

    /// f64 support
    F64(f64),

    /// i32 support
    I32(i32),

    /// f32 support
    F32(f32),

    /// String support
    STRING(String)
}

impl DataElement {

    /// Get a [`DataElement`] from a `&str` value.
    pub fn from_parse(val: &str) -> DataElement {
        
        // Attempt i64
        match val.parse::<i64>().ok() {
            Some(v) => DataElement::from(v),

            // Attempt for f64
            None => match val.parse::<f64>().ok() {
                Some(v) => DataElement::from(v),

                // Attempt for String
                None => match val.parse::<String>().ok() {
                    Some(v) => DataElement::from(v),
                    None => panic!("Unable to parse value!")
                }
            }
        }
    }

}


impl<T: BlackJackData + ToString> From<T> for DataElement {
    fn from(val: T) -> Self {
        match val.dtype() {
            DType::I64 => DataElement::I64(val.to_string().parse::<i64>().unwrap_or_else(|_| panic!("Unable to convert value to i64"))),
            DType::F64 => DataElement::F64(val.to_string().parse::<f64>().unwrap_or_else(|_| panic!("Unable to convert value to f64"))),
            DType::I32 => DataElement::I32(val.to_string().parse::<i32>().unwrap_or_else(|_| panic!("Unable to convert value to i32"))),
            DType::F32 => DataElement::F32(val.to_string().parse::<f32>().unwrap_or_else(|_| panic!("Unable to convert value to f32"))),
            DType::STRING => DataElement::STRING(val.to_string())
        }
    }
}