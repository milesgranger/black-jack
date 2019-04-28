//! Mostly internal level macros for implementing ops per series type

/// Implement `IntoIter` for a dtype (ie. f64) for `Series`
#[macro_export]
macro_rules! impl_series_into_iter {
    // Use: impl_series_into_iter(i32)
    ($primitive:ty) => {
        impl IntoIterator for Series<$primitive> {
            type Item = $primitive;
            type IntoIter = IntoIter<$primitive>;

            fn into_iter(self) -> Self::IntoIter {
                self.values.into_iter()
            }
        }
    };
}

/// Implement various inplace numeric operations for a Series
/// ie. `series += 1`
#[macro_export]
macro_rules! impl_series_by_series_op_inplace {

    // Use: impl_series_by_series_op_inplace!(MulAssign, mul_assign, *=)
    ($operation:ident, $func_name:ident, $op:tt) => {
        impl<T> $operation<Series<T>> for Series<T>
            where T: BlackJackData + $operation
        {
            fn $func_name(&mut self, other: Series<T>) {
                let _ = self.values
                    .iter_mut()
                    .zip(other.values.into_iter())
                    .map(|(v, o)| *v $op o)
                    .collect::<Vec<()>>();
            }
        }
    }
}

/// Implement series by series operations
/// ie. `series1 + series2`
#[macro_export]
macro_rules! impl_series_by_series_op {

    // Use: impl_series_by_series_op(Add, add, +)
    ($operation:ident, $func_name:ident, $op:tt) => {

        /// Support `series + series`
        impl<T> $operation for Series<T>
            where
                T: $operation<Output=T> + BlackJackData,
        {
            type Output = Result<Series<T>, BlackJackError>;

            fn $func_name(self, other: Series<T>) -> Self::Output {
                if self.len() != other.len() {
                    Err(BlackJackError::ValueError(
                        format!("Source series is of size: {}, and other is of size: {}", &self.len(), &other.len())
                    ))
                } else {
                    let result = self.values
                        .into_iter()
                        .zip(other.values.into_iter())
                        .map(|(x1, x2)| x1 $op x2)
                        .collect();
                    Ok(Series::from_vec(result))
                }


            }
        }

    }
}
