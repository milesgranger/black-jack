//! Enums to be used throughout the crate.

use std::string::ToString;
use std::ops::{Mul, Add, Sub, Div};
use num::*;
use prelude::*;

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
    STRING
}

// Implement `From<DataElement>` for each supported primitive
impl_FROM_DataElement_for_primitive!(f64);
impl_FROM_DataElement_for_primitive!(i64);
impl_FROM_DataElement_for_primitive!(i32);
impl_FROM_DataElement_for_primitive!(f32);

impl_FROM_DataElement_for_primitive!(ref mut f64);
impl_FROM_DataElement_for_primitive!(ref mut i64);
impl_FROM_DataElement_for_primitive!(ref mut i32);
impl_FROM_DataElement_for_primitive!(ref mut f32);


impl From<DataElement> for String {
    fn from(val: DataElement) -> Self {
        match val {
            DataElement::F64(v) => v.to_string(),
            DataElement::I64(v) => v.to_string(),
            DataElement::F32(v) => v.to_string(),
            DataElement::I32(v) => v.to_string(),
            DataElement::STRING(v) => v.clone()
        }
    }
}

impl<'a> From<&'a mut DataElement> for String {
    fn from(val: &mut DataElement) -> Self {
        match val {
            DataElement::F64(v) => v.to_string(),
            DataElement::I64(v) => v.to_string(),
            DataElement::F32(v) => v.to_string(),
            DataElement::I32(v) => v.to_string(),
            DataElement::STRING(v) => v.clone()
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
    // TODO: Add support for parsing f32 vs f64 and etc.
    pub fn from_parse<S: Into<String>>(val: S) -> DataElement {

        let val: String = val.into();

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

    /// Get the current [`DType`]
    pub fn dtype(&self) -> DType {
        match self {
            DataElement::I64(_) => DType::I64,
            DataElement::F64(_) => DType::F64,
            DataElement::I32(_) => DType::I32,
            DataElement::F32(_) => DType::F32,
            DataElement::STRING(_) => DType::STRING
        }
    }

    /// Determine if the `DataElement` holds an `NaN` value
    pub fn is_nan(&self) -> bool {
        match self {
            DataElement::F64(v) => v.is_nan(),
            DataElement::F32(v) => v.is_nan(),
            _ => false // Integers and Strings not allowed to be NaN
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

impl<T> Mul<T> for DataElement 
    where 
        T: From<DataElement> + Mul,
        <T as Mul>::Output: BlackJackData
{
    type Output = DataElement;

    fn mul(self, rhs: T) -> DataElement {
        (T::from(self) * rhs).into()
    }
}               


impl<T> Add<T> for DataElement 
    where 
        T: From<DataElement> + Add,
        <T as Add>::Output: BlackJackData
{
    type Output = DataElement;

    fn add(self, rhs: T) -> DataElement {
        (T::from(self) + rhs).into()
    }
}


impl<T> Sub<T> for DataElement 
    where 
        T: From<DataElement> + Sub,
        <T as Sub>::Output: BlackJackData
{
    type Output = DataElement;

    fn sub(self, rhs: T) -> DataElement {
        (T::from(self) - rhs).into()
    }
}

impl<T> Div<T> for DataElement 
    where 
        T: From<DataElement> + Div,
        <T as Div>::Output: BlackJackData
{
    type Output = DataElement;

    fn div(self, rhs: T) -> DataElement {
        (T::from(self) / rhs).into()
    }
}