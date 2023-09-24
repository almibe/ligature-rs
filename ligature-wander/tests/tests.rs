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

#[test]
fn tokenize_identifier() {
    let expected_identifier = Identifier::new("hello123").unwrap();
    let input = "<hello123>";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Identifier(expected_identifier)]);
    assert_eq!(res, expected);
}

#[test]
fn run_wander_identifier() {
    let expected_identifier = Identifier::new("hello").unwrap();
    let input = "<hello>";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Identifier(expected_identifier));
    assert_eq!(res, expected);
}

#[test]
fn graph_literal_with_string() {
    let input = "graph`<a> <b> \"\\\"\"`";
    let res = tokenize(input);
    let expected = Ok(vec![
        Token::Name("graph".to_owned()),
        Token::Backtick,
        Token::Identifier(Identifier::new("a").unwrap()),
        Token::Identifier(Identifier::new("b").unwrap()),
        Token::String("\"\\\"\"".to_owned()),
        Token::Backtick,
    ]);
    assert_eq!(res, expected);
}

#[test]
fn write_entities() -> Result<(), LigatureError> {
    let e = Identifier::new("test")?;
    assert_eq!(write_identifier(&e), "<test>".to_string());
    Ok(())
}

#[test]
fn write_string_literals() {
    assert_eq!(write_value(&Value::String("test".to_string())), "\"test\"");
}

#[test]
fn write_integer_literals() {
    assert_eq!(write_value(&Value::Integer(5)), "5");
}

#[test]
fn write_bytes_literals() {
    assert_eq!(write_value(&Value::Bytes(vec![0, 255])), "0x00ff");
}
