// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the Wander language.

#![deny(missing_docs)]

use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Write}, ops::Range,
};

use environment::Environment;
use identifier::Identifier;
use interpreter::{eval, Expression};
use lexer::{tokenize, tokenize_and_filter, transform, Token};
use parser::{parse, Element};
use serde::{Deserialize, Serialize};
use translation::translate;

#[doc(hidden)]
pub mod environment;
pub mod identifier;
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

/// struct describing a HostFunction.
pub struct HostFunctionBinding {
    /// Name used to bind this HostFunction including Namespaces.
    pub name: String,
    /// The type of the parameters this HostFunction takes.
    pub parameters: Vec<(String, Option<String>)>,
    /// The type of the result of this HostFunction.
    pub result: Option<String>,
    /// The documentation for this HostFunction.
    /// Can be text or Markdown.
    pub doc_string: String,
}

/// A trait representing a function exported from the hosting application that
/// can be called from Wander.
pub trait HostFunction {
    /// The function called when the HostFunction is called from Wander.
    fn run(
        &self,
        arguments: &[WanderValue],
        bindings: &Environment,
    ) -> Result<WanderValue, WanderError>;
    /// Get the binding information for this HostFunction.
    fn binding(&self) -> HostFunctionBinding;
}

/// Type alias used for TokenTransformers.
pub type TokenTransformer = fn(&[Location<Token>]) -> Result<Vec<Location<Token>>, WanderError>;

/// Values in Wander programs used for Wander's implementation and interfacing between
/// Wander and the host application.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum WanderValue {
    /// A Bool value.
    Bool(bool),
    /// A 64-bit signed integer value.
    Int(i64),
    /// A String value.
    String(String),
    /// An Identifier.
    Identifier(Identifier),
    /// A Lambda
    Lambda(String, Option<String>, Option<String>, Box<Location<Element>>),
    /// A List.
    List(Vec<WanderValue>),
    /// A Record.
    Record(HashMap<String, WanderValue>),
}

impl core::hash::Hash for WanderValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

/// A struct represting a partially applied function.
/// The function can be a Lambda or a HostFunction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PartialApplication {
    arguments: Vec<WanderValue>,
    callee: WanderValue,
}

/// Write integer.
pub fn write_integer(integer: &i64) -> String {
    format!("{}", integer)
}

/// Write float.
pub fn write_float(float: &f64) -> String {
    let res = format!("{}", float);
    if res.contains('.') {
        res
    } else {
        res + ".0"
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

fn write_record(
    contents: &HashMap<String, WanderValue>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "{{").unwrap();
    let mut i = 0;
    for (name, value) in contents {
        write!(f, "{name} = {value}").unwrap();
        i += 1;
        if i < contents.len() {
            write!(f, " ").unwrap();
        }
    }
    write!(f, "}}")
}

impl Display for WanderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WanderValue::Bool(value) => write!(f, "{}", value),
            WanderValue::Int(value) => write!(f, "{}", value),
            WanderValue::String(value) => f.write_str(&write_string(value)),
            WanderValue::Identifier(value) => write!(f, "<{}>", value.id()),
            WanderValue::List(contents) => write_list_value("[", ']', contents, f),
            WanderValue::Record(values) => write_record(values, f),
            // WanderValue::Lambda(p, i, o, b) => write!(
            //     f,
            //     "[lambda {:?}]",
            //     WanderValue::Lambda::(p.clone(), i.clone(), o.clone(), b.clone())
            // ),
            WanderValue::Lambda(_p, _i , _o, _b) => todo!(),
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
    let tokens = match transform(&tokens, bindings) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    let elements = match parse(tokens) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    let expression = match translate(elements) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    eval(&expression, bindings)
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Deserialize, Hash)]
/// Store location information alongside a value.
pub struct Location<T: PartialEq + Eq>(pub T, pub usize);

#[derive(Debug, Serialize)]
/// Structure used for debugging or inspecting code.
pub struct Introspection {
    /// A list of all Tokens including whitespace.
    pub tokens_ws: Vec<Location<Token>>,
    /// A list of all Tokens without whitespace.
    pub tokens: Vec<Location<Token>>,
    /// A list of all Tokens after macro transformations.
    pub tokens_transformed: Vec<Location<Token>>,
    /// Element representation.
    pub element: Location<Element>,
    /// Expression representation.
    pub expression: Location<Expression>,
}

/// Run a Wander script with the given Bindings.
pub fn introspect(
    script: &str,
    bindings: &Environment,
) -> Result<Introspection, WanderError> {
    let tokens_ws = tokenize(script).or(Ok(vec![]))?;
    let tokens = tokenize_and_filter(script).or(Ok(vec![]))?;
    let tokens_transformed = transform(&tokens.clone(), bindings).or(Ok(vec![]))?;
    // let element = parse(tokens_transformed.clone()).or(Ok(Location(Element::Nothing, 0)))?; //TODO handle errors better
    //let expression = translate(element.clone()).or(Ok(Location(Expression::Nothing, 0)))?; //TODO handle errors better
    // Ok(Introspection {
    //     tokens_ws,
    //     tokens,
    //     tokens_transformed,
    //     element,
    //     expression,
    // })
    todo!()
}
