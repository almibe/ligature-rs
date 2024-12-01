// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use crate::{Query, Slot, Trip, Trips, TripsError};
use hashbag::HashBag;
use std::collections::{BTreeMap, BTreeSet};
use duckdb::{params, Connection, Result};

/// A DuckDB backed implementation of Trips.
pub struct TripsDuckDB {
    conn: Option<Connection>
}

impl TripsDuckDB {
    /// Create an empty triple store in memory.
    pub fn new() -> Self {
        //let conn = Connection::open_in_memory().unwrap();

        Self {
            conn: None
        }
    }
}

impl Trips for TripsDuckDB {
    fn collections(&self) -> Result<Vec<String>, TripsError> {
        todo!()
    }

    fn add_collection(&mut self, collection: String) -> Result<(), TripsError> {
        todo!()
    }

    fn remove_collection(&mut self, collection: String) -> Result<(), TripsError> {
        todo!()
    }

    fn triples(&self, collection: String) -> Result<BTreeSet<crate::Trip>, TripsError> {
        todo!()
    }

    fn add_triples(
        &mut self,
        collection: String,
        trips: &mut BTreeSet<crate::Trip>,
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn remove_triples(
        &mut self,
        collection: String,
        trips: &mut BTreeSet<crate::Trip>,
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn query(
        &self,
        collection: String,
        pattern: BTreeSet<crate::Query>,
    ) -> Result<HashBag<BTreeMap<String, String>>, TripsError> {
        todo!()
    }
}
