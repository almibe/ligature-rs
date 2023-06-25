// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::LigatureError;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Element {
    Boolean(bool)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Element>, LigatureError> {
    Ok(vec![Element::Boolean(true)])
}
