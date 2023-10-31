// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Wander support for working with Ligature.

use ligature::{Dataset, Ligature};
use wander::environment::Environment;
use wander::HostType;
use std::sync::RwLock;
use std::rc::Rc;
use wander::HostFunction;
use wander::NoHostType;
use wander::WanderValue;
use wander::WanderError;
use wander::HostFunctionBinding;

pub fn bind_instance<T: HostType>(instance: Rc<RwLock<dyn Ligature>>, environment: &mut Environment<T>) {
    environment.bind_host_function(Rc::new(DatasetsFunction {
        instance: instance.clone(),
    }));
    environment.bind_host_function(Rc::new(AddDatasetFunction {
        instance: instance.clone(),
    }));
    environment.bind_host_function(Rc::new(RemoveDatasetFunction {
        instance: instance.clone(),
    }));
    //     bindings.bind_host_function(Rc::new(StatementsFunction {
    //         lim: self.datasets.clone(),
    //     }));
    //     bindings.bind_host_function(Rc::new(AddStatementsFunction {
    //         lim: self.datasets.clone(),
    //     }));
    //     bindings.bind_host_function(Rc::new(RemoveStatementsFunction {
    //         lim: self.datasets.clone(),
    //     }));
    //     bindings.bind_host_function(Rc::new(QueryFunction {
    //         lim: self.datasets.clone(),
    //     }));
}

struct DatasetsFunction {
    instance: Rc<RwLock<dyn Ligature>>,
}
impl <T: HostType>HostFunction<T> for DatasetsFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _environment: &Environment<T>,
    ) -> Result<WanderValue<T>, WanderError> {
        if let [_] = arguments {
            let instance = self.instance.read().unwrap();
            let datasets = instance.datasets().map_err(|e| WanderError(e.0))?;
            Ok(WanderValue::List(datasets.into_iter().map(|d| WanderValue::String(d.name().to_string())).collect()))
        } else {
            Err(WanderError(
                "`datasets` function requires no arguments.".to_owned(),
            ))
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            doc_string: "Show all Datasets.".to_owned(),
            name: "datasets".to_owned(),
            parameters: vec![("unit".to_owned(), None)],
            result: None
        }
    }
}

struct AddDatasetFunction {
    instance: Rc<RwLock<dyn Ligature>>,
}
impl <T: HostType>HostFunction<T> for AddDatasetFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _environment: &Environment<T>,
    ) -> Result<wander::WanderValue<T>, WanderError> {
        match arguments {
            [WanderValue::String(name)] => {
                let mut instance = self.instance.write().unwrap();
                match Dataset::new(name) {
                    Ok(ds) => instance.add_dataset(&ds).map(|_| WanderValue::Nothing).map_err(|e| WanderError(e.0)),
                    Err(err) => Err(WanderError(err.0))
                }
            }
            _ => Err(WanderError(
                "`addDataset` function requires one string parameter.".to_owned(),
            )),
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            doc_string: "Add a new Dataset.".to_owned(),
            name: "addDataset".to_owned(),
            parameters: vec![("name".to_owned(), None)],
            result: None
        }
    }
}

struct RemoveDatasetFunction {
    instance: Rc<RwLock<dyn Ligature>>,
}
impl <T: HostType>HostFunction<T> for RemoveDatasetFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _environment: &Environment<T>,
    ) -> Result<wander::WanderValue<T>, WanderError> {
        match arguments {
            [WanderValue::String(name)] => {
                let mut instance = self.instance.write().unwrap();                
                match Dataset::new(name) {
                    Ok(ds) => instance.remove_dataset(&ds).map(|_| WanderValue::Nothing).map_err(|e| WanderError(e.0)),
                    Err(err) => Err(WanderError(err.0))
                }
            }
            _ => Err(WanderError(
                "`removeDataset` function requires one string parameter.".to_owned(),
            )),
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            doc_string: "Remove a Dataset.".to_owned(),
            name: "removeDataset".to_owned(),
            parameters: vec![("name".to_owned(), None)],
            result: None
        }
    }
}

