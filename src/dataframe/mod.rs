///! DataFrame object and associated functionality

use bincode;
use serde::Serialize;

use prelude::*;


struct SerializedSeries {
    name: String,
    dtype: DType,
    encoded_data: Vec<u8>
}

impl SerializedSeries {

    /// Serialize a [`Series`] into a [`SerializedSeries`]
    pub fn from_series<T: BlackJackData + Serialize>(series: Series<T>) -> Result<Self, Box<Error>> {
        match series.name() {
            Some(name) => {
                let encoded_data = bincode::serialize(&series)?;
                let dtype = series.dtype();
                Ok(SerializedSeries { name, dtype, encoded_data, })
            },
            None => Err("Cannot create a new serialized column from series without a name.")
        }
    }

    /// Deserialize this into a series
    pub fn into_series<T: BlackJackData>(self) -> Result<Series<T>, bincode::Error> {
        match self.dtype {
            DType::F64 => bincode::deserialize::<Series<f64>>(&self.encoded_data),
            DType::I64 => bincode::deserialize::<Series<i64>>(&self.encoded_data),
            DType::F32 => bincode::deserialize::<Series<f32>>(&self.encoded_data),
            DType::I32 => bincode::deserialize::<Series<i32>>(&self.encoded_data),
            DType::STRING => bincode::deserialize::<Series<String>>(&self.encoded_data),
        }
    }

}

pub struct DataFrame {
    data: Vec<SerializedSeries>
}