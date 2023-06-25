// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::lexer::{tokenize, Token};

#[test]
fn tokenize_boolean_true() {
    let input = "true";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Boolean(true)]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_boolean_false() {
    let input = "false";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Boolean(false)]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_booleans() {
    let input = "true false false";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Boolean(true), Token::Boolean(false), Token::Boolean(false)]);
    assert_eq!(res, expected);
}