// struct StatementsFunction {
//     lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
// }
// impl HostFunction<NoHostType> for StatementsFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue<NoHostType>],
//         _bindings: &Bindings,
//     ) -> Result<wander::WanderValue<NoHostType>, WanderError> {
//         match arguments {
//             [WanderValue::String(name)] => {
//                 let instance = self.lim.read().unwrap();
//                 match instance.get(name) {
//                     Some(statements) => {
//                         let mut results = vec![];
//                         let statements = statements.borrow();
//                         for statement in statements.iter() {
//                             let entity = WanderValue::Identifier(statement.entity.clone());
//                             let attribute = WanderValue::Identifier(statement.attribute.clone());
//                             let value = match statement.value.clone() {
//                                 ligature::Value::Identifier(value) => {
//                                     WanderValue::Identifier(value)
//                                 }
//                                 ligature::Value::String(value) => WanderValue::String(value),
//                                 ligature::Value::Integer(value) => WanderValue::Int(value),
//                                 ligature::Value::Bytes(_) => todo!(),
//                             };
//                             results.push(WanderValue::List(vec![entity, attribute, value]));
//                         }
//                         Ok(WanderValue::List(results))
//                     }
//                     _ => Ok(WanderValue::Nothing), // do nothing
//                 }
//             }
//             _ => Err(WanderError(
//                 "`removeDataset` function requires one string parameter.".to_owned(),
//             )),
//         }
//     }

//     fn binding(&self) -> wander::HostFunctionBinding {
//         todo!()
//     }

//     // fn doc(&self) -> String {
//     //     todo!()
//     // }

//     // fn params(&self) -> Vec<WanderType> {
//     //     todo!()
//     // }

//     // fn returns(&self) -> WanderType {
//     //     todo!()
//     // }

//     // fn name(&self) -> String {
//     //     "Ligature.statements".to_owned()
//     // }
// }

// struct AddStatementsFunction {
//     lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
// }
// impl HostFunction<NoHostType> for AddStatementsFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue<NoHostType>],
//         _bindings: &Bindings,
//     ) -> Result<wander::WanderValue<NoHostType>, WanderError> {
//         match arguments {
//             [WanderValue::String(name), WanderValue::List(statements)] => {
//                 let instance = self.lim.write().unwrap();
//                 match instance.get(name) {
//                     Some(ds_statements) => {
//                         //ds_statements.insert( Statement { entity: Identifier::new("test").unwrap(), attribute: todo!(), value: todo!() } );
//                         for statement in statements {
//                             match statement {
//                                 WanderValue::List(statement) => match &statement[..] {
//                                     [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), value] =>
//                                     {
//                                         let value: Value = match value {
//                                             WanderValue::Int(value) => {
//                                                 Value::Integer(value.to_owned())
//                                             }
//                                             WanderValue::String(value) => {
//                                                 Value::String(value.to_owned())
//                                             }
//                                             WanderValue::Identifier(value) => {
//                                                 Value::Identifier(value.to_owned())
//                                             }
//                                             _ => {
//                                                 return Err(WanderError(
//                                                     "Invalid Statement".to_owned(),
//                                                 ))
//                                             }
//                                         };
//                                         let statement = Statement {
//                                             entity: entity.to_owned(),
//                                             attribute: attribute.to_owned(),
//                                             value,
//                                         };
//                                         let mut ds_statements = ds_statements.borrow_mut();
//                                         ds_statements.insert(statement);
//                                     }
//                                     _ => todo!(),
//                                 },
//                                 _ => todo!(),
//                             }
//                         }
//                         Ok(WanderValue::Nothing)
//                     }
//                     _ => Ok(WanderValue::Nothing), // do nothing
//                 }
//             }
//             _ => Err(WanderError(
//                 "`addStatements` function requires one string parameter and a list of Statements."
//                     .to_owned(),
//             )),
//         }
//     }

//     fn binding(&self) -> wander::HostFunctionBinding {
//         todo!()
//     }

