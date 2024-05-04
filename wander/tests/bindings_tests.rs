// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{environment::Environment, NoHostType, WanderValue};

#[test]
fn new_bindings_should_be_empty() {
    let bindings = Environment::<NoHostType>::new();
    let result = bindings.bound_names();
    assert!(result.is_empty());
}

#[test]
fn bind_single_value() {
    let mut bindings = Environment::<NoHostType>::new();
    bindings.bind(String::from("hello"), WanderValue::Int(3));
    let result = bindings.bound_names();
    assert!(result.contains(&String::from("hello")));
    let read_result = bindings.read(&String::from("hello"));
    let none_result = bindings.read(&String::from("nope"));
    assert_eq!(read_result, Some(WanderValue::Int(3)));
    assert_eq!(none_result, None);
}
