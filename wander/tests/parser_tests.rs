// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::{BTreeSet, HashSet};

use ligature::{Element, Entry};
use wander::{Call, Quote, WanderError, WanderValue};

fn parse_str(script: &str) -> Result<Vec<Call>, WanderError> {
    match wander::lexer::tokenize_and_filter(script) {
        Ok(results) => wander::parser::parse(results),
        Err(_) => todo!(),
    }
}

#[test]
fn no_arg_command() {
    let res = parse_str("true");
    let expected = Ok(vec![Call {
        name: Element("true".to_owned()),
        arguments: vec![],
    }]);
    assert_eq!(res, expected);
}

#[test]
fn parse_integers() {
    let res = parse_str("-100");
    let expected = Ok(vec![Call {
        name: Element("-100".to_owned()),
        arguments: vec![],
    }]);
    assert_eq!(res, expected);
}

#[test]
fn parse_strings() {
    let res = parse_str("\"Hello\"");
    let expected = Ok(vec![Call {
        name: Element("Hello".to_owned()),
        arguments: vec![],
    }]);
    assert_eq!(res, expected);
}

#[test]
fn parse_call_with_args() {
    let res = parse_str("Bool.and x true");
    let expected = Ok(vec![Call {
        name: Element("Bool.and".to_owned()),
        arguments: vec![
            WanderValue::Element(Element("x".to_owned())),
            WanderValue::Element(Element("true".to_owned())),
        ],
    }]);
    assert_eq!(res, expected);
}

#[test]
fn parse_applications() {
    let res = parse_str("Bool.not x, true");
    let expected = Ok(vec![
        Call {
            name: Element("Bool.not".to_owned()),
            arguments: vec![WanderValue::Element(Element("x".to_owned()))],
        },
        Call {
            name: Element("true".to_owned()),
            arguments: vec![],
        },
    ]);
    assert_eq!(res, expected);
}

#[test]
fn parse_empty_network_as_() {
    let res = parse_str("test {}");
    let expected = Ok(vec![Call {
        name: Element("test".to_owned()),
        arguments: vec![WanderValue::Network(BTreeSet::from([]))],
    }]);
    assert_eq!(res, expected);
}

#[test]
fn parse_network_with_single_entry() {
    let res = parse_str("test {a b c}");
    let expected = Ok(vec![Call {
        name: Element("test".to_owned()),
        arguments: vec![WanderValue::Network(BTreeSet::from([Entry::Role {
            first: Element("a".to_owned()),
            second: Element("c".to_owned()),
            role: Element("b".to_owned()),
        }]))],
    }]);
    assert_eq!(res, expected);
}

#[test]
fn parse_network_with_single_entry_and_trailing_comma() {
    let res = parse_str("test {a b c,}");
    let expected = Ok(vec![Call {
        name: Element("test".to_owned()),
        arguments: vec![WanderValue::Network(BTreeSet::from([Entry::Role {
            first: Element("a".to_owned()),
            second: Element("c".to_owned()),
            role: Element("b".to_owned()),
        }]))],
    }]);
    assert_eq!(res, expected);
}

#[test]
fn parse_network_with_two_entries_and_trailing_comma() {
    let res = parse_str("test {a b c, a : A,}");

    let expected = Ok(vec![Call {
        name: Element("test".to_owned()),
        arguments: vec![WanderValue::Network(BTreeSet::from([
            Entry::Role {
                first: Element("a".to_owned()),
                second: Element("c".to_owned()),
                role: Element("b".to_owned()),
            },
            Entry::Extends {
                element: Element("a".to_owned()),
                concept: Element("A".to_owned()),
            },
        ]))],
    }]);
    assert_eq!(res, expected);
}

#[test]
fn parse_nested_function_calls() {
    let res = parse_str("Bool.not (Bool.not false)");
    let expected = Ok(vec![Call {
        name: Element("Bool.not".to_owned()),
        arguments: vec![WanderValue::Quote(Quote(vec![
            WanderValue::Element(Element("Bool.not".to_owned())),
            WanderValue::Element(Element("false".to_owned())),
        ]))],
    }]);
    assert_eq!(res, expected);
}