//     // fn doc(&self) -> String {
//     //     todo!()
//     // }

//     // fn params(&self) -> Vec<WanderType> {
//     //     todo!()
//     // }

//     // fn returns(&self) -> WanderType {
//     //     todo!()
//     // }

//     // fn name(&self) -> String {
//     //     "Ligature.addStatements".to_owned()
//     // }
// }

// fn wander_value_to_value(value: &WanderValue<NoHostType>) -> Result<Value, WanderError> {
//     match value {
//         WanderValue::Int(value) => Ok(Value::Integer(value.to_owned())),
//         WanderValue::String(value) => Ok(Value::String(value.to_owned())),
//         WanderValue::Identifier(value) => Ok(Value::Identifier(value.to_owned())),
//         _ => Err(WanderError("Invalid Statement".to_owned())),
//     }
// }

// fn value_to_wander_value(value: &Value) -> WanderValue<NoHostType> {
//     match value {
//         Value::Identifier(value) => WanderValue::Identifier(value.to_owned()),
//         Value::String(value) => WanderValue::String(value.to_owned()),
//         Value::Integer(value) => WanderValue::Int(value.to_owned()),
//         Value::Bytes(_) => todo!(),
//     }
// }

// struct RemoveStatementsFunction {
//     lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
// }
// impl HostFunction<NoHostType> for RemoveStatementsFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue<NoHostType>],
//         _bindings: &Bindings,
//     ) -> Result<wander::WanderValue<NoHostType>, WanderError> {
//         match arguments {
//             [WanderValue::String(name), WanderValue::List(statements)] => {
//                 let instance = self.lim.write().unwrap();
//                 match instance.get(name) {
//                     Some(ds_statements) => {
//                         for statement in statements {
//                             match statement {
//                                 WanderValue::List(statement) => match &statement[..] {
//                                     [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), value] =>
//                                     {
//                                         let value: Value = wander_value_to_value(value)?;
//                                         let statement = Statement {
//                                             entity: entity.to_owned(),
//                                             attribute: attribute.to_owned(),
//                                             value,
//                                         };
//                                         let mut ds_statements = ds_statements.borrow_mut();
//                                         ds_statements.remove(&statement);
//                                         return Ok(WanderValue::Nothing);
//                                     }
//                                     _ => todo!(),
//                                 },
//                                 _ => todo!(),
//                             }
//                         }
//                         Ok(WanderValue::Nothing)
//                     }
//                     _ => Ok(WanderValue::Nothing), // do nothing
//                 }
//             }
//             _ => Err(WanderError(
//                 "`removeStatements` function requires one string parameter and a list of Statements.".to_owned(),
//             )),
//         }
//     }

//     fn binding(&self) -> wander::HostFunctionBinding {
//         todo!()
//     }

    // fn doc(&self) -> String {
    //     todo!()
    // }

    // fn params(&self) -> Vec<WanderType> {
    //     todo!()
    // }

    // fn returns(&self) -> WanderType {
    //     todo!()
    // }

    // fn name(&self) -> String {
    //     "Ligature.removeStatements".to_owned()
    // }
//}

// struct QueryFunction {
//     lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
// }
// impl HostFunction<NoHostType> for QueryFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue<NoHostType>],
//         _bindings: &Bindings,
//     ) -> Result<wander::WanderValue<NoHostType>, WanderError> {
//         match arguments {
//             [WanderValue::String(name), entity, attribute, value] => {
//                 let instance = self.lim.read().unwrap();
//                 match instance.get(name) {
//                     Some(ds_statements) => {
//                         let res: Vec<WanderValue> = ds_statements
//                             .borrow()
//                             .iter()
//                             .filter(|statement| {
//                                 if let WanderValue::Identifier(id) = entity {
//                                     if statement.entity == *id {
//                                         //do nothing
//                                     } else {
//                                         return false;
//                                     }
//                                 } else if let WanderValue::Nothing = entity {
//                                     //do nothing
//                                 } else {
//                                     return false;
//                                 }

