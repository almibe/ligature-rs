// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the Ligature project.
//! It represents to common types and traits used by Ligature.

//#![deny(missing_docs)]

use std::fmt::Debug;

use lexer::Token;

pub mod lexer;
pub mod read;
pub mod write;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Store location information alongside a value.
pub struct Location<T: PartialEq + Eq>(pub T, pub usize);

// #[derive(Debug, Serialize)]
// /// Structure used for debugging or inspecting code.
// pub struct Introspection {
//     /// A list of all Tokens including whitespace.
//     pub tokens_ws: Vec<Location<Token>>,
//     /// A list of all Tokens without whitespace.
//     pub tokens: Vec<Location<Token>>,
//     /// A list of all Tokens after macro transformations.
//     pub tokens_transformed: Vec<Location<Token>>,
//     /// Element representation.
//     pub element: Location<Element>,
//     /// Expression representation.
//     pub expression: Location<Expression>,
// }

// /// Run a Wander script with the given Bindings.
// pub fn introspect(
//     script: &str,
// ) -> Result<Introspection, LigatureError> {
//     let tokens_ws = tokenize(script).or(Ok(vec![]))?;
//     let tokens = tokenize_and_filter(script).or(Ok(vec![]))?;
//     let tokens_transformed = transform(&tokens.clone(), bindings).or(Ok(vec![]))?;
//     // let element = parse(tokens_transformed.clone()).or(Ok(Location(Element::Nothing, 0)))?; //TODO handle errors better
//     //let expression = translate(element.clone()).or(Ok(Location(Expression::Nothing, 0)))?; //TODO handle errors better
//     // Ok(Introspection {
//     //     tokens_ws,
//     //     tokens,
//     //     tokens_transformed,
//     //     element,
//     //     expression,
//     // })
//     todo!()
// }
