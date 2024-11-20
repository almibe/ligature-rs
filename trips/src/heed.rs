// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use crate::mem::TripsError;
use crate::Trips;
use core::hash::Hash;
use hashbag::HashBag;
use heed::{BytesDecode, Database, Env};
use heed_types::{Bytes, Str, Unit, U64};
use std::collections::{BTreeMap, BTreeSet};

/// An in-memory implementation of Trips.
pub struct TripsHeed {
    env: Env,
}

const ids: Option<&str> = Some("ids");
const collection_to_id: Option<&str> = Some("collection_to_id");
const id_to_collection: Option<&str> = Some("id_to_collection");
const value_to_id: Option<&str> = Some("value_to_id");
const id_to_value: Option<&str> = Some("id_to_value");
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
        env.create_database::<Str, U64<byteorder::BigEndian>>(&mut tx, value_to_id);
        env.create_database::<U64<byteorder::BigEndian>, Str>(&mut tx, id_to_value);
        env.create_database::<Bytes, Unit>(&mut tx, cfst);
        env.create_database::<Bytes, Unit>(&mut tx, cfts);
        env.create_database::<Bytes, Unit>(&mut tx, csft);
        env.create_database::<Bytes, Unit>(&mut tx, cstf);
        env.create_database::<Bytes, Unit>(&mut tx, ctfs);
        env.create_database::<Bytes, Unit>(&mut tx, ctsf);
        tx.commit();
        Self { env }
    }
}

