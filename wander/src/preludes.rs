// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{LigatureError, Statement, Value};
use ligature_graph::Graph;
use std::{collections::BTreeSet, rc::Rc};

use crate::{bindings::Bindings, lexer::Token, NativeFunction, TokenTransformer, WanderValue, WanderType};

struct EqFunction {}
impl NativeFunction for EqFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [left, right] = arguments {
            Ok(crate::WanderValue::Boolean(left == right))
        } else {
            Err(LigatureError(
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
}

struct AssertEqFunction {}
impl NativeFunction for AssertEqFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [left, right] = arguments {
            if left == right {
                Ok(crate::WanderValue::Nothing)
            } else {
                Err(LigatureError("Assertion failed!".to_owned()))
            }
        } else {
            Err(LigatureError(
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
}

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

    fn doc(&self) -> String {
        "Check if two boolean values are both true.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Boolean, WanderType::Boolean]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Boolean
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

    fn doc(&self) -> String {
        "Return the opposite of the boolean value passed.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Boolean]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Boolean
    }
}

struct EntityFunction {}
impl NativeFunction for EntityFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::Tuple(value)] = arguments {
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

    fn doc(&self) -> String {
        "Retrieve the Entity from a Statement.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Tuple]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Identifier
    }
}

struct AttributeFunction {}
impl NativeFunction for AttributeFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::List(value)] = arguments {
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

    fn doc(&self) -> String {
        "Retrieve the Attribute from a Statement.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Tuple]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Identifier
    }
}

struct ValueFunction {}
impl NativeFunction for ValueFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::List(value)] = arguments {
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

    fn doc(&self) -> String {
        "Retrieve the Value from a Statement.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Tuple]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Value
    }
}

struct AtFunction {}
impl NativeFunction for AtFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::Int(index), WanderValue::List(value)] = arguments {
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

    fn doc(&self) -> String {
        "Get the value at a given location.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::Int, WanderType::List]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Any
    }
}

struct GraphFunction {}
impl NativeFunction for GraphFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
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

    fn doc(&self) -> String {
        "Create a graph with the given Statements.".to_owned()
    }

    fn params(&self) -> Vec<crate::WanderType> {
        vec![WanderType::List]
    }

    fn returns(&self) -> crate::WanderType {
        WanderType::Graph
    }
}

struct EmptyGraphFunction {}
impl NativeFunction for EmptyGraphFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        match arguments {
            [] => Ok(WanderValue::Graph(Graph::default())),
            _ => Err(LigatureError(
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
}

struct UnionFunction {}
impl NativeFunction for UnionFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        match arguments {
            [WanderValue::Graph(g1), WanderValue::Graph(g2)] => {
                Ok(WanderValue::Graph(g1.add_all(g2.clone())))
            }
            _ => Err(LigatureError(
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
}

struct DifferenceFunction {}
impl NativeFunction for DifferenceFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        match arguments {
            [WanderValue::Graph(g1), WanderValue::Graph(g2)] => {
                Ok(WanderValue::Graph(g1.remove_all(g2.clone())))
            }
            _ => Err(LigatureError(
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
}

struct StatementsFunction {}
impl NativeFunction for StatementsFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
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
            _ => Err(LigatureError(
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
}

// struct FindFunction {}
// impl NativeFunction for FindFunction {
//     fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
//         match &arguments[..] {
//             [WanderValue::String(dataset_name), entity, attribute, value] => {
//                 todo!()
//             }
//             _ => todo!(),
//         }
//     }
// }

struct GraphTransformer {}
impl TokenTransformer for GraphTransformer {
    fn transform(
        &self,
        input: &Vec<crate::lexer::Token>,
    ) -> Result<Vec<crate::lexer::Token>, LigatureError> {
        let tokens: Vec<Token> = input.to_owned();
        let statements: Vec<Statement> = crate::lig::read_tokens(tokens)?;
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
}

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common() -> Bindings {
    let mut bindings = Bindings::new();
    bindings.bind_native_function("Core".to_owned(), "eq".to_owned(), Rc::new(EqFunction {}));

    bindings.bind_native_function(
        "Assert".to_owned(),
        "assertEq".to_owned(),
        Rc::new(AssertEqFunction {}),
    );

    bindings.bind_native_function("Bool".to_owned(), "and".to_owned(), Rc::new(AndFunction {}));
    bindings.bind_native_function("Bool".to_owned(), "not".to_owned(), Rc::new(NotFunction {}));

    bindings.bind_native_function(
        "Statement".to_owned(),
        "entity".to_owned(),
        Rc::new(EntityFunction {}),
    );
    bindings.bind_native_function(
        "Statement".to_owned(),
        "attribute".to_owned(),
        Rc::new(AttributeFunction {}),
    );
    bindings.bind_native_function(
        "Statement".to_owned(),
        "value".to_owned(),
        Rc::new(ValueFunction {}),
    );

    bindings.bind_native_function("List".to_owned(), "at".to_owned(), Rc::new(AtFunction {}));

    bindings.bind_native_function(
        "Graph".to_owned(),
        "empty".to_owned(),
        Rc::new(EmptyGraphFunction {}),
    );
    bindings.bind_native_function(
        "Graph".to_owned(),
        "graph".to_owned(),
        Rc::new(GraphFunction {}),
    );
    bindings.bind_native_function(
        "Graph".to_owned(),
        "union".to_owned(),
        Rc::new(UnionFunction {}),
    );
    bindings.bind_native_function(
        "Graph".to_owned(),
        "difference".to_owned(),
        Rc::new(DifferenceFunction {}),
    );
    bindings.bind_native_function(
        "Graph".to_owned(),
        "statements".to_owned(),
        Rc::new(StatementsFunction {}),
    );
    // bindings.bind_native_function(
    //     "Graph".to_owned(),
    //     "find".to_owned(),
    //     Rc::new(FindFunction {}),
    // );

    bindings.bind_token_transformer("Graph".to_owned(), "graph".to_owned(), Rc::new(GraphTransformer {}));
    bindings
}
