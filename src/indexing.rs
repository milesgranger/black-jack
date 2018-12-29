//!
//! All things related to index a `Series` or `DataFrame`
//!


/// Represent various indexes for Series and DataFrames
#[derive(Clone, From, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Indexer {

    /// Int (`i32`) based indexing
    INT(Vec<i32>)
}

// TODO: Make this into a macro
impl<'b> From<&'b Indexer> for &'b Vec<i32> {
    fn from(indexer: &Indexer) -> &Vec<i32> {
        match indexer {
            Indexer::INT(ref vec) => vec
        }
    }
}
