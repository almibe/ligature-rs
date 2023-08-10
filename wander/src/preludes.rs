// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{LigatureError, Statement, Value};
use ligature_graph::Graph;
use std::{collections::BTreeSet, rc::Rc};

use crate::{bindings::Bindings, NativeFunction, TokenTransformer, WanderValue};

struct AndFunction {}
impl NativeFunction for AndFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
    ) -> Result<crate::WanderValue, ligature::LigatureError> {
        if let [WanderValue::Boolean(left), WanderValue::Boolean(right)] = arguments[..] {
            Ok(crate::WanderValue::Boolean(left && right))
        } else {
            Err(LigatureError(
                "`and` function requires two boolean parameters.".to_owned(),
            ))
        }
    }
}

struct NotFunction {}
impl NativeFunction for NotFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
    ) -> Result<crate::WanderValue, ligature::LigatureError> {
        if let [WanderValue::Boolean(value)] = arguments[..] {
            Ok(crate::WanderValue::Boolean(!value))
        } else {
            Err(LigatureError(
                "`not` function requires one boolean parameter.".to_owned(),
            ))
        }
    }
}

struct EntityFunction {}
impl NativeFunction for EntityFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::List(value)] = &arguments[..] {
            if value.len() == 3 {
                Ok(value.get(0).unwrap().clone())
            } else {
                Err(LigatureError(
                    "`entity` function requires one Statement parameter.".to_owned(),
                ))
            }
        } else {
            Err(LigatureError(
                "`entity` function requires one Statement parameter.".to_owned(),
            ))
        }
    }
}

struct AttributeFunction {}
impl NativeFunction for AttributeFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::List(value)] = &arguments[..] {
            if value.len() == 3 {
                Ok(value.get(1).unwrap().clone())
            } else {
                Err(LigatureError(
                    "`attribute` function requires one Statement parameter.".to_owned(),
                ))
            }
        } else {
            Err(LigatureError(
                "`attribute` function requires one Statement parameter.".to_owned(),
            ))
        }
    }
}

struct ValueFunction {}
impl NativeFunction for ValueFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::List(value)] = &arguments[..] {
            if value.len() == 3 {
                Ok(value.get(2).unwrap().clone())
            } else {
                Err(LigatureError(
                    "`value` function requires one Statement parameter.".to_owned(),
                ))
            }
        } else {
            Err(LigatureError(
                "`value` function requires one Statement parameter.".to_owned(),
            ))
        }
    }
}

struct AtFunction {}
impl NativeFunction for AtFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::Int(index), WanderValue::List(value)] = &arguments[..] {
            let index: usize = index.to_owned().try_into().unwrap();
            if index < value.len() {
                let t: Option<&WanderValue> = value.get(index);
                match t {
                    Some(t) => Ok(t.to_owned()),
                    None => Err(LigatureError("`at` function err.".to_owned())),
                }
            } else {
                Err(LigatureError("`at` function err.".to_owned()))
            }
        } else {
            Err(LigatureError("`at` function err.".to_owned()))
        }
    }
}

struct GraphFunction {}
impl NativeFunction for GraphFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        match &arguments[..] {
            [] => Ok(WanderValue::Graph(Graph::default())),
            [WanderValue::List(statements)] => {
                let mut contents = BTreeSet::new();
                for statement in statements {
                    match statement {
                        WanderValue::Tuple(statement) => match &statement[..] {
                            [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), value] =>
                            {
                                let value = match value {
                                    WanderValue::Int(value) => Value::IntegerLiteral(*value),
                                    WanderValue::String(value) => {
                                        Value::StringLiteral(value.to_owned())
                                    }
                                    WanderValue::Identifier(value) => {
                                        Value::Identifier(value.to_owned())
                                    }
                                    _ => todo!(),
                                };
                                contents.insert(Statement {
                                    entity: entity.to_owned(),
                                    attribute: attribute.to_owned(),
                                    value,
                                });
                            }
                            _ => {
                                return Err(LigatureError(
                                    "Invalid Statement passsed to graph.".to_owned(),
                                ))
                            }
                        },
                        _ => {
                            return Err(LigatureError(
                                "Invalid Statement passsed to graph.".to_owned(),
                            ))
                        }
                    }
                }
                Ok(WanderValue::Graph(Graph::new(contents)))
            }
            _ => Err(LigatureError(
                "`graph` function takes a list of Statements or no arguments.".to_owned(),
            )),
        }
    }
}

struct UnionFunction {}
impl NativeFunction for UnionFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::Graph(g1), WanderValue::Graph(g2)] => {
                Ok(WanderValue::Graph(g1.add_all(g2.clone())))
            }
            _ => Err(LigatureError(
                "`union` function takes two graphs as arguments.".to_owned(),
            )),
        }
    }
}

struct DifferenceFunction {}
impl NativeFunction for DifferenceFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::Graph(g1), WanderValue::Graph(g2)] => {
                Ok(WanderValue::Graph(g1.remove_all(g2.clone())))
            }
            _ => Err(LigatureError(
                "`difference` function takes two graphs as arguments.".to_owned(),
            )),
        }
    }
}

struct StatementsFunction {}
impl NativeFunction for StatementsFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::Graph(graph)] => {
                let g: Vec<WanderValue> = graph
                    .all_statements()
                    .into_iter()
                    .map(|s| {
                        let entity = WanderValue::Identifier(s.entity);
                        let attribute = WanderValue::Identifier(s.attribute);
                        let value = match s.value {
                            Value::Identifier(value) => WanderValue::Identifier(value),
                            Value::StringLiteral(value) => WanderValue::String(value),
                            Value::IntegerLiteral(value) => WanderValue::Int(value),
                            Value::BytesLiteral(_value) => todo!(),
                        };
                        WanderValue::Tuple(vec![entity, attribute, value])
                    })
                    .collect();
                Ok(WanderValue::List(g))
            }
            _ => Err(LigatureError(
                "`statements` function takes one graphs as an argument.".to_owned(),
            )),
        }
    }
}

struct FindFunction {}
impl NativeFunction for FindFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::String(datasetName), entity, attribute, value] => {
                todo!()
            },
            _ => todo!()
        }
    }
}

struct GraphTransformer {}
impl TokenTransformer for GraphTransformer {
    fn transform(
        &self,
        input: &[crate::lexer::Token],
    ) -> Result<Vec<crate::lexer::Token>, LigatureError> {
        todo!()
    }
}

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common() -> Bindings {
    let mut bindings = Bindings::new();
    bindings.bind_native_function(String::from("and"), Rc::new(AndFunction {}));
    bindings.bind_native_function(String::from("not"), Rc::new(NotFunction {}));

    bindings.bind_native_function(String::from("entity"), Rc::new(EntityFunction {}));
    bindings.bind_native_function(String::from("attribute"), Rc::new(AttributeFunction {}));
    bindings.bind_native_function(String::from("value"), Rc::new(ValueFunction {}));

    bindings.bind_native_function(String::from("at"), Rc::new(AtFunction {}));

    bindings.bind_native_function(String::from("graph"), Rc::new(GraphFunction {}));
    bindings.bind_native_function(String::from("union"), Rc::new(UnionFunction {}));
    bindings.bind_native_function(String::from("difference"), Rc::new(DifferenceFunction {}));
    bindings.bind_native_function(String::from("statements"), Rc::new(StatementsFunction {}));
    bindings.bind_native_function(String::from("find"), Rc::new(FindFunction {}));

    bindings.bind_token_transformer(String::from("graph"), Rc::new(GraphTransformer {}));
    bindings
}
