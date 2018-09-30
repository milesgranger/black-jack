
use prelude::*;

pub struct GroupBy {

}

pub trait GroupByBehavior {
    fn split(&self, column: &str) -> Vec<Series>;
    //fn apply(&self) -> Series;
    //fn combine(&self) -> Series;
}
