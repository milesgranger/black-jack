use crate::prelude::*;

/// Representation of a DataFrame row, in which each element
/// can have a different type.
pub struct Row<'a> {
    pub data: Vec<Element<'a>>,
}

impl<'a> Row<'a> {
    pub fn new() -> Self {
        Row { data: vec![] }
    }
    pub fn add(&mut self, data: Element<'a>) {
        self.data.push(data)
    }
}

/// Represent a single data element, the enum of the data itself, and the name
/// for the column it belongs in.
pub struct Element<'a> {
    pub data: Datum<'a>,
    pub name: String,
}

impl<'a> Element<'a> {
    pub fn new(name: String, data: Datum<'a>) -> Self {
        Element { name, data }
    }
}
