// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::lexer::Token;
use wander::parser::{parse, Element};

#[test]
fn parse_boolean_true() {
    let input = vec![Token::Boolean(true), Token::Boolean(false), Token::Boolean(true)];
    let res = parse(input);
    let expected = Ok(vec![Element::Boolean(true)]);
    assert_eq!(res, expected);
}
