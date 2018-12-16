//! Nothing to see here, placeholder until new macros are needed.

#[macro_export]
macro_rules! series_map {

    ($data:expr, $function:tt) => {
        {
            println!("Function: {:?}", $function);
        }
    }

}

#[macro_export]
macro_rules! impl_series_into_iter {
    // Use: impl_series_into_iter(i32)
    ($primitive:ty) => {

        impl IntoIterator for Series<$primitive> {
            type Item =  $primitive;
            type IntoIter = IntoIter<$primitive>;

            fn into_iter(self) -> Self::IntoIter {
                self.values.into_iter()
            }
        }

    }
}