impl Trips<TripsError> for TripsHeed {
    fn collections(&self) -> Result<Vec<String>, TripsError> {
        let mut results: Vec<String> = vec![];
        let tx = self.env.read_txn().unwrap();
        match self
            .env
            .open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id)
            .unwrap()
        {
            Some(db) => {
                for entry in db.iter(&tx).unwrap() {
                    results.push(entry.unwrap().0.to_owned());
                }
            }
            None => todo!(),
        }
        Ok(results)
    }

    fn add_collection(&mut self, collection: String) -> Result<(), TripsError> {
        let mut tx = self.env.write_txn().unwrap();
        match self
            .env
            .open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id)
            .unwrap()
        {
            Some(db) => match db.get(&tx, &collection) {
                Ok(Some(value)) => {
                    return Ok(());
                }
                _ => (),
            },
            None => todo!(),
        }
        let id = match self
            .env
            .open_database::<Str, U64<byteorder::BigEndian>>(&tx, ids)
            .unwrap()
        {
            Some(db) => match db.get(&tx, "id") {
                Ok(Some(value)) => {
                    let next_id = value + 1;
                    db.put(&mut tx, "id", &next_id);
                    next_id
                }
                Ok(None) => {
                    db.put(&mut tx, "id", &0);
                    0
                }
                _ => todo!(),
            },
            None => todo!(),
        };
        match self
            .env
            .open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id)
            .unwrap()
        {
            Some(db) => {
                db.put(&mut tx, &collection, &id);
            }
            None => todo!(),
        }
        match self
            .env
            .open_database::<U64<byteorder::BigEndian>, Str>(&tx, id_to_collection)
            .unwrap()
        {
            Some(db) => {
                db.put(&mut tx, &id, &collection);
            }
            None => todo!(),
        }
        tx.commit();
        Ok(())
    }

    fn remove_collection(&mut self, collection: String) -> Result<(), TripsError> {
        let mut tx = self.env.write_txn().unwrap();
        let id = match self
            .env
            .open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id)
            .unwrap()
        {
            Some(db) => match db.get(&tx, &collection) {
                Ok(Some(id)) => id,
                _ => todo!(),
            },
            None => todo!(),
        };
        match self
            .env
            .open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id)
            .unwrap()
        {
            Some(db) => {
                db.delete(&mut tx, &collection);
            }
            None => todo!(),
        }
        match self
            .env
            .open_database::<U64<byteorder::BigEndian>, Str>(&tx, id_to_collection)
            .unwrap()
        {
            Some(db) => {
                db.delete(&mut tx, &id);
            }
            None => todo!(),
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
        let collection_id = match self
            .env
            .open_database::<Str, U64<byteorder::BigEndian>>(&tx, collection_to_id)
            .unwrap()
        {
            Some(db) => match db.get(&tx, &collection) {
                Ok(Some(id)) => id,
                _ => todo!(),
            },
            None => todo!(),
        };
        let mut value_id = match self
            .env
            .open_database::<Str, U64<byteorder::BigEndian>>(&tx, ids)
            .unwrap()
        {
            Some(db) => match db.get(&tx, "id") {
                Ok(Some(id)) => id,
                _ => todo!(),
            },
            None => todo!(),
        };
        for trip in trips.iter() {
            let (fid, sid, tid) = match self
                .env
                .open_database::<Str, U64<byteorder::BigEndian>>(&tx, value_to_id)
                .unwrap()
            {
                Some(value_to_id_db) => {
                    let fid = match value_to_id_db.get(&tx, &trip.0) {
                        Ok(Some(id)) => id,
                        _ => {
                            match self
                                .env
                                .open_database::<Str, U64<byteorder::BigEndian>>(&tx, id_to_value)
                                .unwrap()
                            {
                                Some(id_to_value_db) => {
                                    value_id = value_id + 1;
                                    value_to_id_db.put(&mut tx, &trip.0, &value_id);
                                    value_id
                                },
                                None => todo!()
                            }
                        }
                    };
                    let sid = match value_to_id_db.get(&tx, &trip.1) {
                        Ok(Some(id)) => id,
                        _ => {
                            match self
                                .env
                                .open_database::<Str, U64<byteorder::BigEndian>>(&tx, id_to_value)
                                .unwrap()
                            {
                                Some(id_to_value_db) => {
                                    value_id = value_id + 1;
                                    value_to_id_db.put(&mut tx, &trip.1, &value_id);
                                    value_id
                                },
                                None => todo!()
                            }
                        },
                    };
                    let tid = match value_to_id_db.get(&tx, &trip.2) {
                        Ok(Some(id)) => id,
                        _ => {
                            match self
                                .env
                                .open_database::<Str, U64<byteorder::BigEndian>>(&tx, id_to_value)
                                .unwrap()
                            {
                                Some(id_to_value_db) => {
                                    value_id = value_id + 1;
                                    value_to_id_db.put(&mut tx, &trip.2, &value_id);
                                    value_id
                                },
                                None => todo!()
                            }
                        },
                    };
                    (fid, sid, tid)
                }
                None => todo!(),
            };

            let cbytes = collection_id.to_be_bytes();
            let fbytes = fid.to_be_bytes();
            let sbytes = sid.to_be_bytes();
            let tbytes = tid.to_be_bytes();

            let cfstbytes: [u8; 32] = merge_arrays(cbytes, fbytes, sbytes, tbytes);
            let cftsbytes: [u8; 32] = merge_arrays(cbytes, fbytes, tbytes, sbytes);
            let csftbytes: [u8; 32] = merge_arrays(cbytes, sbytes, fbytes, tbytes);
            let cstfbytes: [u8; 32] = merge_arrays(cbytes, sbytes, tbytes, fbytes);
            let ctfsbytes: [u8; 32] = merge_arrays(cbytes, tbytes, fbytes, sbytes);
            let ctsfbytes: [u8; 32] = merge_arrays(cbytes, tbytes, sbytes, fbytes);

            match self.env.open_database::<Bytes, Unit>(&tx, cfst).unwrap() {
                Some(db) => {db.put(&mut tx, &cfstbytes, &());},
                None => todo!(),
            }
            match self.env.open_database::<Bytes, Unit>(&tx, cfts).unwrap() {
                Some(db) => {db.put(&mut tx, &cftsbytes, &());},
                None => todo!(),
            }
            match self.env.open_database::<Bytes, Unit>(&tx, csft).unwrap() {
                Some(db) => {db.put(&mut tx, &csftbytes, &());},
                None => todo!(),
            }
            match self.env.open_database::<Bytes, Unit>(&tx, cstf).unwrap() {
                Some(db) => {db.put(&mut tx, &cstfbytes, &());},
                None => todo!(),
            }
            match self.env.open_database::<Bytes, Unit>(&tx, ctfs).unwrap() {
                Some(db) => {db.put(&mut tx, &ctfsbytes, &());},
                None => todo!(),
            }
            match self.env.open_database::<Bytes, Unit>(&tx, ctsf).unwrap() {
                Some(db) => {db.put(&mut tx, &ctsfbytes, &());},
                None => todo!(),
            }
        };
        tx.commit();
        Ok(())
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

fn merge_arrays(a: [u8; 8], b: [u8; 8], c: [u8; 8], d: [u8; 8]) -> [u8; 32] {
    let mut res: [u8; 32] = [0; 32];
    let mut index = 0;
    for val in a {
        res[index] = val;
        index = index + 1;
    }
    for val in b {
        res[index] = val;
        index = index + 1;
    }
    for val in c {
        res[index] = val;
        index = index + 1;
    }
    for val in d {
        res[index] = val;
        index = index + 1;
    }
    res
}
