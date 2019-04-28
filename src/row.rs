use std::ops::Index;

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
