// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Identifier, Statement, Value};
use ligature_graph::Graph;
use std::collections::BTreeSet;
use wander::{preludes::common, run, ScriptValue};

#[test]
fn read_write_test_strings() {
    let input = vec![
        "\"\"".to_owned(),
        "\"hello, world\"".to_owned(),
        "\"hello,\\nworld\"".to_owned(),
    ];
    let res: Vec<ScriptValue> = input
        .iter()
        .map(|s| run(s, &mut common()).unwrap())
        .collect();
    let res: Vec<String> = res.iter().map(|s| format!("{s}")).collect();
    assert_eq!(input, res);
}

#[test]
fn read_write_test_strings_in_graph() {
    let input = "graph`<a> <b> [<c> \"\" \"hello, world\" \"\\\"\" \"\\\"hello,\\nworld\\\"\"]`";
    let res = run(input, &mut common()).unwrap();
    let res = format!("{res}");
    let res = run(&res, &mut common()).unwrap();
    let mut statements = BTreeSet::new();
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::Identifier(Identifier::new("c").unwrap()),
    });
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::String("".to_owned()),
    });
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::String("hello, world".to_owned()),
    });
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::String("\"".to_owned()),
    });
    statements.insert(Statement {
        entity: Identifier::new("a").unwrap(),
        attribute: Identifier::new("b").unwrap(),
        value: Value::String("\"hello,\nworld\"".to_owned()),
    });
    let expected = ScriptValue::Graph(Graph::new(statements));
    assert_eq!(res, expected);
}