//                                 if let WanderValue::Identifier(id) = attribute {
//                                     if statement.attribute == *id {
//                                         //do nothing
//                                     } else {
//                                         return false;
//                                     }
//                                 } else if let WanderValue::Nothing = entity {
//                                     //do nothing
//                                 } else {
//                                     return false;
//                                 }

//                                 match value {
//                                     WanderValue::Boolean(_) => false,
//                                     WanderValue::Int(ovalue) => {
//                                         if let Value::Integer(ivalue) = &statement.value {
//                                             ovalue == ivalue
//                                         } else {
//                                             false
//                                         }
//                                     }
//                                     WanderValue::String(ovalue) => {
//                                         if let Value::String(ivalue) = &statement.value {
//                                             ovalue == ivalue
//                                         } else {
//                                             false
//                                         }
//                                     }
//                                     WanderValue::Identifier(ovalue) => {
//                                         if let Value::Identifier(ivalue) = &statement.value {
//                                             ovalue == ivalue
//                                         } else {
//                                             false
//                                         }
//                                     }
//                                     WanderValue::Nothing => true,
//                                     WanderValue::HostedFunction(_) => false,
//                                     WanderValue::Lambda(_, _) => false,
//                                     WanderValue::List(_) => false,
//                                     WanderValue::Graph(_) => false,
//                                     WanderValue::Tuple(_) => false,
//                                     WanderValue::Record(_) => false,
//                                     WanderValue::Application(_) => false,
//                                 }
//                             })
//                             .map(|statement| {
//                                 let entity = WanderValue::Identifier(statement.entity.to_owned());
//                                 let attribute =
//                                     WanderValue::Identifier(statement.attribute.to_owned());
//                                 let value = value_to_wander_value(&statement.value);
//                                 WanderValue::List(vec![entity, attribute, value])
//                             })
//                             .collect();
//                         Ok(WanderValue::List(res))
//                     }
//                     _ => Ok(WanderValue::Nothing), // do nothing
//                 }
//             }
//             _ => Err(WanderError("Error calling `query` function.".to_owned())),
//         }
//     }

    // fn doc(&self) -> String {
    //     "Query Dataset.".to_owned()
    // }

    // fn params(&self) -> Vec<wander::WanderType> {
    //     vec![
    //         WanderType::String,
    //         WanderType::Optional(Box::new(WanderType::Identifier)),
    //         WanderType::Optional(Box::new(WanderType::Identifier)),
    //         WanderType::Optional(Box::new(WanderType::Value)),
    //     ]
    // }

    // fn returns(&self) -> wander::WanderType {
    //     WanderType::Nothing
    // }

    // fn name(&self) -> String {
    //     "Ligature.query".to_owned()
    // }
//}

// fn write_graph(graph: &Graph, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.write_str("Graph.graph`").unwrap();
//     graph.all_statements().into_iter().for_each(|statement| {
//         f.write_str(write_statement(&statement).as_str()).unwrap();
//     });
//     f.write_str("`")
// }

// struct GraphFunction {}
// impl HostFunction for GraphFunction {
//     fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::List(statements)] => {
//                 let mut contents = BTreeSet::new();
//                 for statement in statements {
//                     match statement {
//                         WanderValue::Tuple(statement) => match &statement[..] {
//                             [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), value] =>
//                             {
//                                 let value = match value {
//                                     WanderValue::Int(value) => Value::Integer(*value),
//                                     WanderValue::String(value) => Value::String(value.to_owned()),
//                                     WanderValue::Identifier(value) => {
//                                         Value::Identifier(value.to_owned())
//                                     }
//                                     _ => todo!(),
//                                 };
//                                 contents.insert(Statement {
//                                     entity: entity.to_owned(),
//                                     attribute: attribute.to_owned(),
//                                     value,
//                                 });
//                             }
//                             _ => {
//                                 return Err(WanderError(
//                                     "Invalid Statement passsed to graph.".to_owned(),
//                                 ))
//                             }
//                         },
//                         _ => {
//                             return Err(WanderError(
//                                 "Invalid Statement passsed to graph.".to_owned(),
//                             ))
//                         }
//                     }
//                 }
//                 Ok(WanderValue::Graph(Graph::new(contents)))
//             }
//             _ => Err(WanderError(
//                 "`graph` function takes a list of Statements or no arguments.".to_owned(),
//             )),
//         }
//     }

