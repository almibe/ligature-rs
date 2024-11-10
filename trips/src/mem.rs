// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use std::collections::{BTreeSet, HashMap, HashSet};
use core::hash::Hash;
use crate::{Trip, Trips};

/// A simple error type.
#[derive(Debug)]
pub struct TripsError(String);

/// An in-memory implementation of Trips.
pub struct TripsMem<C: Clone, T: std::fmt::Debug + Ord> {
    // index: usize,
    // id_to_value: HashMap<usize, T>,
    // value_to_id: HashMap<T, usize>,
    // collections: HashMap<C, BTreeSet<(usize, usize, usize)>>
    collections: HashMap<C, BTreeSet<Trip<T>>>
}

impl <C: Clone, T: std::fmt::Debug + Ord>TripsMem<C, T> {
    /// Create an empty triple store.
    pub fn new() -> Self {
        Self {
            // index: 0,
            // values: HashMap::new(),
            collections: HashMap::new()
        }
    }

    // /// Check if the value is already stored and if not add it.
    // /// Either way return its id.
    // pub fn check_and_add_value(&mut self, value: T) -> usize {

    //     todo!()
    // }
}

impl <C: Clone + Eq + Hash,T: std::fmt::Debug + Eq + Ord + Clone>Trips<C,T,TripsError> for TripsMem<C, T> {
    fn collections(&self) -> Result<Vec<C>, TripsError> {
        let res: Vec<C> = self.collections.keys().cloned().collect();
        Ok(res)
    }

    fn add_collection(&mut self, collection: C) -> Result<(), TripsError> {
        self.collections.insert(collection, BTreeSet::new());
        Ok(())
    }

    fn remove_collection(&mut self, collection: C) -> Result<(), TripsError> {
        self.collections.remove(&collection);
        Ok(())
    }

    fn triples(&self, collection: C) -> Result<BTreeSet<crate::Trip<T>>, TripsError> {
        match self.collections.get(&collection) {
            Some(res) => {
                match self.collections.get(&collection) {
                    Some(res) => Ok(res.clone()),
                    None => todo!()
                }
            },
            None => todo!()
        }
    }

    fn add_triples(
        &mut self,
        collection: C,
        trips: &mut BTreeSet<crate::Trip<T>>,
    ) -> Result<(), TripsError> {
        match self.collections.get_mut(&collection) {
            Some(res) => {
                res.append(trips);
                return Ok(())
            },
            None => todo!()
        }
    }

    fn remove_triples(
        &mut self,
        collection: C,
        trips: &mut BTreeSet<crate::Trip<T>>
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn query(&self) -> Result<Box<dyn crate::Query<T, TripsError>>, TripsError> {
        todo!()
    }
}
