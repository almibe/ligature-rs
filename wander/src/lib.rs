// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the Wander scripting language.

use interpreter::eval;
use ligature::LigatureError;

pub mod lexer;
pub mod parser;
pub mod interpreter;
use crate::lexer::tokenize;
use crate::parser::parse;

#[derive(Debug, PartialEq)]
pub enum WanderValue {
    Boolean(bool)
}

pub fn run(script: &str) -> Result<WanderValue, LigatureError> {
    let tokens = tokenize(script)?;
    let elements = parse(tokens)?;
    eval(elements)
}
