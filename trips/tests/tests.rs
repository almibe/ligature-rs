// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hashbag::HashBag;
use std::collections::{BTreeMap, BTreeSet};
use trips::mem::TripsMem;
use trips::{Query, Slot, Trip, Trips};

#[cfg(feature = "heed")]
use heed::{Env, EnvOpenOptions};
#[cfg(feature = "heed")]
use trips::heed::TripsHeed;
use trips::TripsError;

#[cfg(feature = "duckdb")]
use trips::duckdb::TripsDuckDB;

#[cfg(feature = "heed")]
fn initialize() -> Box<dyn Trips> {
    let dir = tempfile::tempdir().unwrap();
    let env = unsafe { EnvOpenOptions::new().max_dbs(24).open(dir.path()).unwrap() };
    Box::new(trips::heed::TripsHeed::new(env))
}

#[cfg(feature = "duckdb")]
fn initialize() -> Box<dyn Trips> {
    Box::new(TripsDuckDB::new())
}

#[cfg(not(any(feature = "heed", feature = "duckdb")))]
fn initialize() -> Box<dyn Trips> {
    Box::new(trips::mem::TripsMem::new())
}

#[test]
fn store_should_start_empty() {
    let store = initialize();
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec![];
    assert_eq!(collections, result);
}

#[test]
fn add_collection_to_store() {
    let mut store = initialize();
    let _ = store.add_collection("T".to_owned());
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec!["T".to_owned()];
    assert_eq!(collections, result);
}

#[test]
fn remove_collection_from_store() {
    let mut store = initialize();
    let _ = store.add_collection("T".to_owned());
    let _ = store.add_collection("S".to_owned());
    let _ = store.remove_collection("T".to_owned());
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec!["S".to_owned()];
    assert_eq!(collections, result);
}

#[test]
fn triples_should_start_empty() {
    let mut store = initialize();
    let _ = store.add_collection("T".to_owned());
    let collections: BTreeSet<Trip> = store.triples("T".to_owned()).unwrap();
    let result: BTreeSet<Trip> = BTreeSet::new();
    assert_eq!(collections, result);
}

#[test]
fn add_triples_to_collection() {
    let mut store = initialize();
    let _ = store.add_collection("T".to_owned());
    let _ = store.add_triples(
        "T".to_owned(),
        &mut BTreeSet::from([Trip("1".to_owned(), "2".to_owned(), "3".to_owned())]),
    );
    let collections: BTreeSet<Trip> = store.triples("T".to_owned()).unwrap();
    let result: BTreeSet<Trip> =
        BTreeSet::from([Trip("1".to_owned(), "2".to_owned(), "3".to_owned())]);
    assert_eq!(collections, result);
}

#[test]
fn remove_triples_from_collection() {
    let mut store = initialize();
    let _ = store.add_collection("T".to_owned());
    let _ = store.add_triples(
        "T".to_owned(),
        &mut BTreeSet::from([
            Trip("1".to_owned(), "2".to_owned(), "3".to_owned()),
            Trip("1".to_owned(), "2".to_owned(), "6".to_owned()),
            Trip("1".to_owned(), "2".to_owned(), "5".to_owned()),
        ]),
    );
    let _ = store.remove_triples(
        "T".to_owned(),
        &mut BTreeSet::from([Trip("1".to_owned(), "2".to_owned(), "3".to_owned())]),
    );
    let collections: BTreeSet<Trip> = store.triples("T".to_owned()).unwrap();
    let result: BTreeSet<Trip> = BTreeSet::from([
        Trip("1".to_owned(), "2".to_owned(), "5".to_owned()),
        Trip("1".to_owned(), "2".to_owned(), "6".to_owned()),
    ]);
    assert_eq!(collections, result);
}

