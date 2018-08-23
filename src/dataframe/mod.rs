use series::{SeriesObj};


/// 
pub struct DataFrame<S>
    where S: SeriesObj
{
    data: Vec<Box<S>>
}

impl<S: SeriesObj> DataFrame<S> {
    pub fn new() -> DataFrame<S> {
        let data: Vec<Box<S>> = Vec::new();
        DataFrame { data }
    }

    pub fn add_column(&mut self, series: S) -> () {
        self.data.push(Box::new(series));
    }
}