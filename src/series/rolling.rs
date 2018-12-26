//! `.rolling()` functionality for `Series`

use std::iter::Sum;
use std::marker::{Send, Sync};

use num::*;
use rayon::prelude::*;

use crate::prelude::*;


/// Struct for calculating rolling aggregations
pub struct Rolling<'a, T>
    where T: BlackJackData + Send + Sync
{
    window: usize,
    series: &'a Series<T>
}


impl<'a, T> Rolling<'a, T>
    where T: BlackJackData + Send + Sync
{

    /// Create a new `Rolling` instance from a given window and Series reference.
    pub fn new(window: usize, series: &'a Series<T>) -> Self {
        Rolling { window, series }
    }

    /// Calculate a rolling mean from the current instance.
    pub fn mean(self) -> Result<Series<f64>, BlackJackError>
        where T: Sum + Num + ToPrimitive + Copy
    {

        // Pre-populate the beginning with NaNs up to window index
        let mut vals: Vec<f64> = (0..self.window - 1)
            .into_iter()
            .map(|_| Float::nan())
            .collect();

        // Calculate the remaining valid windows
        vals.extend(
            (0..self.series.len() + 1 - self.window)
            .into_par_iter()
            .map(|idx| Series::from_vec(self.series.values[idx..idx + self.window].to_vec()))
            .map(|s| s.mean())
            .collect::<Result<Vec<f64>, _>>()?
        );
        Ok(Series::from_vec(vals))
    }
}