//     fn doc(&self) -> String {
//         "Create a graph with the given Statements.".to_owned()
//     }

//     fn params(&self) -> Vec<crate::WanderType> {
//         vec![WanderType::List]
//     }

//     fn returns(&self) -> crate::WanderType {
//         WanderType::Graph
//     }

//     fn name(&self) -> String {
//         "Graph.graph".to_owned()
//     }
// }

// struct EmptyGraphFunction {}
// impl HostFunction for EmptyGraphFunction {
//     fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [] => Ok(WanderValue::Graph(Graph::default())),
//             _ => Err(WanderError(
//                 "`graph` function takes a list of Statements or no arguments.".to_owned(),
//             )),
//         }
//     }

//     fn doc(&self) -> String {
//         "Create an empty graph.".to_owned()
//     }

//     fn params(&self) -> Vec<crate::WanderType> {
//         vec![]
//     }

//     fn returns(&self) -> crate::WanderType {
//         WanderType::Graph
//     }

//     fn name(&self) -> String {
//         "Graph.empty".to_owned()
//     }
// }

// struct UnionFunction {}
// impl HostFunction for UnionFunction {
//     fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::Graph(g1), WanderValue::Graph(g2)] => {
//                 Ok(WanderValue::Graph(g1.add_all(g2.clone())))
//             }
//             _ => Err(WanderError(
//                 "`union` function takes two graphs as arguments.".to_owned(),
//             )),
//         }
//     }

//     fn doc(&self) -> String {
//         "Compute the union of two graphs.".to_owned()
//     }

//     fn params(&self) -> Vec<crate::WanderType> {
//         vec![WanderType::Graph, WanderType::Graph]
//     }

//     fn returns(&self) -> crate::WanderType {
//         WanderType::Graph
//     }

//     fn name(&self) -> String {
//         "Graph.union".to_owned()
//     }
// }

// struct DifferenceFunction {}
// impl HostFunction for DifferenceFunction {
//     fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::Graph(g1), WanderValue::Graph(g2)] => {
//                 Ok(WanderValue::Graph(g1.remove_all(g2.clone())))
//             }
//             _ => Err(WanderError(
//                 "`difference` function takes two graphs as arguments.".to_owned(),
//             )),
//         }
//     }

//     fn doc(&self) -> String {
//         "Compute the difference of two graphs.".to_owned()
//     }

//     fn params(&self) -> Vec<crate::WanderType> {
//         vec![WanderType::Graph, WanderType::Graph]
//     }

//     fn returns(&self) -> crate::WanderType {
//         WanderType::Graph
//     }

//     fn name(&self) -> String {
//         "Graph.difference".to_owned()
//     }
// }

// struct StatementsFunction {}
// impl HostFunction for StatementsFunction {
//     fn run(&self, arguments: &[WanderValue], _: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::Graph(graph)] => {
//                 let g: Vec<WanderValue> = graph
//                     .all_statements()
//                     .into_iter()
//                     .map(|s| {
//                         let entity = WanderValue::Identifier(s.entity);
//                         let attribute = WanderValue::Identifier(s.attribute);
//                         let value = match s.value {
//                             Value::Identifier(value) => WanderValue::Identifier(value),
//                             Value::String(value) => WanderValue::String(value),
//                             Value::Integer(value) => WanderValue::Int(value),
//                             Value::Bytes(_value) => todo!(),
//                         };
//                         WanderValue::Tuple(vec![entity, attribute, value])
//                     })
//                     .collect();
//                 Ok(WanderValue::List(g))
//             }
//             _ => Err(WanderError(
//                 "`statements` function takes one graphs as an argument.".to_owned(),
//             )),
//         }
//     }

