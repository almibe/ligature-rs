// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the Wander scripting language.

use std::fmt::Display;

use bindings::Bindings;
use interpreter::eval;
use lexer::tokenize;
use ligature::{Identifier, LigatureError};
use parser::{parse, Element};

pub mod bindings;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod preludes;

pub trait NativeFunction {
    fn run(
        &self,
        arguments: &Vec<Element>,
        bindings: &mut Bindings,
    ) -> Result<WanderValue, LigatureError>;
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

impl Display for WanderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WanderValue::Boolean(value) => write!(f, "{}", value),
            WanderValue::Int(value) => write!(f, "{}", value),
            WanderValue::String(value) => write!(f, "{}", value),
            WanderValue::Identifier(value) => write!(f, "{}", value),
            WanderValue::Nothing => write!(f, "nothing"),
            WanderValue::NativeFunction(_) => write!(f, "[function]"),
        }
    }
}

pub fn run(script: &str, bindings: &mut Bindings) -> Result<WanderValue, LigatureError> {
    let tokens = tokenize(script)?;
    let elements = parse(tokens)?;
    eval(&elements, bindings)
}
