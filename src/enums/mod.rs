//! Enums to be used throughout the crate.

/// Possible DType returns, matches [`BlackJackData`]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, PartialOrd)]
pub enum DType {

    /// `f64`
    F64,

    /// `i64`
    I64,

    /// `f32`
    F32,

    /// `i32`
    I32,

    /// `String`
    STRING,
}

/// Join type
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, PartialOrd)]
pub enum Join {

    /// Perform a full join from all keys involved
    Outer,
    /// Join based on common keys shared
    Inner,
    /// Join based on keys in the right
    Right,
    /// Join based on keys in the left
    Left
}
