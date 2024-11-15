// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hashbag::HashBag;
use ligature::{Element, Entry, Ligature};
use ligature_graph::LigatureGraph;
use std::collections::BTreeSet;

#[test]
fn empty_graph() {
    let g = LigatureGraph::new();
    assert_eq!(g.collections().unwrap(), vec![]);
}

#[test]
fn search_empty_graph() {
    let mut g = LigatureGraph::new();
    g.add_collection(Element("test".to_owned()));
    assert_eq!(
        g.query(Element("test".to_owned()), BTreeSet::new())
            .unwrap(),
        HashBag::new()
    );
}

fn statement() -> Entry {
    Entry::Role {
        first: Element("a".to_owned()),
        second: Element("b".to_owned()),
        role: Element("c".to_owned()),
    }
}

fn statements() -> BTreeSet<Entry> {
    let mut statements = BTreeSet::new();
    for statement in vec![
        Entry::Role {
            first: Element("a".to_owned()),
            second: Element("b".to_owned()),
            role: Element("c".to_owned()),
        },
        Entry::Role {
            first: Element("a".to_owned()),
            second: Element("b".to_owned()),
            role: Element("d".to_owned()),
        },
        Entry::Role {
            first: Element("a".to_owned()),
            second: Element("e".to_owned()),
            role: Element("f".to_owned()),
        },
    ] {
        statements.insert(statement);
    }
    statements
}

fn statements_res() -> BTreeSet<Entry> {
    let mut statements = BTreeSet::new();
    for statement in vec![
        Entry::Role {
            first: Element("a".to_owned()),
            second: Element("b".to_owned()),
            role: Element("c".to_owned()),
        },
        Entry::Role {
            first: Element("a".to_owned()),
            second: Element("b".to_owned()),
            role: Element("d".to_owned()),
        },
    ] {
        statements.insert(statement);
    }
    statements
}

#[test]
fn single_element_graph() {
    let mut entries: BTreeSet<Entry> = BTreeSet::new();
    entries.insert(statement());
    let mut g = LigatureGraph::new();
    g.add_collection(Element("test".to_owned()));
    g.add_entries(Element("test".to_owned()), &mut entries);
    let mut res1 = BTreeSet::new();
    res1.insert(statement());
    assert_eq!(g.entries(Element("test".to_owned())).unwrap(), res1);
    //assert_eq!(g.query(Element("test".to_owned()), BTreeSet::new()).unwrap(), res1);
    // assert_eq!(
    //     g.find(Some(Identifier::new("a").unwrap()), None, None),
    //     res1
    // );
    // assert_eq!(
    //     g.find(Some(Identifier::new("b").unwrap()), None, None),
    //     BTreeSet::new()
    // );
}

// #[test]
// fn multi_statement_graph() {
//     let g = Graph::new(statements());
//     let res1 = statements();
//     assert_eq!(g.all_statements(), res1);
//     assert_eq!(g.find(None, None, None), res1);
//     assert_eq!(
//         g.find(None, Some(Identifier::new("b").unwrap()), None),
//         statements_res()
//     );
//     assert_eq!(
//         g.find(Some(Identifier::new("c").unwrap()), None, None),
//         BTreeSet::new()
//     );
// }

// #[test]
// fn add_empty_graphs() {
//     let g1 = Graph::default();
//     let g2 = Graph::default();
//     let g3 = g1.add_all(g2);
//     assert_eq!(g3, Graph::default());
// }

// #[test]
// fn add_graphs() {
//     let g1 = Graph::default();
//     let g2 = Graph::new(statements());
//     let g3 = g1.add_all(g2);
//     assert_eq!(g3, Graph::new(statements()));
// }
