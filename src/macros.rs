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

    ($primitive:ty) => {
            impl From<DataElement> for $primitive {
                fn from(val: DataElement) -> Self {
                    match val {
                        DataElement::I64(v) => v as $primitive,
                        DataElement::F64(v) => v as $primitive,
                        DataElement::I32(v) => v as $primitive,
                        DataElement::F32(v) => v as $primitive,
                        _ => panic!("Unable to implement From<DataElement>")
                    }    
                }
            }
        
    }

}
