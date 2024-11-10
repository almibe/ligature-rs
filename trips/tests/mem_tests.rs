// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use trips::{Trip, Trips};
use trips::mem::TripsMem;
use std::collections::BTreeSet;

#[test]
fn store_should_start_empty() {
    let store: TripsMem<String, String> = trips::mem::TripsMem::new();
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec![];
    assert_eq!(collections, result);
}

#[test]
fn add_collection_to_store() {
    let mut store: TripsMem<String, String> = trips::mem::TripsMem::new();
    let _ = store.add_collection("T".to_owned());
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec!["T".to_owned()];
    assert_eq!(collections, result);
}

#[test]
fn remove_collection_from_store() {
    let mut store: TripsMem<String, String> = trips::mem::TripsMem::new();
    let _ = store.add_collection("T".to_owned());
    let _ = store.add_collection("S".to_owned());
    let _ = store.remove_collection("T".to_owned());
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec!["S".to_owned()];
    assert_eq!(collections, result);
}

#[test]
fn triples_should_start_empty() {
    let mut store: TripsMem<String, String> = trips::mem::TripsMem::new();
    let _ = store.add_collection("T".to_owned());
    let collections: BTreeSet<Trip<String>> = store.triples("T".to_owned()).unwrap();
    let result: BTreeSet<Trip<String>> = BTreeSet::new();
    assert_eq!(collections, result);
}

#[test]
fn add_triples_to_collection() {
    let mut store: TripsMem<String, u64> = trips::mem::TripsMem::new();
    let _ = store.add_collection("T".to_owned());
    let _ = store.add_triples("T".to_owned(), &mut BTreeSet::from([Trip(1, 2, 3)]));
    let collections: BTreeSet<Trip<u64>> = store.triples("T".to_owned()).unwrap();
    let result: BTreeSet<Trip<u64>> = BTreeSet::from([Trip(1, 2, 3)]);
    assert_eq!(collections, result);
}
