// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Identifier, Statement, Value};
use ligature_graph::Graph;
use std::{collections::BTreeSet, rc::Rc};

use crate::{bindings::Bindings, lexer::Token, HostFunction, WanderError, WanderType, WanderValue};

struct EqFunction {}
impl HostFunction for EqFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        if let [left, right] = arguments {
            Ok(crate::WanderValue::Boolean(left == right))
        } else {
            Err(WanderError(
                "`eq` function requires two parameters.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        "Check if two values are equal.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Any, WanderType::Any]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Any
    }

    fn name(&self) -> String {
        "Core.eq".to_owned()
    }
}

struct AssertEqFunction {}
impl HostFunction for AssertEqFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        if let [left, right] = arguments {
            if left == right {
                Ok(crate::WanderValue::Nothing)
            } else {
                Err(WanderError("Assertion failed!".to_owned()))
            }
        } else {
            Err(WanderError(
                "`assertEq` function requires two parameters.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        "Assert that two values are equal.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Any, WanderType::Any]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Nothing
    }

    fn name(&self) -> String {
        "Assert.assertEq".to_owned()
    }
}

struct AndFunction {}
impl HostFunction for AndFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<crate::WanderValue, WanderError> {
        if let [WanderValue::Boolean(left), WanderValue::Boolean(right)] = arguments[..] {
            Ok(crate::WanderValue::Boolean(left && right))
        } else {
            Err(WanderError(
                "`and` function requires two boolean parameters.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        "Check if two boolean values are both true.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Boolean, WanderType::Boolean]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Boolean
    }

    fn name(&self) -> String {
        "Bool.and".to_owned()
    }
}

struct NotFunction {}
impl HostFunction for NotFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<crate::WanderValue, WanderError> {
        if let [WanderValue::Boolean(value)] = arguments[..] {
            Ok(crate::WanderValue::Boolean(!value))
        } else {
            Err(WanderError(
                "`not` function requires one boolean parameter.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        "Return the opposite of the boolean value passed.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Boolean]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Boolean
    }

    fn name(&self) -> String {
        "Bool.not".to_owned()
    }
}

struct EntityFunction {}
impl HostFunction for EntityFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        if let [WanderValue::Tuple(value)] = arguments {
            if value.len() == 3 {
                Ok(value.get(0).unwrap().clone())
            } else {
                Err(WanderError(
                    "`entity` function requires one Statement parameter.".to_owned(),
                ))
            }
        } else {
            Err(WanderError(
                "`entity` function requires one Statement parameter.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        "Retrieve the Entity from a Statement.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Tuple]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Identifier
    }

    fn name(&self) -> String {
        "Statement.entity".to_owned()
    }
}

struct AttributeFunction {}
impl HostFunction for AttributeFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        if let [WanderValue::List(value)] = arguments {
            if value.len() == 3 {
                Ok(value.get(1).unwrap().clone())
            } else {
                Err(WanderError(
                    "`attribute` function requires one Statement parameter.".to_owned(),
                ))
            }
        } else {
            Err(WanderError(
                "`attribute` function requires one Statement parameter.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        "Retrieve the Attribute from a Statement.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Tuple]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Identifier
    }

    fn name(&self) -> String {
        "Statement.attribute".to_owned()
    }
}

struct ValueFunction {}
impl HostFunction for ValueFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        if let [WanderValue::List(value)] = arguments {
            if value.len() == 3 {
                Ok(value.get(2).unwrap().clone())
            } else {
                Err(WanderError(
                    "`value` function requires one Statement parameter.".to_owned(),
                ))
            }
        } else {
            Err(WanderError(
                "`value` function requires one Statement parameter.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        "Retrieve the Value from a Statement.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Tuple]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Value
    }

    fn name(&self) -> String {
        "Statement.value".to_owned()
    }
}

struct AtFunction {}
impl HostFunction for AtFunction {
    fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
        if let [WanderValue::Int(index), WanderValue::List(value)] = arguments {
            let index: usize = index.to_owned().try_into().unwrap();
            if index < value.len() {
                let t: Option<&WanderValue> = value.get(index);
                match t {
                    Some(t) => Ok(t.to_owned()),
                    None => Err(WanderError("`at` function err.".to_owned())),
                }
            } else {
                Err(WanderError("`at` function err.".to_owned()))
            }
        } else {
            Err(WanderError("`at` function err.".to_owned()))
        }
    }

