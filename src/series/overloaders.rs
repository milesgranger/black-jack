//! 
//! Module holds overloading implementations for [`Series`].
//! 

use std::ops::{Mul, Add, Sub, Div};
use num::*;
use prelude::*;


/// Support `series * scalar`
impl<T> Mul<T> for Series 
    where T: Num + Copy + From<DataElement> + BlackJackData
{
    type Output = Series;

    fn mul(self, scalar_val: T) -> Series {
        let vec: Vec<DataElement> = self.values.into_iter()
                                                // Generic params ensure no error;
                                                // reason for unwrap()
                                                .map(|v| v * scalar_val)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series + scalar`
impl<T> Add<T> for Series 
    where T: Num + Copy + From<DataElement> + BlackJackData
{
    type Output = Series;

    fn add(self, scalar_val: T) -> Series {
        let vec: Vec<DataElement> = self.values.into_iter()
                                                // Generic params ensure no error;
                                                // reason for unwrap()
                                                .map(|v| v + scalar_val)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series - scalar`
impl<T> Sub<T> for Series 
    where T: Num + Copy + From<DataElement> + BlackJackData
{
    type Output = Series;

    fn sub(self, scalar_val: T) -> Series {
        let vec: Vec<DataElement> = self.values.into_iter()
                                                // Generic params ensure no error;
                                                // reason for unwrap()
                                                .map(|v| v - scalar_val)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series - scalar`
impl<T> Div<T> for Series 
    where T: Num + Copy + From<DataElement> + BlackJackData
{
    type Output = Series;

    fn div(self, scalar_val: T) -> Series {
        let vec: Vec<DataElement> = self.values.into_iter()
                                                // Generic params ensure no error;
                                                // reason for unwrap()
                                                .map(|v| v / scalar_val)
                                                .collect();
        Series::from_data_elements(vec)
    }
}