//     fn doc(&self) -> String {
//         "Get all of the Statements in a Dataset.".to_owned()
//     }

//     fn params(&self) -> Vec<crate::WanderType> {
//         vec![WanderType::Graph]
//     }

//     fn returns(&self) -> crate::WanderType {
//         WanderType::List
//     }

//     fn name(&self) -> String {
//         "Graph.statements".to_owned()
//     }
// }

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

// fn graph_transform(input: &[crate::lexer::Token]) -> Result<Vec<crate::lexer::Token>, WanderError> {
//     let tokens: Vec<Token> = input.to_owned();
//     let statements: Vec<Statement> =
//         crate::lig::read_tokens(tokens).map_err(|e| WanderError(e.0))?;
//     let mut results = vec![];
//     results.append(&mut vec![
//         Token::Name("Graph.graph".to_owned()),
//         Token::OpenParen,
//         Token::OpenSquare,
//     ]);
//     for statement in statements {
//         results.push(Token::OpenParen);
//         results.push(Token::Identifier(statement.entity));
//         results.push(Token::Identifier(statement.attribute));
//         match statement.value {
//             Value::Identifier(value) => results.push(Token::Identifier(value)),
//             Value::String(value) => results.push(Token::String(value)),
//             Value::Integer(value) => results.push(Token::Int(value)),
//             Value::Bytes(_) => todo!(),
//         }
//         results.push(Token::CloseParen);
//     }
//     results.push(Token::CloseSquare);
//     results.push(Token::CloseParen);
//     Ok(results)
// }

    // bindings.bind_host_function(Rc::new(EmptyGraphFunction {}));
    // bindings.bind_host_function(Rc::new(GraphFunction {}));
    // bindings.bind_host_function(Rc::new(UnionFunction {}));
    // bindings.bind_host_function(Rc::new(DifferenceFunction {}));
    // bindings.bind_host_function(Rc::new(StatementsFunction {}));
    // bindings.bind_native_function(
    //     "Graph".to_owned(),
    //     "find".to_owned(),
    //     Rc::new(FindFunction {}),
    // );
    // bindings.bind_token_transformer(
    //     "Graph".to_owned(),
    //     "graph".to_owned(),
    //     Rc::new(graph_transform),
    // );
//     fn identifier(gaze: &mut Gaze<Token>) -> Option<Element> {
//         match gaze.next() {
//             Some(Token::Identifier(value)) => Some(Element::Identifier(value)),
//             _ => None,
//         }
//     }
//     fn identifier(lex: &mut Lexer<Token>) -> Option<Identifier> {
//         let slice = lex.slice();
//         match Identifier::new(slice.slice(1..(slice.len() - 1)).unwrap()) {
//             Ok(ident) => Some(ident),
//             Err(_) => None,
//         }
//     }
//     // #[regex("<[a-zA-Z0-9-._~:/?#\\[\\]@!$&'()*+,;%=\\x{00A0}-\\x{D7FF}\\x{F900}-\\x{FDCF}\\x{FDF0}-\\x{FFEF}\\x{10000}-\\x{1FFFD}\\x{20000}-\\x{2FFFD}\\x{30000}-\\x{3FFFD}\\x{40000}-\\x{4FFFD}\\x{50000}-\\x{5FFFD}\\x{60000}-\\x{6FFFD}\\x{70000}-\\x{7FFFD}\\x{80000}-\\x{8FFFD}\\x{90000}-\\x{9FFFD}\\x{A0000}-\\x{AFFFD}\\x{B0000}-\\x{BFFFD}\\x{C0000}-\\x{CFFFD}\\x{D0000}-\\x{DFFFD}\\x{E1000}-\\x{EFFFD}]+>", identifier)]
//     // Identifier(Identifier),

