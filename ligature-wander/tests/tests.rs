// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature_graph::Graph;

#[test]
fn token_transformer_graph_empty() {
    let input = "Graph.graph``";
    let res = run(input, &mut common());
    let expected = run("Graph.graph([])", &mut common());
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_graph() {
    let input = "Graph.graph`<a> <b> <c>`";
    let res = run(input, &mut common());
    let expected = run("Graph.graph([(<a> <b> <c>)])", &mut common());
    assert_eq!(res, expected);
}


#[test]
fn read_write_test_strings_in_graph() {
    let input =
        "Graph.graph`<a> <b> [<c> \"\" \"hello, world\" \"\\\"\" \"\\\"hello,\\nworld\\\"\"]`";
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
    let expected = WanderValue::Graph(Graph::new(statements));
    assert_eq!(res, expected);
}
