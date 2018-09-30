
use prelude::*;

pub struct GroupBy {

}

pub trait GroupByBehavior {
    fn split(&self, keys: Series) -> Vec<Series>;
    fn apply<F, T>(&self, agg_func: F) -> T
        where 
            F: Fn(&Series) -> T,
            T: BlackJackData;
    //fn combine(&self) -> Series;
}
