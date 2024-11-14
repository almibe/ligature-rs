// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the an in-memory, non-transactional knowledge graph.

use ligature::{Element, Entry, Ligature};
use std::collections::BTreeSet;
use trips::{Trip, Trips, Query, Slot};
use hashbag::HashBag;
use trips::mem::{TripsMem, TripsError};

#[derive()]
/// An implementation of the Graph trait that stores all Data in a sorted set.
pub struct LigatureGraph<E> {
    store: Box<dyn Trips<Element, Element, E>>,
}

impl LigatureGraph<TripsError> {
    pub fn new() -> Self {
        Self { store: Box::new(TripsMem::<Element, Element>::new()) }
    }
}

impl<E> Ligature<E> for LigatureGraph<E> {
    fn collections(&self) -> Result<Vec<Element>, E> {
        self.store.collections()
    }

    fn add_collection(&mut self, collection: Element) -> Result<(), E> {
        self.store.add_collection(collection)
    }

    fn remove_collection(&mut self, collection: Element) -> Result<(), E> {
        self.store.remove_collection(collection)
    }

    fn entries(&self, collection: Element) -> Result<BTreeSet<ligature::Entry>, E> {
        self.store.triples(collection).map(|set| {
            set.into_iter().map(|entry: Trip<Element>| {
                if entry.1 == Element(":".to_owned()) {
                    Entry::Extends { element: entry.0, concept: entry.2 }
                } else if entry.1 == Element("¬:".to_owned()) {
                    Entry::NotExtends { element: entry.0, concept: entry.2 }
                } else {
                    Entry::Role { first: entry.0, second: entry.2, role: entry.1 }
                }
            }).collect()
        })
    }

    fn add_entries(&mut self, collection: Element, entries: &mut BTreeSet<ligature::Entry>) -> Result<(), E> {
        let mut triples: BTreeSet<Trip<Element>> = BTreeSet::from_iter(entries.iter().map(|entry| {
            match entry {
                Entry::Extends { element, concept } => Trip(element.clone(), Element(":".to_owned()), concept.clone()),
                Entry::Role { first, second, role } => Trip(first.clone(), role.clone(), second.clone()),
                Entry::NotExtends { element, concept } => Trip(element.clone(), Element("¬:".to_owned()), concept.clone()),
            }
        }));
        self.store.add_triples(collection, &mut triples)
    }

    fn remove_entries(&mut self, collection: Element, entries: &mut BTreeSet<ligature::Entry>) -> Result<(), E> {
        let mut triples: BTreeSet<Trip<Element>> = BTreeSet::from_iter(entries.iter().map(|entry| {
            match entry {
                Entry::Extends { element, concept } => Trip(element.clone(), Element(":".to_owned()), concept.clone()),
                Entry::Role { first, second, role } => Trip(first.clone(), role.clone(), second.clone()),
                Entry::NotExtends { element, concept } => Trip(element.clone(), Element("¬:".to_owned()), concept.clone()),
            }
        }));
        self.store.remove_triples(collection, &mut triples)
    }

    fn query(
        &self,
        collection: Element,
        pattern: BTreeSet<ligature::Entry>,
    ) -> Result<HashBag<std::collections::BTreeMap<String, Element>>, E> {
        let query_pattern: BTreeSet<Query<Element>> = BTreeSet::from_iter(pattern.iter().map(|entry| {
            match entry {
                Entry::Extends { element, concept } => 
                    Query(Slot::Value(element.clone()), Slot::Value(Element(":".to_owned())), Slot::Value(concept.clone())),
                Entry::Role { first, second, role } => 
                    Query(Slot::Value(first.clone()), Slot::Value(role.clone()), Slot::Value(second.clone())),
                Entry::NotExtends { element, concept } => 
                    Query(Slot::Value(element.clone()), Slot::Value(Element("¬:".to_owned())), Slot::Value(concept.clone())),
            }
        }));
        self.store.query(collection, query_pattern)
    }
}
