//! `.rolling()` functionality for `Series`

use std::iter::Sum;
use std::marker::{Send, Sync};

use num::*;
use ndarray::aview1 as arrayview;
use rayon::prelude::*;

use crate::prelude::*;
use crate::funcs;


/// Struct for calculating rolling aggregations
pub struct Rolling<'a, T>
    where T: BlackJackData + Send + Sync
{
    window: usize,
    series: &'a Series<T>,
    nans: Vec<f64>
}

// TODO: These impls need to be refactored (DRY) - lots of repeated code
impl<'a, T> Rolling<'a, T>
    where T: BlackJackData + Send + Sync
{

    /// Create a new `Rolling` instance from a given window and Series reference.
    pub fn new(window: usize, series: &'a Series<T>) -> Self {
        let nans: Vec<f64> = (0..window - 1)
            .into_iter()
            .map(|_| Float::nan())
            .collect();
        Rolling { window, series, nans }
    }

    /// Calculate a rolling mean from the current instance.
    pub fn mean(&self) -> Result<Series<f64>, BlackJackError>
        where T: Sum + Num + ToPrimitive + Copy,
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals = self.nans.clone();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                let view = arrayview(&self.series.values[idx..idx + self.window]);
                match view.sum().to_f64() {
                    Some(d) => Ok(d / view.len() as f64),
                    None => Err(BlackJackError::from("Unable to cast windowed sum to f64."))
                }
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }

    /// Calculate a rolling sum from the current instance.
    pub fn sum(&self) -> Result<Series<f64>, BlackJackError>
        where T: Sum + Num + ToPrimitive + Copy,
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals = self.nans.clone();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                let view = arrayview(&self.series.values[idx..idx + self.window]);
                match view.sum().to_f64() {
                    Some(s) => Ok(s),
                    None => Err(BlackJackError::from("Unable to cast windowed sum to f64."))
                }
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }

    /// Calculate a rolling variance from the current instance, using either population or sample variance
    /// > Population: `ddof` == 0_f64
    /// > Sample: `ddof` == 1_f64
    pub fn var(&self, ddof: f64) -> Result<Series<f64>, BlackJackError>
        where T: Num + ToPrimitive
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals = self.nans.clone();

        // Calculate the remaining valid windows
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                match funcs::variance(&self.series.values[idx..idx + self.window], ddof) {
                    Some(var) => Ok(var),
                    None => Err(BlackJackError::from("Failed to calculate variance for window"))
                }
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }

    /// Calculate the rolling standard deviation for each window,
    /// using either population or sample variance
    /// > Population: `ddof` == 0_f64
    /// > Sample: `ddof` == 1_f64
    pub fn std(&self, ddof: f64) -> Result<Series<f64>, BlackJackError>
        where T: Num + ToPrimitive + Copy
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals = self.nans.clone();

        // Calculate the remaining valid windows
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                match funcs::std(&self.series.values[idx..idx + self.window], ddof) {
                    Some(std) => Ok(std),
                    None => Err(BlackJackError::from("Failed to calculate standard deviation for window"))
                }
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }

    /// Calculate a rolling median from the current instance.
    pub fn median(&self) -> Result<Series<f64>, BlackJackError>
        where T: PartialOrd + Num + ToPrimitive + Copy,
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals = self.nans.clone();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_par_iter()
            .map(|idx| {
                match stats::median(self.series.values[idx..idx + self.window].iter().map(|v| *v)) {
                    Some(med) => Ok(med),
                    None => Err(BlackJackError::from("Failed to compute median for window"))
                }
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }

    /// Calculate a rolling min from the current instance.
    pub fn min(&self) -> Result<Series<f64>, BlackJackError>
        where T: Num + PartialOrd + Copy + ToPrimitive,
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals = self.nans.clone();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                match funcs::min(&self.series.values[idx..idx + self.window]) {
                    Some(min) => Ok(min.to_f64().unwrap()),
                    None => Err(BlackJackError::from("Failed to calculate min for window"))
                }
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }

    /// Calculate a rolling min from the current instance.
    pub fn max(&self) -> Result<Series<f64>, BlackJackError>
        where T: PartialOrd + Num + ToPrimitive + Copy,
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals = self.nans.clone();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                match funcs::max(&self.series.values[idx..idx + self.window]) {
                    Some(max) => Ok(max.to_f64().unwrap()),
                    None => Err(BlackJackError::from("Failed to calculate min for window"))
                }
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }
}
