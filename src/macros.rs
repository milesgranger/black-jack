#[macro_export]
macro_rules! series_map {

    ($data:expr, $function:tt) => {
        {
            println!("Function: {:?}", $function);
        }
    }

}


#[macro_export]
macro_rules! operate_on_vec_by_scalar {

    // Multiply a vector inplace
    // ie: operate_on_vec_by_scalar(inplace vec, +, 2.0);
    (inplace $data:expr, $op:tt, $scalar:expr) => {

        {

            use containers::{Data};
            use std::any::Any;

            // Get vec which can contain any primitive
            // ...and perform an op on it by consuming it

            // If scalar is f64...
            if let Some(scalar) = (&$scalar as &Any).downcast_ref::<f64>() {

                match $data {
                    Data::Float64(vec) => Data::Float64(vec.into_iter().map(|v| v $op *scalar).collect()),
                    Data::Int32(vec) => Data::Float64(vec.into_iter().map(|v| v as f64 $op *scalar).collect())
                }

            // If scalar is i32...
            } else if let Some(scalar) = (&$scalar as &Any).downcast_ref::<i32>() {

                match $data {
                    Data::Float64(vec) => Data::Float64(vec.into_iter().map(|v| v $op *scalar as f64).collect()),
                    Data::Int32(vec) => Data::Int32(vec.into_iter().map(|v| v $op scalar).collect())
                }

            } else {
                panic!("Unexpected Scalar type!")
            }
        }
    };

    (!inplace $data:expr, $op:tt, $scalar:expr) => {

        {
            use containers::{Data};
            use std::any::Any;

            // Get vec which can contain any primitive
            // ...and perform an inplace op on it by scalar which can be any primitive.

            // If scalar is f64...
            if let Some(scalar) = (&$scalar as &Any).downcast_ref::<f64>() {

                match $data {
                    Data::Float64(ref vec) => Data::Float64(vec.iter().map(|v| *v $op *scalar).collect()),
                    Data::Int32(ref vec) => Data::Float64(vec.iter().map(|v| *v as f64 $op *scalar).collect())
                }

            // If scalar is i32...
            } else if let Some(scalar) = (&$scalar as &Any).downcast_ref::<i32>() {

                match $data {
                    Data::Float64(ref vec) => Data::Float64(vec.iter().map(|v| *v $op *scalar as f64).collect()),
                    Data::Int32(ref vec) => Data::Int32(vec.iter().map(|v| *v $op scalar).collect())
                }

            } else {
                panic!("Unexpected Scalar type!")
            }
        }
    }
}
