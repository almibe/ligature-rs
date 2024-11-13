// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the Wander language.

#![deny(missing_docs)]

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
    rc::Rc,
};

use lexer::tokenize_and_filter;
use ligature::{Element, Entry};
use parser::parse;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub mod lexer;
#[doc(hidden)]
pub mod parser;
#[doc(hidden)]
pub mod preludes;

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
        state: &HashMap<String, HashSet<Entry>>,
    ) -> Result<WanderValue, WanderError>;
}

/// A function call.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Call {
    /// The name of the command being called.
    pub name: ligature::Element,
    /// The arguments to the command.
    pub arguments: Vec<WanderValue>,
}

/// A quote of WanderValues.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Quote(
    /// The arguments to the quoted call.
    pub Vec<WanderValue>,
);

/// Values in Wander programs used for Wander's implementation and interfacing between
/// Wander and the host application.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum WanderValue {
    /// An Element.
    Element(ligature::Element),
    /// A Quote
    Quote(Quote),
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
            WanderValue::Quote(_quote) => todo!(),
        }
    }
}

/// Run a Wander script with the given Bindings.
pub fn run(
    script: &str,
    commands: HashMap<String, Box<dyn Command>>,
    state: HashMap<String, HashSet<Entry>>
) -> Result<WanderValue, WanderError> {
    let tokens = match tokenize_and_filter(script) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    let calls = match parse(tokens) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    let mut result = Ok(WanderValue::Network(HashSet::new()));
    calls.iter().for_each(|call| {
        match commands.get(&call.name.0) {
            Some(res) => {
                match res.run(&call.arguments, &state) {
                    Ok(res) => result = Ok(res),
                    _ => todo!()
                }
            },
            _ => todo!()
        }
    });
    result
    //    eval(&calls, bindings)
}
