// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::{BTreeSet, HashSet};

use ligature::{Element, Entry};
use trips::{Trip, Trips};
use trips::mem::{TripsMem, TripsError};
use wander::{preludes::common, run, WanderValue};

#[test]
fn calling_ignore() {
    let input = "ignore";
    let res = run(input, common(), &mut TripsMem::new());
    let expected = Ok(WanderValue::Network(HashSet::new()));
    assert_eq!(res, expected);
}

#[test]
fn calling_ignore_with_args() {
    let input = "ignore test (test {test test test}) {test test test}";
    let res = run(input, common(), &mut TripsMem::new());
    let expected = Ok(WanderValue::Network(HashSet::new()));
    assert_eq!(res, expected);
}

#[test]
fn basic_let() {
    let input = "let test {a b c}";
    let mut store = TripsMem::new();
    let _ = run(input, common(), &mut store);
    let mut expected: TripsMem<Element, Element> = TripsMem::new();
    let _ = expected.add_collection(Element("test".to_owned()));
    let _ = expected.add_triples(Element("test".to_owned()), &mut BTreeSet::from([
        Trip(Element("a".to_owned()), Element("b".to_owned()), Element("c".to_owned()))
    ]));
    assert_eq!(store, expected);
}

#[test]
fn passing_assert_eq_call() {
    let input = "assert-eq true true";
    let res = run(input, common(), &mut TripsMem::new());
    let expected = Ok(WanderValue::Network(HashSet::new()));
    assert_eq!(res, expected);
}

#[test]
fn failing_assert_eq_call() {
    let input = "assert-eq true bug";
    let res = run(input, common(), &mut TripsMem::new());
    assert!(res.is_err());
}
