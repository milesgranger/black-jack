//! `.rolling()` functionality for `Series`

use std::iter::Sum;
use std::marker::{Send, Sync};

use num::*;
use ndarray::ArrayView1 as ArrayView;
use rayon::prelude::*;

use crate::prelude::*;


/// Struct for calculating rolling aggregations
pub struct Rolling<'a, T>
    where T: BlackJackData + Send + Sync
{
    window: usize,
    series: &'a Series<T>
}

// TODO: These impls need to be refactored (DRY) - lots of repeated code.
impl<'a, T> Rolling<'a, T>
    where T: BlackJackData + Send + Sync
{

    /// Create a new `Rolling` instance from a given window and Series reference.
    pub fn new(window: usize, series: &'a Series<T>) -> Self {
        Rolling { window, series }
    }

    /// Calculate a rolling mean from the current instance.
    pub fn mean(&self) -> Result<Series<f64>, BlackJackError>
        where T: Sum + Num + ToPrimitive + Copy,
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals: Vec<f64> = (0..self.window - 1)
            .into_iter()
            .map(|_| Float::nan())
            .collect();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                let view = ArrayView::from(&self.series.values[idx..idx + self.window]);
                match view.sum().to_f64() {
                    Some(d) => Ok(d / view.len() as f64),
                    None => Err(BlackJackError::from("Unable to cast windowed sum to f64."))
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
        let mut vals: Vec<f64> = (0..self.window - 1)
            .into_par_iter()
            .map(|_| Float::nan())
            .collect();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                let series = Series::from_vec(self.series.values[idx..idx + self.window].to_vec());
                series.median()
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }

    /// Calculate a rolling min from the current instance.
    pub fn min(&self) -> Result<Series<f64>, BlackJackError>
        where T: PartialOrd + Num + ToPrimitive + Copy,
    {
        // Pre-populate the beginning with NaNs up to window index
        let mut vals: Vec<f64> = (0..self.window - 1)
            .into_par_iter()
            .map(|_| Float::nan())
            .collect();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                let series = Series::from_vec(self.series.values[idx..idx + self.window].to_vec());
                series.min().map(|r| r.to_f64().unwrap())
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
        let mut vals: Vec<f64> = (0..self.window - 1)
            .into_par_iter()
            .map(|_| Float::nan())
            .collect();

        // Calculate the remaining valid windows
        // REMINDER: Using ArrayVeiw and re-implementing .mean() until Series has an ArrayView impl
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_iter()
            .map(|idx| {
                let series = Series::from_vec(self.series.values[idx..idx + self.window].to_vec());
                series.max().map(|r| r.to_f64().unwrap())
            })
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }
}
