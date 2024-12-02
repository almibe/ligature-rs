// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hashbag::HashBag;
#[cfg(feature = "heed")]
use heed::{Env, EnvOpenOptions};
use std::collections::{BTreeMap, BTreeSet};
#[cfg(feature = "heed")]
use trips::heed::TripsHeed;
use trips::TripsError;
use trips::{Query, Slot, Trip, Trips};

#[cfg(feature = "heed")]
fn create_temp() -> Env {
    let dir = tempfile::tempdir().unwrap();
    let env = unsafe { EnvOpenOptions::new().max_dbs(24).open(dir.path()).unwrap() };
    // let mut wtxn = env.write_txn()?;
    // let db: Database<Str, U32<byteorder::NativeEndian>> = env.create_database(&mut wtxn, Some("test"))?;
    env
}

#[test]
#[cfg(feature = "heed")]
fn store_should_start_empty() {
    let env = create_temp();
    let store = trips::heed::TripsHeed::new(env);
    let collections: Vec<String> = <TripsHeed as Trips>::collections(&store).unwrap();
    let result: Vec<String> = vec![];
    assert_eq!(collections, result);
}

#[test]
#[cfg(feature = "heed")]
fn add_collection_to_store() {
    let env = create_temp();
    let mut store = TripsHeed::new(env);
    let _ = store.add_collection("T".to_owned());
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec!["T".to_owned()];
    assert_eq!(collections, result);
}

#[test]
#[cfg(feature = "heed")]
fn remove_collection_from_store() {
    let env = create_temp();
    let mut store = TripsHeed::new(env);
    let _ = store.add_collection("T".to_owned());
    let _ = store.add_collection("S".to_owned());
    let _ = store.remove_collection("T".to_owned());
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec!["S".to_owned()];
    assert_eq!(collections, result);
}

#[test]
#[cfg(feature = "heed")]
fn triples_should_start_empty() {
    let env = create_temp();
    let mut store = TripsHeed::new(env);
    let _ = store.add_collection("T".to_owned());
    let collections = store.triples("T".to_owned()).unwrap();
    let result = BTreeSet::new();
    assert_eq!(collections, result);
}

#[test]
#[cfg(feature = "heed")]
fn add_triples_to_collection() {
    let env = create_temp();
    let mut store: TripsHeed = TripsHeed::new(env);
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
#[cfg(feature = "heed")]
fn remove_triples_from_collection() {
    let env = create_temp();
    let mut store = TripsHeed::new(env);
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
#[cfg(feature = "heed")]
fn match_all_query_collection() {
    let env = create_temp();
    let mut store = TripsHeed::new(env);
    let _ = store.add_collection("T".to_owned());
    let _ = store.add_triples(
        "T".to_owned(),
        &mut BTreeSet::from([
            Trip("1".to_owned(), "2".to_owned(), "3".to_owned()),
            Trip("1".to_owned(), "2".to_owned(), "6".to_owned()),
            Trip("1".to_owned(), "2".to_owned(), "5".to_owned()),
        ]),
    );
    let results = store
        .query(
            "T".to_owned(),
            BTreeSet::from([Query(Slot::Any, Slot::Any, Slot::Any)]),
        )
        .unwrap();
    let expected: HashBag<BTreeMap<String, String>> = HashBag::from_iter([
        BTreeMap::from_iter([]),
        BTreeMap::from_iter([]),
        BTreeMap::from_iter([]),
    ]);
    assert_eq!(results, expected);
}

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
