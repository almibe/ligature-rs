// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the Wander scripting language.

use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use bindings::Bindings;
use hex::encode;
use interpreter::eval;
use lexer::{tokenize, transform, Token};
use parser::{parse, Element};
use serde::{Deserialize, Serialize};
use translation::translate;

pub mod bindings;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod preludes;
pub mod translation;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WanderError(pub String);

pub trait HostFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        bindings: &Bindings,
    ) -> Result<WanderValue, WanderError>;
    fn name(&self) -> String;
    fn doc(&self) -> String;
    fn params(&self) -> Vec<WanderType>;
    fn returns(&self) -> WanderType;
}

pub type TokenTransformer = fn(&[Token]) -> Result<Vec<Token>, WanderError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WanderType {
    Any,
    Value, // String | Int | Identifier
    Boolean,
    Int,
    String,
    Identifier,
    Nothing,
    /// A named reference to a NativeFunction.
    NativeFunction,
    Lambda,
    List,
    Tuple,
    Graph,
    Optional(Box<WanderType>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HostValue{
    pub type_name: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum WanderValue {
    Boolean(bool),
    Int(i64),
    String(String),
    Nothing,
    /// A named reference to a HostedFunction.
    HostedFunction(String),
    Lambda(Vec<String>, Vec<Element>),
    Application(Box<Application>),
    List(Vec<WanderValue>),
    Tuple(Vec<WanderValue>),
    Record(HashMap<String, WanderValue>),
    HostValue(HostValue),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Application {
    arguments: Vec<WanderValue>,
    callee: WanderValue
}

pub fn write_integer(integer: &i64) -> String {
    format!("{}", integer)
}

pub fn write_float(float: &f64) -> String {
    let res = format!("{}", float);
    if res.contains('.') {
        res
    } else {
        res + ".0"
    }
}

// pub fn write_bytes(bytes: &Bytes) -> String {
//     format!("0x{}", encode(bytes))
// }



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


fn write_list_or_tuple_wander_value(
    open: char,
    close: char,
    contents: &Vec<WanderValue>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    f.write_char(open).unwrap();
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

fn write_host_value(value: &HostValue, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", value.type_name)
}

fn write_record(
    contents: &HashMap<String, WanderValue>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "(").unwrap();
    let mut i = 0;
    for (name, value) in contents {
        write!(f, "{name}: {value}").unwrap();
        i += 1;
        if i < contents.len() {
            write!(f, " ").unwrap();
        }
    }
    write!(f, ")")
}

impl Display for WanderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WanderValue::Boolean(value) => write!(f, "{}", value),
            WanderValue::Int(value) => write!(f, "{}", value),
            WanderValue::String(value) => f.write_str(&write_string(value)),
            WanderValue::Nothing => write!(f, "nothing"),
            WanderValue::HostedFunction(_) => write!(f, "[function]"),
            WanderValue::List(contents) => write_list_or_tuple_wander_value('[', ']', contents, f),
            WanderValue::Lambda(_, _) => write!(f, "[lambda]"),
            WanderValue::HostValue(value) => write_host_value(value, f),
            WanderValue::Tuple(contents) => write_list_or_tuple_wander_value('(', ')', contents, f),
            WanderValue::Record(values) => write_record(values, f),
            WanderValue::Application(_) => write!(f, "[application]"),
        }
    }
}

pub fn run(script: &str, bindings: &mut Bindings) -> Result<WanderValue, WanderError> {
    let tokens = tokenize(script)?;
    let tokens = transform(&tokens, bindings)?;
    let elements = parse(tokens)?;
    let elements = translate(elements)?;
    eval(&elements, bindings)
}
