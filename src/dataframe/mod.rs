
use series::{Series};


pub struct DataFrame
{
    data: Vec<Series>
}

impl DataFrame
{

    /// Constructs a new `DataFrame<'a>`
    /// 
    /// # Example
    /// 
    /// ```
    /// use blackjack::dataframe::DataFrame;
    /// 
    /// let df: DataFrame = DataFrame::new();
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
    pub fn get_column_by_name(&self, name: String) -> Option<&Series> 
    {
        for (i, s_name) in self.data.iter().enumerate().map(|(i, v)| {(i, v.name.clone().unwrap())}) {
            if &name == &s_name {
                return Some(&self.data[i])
            }
        }
        None
    }

    pub fn add_column(&mut self, mut series: Series) -> Result<(), &'static str> {
        // Can only add column if series length matches or this is an empty dataframe
        if (series.len() != self.len()) & (self.len() > 0){
            Err("Length of new column does not match length of index!")
        } else {
            series.name = match series.name {
                Some(name) => Some(name),
                None => {
                    let name = format!("{}", self.len());
                    Some(name)
                }
            };
            self.data.push(series);
            Ok(())
        }
    }
}


