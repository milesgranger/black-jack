//! 
//! Module holds overloading implementations for [`Series`].
//! 

use std::marker::Send;
use std::ops::{Mul, Add, Sub, Div, MulAssign, AddAssign, SubAssign, DivAssign};

use rayon::prelude::*;
use rayon::iter::{IntoParallelIterator, IndexedParallelIterator};
use num::*;

use prelude::*;


/// Support `series * scalar`
impl<T> Mul<T> for Series<T>
    where 
        T: Num + Copy + BlackJackData + Send + Sync,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    type Output = Series<T>;

    fn mul(self, scalar_val: T) -> Series<T> {
        let vec: Vec<T> = self.values
            .into_par_iter()
            .map(|v| v * scalar_val)
            .collect();
        Series::from_vec(vec)
    }
}

impl<T> MulAssign<T> for Series<T>
    where 
        T: Num + Copy + BlackJackData + Send + Sync + MulAssign<T>,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    fn mul_assign(&mut self, scalar_val: T) -> () {
        self.values
            .par_iter_mut()
            .map(|v| *v *= scalar_val)
            .collect::<Vec<()>>();
    }
}

// Support `series + other_series` ect.
impl_series_by_series_op!(Add, add, +);
impl_series_by_series_op!(Sub, sub, -);
impl_series_by_series_op!(Div, div, /);
impl_series_by_series_op!(Mul, mul, *);

// Support `series += other_series`
impl_series_by_series_op_inplace!(MulAssign, mul_assign, *=);
impl_series_by_series_op_inplace!(DivAssign, div_assign, /=);
impl_series_by_series_op_inplace!(AddAssign, add_assign, +=);
impl_series_by_series_op_inplace!(SubAssign, sub_assign, -=);

/// Support `series + scalar`
impl<T> Add<T> for Series<T>
    where 
        T: Num + Copy + BlackJackData + Send + Sync,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    type Output = Series<T>;

    fn add(self, scalar_val: T) -> Series<T> {
        let vec: Vec<T> = self.values
            .into_par_iter()
            .map(|v| v + scalar_val)
            .collect();
        Series::from_vec(vec)
    }
}

/// Support `series += scalar`
impl<T> AddAssign<T> for Series<T>
    where 
        T: Num + Copy + BlackJackData + Send + Sync + FromPrimitive + AddAssign<T>,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    fn add_assign(&mut self, scalar_val: T) -> () {
        self.values.par_iter_mut()
                    .map(|v| *v += scalar_val)
                    .collect::<Vec<()>>();
    }
}

/// Support `series - scalar`
impl<T> Sub<T> for Series<T>
    where 
        T: Num + Copy + BlackJackData + Send + Sync + Sub,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    type Output = Series<T>;

    fn sub(self, scalar_val: T) -> Series<T> {
        let vec: Vec<T> = self.values.into_par_iter()
                                                .map(|v| v - scalar_val)
                                                .collect();
        Series::from_vec(vec)
    }
}

/// Support `series -= scalar`
impl<T> SubAssign<T> for Series<T>
    where 
        T: Num + Copy + BlackJackData + Send + Sync + SubAssign<T>,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    fn sub_assign(&mut self, scalar_val: T) -> () {
        self.values.par_iter_mut()
                    .map(|v| *v -= scalar_val)
                    .collect::<Vec<()>>();
    }
}

/// Support `series - scalar`
impl<T> Div<T> for Series<T>
    where 
        T: Num + Copy + BlackJackData + Send + Sync,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    type Output = Series<T>;

    fn div(self, scalar_val: T) -> Series<T> {
        let vec: Vec<T> = self.values.into_par_iter()
                                                .map(|v| v / scalar_val)
                                                .collect();
        Series::from_vec(vec)
    }
}

/// Support `series += scalar`
impl<T> DivAssign<T> for Series<T>
    where 
        T: Num + Copy + BlackJackData + Send + Sync + DivAssign<T>,
        Vec<T>: IntoParallelIterator<Item=T>,
        <Vec<T> as IntoParallelIterator>::Iter: IndexedParallelIterator
{
    fn div_assign(&mut self, scalar_val: T) -> () {
        self.values.par_iter_mut()
                    .map(|v| *v /= scalar_val)
                    .collect::<Vec<()>>();
    }
}
