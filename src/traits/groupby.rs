
use prelude::*;

pub struct GroupBy {

}

pub trait GroupByBehavior {
    fn split(&self, keys: Series) -> Vec<Series>;
    //fn apply(&self) -> Series;
    //fn combine(&self) -> Series;
}
