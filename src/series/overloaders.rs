//! 
//! Module holds overloading implementations for [`Series`].
//! 

use std::ops::{Mul, Add, Sub, Div, MulAssign, AddAssign, SubAssign, DivAssign};
use num::*;
use prelude::*;


/// Support `series * scalar`
impl<T> Mul<T> for Series 
    where T: Num + Copy + From<DataElement> + BlackJackData
{
    type Output = Series;

    fn mul(self, scalar_val: T) -> Series {
        let vec: Vec<DataElement> = self.values.into_iter()
                                                .map(|v| v * scalar_val)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

impl<T> MulAssign<T> for Series 
    where 
        T: Num + Copy + From<DataElement> + BlackJackData,
        DataElement: MulAssign<T>
{
    fn mul_assign(&mut self, scalar_val: T) -> () {
        for val in &mut self.values {
            *val *= scalar_val;
        }
    }
}

/// Support `series + scalar`
impl<T> Add<T> for Series 
    where T: Num + Copy + From<DataElement> + BlackJackData
{
    type Output = Series;

    fn add(self, scalar_val: T) -> Series {
        let vec: Vec<DataElement> = self.values.into_iter()
                                                .map(|v| v + scalar_val)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series += scalar`
impl<T> AddAssign<T> for Series
    where 
        T: Num + Copy + From<DataElement> + BlackJackData,
        DataElement: AddAssign<T>
{
    fn add_assign(&mut self, scalar_val: T) -> () {
        for val in &mut self.values {
            *val += scalar_val;
        }
    }
}

/// Support `series - scalar`
impl<T> Sub<T> for Series 
    where T: Num + Copy + From<DataElement> + BlackJackData
{
    type Output = Series;

    fn sub(self, scalar_val: T) -> Series {
        let vec: Vec<DataElement> = self.values.into_iter()
                                                .map(|v| v - scalar_val)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series -= scalar`
impl<T> SubAssign<T> for Series
    where 
        T: Num + Copy + From<DataElement> + BlackJackData,
        DataElement: SubAssign<T>
{
    fn sub_assign(&mut self, scalar_val: T) -> () {
        for val in &mut self.values {
            *val -= scalar_val;
        }
    }
}

/// Support `series - scalar`
impl<T> Div<T> for Series 
    where T: Num + Copy + From<DataElement> + BlackJackData
{
    type Output = Series;

    fn div(self, scalar_val: T) -> Series {
        let vec: Vec<DataElement> = self.values.into_iter()
                                                .map(|v| v / scalar_val)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series += scalar`
impl<T> DivAssign<T> for Series
    where 
        T: Num + Copy + From<DataElement> + BlackJackData,
        DataElement: DivAssign<T>
{
    fn div_assign(&mut self, scalar_val: T) -> () {
        for val in &mut self.values {
            *val /= scalar_val;
        }
    }
}
