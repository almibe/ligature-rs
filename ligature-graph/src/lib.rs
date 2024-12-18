// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the an in-memory, non-transactional knowledge graph.

use ligature::{Element, Entry, Ligature, LigatureError};
use std::collections::BTreeSet;
use trips::mem::TripsMem;
use trips::{Query, Slot, Trip, Trips};

#[derive()]
/// An implementation of the Graph trait that stores all Data in a sorted set.
pub struct LigatureGraph {
    store: Box<dyn Trips>,
}

impl LigatureGraph {
    pub fn from_trips(trips: Box<dyn Trips>) -> Self {
        Self { store: trips }
    }
}

impl LigatureGraph {
    pub fn new() -> Self {
        Self {
            store: Box::new(TripsMem::new()),
        }
    }
}

impl Ligature for LigatureGraph {
    fn collections(&self) -> Result<Vec<Element>, LigatureError> {
        self.store
            .collections()
            .map(|r| r.into_iter().map(|e| Element(e)).collect())
            .map_err(|e| todo!())
    }

    fn add_collection(&mut self, collection: Element) -> Result<(), LigatureError> {
        self.store.add_collection(collection.0).map_err(|e| todo!())
    }

    fn remove_collection(&mut self, collection: Element) -> Result<(), LigatureError> {
        self.store
            .remove_collection(collection.0)
            .map_err(|e| todo!())
    }

    fn entries(&self, collection: &Element) -> Result<BTreeSet<ligature::Entry>, LigatureError> {
        self.store
            .triples(collection.clone().0)
            .map(|set| {
                set.into_iter()
                    .map(|entry: Trip| {
                        if entry.1 == ":".to_owned() {
                            Entry::Extends {
                                element: Element(entry.0),
                                concept: Element(entry.2),
                            }
                        } else if entry.1 == "¬:".to_owned() {
                            Entry::NotExtends {
                                element: Element(entry.0),
                                concept: Element(entry.2),
                            }
                        } else {
                            Entry::Role {
                                first: Element(entry.0),
                                second: Element(entry.2),
                                role: Element(entry.1),
                            }
                        }
                    })
                    .collect()
            })
            .map_err(|e| LigatureError(e.0))
    }

    fn add_entries(
        &mut self,
        collection: Element,
        entries: &mut BTreeSet<ligature::Entry>,
    ) -> Result<(), LigatureError> {
        let mut triples: BTreeSet<Trip> =
            BTreeSet::from_iter(entries.iter().map(|entry| match entry {
                Entry::Extends { element, concept } => {
                    Trip(element.clone().0, ":".to_owned(), concept.clone().0)
                }
                Entry::Role {
                    first,
                    second,
                    role,
                } => Trip(first.clone().0, role.clone().0, second.clone().0),
                Entry::NotExtends { element, concept } => {
                    Trip(element.clone().0, "¬:".to_owned(), concept.clone().0)
                }
            }));
        self.store
            .add_triples(collection.0, &mut triples)
            .map_err(|e| todo!())
    }

    fn remove_entries(
        &mut self,
        collection: Element,
        entries: &mut BTreeSet<ligature::Entry>,
    ) -> Result<(), LigatureError> {
        let mut triples: BTreeSet<Trip> =
            BTreeSet::from_iter(entries.iter().map(|entry| match entry {
                Entry::Extends { element, concept } => {
                    Trip(element.clone().0, ":".to_owned(), concept.clone().0)
                }
                Entry::Role {
                    first,
                    second,
                    role,
                } => Trip(first.clone().0, role.clone().0, second.clone().0),
                Entry::NotExtends { element, concept } => {
                    Trip(element.clone().0, "¬:".to_owned(), concept.clone().0)
                }
            }));
        self.store
            .remove_triples(collection.0, &mut triples)
            .map_err(|e| todo!())
    }

    fn filter(
        &self,
        collection: Element,
        pattern: Entry,
    ) -> Result<BTreeSet<Entry>, LigatureError> {
        let query_pattern: Query =
            match pattern {
                Entry::Extends { element, concept } => Query(
                    check_value(element.clone().0),
                    Slot::Value(":".to_owned()),
                    check_value(concept.clone().0),
                ),
                Entry::Role {
                    first,
                    second,
                    role,
                } => Query(
                    check_value(first.clone().0),
                    check_value(role.clone().0),
                    check_value(second.clone().0),
                ),
                Entry::NotExtends { element, concept } => Query(
                    check_value(element.clone().0),
                    Slot::Value("¬:".to_owned()),
                    check_value(concept.clone().0),
                ),
            };
        self.store
            .filter(collection.0, query_pattern)
            .map(|trips| {
                trips.iter().map(|trip| {
                    match trip.1.as_str() {
                        ":" => Entry::Extends { element: Element(trip.0.clone()), concept: Element(trip.2.clone()) },
                        "¬:" => Entry::NotExtends { element: Element(trip.0.clone()), concept: Element(trip.2.clone()) },
                        _ => Entry::Role { first: Element(trip.0.clone()), second: Element(trip.2.clone()), role: Element(trip.1.clone()) }
                    }
                }).collect()
            })
            .map_err(|e| todo!())
    }
    
    // fn query(
    //     &self,
    //     collection: Element,
    //     pattern: BTreeSet<ligature::Entry>,
    // ) -> Result<HashBag<std::collections::BTreeMap<String, String>>, LigatureError> {
    //     let query_pattern: BTreeSet<Query> =
    //         BTreeSet::from_iter(pattern.iter().map(|entry| match entry {
    //             Entry::Extends { element, concept } => Query(
    //                 Slot::Value(element.clone().0),
    //                 Slot::Value(":".to_owned()),
    //                 Slot::Value(concept.clone().0),
    //             ),
    //             Entry::Role {
    //                 first,
    //                 second,
    //                 role,
    //             } => Query(
    //                 Slot::Value(first.clone().0),
    //                 Slot::Value(role.clone().0),
    //                 Slot::Value(second.clone().0),
    //             ),
    //             Entry::NotExtends { element, concept } => Query(
    //                 Slot::Value(element.clone().0),
    //                 Slot::Value("¬:".to_owned()),
    //                 Slot::Value(concept.clone().0),
    //             ),
    //         }));
    //     self.store
    //         .query(collection.0, query_pattern)
    //         .map_err(|e| todo!())
    // }
}

fn check_value(value: String) -> Slot {
    if value == "?" {
        Slot::Any
    } else if value.starts_with("?") {
        let mut chars = value.chars();
        chars.next();
        Slot::Variable(chars.as_str().to_owned())
    } else {
        Slot::Value(value.to_owned())
    }
}