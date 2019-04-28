//!
//! Representation of a row in a `DataFrame` and related structs

use std::ops::Index;

use crate::prelude::*;

/// Representation of a DataFrame row, in which each element
/// can have a different type.
pub struct Row<'a> {
    /// Represents the elements in the `Row`
    pub data: Vec<Element<'a>>,
}

impl<'a> Row<'a> {
    /// Create an empty `Row`
    pub fn new() -> Self {
        Row { data: vec![] }
    }

    /// Push an `Element` into the `Row`
    pub fn add(&mut self, data: Element<'a>) {
        self.data.push(data)
    }
}

/// Represent a single data element, the enum of the data itself, and the name
/// for the column it belongs in.
pub struct Element<'a> {
    /// Enum containing a reference to the data within the dataframe.
    pub data: Datum<'a>,

    /// The name of the column, of which this Element belongs
    pub name: String,
}

impl<'a> Element<'a> {
    /// Create a new element, which represents an element of a `Row`
    pub fn new(name: String, data: Datum<'a>) -> Self {
        Element { name, data }
    }
}

impl<'a, 'b> Index<&'b str> for Row<'a> {
    type Output = Datum<'a>;
    fn index(&self, name: &str) -> &Self::Output {
        for element in &self.data {
            if element.name == name {
                return &element.data;
            }
        }
        panic!("Element named: {} now found", name);
    }
}
