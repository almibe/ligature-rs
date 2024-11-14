// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Element, Ligature};
use ligature_graph::LigatureGraph;
use std::collections::BTreeSet;
use hashbag::HashBag;

#[test]
fn empty_graph() {
    let g = LigatureGraph::new();
    assert_eq!(g.collections().unwrap(), vec![]);
}

#[test]
fn search_empty_graph() {
    let g = LigatureGraph::new();
    assert_eq!(g.query(Element("test".to_owned()) ,BTreeSet::new()).unwrap(), HashBag::new());
}

// fn statement() -> Role {
//     Role {
//         first: Identifier::new("a").unwrap(),
//         second: Identifier::new("b").unwrap(),
//         role: Value::Identifier(Identifier::new("c").unwrap()),
//     }
// }

// fn statements() -> BTreeSet<Role> {
//     let mut statements = BTreeSet::new();
//     for statement in vec![
//         Role {
//             first: Identifier::new("a").unwrap(),
//             second: Identifier::new("b").unwrap(),
//             role: Value::Identifier(Identifier::new("c").unwrap()),
//         },
//         Role {
//             first: Identifier::new("a").unwrap(),
//             second: Identifier::new("b").unwrap(),
//             role: Value::Identifier(Identifier::new("d").unwrap()),
//         },
//         Role {
//             first: Identifier::new("a").unwrap(),
//             second: Identifier::new("e").unwrap(),
//             role: Value::Identifier(Identifier::new("f").unwrap()),
//         },
//     ] {
//         statements.insert(statement);
//     }
//     statements
// }

// fn statements_res() -> BTreeSet<Role> {
//     let mut statements = BTreeSet::new();
//     for statement in vec![
//         Role {
//             first: Identifier::new("a").unwrap(),
//             second: Identifier::new("b").unwrap(),
//             role: Value::Identifier(Identifier::new("c").unwrap()),
//         },
//         Role {
//             first: Identifier::new("a").unwrap(),
//             second: Identifier::new("b").unwrap(),
//             role: Value::Identifier(Identifier::new("d").unwrap()),
//         },
//     ] {
//         statements.insert(statement);
//     }
//     statements
// }

// #[test]
// fn single_element_graph() {
//     let mut statements = BTreeSet::new();
//     statements.insert(statement());
//     let g = Graph::new(statements);
//     let mut res1 = BTreeSet::new();
//     res1.insert(statement());
//     assert_eq!(g.all_statements(), res1);
//     assert_eq!(g.find(None, None, None), res1);
//     assert_eq!(
//         g.find(Some(Identifier::new("a").unwrap()), None, None),
//         res1
//     );
//     assert_eq!(
//         g.find(Some(Identifier::new("b").unwrap()), None, None),
//         BTreeSet::new()
//     );
// }

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
