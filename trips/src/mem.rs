// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use crate::{Query, Slot, Trip, Trips, TripsError};
use hashbag::HashBag;
use std::collections::{BTreeMap, BTreeSet};

/// An in-memory implementation of Trips.
#[derive(Debug, PartialEq, Eq)]

pub struct TripsMem {
    // index: usize,
    // id_to_value: HashMap<usize, T>,
    // value_to_id: HashMap<T, usize>,
    // collections: HashMap<C, BTreeSet<(usize, usize, usize)>>
    collections: BTreeMap<String, BTreeSet<Trip>>,
}

impl TripsMem {
    /// Create an empty triple store.
    pub fn new() -> Self {
        Self {
            // index: 0,
            // values: HashMap::new(),
            collections: BTreeMap::new(),
        }
    }

    // /// Check if the value is already stored and if not add it.
    // /// Either way return its id.
    // pub fn check_and_add_value(&mut self, value: T) -> usize {

    //     todo!()
    // }
}

impl Trips for TripsMem {
    fn collections(&self) -> Result<Vec<String>, TripsError> {
        let res: Vec<String> = self.collections.keys().cloned().collect();
        Ok(res)
    }

    fn add_collection(&mut self, collection: String) -> Result<(), TripsError> {
        self.collections.insert(collection, BTreeSet::new());
        Ok(())
    }

    fn remove_collection(&mut self, collection: String) -> Result<(), TripsError> {
        self.collections.remove(&collection);
        Ok(())
    }

    fn triples(&self, collection: String) -> Result<BTreeSet<crate::Trip>, TripsError> {
        match self.collections.get(&collection) {
            Some(res) => Ok(res.clone()),
            None => Err(TripsError(format!(
                "Collection `{}` not found.",
                collection
            ))),
        }
    }

    fn add_triples(
        &mut self,
        collection: String,
        trips: &mut BTreeSet<crate::Trip>,
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
        collection: String,
        trips: &mut BTreeSet<crate::Trip>,
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

    // fn query(
    //     &self,
    //     collection: String,
    //     pattern: BTreeSet<crate::Query>,
    // ) -> Result<HashBag<BTreeMap<String, String>>, TripsError> {
    //     match self.collections.get(&collection) {
    //         Some(collection) => {
    //             let mut results: HashBag<BTreeMap<String, String>> = HashBag::new();
    //             let terms: Vec<&Query> = pattern.iter().collect();
    //             let mut index = 0;
    //             while index < terms.len() {
    //                 match terms.get(index) {
    //                     Some(query) => {
    //                         let matches = match_query(query, collection.clone());
    //                         if matches.is_empty() {
    //                             return Ok(HashBag::new());
    //                         } else {
    //                             if index == 0 {
    //                                 matches.iter().for_each(|m| {
    //                                     results.insert(m.clone());
    //                                     ()
    //                                 });
    //                             } else {
    //                                 () //TODO  double check this
    //                             }
    //                         }
    //                     }
    //                     None => panic!("Should never reach."),
    //                 }
    //                 index = index + 1;
    //             }
    //             Ok(results)
    //         } //Ok(res.clone()),
    //         None => Err(TripsError("Collection not found.".to_owned())),
    //     }
    // }

    fn filter(&self, collection: String, pattern: Query) -> Result<BTreeSet<Trip>, TripsError> {
        match self.collections.get(&collection) {
            Some(collection) => {
                let mut results: BTreeSet<Trip> = BTreeSet::new();
                for trip in collection.iter() {
                    match pattern.0 {
                        Slot::Any => (),
                        Slot::Value(ref value) => {
                            if *value == trip.0 {
                                ()
                            } else {
                                continue;
                            }
                        }
                        Slot::Variable(name) => todo!(),
                    }
                    match pattern.1 {
                        Slot::Any => (),
                        Slot::Value(ref value) => {
                            if *value == trip.1 {
                                ()
                            } else {
                                continue;
                            }
                        }
                        Slot::Variable(name) => todo!(),
                    }
                    match pattern.2 {
                        Slot::Any => (),
                        Slot::Value(ref value) => {
                            if *value == trip.2 {
                                ()
                            } else {
                                continue;
                            }
                        }
                        Slot::Variable(name) => todo!(),
                    }
                    results.insert(trip.clone());
                }
                Ok(results)
            }
            None => Err(TripsError("Collection not found.".to_owned())),
        }
    }
}

fn match_query(query: &Query, triples: BTreeSet<Trip>) -> Vec<BTreeMap<String, String>> {
    let mut results = vec![];
    for trip in triples {
        match match_query_single(query, trip) {
            Some(res) => results.push(res),
            None => (),
        }
    }
    results
}

fn match_query_single(query: &Query, triple: Trip) -> Option<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    match &query.0 {
        Slot::Variable(variable_name) => {
            result.insert(variable_name.to_owned(), triple.0);
            ()
        }
        Slot::Value(value) => {
            if *value == triple.0 {
                ()
            } else {
                return None;
            }
        }
        Slot::Any => (),
    }
    match &query.1 {
        Slot::Variable(variable_name) => match result.get(variable_name) {
            Some(value) => {
                if *value == triple.1 {
                    ()
                } else {
                    return None;
                }
            }
            None => {
                result.insert(variable_name.to_owned(), triple.1);
                ()
            }
        },
        Slot::Value(value) => {
            if *value == triple.1 {
                ()
            } else {
                return None;
            }
        }
        Slot::Any => (),
    }
    match &query.2 {
        Slot::Variable(variable_name) => match result.get(variable_name) {
            Some(value) => {
                if *value == triple.2 {
                    ()
                } else {
                    return None;
                }
            }
            None => {
                result.insert(variable_name.to_owned(), triple.2);
                ()
            }
        },
        Slot::Value(value) => {
            if *value == triple.2 {
                ()
            } else {
                return None;
            }
        }
        Slot::Any => (),
    }
    Some(result)
}