#[test]
fn basic_filter() {
    let mut store = initialize();
    let _ = store.add_collection("T".to_owned());
    let _ = store.add_triples(
        "T".to_owned(),
        &mut BTreeSet::from([
            Trip("1".to_owned(), "2".to_owned(), "3".to_owned()),
            Trip("1".to_owned(), "2".to_owned(), "6".to_owned()),
            Trip("1".to_owned(), "2".to_owned(), "5".to_owned()),
        ]),
    );
    let _ = store.filter("T".to_owned(), Query(Slot::Any, Slot::Any, Slot::Any));
    let collections: BTreeSet<Trip> = store.triples("T".to_owned()).unwrap();
    let result: BTreeSet<Trip> = BTreeSet::from([
        Trip("1".to_owned(), "2".to_owned(), "3".to_owned()),
        Trip("1".to_owned(), "2".to_owned(), "5".to_owned()),
        Trip("1".to_owned(), "2".to_owned(), "6".to_owned()),
    ]);
    assert_eq!(collections, result);
}

// #[test]
// fn match_all_query_collection() {
//     let mut store: TripsMem = trips::mem::TripsMem::new();
//     let _ = store.add_collection("T".to_owned());
//     let _ = store.add_triples(
//         "T".to_owned(),
//         &mut BTreeSet::from([
//             Trip("1".to_owned(), "2".to_owned(), "3".to_owned()),
//             Trip("1".to_owned(), "2".to_owned(), "6".to_owned()),
//             Trip("1".to_owned(), "2".to_owned(), "5".to_owned()),
//         ]),
//     );
//     let results = store
//         .query(
//             "T".to_owned(),
//             BTreeSet::from([Query(Slot::Any, Slot::Any, Slot::Any)]),
//         )
//         .unwrap();
//     let expected = HashBag::from_iter([
//         BTreeMap::from_iter([]),
//         BTreeMap::from_iter([]),
//         BTreeMap::from_iter([]),
//     ]);
//     assert_eq!(results, expected);
// }

// #[test]
// fn basic_query_collection() {
//     let mut store: TripsMem = trips::mem::TripsMem::new();
//     let _ = store.add_collection("T".to_owned());
//     let _ = store.add_triples(
//         "T".to_owned(),
//         &mut BTreeSet::from([
//             Trip("1".to_owned(), "2".to_owned(), "3".to_owned()),
//             Trip("1".to_owned(), "2".to_owned(), "6".to_owned()),
//             Trip("1".to_owned(), "2".to_owned(), "5".to_owned()),
//         ]),
//     );
//     let results = store
//         .query(
//             "T".to_owned(),
//             BTreeSet::from([Query(
//                 Slot::Variable("A".to_owned()),
//                 Slot::Value("2".to_owned()),
//                 Slot::Any,
//             )]),
//         )
//         .unwrap();
//     let expected = HashBag::from_iter([
//         BTreeMap::from_iter([("A".to_owned(), "1".to_owned())]),
//         BTreeMap::from_iter([("A".to_owned(), "1".to_owned())]),
//         BTreeMap::from_iter([("A".to_owned(), "1".to_owned())]),
//     ]);
//     assert_eq!(results, expected);
// }

// #[test]
// fn complex_single_query_collection() {
//     let mut store: TripsMem = trips::mem::TripsMem::new();
//     let _ = store.add_collection("T".to_owned());
//     let _ = store.add_triples(
//         "T".to_owned(),
//         &mut BTreeSet::from([
//             Trip("2".to_owned(), "2".to_owned(), "3".to_owned()),
//             Trip("1".to_owned(), "2".to_owned(), "6".to_owned()),
//             Trip("1".to_owned(), "3".to_owned(), "5".to_owned()),
//         ]),
//     );
//     let results = store
//         .query(
//             "T".to_owned(),
//             BTreeSet::from([
//                 Query(
//                     Slot::Variable("A".to_owned()),
//                     Slot::Value("2".to_owned()),
//                     Slot::Variable("C".to_owned()),
//                 ),
//                 Query(Slot::Any, Slot::Variable("C".to_owned()), Slot::Any),
//             ]),
//         )
//         .unwrap();
//     let expected = HashBag::from_iter([
//         BTreeMap::from_iter([
//             ("A".to_owned(), "1".to_owned()),
//             ("C".to_owned(), "6".to_owned()),
//         ]),
//         BTreeMap::from_iter([
//             ("A".to_owned(), "2".to_owned()),
//             ("C".to_owned(), "3".to_owned()),
//         ]),
//     ]);
//     assert_eq!(results, expected);
// }
