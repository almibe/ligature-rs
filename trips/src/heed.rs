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
use heed_types::{Str, U64, Bytes, Unit};

/// An in-memory implementation of Trips.
pub struct TripsHeed {
    env: Env,
}

const ids: Option<&str> = Some("ids");
const collection_to_id: Option<&str> = Some("collection_to_id");
const id_to_collection: Option<&str> = Some("id_to_collection");
const cfst: Option<&str> = Some("cfst");
const cfts: Option<&str> = Some("cfts");
const csft: Option<&str> = Some("csft");
const cstf: Option<&str> = Some("cstf");
const ctfs: Option<&str> = Some("ctfs");
const ctsf: Option<&str> = Some("ctsf");

impl TripsHeed {
    /// Create an empty triple store.
    pub fn new(env: Env) -> Self {
        let mut tx = env.write_txn().unwrap();
        env.create_database::<Str, U64<byteorder::BigEndian>>(&mut tx, ids);
        env.create_database::<Str, U64<byteorder::BigEndian>>(&mut tx, collection_to_id);
        env.create_database::<U64<byteorder::BigEndian>, Str>(&mut tx, id_to_collection);
        env.create_database::<Bytes, Unit>(&mut tx, cfst);
        env.create_database::<Bytes, Unit>(&mut tx, cfts);
        env.create_database::<Bytes, Unit>(&mut tx, csft);
        env.create_database::<Bytes, Unit>(&mut tx, cstf);
        env.create_database::<Bytes, Unit>(&mut tx, ctfs);
        env.create_database::<Bytes, Unit>(&mut tx, ctsf);
        tx.commit();
        Self {
            env
        }
    }
}

impl Trips<TripsError> for TripsHeed
{
    fn collections(&self) -> Result<Vec<String>, TripsError> {
        let mut results: Vec<String> = vec![];
        let tx = self.env.read_txn().unwrap();
        match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id).unwrap() {
            Some(db) => {
                for entry in db.iter(&tx).unwrap() {
                    results.push(entry.unwrap().0.to_owned());
                }
            },
            None => todo!()
        }
        Ok(results)
    }

    fn add_collection(&mut self, collection: String) -> Result<(), TripsError> {
        let mut tx = self.env.write_txn().unwrap();
        match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id).unwrap() {
            Some(db) => {
                match db.get(&tx, &collection) {
                    Ok(Some(value)) => {
                        return Ok(());
                    },
                    _ => ()
                }
            },
            None => todo!()
        }
        let id = match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, ids).unwrap() {
            Some(db) => {
                match db.get(&tx, "id") {
                    Ok(Some(value)) => {
                        let next_id = value + 1;
                        db.put(&mut tx, "id", &next_id);
                        next_id
                    },
                    Ok(None) => {
                        db.put(&mut tx, "id", &0);
                        0
                    },
                    _ => todo!()
                }
            },
            None => todo!()
        };
        match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id).unwrap() {
            Some(db) => {
                db.put(&mut tx, &collection, &id);
            },
            None => todo!()
        }
        match self.env.open_database::<U64<byteorder::BigEndian>, Str>(&tx, id_to_collection).unwrap() {
            Some(db) => {
                db.put(&mut tx, &id, &collection);
            },
            None => todo!()
        }
        tx.commit();
        Ok(())
    }

    fn remove_collection(&mut self, collection: String) -> Result<(), TripsError> {
        let mut tx = self.env.write_txn().unwrap();
        let id = match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id).unwrap() {
            Some(db) => {
                match db.get(&tx, &collection) {
                    Ok(Some(id)) => id,
                    _ => todo!()
                }
            },
            None => todo!()
        };
        match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id).unwrap() {
            Some(db) => {
                db.delete(&mut tx, &collection);
            },
            None => todo!()
        }
        match self.env.open_database::<U64<byteorder::BigEndian>, Str>(&tx, id_to_collection).unwrap() {
            Some(db) => {
                db.delete(&mut tx, &id);
            },
            None => todo!()
        }
        tx.commit();
        Ok(())
    }

    fn triples(&self, _collection: String) -> Result<BTreeSet<crate::Trip>, TripsError> {
        let _tx = self.env.read_txn().unwrap();
        //look up collection id
        //look up triples in cfst
        Ok(BTreeSet::new())
    }

    fn add_triples(
        &mut self,
        collection: String,
        trips: &mut BTreeSet<crate::Trip>,
    ) -> Result<(), TripsError> {
        let mut tx = self.env.write_txn().unwrap();
        let id = match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id).unwrap() {
            Some(db) => {
                match db.get(&tx, &collection) {
                    Ok(Some(id)) => id,
                    _ => todo!()
                }
            },
            None => todo!()
        };
        for trip in trips {
            let fid = 0; //TODO lookup / create id
            let sid = 0; //TODO lookup / create id
            let tid = 0; //TODO lookup / create id

            match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, cfst).unwrap() {
                Some(db) => {
                    todo!()
                },
                None => todo!()
            }
            match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, cfts).unwrap() {
                Some(db) => {
                    todo!()
                },
                None => todo!()
            }
            match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, csft).unwrap() {
                Some(db) => {
                    todo!()
                },
                None => todo!()
            }
            match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, cstf).unwrap() {
                Some(db) => {
                    todo!()
                },
                None => todo!()
            }
            match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, ctfs).unwrap() {
                Some(db) => {
                    todo!()
                },
                None => todo!()
            }
            match self.env.open_database::<Str, U64<byteorder::BigEndian>>(&tx, ctsf).unwrap() {
                Some(db) => {
                    todo!()
                },
                None => todo!()
            }
        }
        tx.commit();
    }

    fn remove_triples(
        &mut self,
        _collection: String,
        _trips: &mut BTreeSet<crate::Trip>,
    ) -> Result<(), TripsError> {
        todo!()
    }

    fn query(
        &self,
        _collection: String,
        _pattern: BTreeSet<crate::Query>,
    ) -> Result<HashBag<BTreeMap<String, String>>, TripsError> {
        todo!()
    }
}
