// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use crate::Trips;
use core::hash::Hash;
use hashbag::HashBag;
use redb::backends::InMemoryBackend;
use redb::{Database, Key, TableDefinition, Value};
use std::collections::{BTreeMap, BTreeSet};

/// A simple error type.
#[derive(Debug)]
pub struct TripsError(String);

/// An in-memory implementation of Trips.
pub struct TripsReDB<'a, C: Key + Value + 'static> {
    db: Database,
    ids_tbl: TableDefinition<'a, String, u64>,
    collection_to_id_tbl: TableDefinition<'a, C, u64>,
    id_to_collection_tbl: TableDefinition<'a, u64, C>,
}

impl<C: Value + Key + 'static> TripsReDB<'_, C> {
    /// Create an empty triple store.
    pub fn new() -> Self {
        Self {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::new())
                .unwrap(),
            ids_tbl: TableDefinition::new("ids"),
            collection_to_id_tbl: TableDefinition::new("collection_to_id"),
            id_to_collection_tbl: TableDefinition::new("id_to_collection"),
        }
    }
}

impl<C: Clone + Eq + Hash + Ord + Key, T: std::fmt::Debug + Eq + Ord + Clone + Hash>
    Trips<C, T, TripsError> for TripsReDB<'_, C>
{
    fn collections(&self) -> Result<Vec<C>, TripsError> {
        todo!()
    }

    fn add_collection(&mut self, _collection: C) -> Result<(), TripsError> {
        todo!()
    }

    fn remove_collection(&mut self, _collection: C) -> Result<(), TripsError> {
        todo!()
    }

    fn triples(&self, _collection: C) -> Result<BTreeSet<crate::Trip<T>>, TripsError> {
        todo!()
    }

    fn add_triples(
        &mut self,
        _collection: C,
        _trips: &mut BTreeSet<crate::Trip<T>>,
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn remove_triples(
        &mut self,
        _collection: C,
        _trips: &mut BTreeSet<crate::Trip<T>>,
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn query(
        &self,
        _collection: C,
        _pattern: BTreeSet<crate::Query<T>>,
    ) -> Result<HashBag<BTreeMap<String, T>>, TripsError> {
        todo!()
    }
}