//     pub fn write_value(value: &Value) -> String {
//         match value {
//             Value::Identifier(entity) => write_identifier(entity),
//             Value::Integer(integer) => write_integer(integer),
//             //Value::FloatLiteral(float) => write_float(float),
//             Value::String(string) => write_string(string),
//             Value::Bytes(bytes) => write_bytes(bytes),
//         }
//     }
//     pub fn write_statement(statement: &Statement) -> String {
//         format!(
//             "{} {} {}\n",
//             write_identifier(&statement.entity),
//             write_identifier(&statement.attribute),
//             write_value(&statement.value),
//         )
//     }
//     /// Writes out an Entity to a String.
// pub fn write_identifier(entity: &Identifier) -> String {
//     format!("<{}>", entity.id())
// }

// struct EntityFunction {}
// impl<T: Clone + PartialEq> HostFunction<T> for EntityFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue<T>],
//         _bindings: &Bindings<T>,
//     ) -> Result<WanderValue<T>, WanderError> {
//         if let [WanderValue::Tuple(value)] = arguments {
//             if value.len() == 3 {
//                 Ok(value.get(0).unwrap().clone())
//             } else {
//                 Err(WanderError(
//                     "`entity` function requires one Statement parameter.".to_owned(),
//                 ))
//             }
//         } else {
//             Err(WanderError(
//                 "`entity` function requires one Statement parameter.".to_owned(),
//             ))
//         }
//     }

//     fn doc(&self) -> String {
//         "Retrieve the Entity from a Statement.".to_owned()
//     }

//     fn params(&self) -> Vec<crate::WanderType> {
//         vec![WanderType::Tuple]
//     }

//     fn returns(&self) -> crate::WanderType {
//         WanderType::Identifier
//     }

//     fn name(&self) -> String {
//         "Statement.entity".to_owned()
//     }
// }

// struct AttributeFunction {}
// impl<T: Clone + PartialEq> HostFunction<T> for AttributeFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue<T>],
//         _bindings: &Bindings<T>,
//     ) -> Result<WanderValue<T>, WanderError> {
//         if let [WanderValue::List(value)] = arguments {
//             if value.len() == 3 {
//                 Ok(value.get(1).unwrap().clone())
//             } else {
//                 Err(WanderError(
//                     "`attribute` function requires one Statement parameter.".to_owned(),
//                 ))
//             }
//         } else {
//             Err(WanderError(
//                 "`attribute` function requires one Statement parameter.".to_owned(),
//             ))
//         }
//     }

//     fn doc(&self) -> String {
//         "Retrieve the Attribute from a Statement.".to_owned()
//     }

//     fn params(&self) -> Vec<crate::WanderType> {
//         vec![WanderType::Tuple]
//     }

//     fn returns(&self) -> crate::WanderType {
//         WanderType::Identifier
//     }

//     fn name(&self) -> String {
//         "Statement.attribute".to_owned()
//     }
// }

// struct ValueFunction {}
// impl<T: Clone + PartialEq> HostFunction<T> for ValueFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue<T>],
//         _bindings: &Bindings<T>,
//     ) -> Result<WanderValue<T>, WanderError> {
//         if let [WanderValue::List(value)] = arguments {
//             if value.len() == 3 {
//                 Ok(value.get(2).unwrap().clone())
//             } else {
//                 Err(WanderError(
//                     "`value` function requires one Statement parameter.".to_owned(),
//                 ))
//             }
//         } else {
//             Err(WanderError(
//                 "`value` function requires one Statement parameter.".to_owned(),
//             ))
//         }
//     }

    // fn doc(&self) -> String {
    //     "Retrieve the Value from a Statement.".to_owned()
    // }

    // fn params(&self) -> Vec<crate::WanderType> {
    //     vec![WanderType::Tuple]
    // }

    // fn returns(&self) -> crate::WanderType {
    //     WanderType::Value
    // }

    // fn name(&self) -> String {
    //     "Statement.value".to_owned()
    // }
// }

// bindings.bind_host_function(Rc::new(EntityFunction {}));
// bindings.bind_host_function(Rc::new(AttributeFunction {}));
// bindings.bind_host_function(Rc::new(ValueFunction {}));
