//! 
//! Module holds overloading implementations for [`DataElement`].
//! 
use std::ops::{
    Mul, Add, Sub, Div, 
    MulAssign, AddAssign, SubAssign, DivAssign
};
use num::*;
use prelude::*;


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

impl<T> MulAssign<T> for DataElement 
    where 
        T: BlackJackData + Mul + FromPrimitive,
        <T as Mul>::Output: BlackJackData
{
    fn mul_assign(&mut self, val: T) {
        *self = match *self {
            DataElement::F64(v) => (T::from_f64(v).unwrap() * val).into(),
            DataElement::I64(v) => (T::from_i64(v).unwrap() * val).into(),
            DataElement::F32(v) => (T::from_f32(v).unwrap() * val).into(),
            DataElement::I32(v) => (T::from_i32(v).unwrap() * val).into(),
            _ => {
                let nan: f64 = Float::nan();
                nan.into()
            },
        }
    }
}

impl<T> AddAssign<T> for DataElement 
    where 
        T: BlackJackData + Add + FromPrimitive,
        <T as Add>::Output: BlackJackData
{
    fn add_assign(&mut self, val: T) {
        *self = match *self {
            DataElement::F64(v) => (T::from_f64(v).unwrap() + val).into(),
            DataElement::I64(v) => (T::from_i64(v).unwrap() + val).into(),
            DataElement::F32(v) => (T::from_f32(v).unwrap() + val).into(),
            DataElement::I32(v) => (T::from_i32(v).unwrap() + val).into(),
            _ => {
                let nan: f64 = Float::nan();
                nan.into()
            },
        }
    }
}

impl<T> SubAssign<T> for DataElement 
    where 
        T: BlackJackData + Sub + FromPrimitive,
        <T as Sub>::Output: BlackJackData
{
    fn sub_assign(&mut self, val: T) {
        *self = match *self {
            DataElement::F64(v) => (T::from_f64(v).unwrap() - val).into(),
            DataElement::I64(v) => (T::from_i64(v).unwrap() - val).into(),
            DataElement::F32(v) => (T::from_f32(v).unwrap() - val).into(),
            DataElement::I32(v) => (T::from_i32(v).unwrap() - val).into(),
            _ => {
                let nan: f64 = Float::nan();
                nan.into()
            },
        }
    }
}

impl<T> DivAssign<T> for DataElement 
    where 
        T: BlackJackData + Div + FromPrimitive,
        <T as Div>::Output: BlackJackData
{
    fn div_assign(&mut self, val: T) {
        *self = match *self {
            DataElement::F64(v) => (T::from_f64(v).unwrap() / val).into(),
            DataElement::I64(v) => (T::from_i64(v).unwrap() / val).into(),
            DataElement::F32(v) => (T::from_f32(v).unwrap() / val).into(),
            DataElement::I32(v) => (T::from_i32(v).unwrap() / val).into(),
            _ => {
                let nan: f64 = Float::nan();
                nan.into()
            },
        }
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