    fn doc(&self) -> String {
        "Get the value at a given location.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Int, WanderType::List]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Any
    }

    fn name(&self) -> String {
        "List.at".to_owned()
    }
}

struct GraphFunction {}
impl HostFunction for GraphFunction {
    fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::List(statements)] => {
                let mut contents = BTreeSet::new();
                for statement in statements {
                    match statement {
                        WanderValue::Tuple(statement) => match &statement[..] {
                            [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), value] =>
                            {
                                let value = match value {
                                    WanderValue::Int(value) => Value::Integer(*value),
                                    WanderValue::String(value) => Value::String(value.to_owned()),
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
                                return Err(WanderError(
                                    "Invalid Statement passsed to graph.".to_owned(),
                                ))
                            }
                        },
                        _ => {
                            return Err(WanderError(
                                "Invalid Statement passsed to graph.".to_owned(),
                            ))
                        }
                    }
                }
                Ok(WanderValue::Graph(Graph::new(contents)))
            }
            _ => Err(WanderError(
                "`graph` function takes a list of Statements or no arguments.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        "Create a graph with the given Statements.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::List]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Graph
    }

    fn name(&self) -> String {
        "Graph.graph".to_owned()
    }
}

struct EmptyGraphFunction {}
impl HostFunction for EmptyGraphFunction {
    fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
        match arguments {
            [] => Ok(WanderValue::Graph(Graph::default())),
            _ => Err(WanderError(
                "`graph` function takes a list of Statements or no arguments.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        "Create an empty graph.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Graph
    }

    fn name(&self) -> String {
        "Graph.empty".to_owned()
    }
}

struct UnionFunction {}
impl HostFunction for UnionFunction {
    fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::Graph(g1), WanderValue::Graph(g2)] => {
                Ok(WanderValue::Graph(g1.add_all(g2.clone())))
            }
            _ => Err(WanderError(
                "`union` function takes two graphs as arguments.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        "Compute the union of two graphs.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Graph, WanderType::Graph]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Graph
    }

    fn name(&self) -> String {
        "Graph.union".to_owned()
    }
}

struct DifferenceFunction {}
impl HostFunction for DifferenceFunction {
    fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::Graph(g1), WanderValue::Graph(g2)] => {
                Ok(WanderValue::Graph(g1.remove_all(g2.clone())))
            }
            _ => Err(WanderError(
                "`difference` function takes two graphs as arguments.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        "Compute the difference of two graphs.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Graph, WanderType::Graph]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Graph
    }

    fn name(&self) -> String {
        "Graph.difference".to_owned()
    }
}

struct StatementsFunction {}
impl HostFunction for StatementsFunction {
    fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::Graph(graph)] => {
                let g: Vec<WanderValue> = graph
                    .all_statements()
                    .into_iter()
                    .map(|s| {
                        let entity = WanderValue::Identifier(s.entity);
                        let attribute = WanderValue::Identifier(s.attribute);
                        let value = match s.value {
                            Value::Identifier(value) => WanderValue::Identifier(value),
                            Value::String(value) => WanderValue::String(value),
                            Value::Integer(value) => WanderValue::Int(value),
                            Value::Bytes(_value) => todo!(),
                        };
                        WanderValue::Tuple(vec![entity, attribute, value])
                    })
                    .collect();
                Ok(WanderValue::List(g))
            }
            _ => Err(WanderError(
                "`statements` function takes one graphs as an argument.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        "Get all of the Statements in a Dataset.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Graph]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::List
    }

    fn name(&self) -> String {
        "Graph.statements".to_owned()
    }
}

