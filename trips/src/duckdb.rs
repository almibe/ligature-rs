// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use crate::{Query, Slot, Trip, Trips, TripsError};
use duckdb::{params, Connection, Result, Transaction};
use hashbag::HashBag;
use std::collections::{BTreeMap, BTreeSet};

/// A DuckDB backed implementation of Trips.
pub struct TripsDuckDB {
    conn: Connection,
}

impl TripsDuckDB {
    /// Create an empty triple store in memory.
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute_batch(
            r"CREATE SEQUENCE seq;
              CREATE TABLE element (
                      id              UBIGINT PRIMARY KEY DEFAULT NEXTVAL('seq'),
                      element         TEXT NOT NULL UNIQUE,
                      );
              CREATE TABLE network (
                      id              UBIGINT PRIMARY KEY DEFAULT NEXTVAL('seq'),
                      name            TEXT NOT NULL UNIQUE,
                      );
              CREATE TABLE entry (
                      id              UBIGINT PRIMARY KEY DEFAULT NEXTVAL('seq'),
                      network         UBIGINT REFERENCES network(id),
                      first           UBIGINT REFERENCES element(id),
                      second          UBIGINT REFERENCES element(id),
                      third           UBIGINT REFERENCES element(id),
                      );
            ",
        )
        .unwrap();

        Self { conn }
    }
}

fn get_collection_id(tx: Transaction, collection: &str) -> Result<u64> {
    todo!()
}

impl Trips for TripsDuckDB {
    fn collections(&self) -> Result<Vec<String>, TripsError> {
        let mut names: Vec<String> = vec![];
        let mut stmt = self.conn.prepare("SELECT name from network;").unwrap();
        let mut itr = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
        for name in itr {
            names.push(name.unwrap());
        }
        Ok(names)
    }

    fn add_collection(&mut self, collection: String) -> Result<(), TripsError> {
        self.conn
            .execute("insert into network (name) values (?)", params![collection])
            .unwrap();
        Ok(())
    }

    fn remove_collection(&mut self, collection: String) -> Result<(), TripsError> {
        self.conn
            .execute("delete from network where name = ?", params![collection])
            .unwrap();
        Ok(())
    }

    fn triples(&self, collection: String) -> Result<BTreeSet<Trip>, TripsError> {
        let mut trips: BTreeSet<Trip> = BTreeSet::new();
        let mut stmt = self
            .conn
            .prepare(
                r"SELECT (e1.element, e2.element, e3.element) from entry
            left join network n on n.id = entry.network
            left join element e1 on e1.id = entry.first
            left join element e2 on e2.id = entry.second
            left join element e3 on e3.id = entry.third;",
            )
            .unwrap();
        let mut itr = stmt
            .query_map([], |row| {
                Ok(Trip(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                ))
            })
            .unwrap();
        for trip in itr {
            trips.insert(trip.unwrap());
        }
        Ok(trips)
    }

    fn add_triples(
        &mut self,
        collection: String,
        trips: &mut BTreeSet<Trip>,
    ) -> Result<(), TripsError> {
        let tx = self.conn.transaction().unwrap();
        //get collection id

        tx.execute("insert into network (name) values (?)", params![collection])
            .unwrap();
        tx.commit().unwrap();
        Ok(())
    }

    fn remove_triples(
        &mut self,
        _collection: String,
        _trips: &mut BTreeSet<Trip>,
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn query(
        &self,
        _collection: String,
        _pattern: BTreeSet<Query>,
    ) -> Result<HashBag<BTreeMap<String, String>>, TripsError> {
        todo!()
    }
}
