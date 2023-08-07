// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;
use wander::{lexer::Token, preludes::common, run, ScriptValue, TokenTransformer};

struct NothingTransformer {}
impl TokenTransformer for NothingTransformer {
    fn transform(
        &self,
        _input: &[wander::lexer::Token],
    ) -> Result<Vec<Token>, ligature::LigatureError> {
        Ok([Token::Nothing].to_vec())
    }
}

struct UpperCaseTransformer {}
impl TokenTransformer for UpperCaseTransformer {
    fn transform(
        &self,
        input: &[wander::lexer::Token],
    ) -> Result<Vec<Token>, ligature::LigatureError> {
        if let Some(Token::String(value)) = input.get(0) {
            let t = value.clone().to_ascii_uppercase();
            let t = Token::String(t);
            Ok(vec![t])
        } else {
            panic!()
        }
    }
}

#[test]
fn token_transformer_no_input_test() {
    let input = "none``";
    let mut bindings = common();
    bindings.bind_token_transformer("none".to_owned(), Rc::new(NothingTransformer {}));
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_none() {
    let input = "none`this (will) >>  [be ] {ignored}} `";
    let mut bindings = common();
    bindings.bind_token_transformer("none".to_owned(), Rc::new(NothingTransformer {}));
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_upper() {
    let input = "upper`\"test\"`";
    let mut bindings = common();
    bindings.bind_token_transformer("upper".to_owned(), Rc::new(UpperCaseTransformer {}));
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::String("TEST".to_owned()));
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_graph() {
    let input = "graph`<a> <b> <c>`";
    let res = run(input, &mut common());
    let expected = run("[[<a> <b> <c>]]", &mut common());
    assert_eq!(res, expected);
}
