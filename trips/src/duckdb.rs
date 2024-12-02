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
              CREATE TABLE part (
                      id              UBIGINT PRIMARY KEY DEFAULT NEXTVAL('seq'),
                      part            TEXT NOT NULL UNIQUE,
                      );
              CREATE TABLE collection (
                      id              UBIGINT PRIMARY KEY DEFAULT NEXTVAL('seq'),
                      name            TEXT NOT NULL UNIQUE,
                      );
              CREATE TABLE trip (
                      id              UBIGINT PRIMARY KEY DEFAULT NEXTVAL('seq'),
                      collection      UBIGINT REFERENCES collection(id),
                      first           UBIGINT REFERENCES part(id),
                      second          UBIGINT REFERENCES part(id),
                      third           UBIGINT REFERENCES part(id),
                      );
            ",
        )
        .unwrap();

        Self { conn }
    }
}

fn get_collection_id(tx: &Transaction, collection: &str) -> Result<u64, TripsError> {
    let mut stmt = tx
        .prepare("SELECT id from collection where name = ?;")
        .unwrap();
    let mut itr = stmt
        .query_map([collection], |row| Ok(row.get(0).unwrap()))
        .unwrap();
    for id in itr {
        return Ok(id.unwrap());
    }
    Err(TripsError(format!("Collection {} not found.", collection)))
}

fn add_part(tx: &Transaction, part: &str) -> Result<(), TripsError> {
    tx.execute("insert into part (part) values (?) ON CONFLICT DO NOTHING", params![part]); //TODO just ignoring errors for now
    Ok(())
}

fn add_trip(tx: &Transaction, collection_id: u64, trip: &Trip) -> Result<(), TripsError> {
    tx.execute("insert into trip (collection, first, second, third) values (?, (select id from part where part = ?), (select id from part where part = ?), (select id from part where part = ?))",
        params![collection_id, trip.0, trip.1, trip.2])
        .unwrap();
    Ok(())
}

impl Trips for TripsDuckDB {
    fn collections(&self) -> Result<Vec<String>, TripsError> {
        let mut names: Vec<String> = vec![];
        let mut stmt = self.conn.prepare("SELECT name from collection;").unwrap();
        let mut itr = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
        for name in itr {
            names.push(name.unwrap());
        }
        Ok(names)
    }

    fn add_collection(&mut self, collection: String) -> Result<(), TripsError> {
        self.conn
            .execute(
                "insert into collection (name) values (?)",
                params![collection],
            )
            .unwrap();
        Ok(())
    }

    fn remove_collection(&mut self, collection: String) -> Result<(), TripsError> {
        self.conn
            .execute("delete from collection where name = ?", params![collection])
            .unwrap();
        Ok(())
    }

    fn triples(&self, collection: String) -> Result<BTreeSet<Trip>, TripsError> {
        let mut trips: BTreeSet<Trip> = BTreeSet::new();
        let mut stmt = self
            .conn
            .prepare(
                r"SELECT p1.part, p2.part, p3.part from trip
            left join collection c on c.id = trip.collection
            left join part p1 on p1.id = trip.first
            left join part p2 on p2.id = trip.second
            left join part p3 on p3.id = trip.third;",
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
        let id = get_collection_id(&tx, &collection)?;
        for trip in trips.iter() {
            add_part(&tx, &trip.0)?;
            add_part(&tx, &trip.1)?;
            add_part(&tx, &trip.2)?;
            add_trip(&tx, id, trip);
        }
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
