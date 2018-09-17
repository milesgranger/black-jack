//! 
//! Module holds overloading implementations for [`Series`].
//! 

use std::iter;
use std::marker::Send;
use std::ops::{Mul, Add, Sub, Div, MulAssign, AddAssign, SubAssign, DivAssign};

use rayon::prelude::*;
use rayon::iter::{IntoParallelIterator, IndexedParallelIterator};
use num::*;
use prelude::*;


/// Support `series * scalar`
impl<T> Mul<T> for Series 
    where 
        T: Num + Copy + From<DataElement> + BlackJackData + Send,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    type Output = Series;

    fn mul(self, scalar_val: T) -> Series {
        let vals = iter::repeat(scalar_val).take(self.len()).collect::<Vec<T>>();
        let vec: Vec<DataElement> = self.values.into_par_iter()
                                                .zip(vals)
                                                .map(|(v, s)| v * s)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

impl<T> MulAssign<T> for Series 
    where 
        T: Num + Copy + From<DataElement> + BlackJackData + Send,
        DataElement: MulAssign<T>,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    fn mul_assign(&mut self, scalar_val: T) -> () {

        let vals = iter::repeat(scalar_val).take(self.len()).collect::<Vec<T>>();
        self.values.par_iter_mut()
                    .zip(vals)
                    .map(|(v, s)| *v *= s)
                    .collect::<Vec<()>>();
    }
}

/// Support `series + scalar`
impl<T> Add<T> for Series 
    where 
        T: Num + Copy + From<DataElement> + BlackJackData + Send,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    type Output = Series;

    fn add(self, scalar_val: T) -> Series {
        let vals = iter::repeat(scalar_val).take(self.len()).collect::<Vec<T>>();
        let vec: Vec<DataElement> = self.values.into_par_iter()
                                                .zip(vals)
                                                .map(|(v, s)| v + s)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series += scalar`
impl<T> AddAssign<T> for Series
    where 
        T: Num + Copy + From<DataElement> + BlackJackData + Send + FromPrimitive,
        DataElement: AddAssign<T>,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    fn add_assign(&mut self, scalar_val: T) -> () {

        let vals = iter::repeat(scalar_val).take(self.len()).collect::<Vec<T>>();
        self.values.par_iter_mut()
                    .zip(vals)
                    .map(|(v, s)| *v += s)
                    .collect::<Vec<()>>();
    }
}

/// Support `series - scalar`
impl<T> Sub<T> for Series 
    where 
        T: Num + Copy + From<DataElement> + BlackJackData + Send,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    type Output = Series;

    fn sub(self, scalar_val: T) -> Series {
        let vals = iter::repeat(scalar_val).take(self.len()).collect::<Vec<T>>();
        let vec: Vec<DataElement> = self.values.into_par_iter()
                                                .zip(vals)
                                                .map(|(v, s)| v - s)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series -= scalar`
impl<T> SubAssign<T> for Series
    where 
        T: Num + Copy + From<DataElement> + BlackJackData + Send,
        DataElement: SubAssign<T>,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    fn sub_assign(&mut self, scalar_val: T) -> () {

        let vals = iter::repeat(scalar_val).take(self.len()).collect::<Vec<T>>();
        self.values.par_iter_mut()
                    .zip(vals)
                    .map(|(v, s)| *v -= s)
                    .collect::<Vec<()>>();
    }
}

/// Support `series - scalar`
impl<T> Div<T> for Series 
    where 
        T: Num + Copy + From<DataElement> + BlackJackData + Send,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    type Output = Series;

    fn div(self, scalar_val: T) -> Series {
        let vals = iter::repeat(scalar_val).take(self.len()).collect::<Vec<T>>();
        let vec: Vec<DataElement> = self.values.into_par_iter()
                                                .zip(vals)
                                                .map(|(v, s)| v / s)
                                                .collect();
        Series::from_data_elements(vec)
    }
}

/// Support `series += scalar`
impl<T> DivAssign<T> for Series
    where 
        T: Num + Copy + From<DataElement> + BlackJackData + Send,
        DataElement: DivAssign<T>,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    fn div_assign(&mut self, scalar_val: T) -> () {

        let vals = iter::repeat(scalar_val).take(self.len()).collect::<Vec<T>>();
        self.values.par_iter_mut()
                    .zip(vals)
                    .map(|(v, s)| *v /= s)
                    .collect::<Vec<()>>();
    }
}
