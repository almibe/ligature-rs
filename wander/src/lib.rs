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
use serde::{Serialize, Deserialize};

pub mod bindings;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod preludes;

pub trait NativeFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError>;
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
    Lambda(Vec<String>, Vec<Element>),
    List(Vec<WanderValue>),
}

impl WanderValue {
    pub fn to_script_value(&self) -> Result<ScriptValue, LigatureError> {
        match self {
            WanderValue::Boolean(value) => Ok(ScriptValue::Boolean(value.to_owned())),
            WanderValue::Int(value) => Ok(ScriptValue::Int(value.to_owned())),
            WanderValue::String(value) => Ok(ScriptValue::String(value.to_owned())),
            WanderValue::Identifier(value) => Ok(ScriptValue::Identifier(value.to_owned())),
            WanderValue::Nothing => Ok(ScriptValue::Nothing),
            WanderValue::NativeFunction(_) => Err(LigatureError("Cannot convert NativeFunction to ScriptValue.".to_owned())),
            WanderValue::Lambda(_, _) => Err(LigatureError("Cannot convert Labda to ScriptValue.".to_owned())),
            WanderValue::List(values) => {
                let mut script_values = vec![];
                for value in values {
                    match value.to_script_value() {
                        Ok(value) => script_values.push(value),
                        Err(err) => return Err(err),
                    }
                }
                Ok(ScriptValue::List(script_values))
            },
        }
    }
}

/// A ScriptValue is a subset of WanderValue that can be returned from a script.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ScriptValue {
    Boolean(bool),
    Int(i64),
    String(String),
    Identifier(Identifier),
    Nothing,
    List(Vec<ScriptValue>),
}

impl Display for WanderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WanderValue::Boolean(value) => write!(f, "{}", value),
            WanderValue::Int(value) => write!(f, "{}", value),
            WanderValue::String(value) => write!(f, "\"{}\"", value),
            WanderValue::Identifier(value) => write!(f, "{}", value),
            WanderValue::Nothing => write!(f, "nothing"),
            WanderValue::NativeFunction(_) => write!(f, "[function]"),
            WanderValue::List(contents) => {
                write!(f, "[").unwrap();
                let mut i = 0;
                for value in contents {
                    write!(f, "{value}").unwrap();
                    i += 1;
                    if i < contents.len() {
                        write!(f, " ").unwrap();
                    }
                }
                write!(f, "]")
            }
            WanderValue::Lambda(_, _) => write!(f, "[lambda]"),
        }
    }
}

pub fn run(script: &str, bindings: &mut Bindings) -> Result<WanderValue, LigatureError> {
    let tokens = tokenize(script)?;
    let elements = parse(tokens)?;
    eval(&elements, bindings)
}
