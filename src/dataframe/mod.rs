//! A `DataFrame` represents a collection of varying types of `Series` objects.
//! 
//! ## Example use:
//! 
//! ```
//! use blackjack::prelude::*;
//! 
//! let mut df = DataFrame::new();
//! let series = Series::arange(0, 5);
//! 
//! df.add_column(series);
//! ```
//! 

use std::collections::HashMap;
use std::any::{Any, TypeId};

use series::{Series, Container};

pub struct DataFrame {
    containers: HashMap<TypeId, Box<Any>>,
}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
        }
    }

    pub fn get_storage_mut<C: Series>(&mut self) -> &mut <C as Series>::Container {
        let type_id = TypeId::of::<C>();

        // Add a storage if it doesn't exist yet
        if !self.containers.contains_key(&type_id) {
            let new_storage = <C as Series>::Container::new();

            self.containers.insert(type_id, Box::new(new_storage));
        }

        // Get the storage for this type
        match self.containers.get_mut(&type_id) {
            Some(probably_storage) => {
                // Turn the Any into the storage for that type
                match probably_storage.downcast_mut::<<C as Series>::Container>() {
                    Some(storage) => storage,
                    None => unreachable!(), // <- you may want to do something less explosive here
                }
            }
            None => unreachable!(),
        }
    }
}