// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the Wander language.

#![deny(missing_docs)]

use std::{
    collections::HashSet,
    fmt::Display,
};

use environment::Environment;
use interpreter::eval;
use lexer::tokenize_and_filter;
use parser::parse;
use serde::{Deserialize, Serialize};
use translation::translate;

#[doc(hidden)]
pub mod environment;
#[doc(hidden)]
pub mod interpreter;
#[doc(hidden)]
pub mod lexer;
#[doc(hidden)]
pub mod parser;
#[doc(hidden)]
pub mod preludes;
#[doc(hidden)]
pub mod translation;

/// An error that occurs while running a Wander script.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct WanderError(pub String);

/// A trait representing a function exported from the hosting application that
/// can be called from Wander.
pub trait Command {
    /// The function called when the HostFunction is called from Wander.
    fn run(
        &self,
        arguments: &[WanderValue],
        bindings: &Environment,
    ) -> Result<WanderValue, WanderError>;
}

/// A function call.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Call {
    name: ligature::Element,
    arguments: Vec<WanderValue>,
}

/// Values in Wander programs used for Wander's implementation and interfacing between
/// Wander and the host application.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum WanderValue {
    /// An Element.
    Element(ligature::Element),
    /// A Call
    Call(Call),
    /// A Network.
    Network(HashSet<ligature::Entry>),
}

impl core::hash::Hash for WanderValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

// Write out Bytes as a String.
// pub fn write_bytes(bytes: &Bytes) -> String {
//     format!("0x{}", encode(bytes))
// }

/// Escape a String value.
pub fn write_string(string: &str) -> String {
    //TODO this could be done better
    let escaped_string = string
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        //.replace("\f", "\\b") <-- TODO not sure how to handle this or if I really need to
        //.replace("\b", "\\b") <-- TODO not sure how to handle this or if I really need to
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("\"{}\"", escaped_string)
}

fn write_list_value(
    open: &str,
    close: char,
    contents: &Vec<WanderValue>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    f.write_str(open).unwrap();
    let mut i = 0;
    for value in contents {
        write!(f, "{value}").unwrap();
        i += 1;
        if i < contents.len() {
            write!(f, " ").unwrap();
        }
    }
    write!(f, "{close}")
}

fn write_network(
    _contents: &HashSet<ligature::Entry>,
    _f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    todo!()
    // write!(f, "{{").unwrap();
    // let mut i = 0;
    // for (name, value) in contents {
    //     write!(f, "{name} = {value}").unwrap();
    //     i += 1;
    //     if i < contents.len() {
    //         write!(f, " ").unwrap();
    //     }
    // }
    // write!(f, "}}")
}

impl Display for WanderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WanderValue::Element(value) => write!(f, "{}", value.0),
            WanderValue::Network(values) => write_network(values, f),
            WanderValue::Call(_call) => todo!(),
        }
    }
}

/// Run a Wander script with the given Bindings.
pub fn run(
    script: &str,
    bindings: &mut Environment,
) -> Result<WanderValue, WanderError> {
    let tokens = match tokenize_and_filter(script) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    let calls = match parse(tokens) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    let expression = match translate(calls) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    eval(&expression, bindings)
}
