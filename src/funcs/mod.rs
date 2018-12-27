//! Functions / processors on slices with a goal of being memory efficient & fast.
//! ...in that order.
use std::iter::Sum;
use std::cmp::Ordering;
use num::*;
use num::traits::Pow;


/// Calculate the variance where `ddof` is either 0_f64 or 1_f64 for population or sample variance.
pub fn variance<T>(values: &[T], ddof: f64) -> Option<f64>
    where T: Num + ToPrimitive
{
    let m = mean(&values)?;
    let numerator = values
        .iter()
        .map(|v| (v.to_f64().unwrap() - m).pow(2.))
        .sum::<f64>();
    Some(numerator / (values.len() as f64 - ddof))
}

/// Calculate the standard deviation where
/// `ddof` is either 0_f64 or 1_f64 for population or sample variance.
pub fn std<T>(values: &[T], ddof: f64) -> Option<f64>
    where T: Num + ToPrimitive
{
    let var = variance(&values, ddof)?;
    Some(var.sqrt())
}

/// Calculate mean / average
pub fn mean<T>(values: &[T]) -> Option<f64>
    where T: Num + ToPrimitive
{
    Some(values.iter().map(|v| v.to_f64().unwrap()).sum::<f64>() / values.len() as f64)
}

/// Calculate sum
pub fn sum<T>(values: &[T]) -> T
    where T: Num + Copy + Sum
{
    values
        .iter()
        .map(|v| *v)
        .sum()
}

/// Calculate min
pub fn min<T>(values: &[T]) -> Option<&T>
    where T: Num + PartialOrd + Copy
{
    values.iter()
        .min_by(|a, b| {
            match a.partial_cmp(b) {
                Some(Ordering::Less) => Ordering::Less,
                Some(Ordering::Greater) => Ordering::Greater,
                Some(Ordering::Equal) => Ordering::Equal,
                None => Ordering::Equal
            }
    })
}

/// Calculate max
pub fn max<T>(values: &[T]) -> Option<&T>
    where T: Num + PartialOrd + Copy
{
    values.iter()
        .min_by(|a, b| {
            match b.partial_cmp(a) {
                Some(Ordering::Less) => Ordering::Less,
                Some(Ordering::Greater) => Ordering::Greater,
                Some(Ordering::Equal) => Ordering::Equal,
                None => Ordering::Equal
            }
    })
}
