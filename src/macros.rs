//! Nothing to see here, placeholder until new macros are needed.

#[macro_export]
macro_rules! series_map {

    ($data:expr, $function:tt) => {
        {
            println!("Function: {:?}", $function);
        }
    }

}
