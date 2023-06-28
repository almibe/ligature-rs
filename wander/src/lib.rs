// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the Wander scripting language.

use interpreter::eval;
use lexer::tokenize;
use ligature::{Identifier, LigatureError};
use parser::parse;

pub mod bindings;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod preludes;

pub trait NativeFunction {
    fn run(&self) -> Result<WanderValue, LigatureError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum WanderValue {
    Boolean(bool),
    Int(i64),
    String(String),
    Identifier(Identifier),
    Nothing,
    /// A named reference to a NativeFunction.
    NativeFunction(String),
}

pub fn run(script: &str) -> Result<WanderValue, LigatureError> {
    let tokens = tokenize(script)?;
    let elements = parse(tokens)?;
    eval(elements)
}
