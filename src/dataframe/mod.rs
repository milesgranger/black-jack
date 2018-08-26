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

use series::{SeriesTrait, Container, Series, BlackJackData};

pub struct DataFrame {
    containers: HashMap<TypeId, Box<Any>>,
}

impl DataFrame {
    
    pub fn new() -> Self {
        Self { containers: HashMap::new() }
    }

    pub fn get_storage_mut<C: SeriesTrait>(&mut self) -> &mut <C as SeriesTrait>::Container {
        let type_id = TypeId::of::<C>();

        // Add a storage if it doesn't exist yet
        if !self.containers.contains_key(&type_id) {
            let new_container = <C as SeriesTrait>::Container::new();

            self.containers.insert(type_id, Box::new(new_container));
        }

        // Get the storage for this type
        match self.containers.get_mut(&type_id) {
            Some(probably_container) => {
                // Turn the Any into the storage for that type
                match probably_container.downcast_mut::<<C as SeriesTrait>::Container>() {
                    Some(container) => container,
                    None => unreachable!(), // <- you may want to do something less explosive here
                }
            }
            None => unreachable!(),
        }
    }

    pub fn add_column<T: BlackJackData>(&mut self, series: Series<T>) -> () {
        self.get_storage_mut::<Series<T>>().insert(series);
    }

}