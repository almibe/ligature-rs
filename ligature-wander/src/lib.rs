// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Wander support for working with Ligature.

use ligature_graph::Graph;

fn write_graph(graph: &Graph, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Graph.graph`").unwrap();
    graph.all_statements().into_iter().for_each(|statement| {
        f.write_str(write_statement(&statement).as_str()).unwrap();
    });
    f.write_str("`")
}

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
    fn identifier(gaze: &mut Gaze<Token>) -> Option<Element> {
        match gaze.next() {
            Some(Token::Identifier(value)) => Some(Element::Identifier(value)),
            _ => None,
        }
    }
    fn identifier(lex: &mut Lexer<Token>) -> Option<Identifier> {
        let slice = lex.slice();
        match Identifier::new(slice.slice(1..(slice.len() - 1)).unwrap()) {
            Ok(ident) => Some(ident),
            Err(_) => None,
        }
    }
    // #[regex("<[a-zA-Z0-9-._~:/?#\\[\\]@!$&'()*+,;%=\\x{00A0}-\\x{D7FF}\\x{F900}-\\x{FDCF}\\x{FDF0}-\\x{FFEF}\\x{10000}-\\x{1FFFD}\\x{20000}-\\x{2FFFD}\\x{30000}-\\x{3FFFD}\\x{40000}-\\x{4FFFD}\\x{50000}-\\x{5FFFD}\\x{60000}-\\x{6FFFD}\\x{70000}-\\x{7FFFD}\\x{80000}-\\x{8FFFD}\\x{90000}-\\x{9FFFD}\\x{A0000}-\\x{AFFFD}\\x{B0000}-\\x{BFFFD}\\x{C0000}-\\x{CFFFD}\\x{D0000}-\\x{DFFFD}\\x{E1000}-\\x{EFFFD}]+>", identifier)]
    // Identifier(Identifier),

    pub fn write_value(value: &Value) -> String {
        match value {
            Value::Identifier(entity) => write_identifier(entity),
            Value::Integer(integer) => write_integer(integer),
            //Value::FloatLiteral(float) => write_float(float),
            Value::String(string) => write_string(string),
            Value::Bytes(bytes) => write_bytes(bytes),
        }
    }
    pub fn write_statement(statement: &Statement) -> String {
        format!(
            "{} {} {}\n",
            write_identifier(&statement.entity),
            write_identifier(&statement.attribute),
            write_value(&statement.value),
        )
    }
    /// Writes out an Entity to a String.
pub fn write_identifier(entity: &Identifier) -> String {
    format!("<{}>", entity.id())
}
