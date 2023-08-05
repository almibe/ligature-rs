// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Identifier, Statement, Value};
use ligature_graph::Graph;
use std::collections::BTreeSet;

#[test]
fn empty_graph() {
    let g = Graph::default();
    assert_eq!(g.all_statements(), BTreeSet::new());
}

#[test]
fn search_empty_graph() {
    let g = Graph::default();
    assert_eq!(g.find(None, None, None), BTreeSet::new());
}

fn statement() -> Statement {
    Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Identifier(Identifier::new("c").unwrap()),
    }
}

fn statements() -> BTreeSet<Statement> {
    let mut statements = BTreeSet::new();
    for statement in vec![
        Statement {
            entity: Identifier::new("a").unwrap(),
            attribute: Identifier::new("b").unwrap(),
            value: Value::Identifier(Identifier::new("c").unwrap()),
        },
        Statement {
            entity: Identifier::new("a").unwrap(),
            attribute: Identifier::new("b").unwrap(),
            value: Value::Identifier(Identifier::new("d").unwrap()),
        },
        Statement {
            entity: Identifier::new("a").unwrap(),
            attribute: Identifier::new("e").unwrap(),
            value: Value::Identifier(Identifier::new("f").unwrap()),
        },
    ] {
        statements.insert(statement);
    }
    statements
}

fn statements_res() -> BTreeSet<Statement> {
    let mut statements = BTreeSet::new();
    for statement in vec![
        Statement {
            entity: Identifier::new("a").unwrap(),
            attribute: Identifier::new("b").unwrap(),
            value: Value::Identifier(Identifier::new("c").unwrap()),
        },
        Statement {
            entity: Identifier::new("a").unwrap(),
            attribute: Identifier::new("b").unwrap(),
            value: Value::Identifier(Identifier::new("d").unwrap()),
        },
    ] {
        statements.insert(statement);
    }
    statements
}

#[test]
fn single_element_graph() {
    let mut statements = BTreeSet::new();
    statements.insert(statement());
    let g = Graph::new(statements);
    let mut res1 = BTreeSet::new();
    res1.insert(statement());
    assert_eq!(g.all_statements(), res1);
    assert_eq!(g.find(None, None, None), res1);
    assert_eq!(
        g.find(Some(Identifier::new("a").unwrap()), None, None),
        res1
    );
    assert_eq!(
        g.find(Some(Identifier::new("b").unwrap()), None, None),
        BTreeSet::new()
    );
}

#[test]
fn multi_statement_graph() {
    let g = Graph::new(statements());
    let res1 = statements();
    assert_eq!(g.all_statements(), res1);
    assert_eq!(g.find(None, None, None), res1);
    assert_eq!(
        g.find(None, Some(Identifier::new("b").unwrap()), None),
        statements_res()
    );
    assert_eq!(
        g.find(Some(Identifier::new("c").unwrap()), None, None),
        BTreeSet::new()
    );
}

#[test]
fn add_empty_graphs() {
    let g1 = Graph::default();
    let g2 = Graph::default();
    let g3 = g1.add_all(g2);
    assert_eq!(g3, Graph::default());
}

#[test]
fn add_graphs() {
    let g1 = Graph::default();
    let g2 = Graph::new(statements());
    let g3 = g1.add_all(g2);
    assert_eq!(g3, Graph::new(statements()));
}
