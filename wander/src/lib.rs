// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the Wander scripting language.

use std::fmt::{Display, Write};

use bindings::Bindings;
use hex::encode;
use interpreter::eval;
use lexer::{tokenize, transform, Token};
use ligature::{Bytes, Identifier, LigatureError, Statement, Value};
use ligature_graph::Graph;
use parser::{parse, Element};
use serde::{Deserialize, Serialize};
use translation::translate;

pub mod bindings;
pub mod interpreter;
pub mod lexer;
pub mod lig;
pub mod parser;
pub mod preludes;
pub mod translation;

pub trait NativeFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError>;
    fn doc(&self) -> String;
    fn params(&self) -> Vec<WanderType>;
    fn returns(&self) -> WanderType;
}

pub trait TokenTransformer {
    fn transform(&self, input: &Vec<Token>) -> Result<Vec<Token>, LigatureError>;
}

#[derive(Debug, Clone, PartialEq)]
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
    Tuple(Vec<WanderValue>),
    Graph(Graph),
}

impl WanderValue {
    pub fn to_script_value(&self) -> Result<ScriptValue, LigatureError> {
        match self {
            WanderValue::Boolean(value) => Ok(ScriptValue::Boolean(value.to_owned())),
            WanderValue::Int(value) => Ok(ScriptValue::Int(value.to_owned())),
            WanderValue::String(value) => Ok(ScriptValue::String(value.to_owned())),
            WanderValue::Identifier(value) => Ok(ScriptValue::Identifier(value.to_owned())),
            WanderValue::Nothing => Ok(ScriptValue::Nothing),
            WanderValue::NativeFunction(_) => Err(LigatureError(
                "Cannot convert NativeFunction to ScriptValue.".to_owned(),
            )),
            WanderValue::Lambda(_, _) => Err(LigatureError(
                "Cannot convert Lambda to ScriptValue.".to_owned(),
            )),
            WanderValue::List(values) => {
                let mut script_values = vec![];
                for value in values {
                    match value.to_script_value() {
                        Ok(value) => script_values.push(value),
                        Err(err) => return Err(err),
                    }
                }
                Ok(ScriptValue::List(script_values))
            }
            WanderValue::Tuple(values) => {
                let mut script_values = vec![];
                for value in values {
                    match value.to_script_value() {
                        Ok(value) => script_values.push(value),
                        Err(err) => return Err(err),
                    }
                }
                Ok(ScriptValue::Tuple(script_values))
            }
            WanderValue::Graph(graph) => Ok(ScriptValue::Graph(graph.clone())),
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
    Tuple(Vec<ScriptValue>),
    Graph(Graph),
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

pub fn write_bytes(bytes: &Bytes) -> String {
    format!("0x{}", encode(bytes))
}

/// Writes out an Entity to a String.
pub fn write_identifier(entity: &Identifier) -> String {
    format!("<{}>", entity.id())
}

pub fn write_value(value: &Value) -> String {
    match value {
        Value::Identifier(entity) => write_identifier(entity),
        Value::Integer(integer) => write_integer(integer),
        //Value::FloatLiteral(float) => write_float(float),
        Value::String(string) => write_string(string),
        Value::Bytes(bytes) => write_bytes(bytes),
    }
}

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

pub fn write_statement(statement: &Statement) -> String {
    format!(
        "{} {} {}\n",
        write_identifier(&statement.entity),
        write_identifier(&statement.attribute),
        write_value(&statement.value),
    )
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

fn write_graph(graph: &Graph, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Graph.graph`").unwrap();
    graph.all_statements().into_iter().for_each(|statement| {
        f.write_str(write_statement(&statement).as_str()).unwrap();
    });
    f.write_str("`")
}

fn write_list_or_tuple_script_value(
    open: char,
    close: char,
    contents: &Vec<ScriptValue>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "{open}").unwrap();
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

impl Display for WanderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WanderValue::Boolean(value) => write!(f, "{}", value),
            WanderValue::Int(value) => write!(f, "{}", value),
            WanderValue::String(value) => f.write_str(&write_string(value)),
            WanderValue::Identifier(value) => write!(f, "{}", value),
            WanderValue::Nothing => write!(f, "nothing"),
            WanderValue::NativeFunction(_) => write!(f, "[function]"),
            WanderValue::List(contents) => write_list_or_tuple_wander_value('[', ']', contents, f),
            WanderValue::Lambda(_, _) => write!(f, "[lambda]"),
            WanderValue::Graph(graph) => write_graph(graph, f),
            WanderValue::Tuple(contents) => write_list_or_tuple_wander_value('(', ')', contents, f),
        }
    }
}

impl Display for ScriptValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptValue::Boolean(value) => write!(f, "{}", value),
            ScriptValue::Int(value) => write!(f, "{}", value),
            ScriptValue::String(value) => f.write_str(&write_string(value)),
            ScriptValue::Identifier(value) => write!(f, "{}", value),
            ScriptValue::Nothing => write!(f, "nothing"),
            ScriptValue::List(contents) => write_list_or_tuple_script_value('[', ']', contents, f),
            ScriptValue::Graph(graph) => write_graph(graph, f),
            ScriptValue::Tuple(contents) => write_list_or_tuple_script_value('(', ')', contents, f),
        }
    }
}

pub fn run(script: &str, bindings: &mut Bindings) -> Result<ScriptValue, LigatureError> {
    let tokens = tokenize(script)?;
    let tokens = transform(&tokens, bindings)?;
    let elements = parse(tokens)?;
    let elements = translate(elements)?;
    let eval_result = eval(&elements, bindings)?;
    eval_result.to_script_value()
}
