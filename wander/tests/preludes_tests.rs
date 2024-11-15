// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::{BTreeSet, HashSet};

use ligature::{Element, Entry, Ligature};
use trips::{Trip, Trips};
use trips::mem::{TripsMem, TripsError};
use ligature_graph::LigatureGraph;
use wander::{preludes::common, run, WanderValue};

#[test]
fn calling_ignore() {
    let input = "ignore";
    let res = run(input, common(), &mut LigatureGraph::new());
    let expected = Ok(WanderValue::Network(BTreeSet::new()));
    assert_eq!(res, expected);
}

#[test]
fn calling_ignore_with_args() {
    let input = "ignore test (test {test test test}) {test test test}";
    let res = run(input, common(), &mut LigatureGraph::new());
    let expected = Ok(WanderValue::Network(BTreeSet::new()));
    assert_eq!(res, expected);
}

#[test]
fn passing_assert_eq_call() {
    let input = "assert-eq true true";
    let res = run(input, common(), &mut LigatureGraph::new());
    let expected = Ok(WanderValue::Network(BTreeSet::new()));
    assert_eq!(res, expected);
}

#[test]
fn failing_assert_eq_call() {
    let input = "assert-eq true bug";
    let res = run(input, common(), &mut LigatureGraph::new());
    assert!(res.is_err());
}
