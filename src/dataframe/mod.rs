use containers::Data;
use series::Series;


pub struct DataFrame<'a> {
    data: Vec<Series<'a>>
}

impl<'a> DataFrame<'a> {

    /// Constructs a new `DataFrame<'a>`
    /// 
    /// # Example
    /// 
    /// ```
    /// use blackjack::dataframe::DataFrame;
    /// 
    /// let df = DataFrame::new();
    /// ```
    pub fn new() -> Self {
        let data = Vec::new();
        DataFrame { data }
    }

    /// Return length of the dataframe
    pub fn len(&self) -> usize {
        if self.data.len() > 0 {
            self.data[0].len()
        } else {
            0
        }
    }

    /// Attempt to get a reference to a series in the dataframe by name
    /// 
    /// # Example
    /// 
    /// ```
    /// use blackjack::dataframe::DataFrame;
    /// use blackjack::series::Series;
    /// 
    /// ```
    pub fn get_column_by_name(&self, name: &'a str) -> Option<&Series<'a>> {
        for series in self.data.iter() {
            if name == series.name {
                return Some(series)
            }
        }
        None
    }

    pub fn add_column(&mut self, series: Series<'a>) -> Result<(), &'static str> {
        // Can only add column if series length matches or this is an empty dataframe
        if (series.len() != self.len()) & (self.len() > 0){
            Err("Length of new column does not match length of index!")
        } else {
            self.data.push(series);
            Ok(())
        }
    }
}


