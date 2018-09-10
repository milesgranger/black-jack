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
macro_rules! impl_FROM_DataElement_for_primitive {

    // Use: impl_DataElement_for_primitive!(f32)
    // Use: impl_DataElement_for_primitive!(&mut f32)
    // NOT to be used 'for String' impls.

    ($primitive:ty) => {
        impl From<DataElement> for $primitive {
            fn from(val: DataElement) -> Self {
                match val {
                    DataElement::I64(v) => v as $primitive,
                    DataElement::F64(v) => v as $primitive,
                    DataElement::I32(v) => v as $primitive,
                    DataElement::F32(v) => v as $primitive,
                    DataElement::STRING(v) => {
                        let nan: f64 = Float::nan();
                        v.parse::<$primitive>()
                            .unwrap_or(nan as $primitive)
                    }
                }    
            }
        }
    };

    (ref mut $primitive:ty) => {
        impl<'a> From<&'a mut DataElement> for $primitive {
            fn from(val: &mut DataElement) -> Self {
                match val {
                    DataElement::F64(v) => *v as $primitive,
                    DataElement::I64(v) => *v as $primitive,
                    DataElement::F32(v) => *v as $primitive,
                    DataElement::I32(v) => *v as $primitive,
                    DataElement::STRING(v) => {
                        let nan: f64 = Float::nan();
                        v.parse::<$primitive>()
                            .unwrap_or(nan as $primitive)
                    }
                }
            }
        }
    }

}
