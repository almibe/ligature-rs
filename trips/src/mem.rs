// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use crate::{Query, Slot, Trip, Trips};
use core::hash::Hash;
use hashbag::HashBag;
use std::{collections::{BTreeMap, BTreeSet, HashMap}, env::var};

/// A simple error type.
#[derive(Debug)]
pub struct TripsError(String);

/// An in-memory implementation of Trips.
pub struct TripsMem<C: Clone, T: std::fmt::Debug + Ord> {
    // index: usize,
    // id_to_value: HashMap<usize, T>,
    // value_to_id: HashMap<T, usize>,
    // collections: HashMap<C, BTreeSet<(usize, usize, usize)>>
    collections: HashMap<C, BTreeSet<Trip<T>>>,
}

impl<C: Clone, T: std::fmt::Debug + Ord> TripsMem<C, T> {
    /// Create an empty triple store.
    pub fn new() -> Self {
        Self {
            // index: 0,
            // values: HashMap::new(),
            collections: HashMap::new(),
        }
    }

    // /// Check if the value is already stored and if not add it.
    // /// Either way return its id.
    // pub fn check_and_add_value(&mut self, value: T) -> usize {

    //     todo!()
    // }
}

impl<C: Clone + Eq + Hash, T: std::fmt::Debug + Eq + Ord + Clone + Hash> Trips<C, T, TripsError>
    for TripsMem<C, T>
{
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
            Some(res) => Ok(res.clone()),
            None => todo!(),
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
                return Ok(());
            }
            None => todo!(),
        }
    }

    fn remove_triples(
        &mut self,
        collection: C,
        trips: &mut BTreeSet<crate::Trip<T>>,
    ) -> Result<(), TripsError> {
        match self.collections.get_mut(&collection) {
            Some(res) => {
                trips.iter().for_each(|trip| {
                    res.remove(trip);
                });
                return Ok(());
            }
            None => todo!(),
        }
    }

    fn query(
        &self,
        collection: C,
        pattern: BTreeSet<crate::Query<T>>,
    ) -> Result<HashBag<BTreeMap<String, T>>, TripsError> {
        match self.collections.get(&collection) {
            Some(collection) => {
                let mut results: HashBag<BTreeMap<String, T>> = HashBag::new();
                let terms: Vec<&Query<T>> = pattern.iter().collect();
                let mut index = 0;
                while index < terms.len() {
                    match terms.get(index) {
                        Some(query) => {
                            let matches = match_query(query, collection.clone());
                            if matches.is_empty() {
                                return Ok(HashBag::new());
                            } else {
                                if index == 0 {
                                    matches.iter().for_each(|m| {
                                        println!("***");
                                        results.insert(m.clone());
                                        ()
                                    });
                                } else {
                                    todo!()
                                }
                            }
                        }
                        None => todo!(),
                    }
                    index = index + 1;
                }
                Ok(results)
            } //Ok(res.clone()),
            None => todo!(),
        }
    }
}

fn match_query<T: std::fmt::Debug + Ord>(
    query: &Query<T>,
    triples: BTreeSet<Trip<T>>,
) -> Vec<BTreeMap<String, T>> {
    let mut results = vec![];
    for trip in triples {
        match match_query_single(query, trip) {
            Some(res) => results.push(res),
            None => (),
        }
    }
    results
}

fn match_query_single<T: std::fmt::Debug + Ord>(
    query: &Query<T>,
    triple: Trip<T>,
) -> Option<BTreeMap<String, T>> {
    let mut result = BTreeMap::new();
    match &query.0 {
        Slot::Variable(variable_name) => {
            result.insert(variable_name.to_owned(), triple.0);
            ()
        },
        Slot::Value(value) =>
            if *value == triple.0 {
                ()
            } else {
                return None
            },
        Slot::Any => (),
    }
    match &query.1 {
        Slot::Variable(variable_name) => {
            match result.get(variable_name) {
                Some(value) => {
                    if *value == triple.1 {
                        ()
                    } else {
                        return None
                    }
                },
                None => {
                    result.insert(variable_name.to_owned(), triple.1);
                    ()    
                }
            }
        },
        Slot::Value(value) =>
            if *value == triple.1 {
                ()
            } else {
                return None
            },
        Slot::Any => (),
    }
    match &query.2 {
        Slot::Variable(variable_name) => {
            match result.get(variable_name) {
                Some(value) => {
                    if *value == triple.2 {
                        ()
                    } else {
                        return None
                    }
                },
                None => {
                    result.insert(variable_name.to_owned(), triple.2);
                    ()
                }
            }
        },
        Slot::Value(value) =>
            if *value == triple.2 {
                ()
            } else {
                return None
            },
        Slot::Any => (),
    }
    Some(result)
}