// struct FindFunction {}
// impl NativeFunction for FindFunction {
//     fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, WanderError> {
//         match &arguments[..] {
//             [WanderValue::String(dataset_name), entity, attribute, value] => {
//                 todo!()
//             }
//             _ => todo!(),
//         }
//     }
// }

struct EnvironmentFunction {}
impl HostFunction for EnvironmentFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        if arguments.is_empty() {
            let b: BTreeSet<Statement> = bindings
                .environment()
                .iter()
                .flat_map(|e| {
                    let mut statements = vec![];
                    let name = Identifier::new(e.name.as_str()).unwrap();
                    statements.push(Statement {
                        entity: name.clone(),
                        attribute: Identifier::new("doc").unwrap(),
                        value: Value::String(e.doc_string.clone()),
                    });
                    statements.push(Statement {
                        entity: name.clone(),
                        attribute: Identifier::new("parameters").unwrap(),
                        value: Value::String(format!("{:?}", e.parameters)),
                    });
                    statements.push(Statement {
                        entity: name.clone(),
                        attribute: Identifier::new("result").unwrap(),
                        value: Value::String(format!("{:?}", e.result)),
                    });
                    statements
                })
                .collect();
            Ok(WanderValue::Graph(Graph::new(b)))
        } else {
            panic!("should never reach")
        }
    }

    fn doc(&self) -> String {
        "All Functions in the current Environment.".to_owned()
    }

    fn params(&self) -> Vec<WanderType> {
        vec![]
    }

    fn returns(&self) -> WanderType {
        WanderType::Graph
    }

    fn name(&self) -> String {
        "Halp.environment".to_owned()
    }
}

fn graph_transform(input: &[crate::lexer::Token]) -> Result<Vec<crate::lexer::Token>, WanderError> {
    let tokens: Vec<Token> = input.to_owned();
    let statements: Vec<Statement> =
        crate::lig::read_tokens(tokens).map_err(|e| WanderError(e.0))?;
    let mut results = vec![];
    results.append(&mut vec![
        Token::Name("Graph.graph".to_owned()),
        Token::OpenParen,
        Token::OpenSquare,
    ]);
    for statement in statements {
        results.push(Token::OpenParen);
        results.push(Token::Identifier(statement.entity));
        results.push(Token::Identifier(statement.attribute));
        match statement.value {
            Value::Identifier(value) => results.push(Token::Identifier(value)),
            Value::String(value) => results.push(Token::String(value)),
            Value::Integer(value) => results.push(Token::Int(value)),
            Value::Bytes(_) => todo!(),
        }
        results.push(Token::CloseParen);
    }
    results.push(Token::CloseSquare);
    results.push(Token::CloseParen);
    Ok(results)
}

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common() -> Bindings {
    let mut bindings = Bindings::new();
    bindings.bind_host_function(Rc::new(EqFunction {}));

    bindings.bind_host_function(Rc::new(AssertEqFunction {}));

    bindings.bind_host_function(Rc::new(AndFunction {}));
    bindings.bind_host_function(Rc::new(NotFunction {}));

    bindings.bind_host_function(Rc::new(EntityFunction {}));
    bindings.bind_host_function(Rc::new(AttributeFunction {}));
    bindings.bind_host_function(Rc::new(ValueFunction {}));

    bindings.bind_host_function(Rc::new(AtFunction {}));

    bindings.bind_host_function(Rc::new(EmptyGraphFunction {}));
    bindings.bind_host_function(Rc::new(GraphFunction {}));
    bindings.bind_host_function(Rc::new(UnionFunction {}));
    bindings.bind_host_function(Rc::new(DifferenceFunction {}));
    bindings.bind_host_function(Rc::new(StatementsFunction {}));
    bindings.bind_host_function(Rc::new(EnvironmentFunction {}));
    // bindings.bind_native_function(
    //     "Graph".to_owned(),
    //     "find".to_owned(),
    //     Rc::new(FindFunction {}),
    // );

    bindings.bind_token_transformer(
        "Graph".to_owned(),
        "graph".to_owned(),
        Rc::new(graph_transform),
    );
    bindings
}
