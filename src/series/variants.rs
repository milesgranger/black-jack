//! Variations of `Series` and various helper objects

use crate::prelude::*;

/// Enum for holding valid Series types
pub enum GenericSeriesContainer {
    /// Hold `i64` type series
    I64(Series<i64>),
    /// Hold `f64` type series
    F64(Series<f64>),
    /// Hold `i32` type series
    I32(Series<i32>),
    /// Hold `f32` type series
    F32(Series<f32>),
    /// Hold `String` type series
    STRING(Series<String>),
}

impl GenericSeriesContainer {
    /// Convert a `GenericSeriesContainer` into a `Vec<String>`
    pub fn into_string_vec(self) -> Vec<String> {
        // TODO: `.unwrap()` is pretty safe here, but should avoid it anyhow.
        match self {
            GenericSeriesContainer::I64(series) => series.into_type::<String>().unwrap().into_vec(),
            GenericSeriesContainer::F64(series) => series.into_type::<String>().unwrap().into_vec(),
            GenericSeriesContainer::I32(series) => series.into_type::<String>().unwrap().into_vec(),
            GenericSeriesContainer::F32(series) => series.into_type::<String>().unwrap().into_vec(),
            GenericSeriesContainer::STRING(series) => series.into_vec(),
        }
    }
}

/// Serialized version of `Series<T>`, enabling storage inside a homogeneous container
/// where metadata is stored and data is stored in byte/compressed format.
#[derive(Debug, Clone)]
pub struct SeriesMeta {
    /// Name of a `Series`
    pub name: String,
    /// The length of a `Series`
    pub len: usize,
    /// The `DType` of a `Series`
    pub dtype: DType,
}

impl<T: BlackJackData> From<&Series<T>> for SeriesMeta {
    fn from(series: &Series<T>) -> SeriesMeta {
        SeriesMeta {
            name: series.name().unwrap(),
            len: series.len(),
            dtype: series.dtype().unwrap(),
        }
    }
}
