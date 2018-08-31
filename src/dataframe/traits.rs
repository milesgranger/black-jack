//! Traits defined to be used by [DataFrame](struct.DataFrame.html)
//! 


use prelude::*;

/// Define the behavior for managing columns/series within a dataframe
pub trait ColumnManager {
    /// Add a new series to the dataframe as a column.
    fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> ();

    /// Get a reference to a series by name, **will also have to know the primitive type stored**.
    ///
    /// ## Example
    /// ```
    /// use blackjack::prelude::*;
    ///
    /// let mut df = DataFrame::new();
    /// let mut series = Series::from_vec(vec![1, 2, 3]);
    /// series.set_name("series1");
    ///
    /// let series_clone = series.clone(); // Create a clone to compare later
    ///
    /// df.add_column(series);  // Add the column to dataframe
    ///
    /// let series_ref: &Series<i32> = df.get_column("series1").unwrap();  // Fetch the column back as a reference.
    ///
    /// assert_eq!(*series_ref, series_clone)  // ensure they equal.
    /// ```
    fn get_column<T: BlackJackData>(&self, name: &str) -> Option<&Series<T>>;

    /// Get a column of which the type is unknown
    /// Returns a [SeriesEnum](enum.SeriesEnum.html) of which will need to `match` the 
    /// resulting series type and deal with accordingly.
    fn get_column_unknown_type(&self, name: &str) -> Option<SeriesEnumRef>;

    /// Get the number of columns
    fn n_columns(&self) -> usize;
}
