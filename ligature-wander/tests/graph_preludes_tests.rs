// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Identifier, Statement, Value};
use ligature_graph::Graph;
use std::collections::BTreeSet;
use wander::{preludes::common, run, WanderValue};

#[test]
fn empty_graph() {
    let input = "Graph.empty()";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Graph(Graph::default()));
    assert_eq!(res, expected);
}

#[test]
fn graph_with_empty_statements_list() {
    let input = "Graph.graph([])";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Graph(Graph::default()));
    assert_eq!(res, expected);
}

#[test]
fn graph_with_statements_list() {
    let input = "Graph.graph([(<a> <b> <c>)(<a> <b> 123)])";
    let res = run(input, &mut common());
    let mut statements = BTreeSet::default();
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Identifier(Identifier::new("c").unwrap()),
    });
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Integer(123),
    });
    let expected = Ok(WanderValue::Graph(Graph::new(statements)));
    assert_eq!(res, expected);
}

#[test]
fn graph_union() {
    let input = "Graph.union(Graph.graph([(<a> <b> <c>)]) Graph.graph([(<a> <b> 123)]))";
    let res = run(input, &mut common());
    let mut statements = BTreeSet::default();
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Identifier(Identifier::new("c").unwrap()),
    });
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Integer(123),
    });
    let expected = Ok(WanderValue::Graph(Graph::new(statements)));
    assert_eq!(res, expected);
}

#[test]
fn graph_difference() {
    let input =
        "Graph.difference(Graph.graph([(<a> <b> <c>)(<a> <b> \"dog\")]) Graph.graph([(<a> <b> <c>) (<a> <b> 123)]))";
    let res = run(input, &mut common());
    let mut statements = BTreeSet::default();
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::String("dog".to_owned()),
    });
    let expected = Ok(WanderValue::Graph(Graph::new(statements)));
    assert_eq!(res, expected);
}

#[test]
fn graph_statements() {
    let input = "Graph.statements(Graph.graph([(<a> <b> <c>)]))";
    let res = run(input, &mut common());
    let mut statements = BTreeSet::default();
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Identifier(Identifier::new("c").unwrap()),
    });
    let expected = Ok(WanderValue::List(vec![WanderValue::Tuple(vec![
        WanderValue::Identifier(Identifier::new("a").unwrap()),
        WanderValue::Identifier(Identifier::new("b").unwrap()),
        WanderValue::Identifier(Identifier::new("c").unwrap()),
    ])]));
    assert_eq!(res, expected);
}

#[test]
fn graph_transformer_test() {
    let input = "Graph.graph`<a> <b> [<c> \"c\" 20]`";
    let res = run(input, &mut common());
    let mut statements = BTreeSet::default();
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Identifier(Identifier::new("c").unwrap()),
    });
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::String("c".to_owned()),
    });
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Integer(20),
    });
    let expected = Ok(WanderValue::Graph(Graph::new(statements)));
    assert_eq!(res, expected);
}
