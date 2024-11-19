// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use crate::Trips;
use crate::mem::TripsError;
use core::hash::Hash;
use hashbag::HashBag;
use std::collections::{BTreeMap, BTreeSet};
use heed::{Env, Database, BytesDecode};
use heed_types::{Str, U64};

/// An in-memory implementation of Trips.
pub struct TripsHeed {
    env: Env,
}

const ids: Option<&str> = Some("ids");
const collection_to_id: Option<&str> = Some("collection_to_id");
const id_to_collection: Option<&str> = Some("id_to_collection");

impl<C: BytesDecode + 'static> TripsHeed {
    /// Create an empty triple store.
    pub fn new(env: Env) -> Self {
        let mut tx = env.write_txn().unwrap();
        env.create_database::<Str, U64<byteorder::BigEndian>>(&mut tx, ids);
        env.create_database::<Str, Str>(&mut tx, collection_to_id);
        env.create_database::<U64<byteorder::BigEndian>, Str>(&mut tx, id_to_collection);
        tx.commit();

        Self {
            env
        }
    }
}

impl<C: Clone + Eq + Hash + Ord + BytesDecode, T: std::fmt::Debug + Eq + Ord + Clone + Hash>
    Trips<C, T, TripsError> for TripsHeed
{
    fn collections(&self) -> Result<Vec<C>, TripsError> {
        let results: Vec<C> = vec![];
        let tx = self.env.read_txn().unwrap();
        match self.env.open_database::<C, C>(&tx, collection_to_id).unwrap() {
            Some(db) => {
                for entry in db.iter(&tx).unwrap() {
                    results.push(entry.unwrap().1);
                }
            },
            None => todo!()
        }
        Ok(results)
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
