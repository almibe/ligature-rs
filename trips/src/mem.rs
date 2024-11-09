// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use std::collections::{BTreeSet, HashMap};

use crate::Trips;

/// A simple error type.
#[derive(Debug)]
pub struct TripsError(String);

/// An in-memory implementation of Trips.
pub struct TripsMem<C: Clone, T> {
    values: HashMap<usize, T>,
    collections: HashMap<C, BTreeSet<(usize, usize, usize)>>
}

impl <C: Clone, T>TripsMem<C, T> {
    /// Create an empty triple store.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            collections: HashMap::new()
        }
    }
}

impl <C: Clone,T>Trips<C,T,TripsError> for TripsMem<C, T> {
    fn collections(&self) -> Result<Vec<C>, TripsError> {
        let res: Vec<C> = self.collections.keys().cloned().collect();
        Ok(res)
    }

    fn add_collection(&mut self, collection: &C) -> Result<(), TripsError> {
        todo!()
    }

    fn remove_collection(&mut self, collection: &C) -> Result<(), TripsError> {
        todo!()
    }

    fn statements(&self, collection: &C) -> Result<Vec<crate::Trip<T>>, TripsError> {
        todo!()
    }

    fn add_triples(
        &mut self,
        collection: &C,
        trips: Vec<crate::Trip<T>>,
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn remove_triples(
        &mut self,
        collection: &C,
        trips: Vec<crate::Trip<T>>
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn query(&self) -> Result<Box<dyn crate::Query<T, TripsError>>, TripsError> {
        todo!()
    }
}
