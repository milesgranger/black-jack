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
macro_rules! impl_OP_Assign_DataElement {

    // Use: impl_OP_Assign_DataElement!(MulAssign, mul_assign, *=, i64);
    // will implement MulAssign<i64> for DataElement

    ($TRAIT:ident, $FUNC_SIG:ident, $OP:tt, $primitive:ty) => {
        impl $TRAIT<$primitive> for DataElement 
            where
                $primitive: $TRAIT<$primitive> + BlackJackData,
        {
            fn $FUNC_SIG(&mut self, val: $primitive) {
                match self {
                    DataElement::F64(v) => *v $OP val as f64,
                    DataElement::I64(v) => {
                        match val.dtype() {
                            DType::I64 => *v $OP val as i64,
                            DType::I32 => *v $OP val as i64,
                            _ => panic!(
                                r#"Cannot do inplace mod of i64 for anthing 
                                other than an integer; either convert to float
                                or convert scalar value to integer."#)
                        }
                    }
                    DataElement::F32(v) => *v $OP val as f32,
                    DataElement::I32(v) => {
                        match val.dtype() {
                            DType::I64 => *v $OP val as i32,
                            DType::I32 => *v $OP val as i32,
                            _ => panic!(
                                r#"Cannot do inplace mod of i32 for anthing 
                                other than an integer; either convert to float
                                or convert scalar value to integer."#)
                        }
                    },
                    DataElement::STRING(_v) => panic!("Cannot multiply string by numeric value.")
                }
            }
        }
    };
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
