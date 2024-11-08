// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Call, WanderError, WanderValue};

// Handle any tranlations needed before creating an expression.
pub fn translate(calls: Vec<Call>) -> Result<WanderValue, WanderError> {
    //let element = process_pipes(&element)?;
    express(&calls)
}

// fn process_pipes(element: &ParserElement) -> Result<ParserElement, WanderError> {
//     let elements = match element {
//         ParserElement::Grouping(elements) => elements,
//         e => return Ok(e.clone()),
//     };
//     let mut index = 0;
//     let mut results = vec![];
//     while let Some(element) = elements.get(index) {
//         if *element == ParserElement::Pipe {
//             index += 1;
//             match elements.get(index) {
//                 Some(ParserElement::Grouping(next_elements)) => {
//                     let mut next_elements = next_elements.clone();
//                     let mut new_results = vec![];
//                     next_elements.append(&mut results);
//                     new_results.push(ParserElement::Grouping(next_elements.clone()));
//                     results = new_results;
//                 }
//                 _ => return Err(WanderError("Invalid pipe.".to_owned())),
//             }
//         } else {
//             results.push(element.clone());
//         }
//         index += 1;
//     }
//     Ok(ParserElement::Grouping(results))
// }

pub fn express(_element: &Vec<Call>) -> Result<WanderValue, WanderError> {
    todo!()
    // let expression = match element {
    //     ParserElement::Element(value) => Expression::Element(value.clone()),
    //     ParserElement::Grouping(elements) => return handle_grouping(elements),
    //     ParserElement::Pipe => {
    //         return Err(WanderError(
    //             "Cannot process pipe, Should never reach.".to_owned(),
    //         ))
    //     }
    //     ParserElement::HostFunction(name) => Expression::HostFunction(name.clone()),
    // };
    // Ok(expression)
}

fn handle_grouping(_elements: &[WanderValue]) -> Result<WanderValue, WanderError> {
    todo!()
    // let expressions: Vec<Expression> = elements.iter().map(|e| express(e).unwrap()).collect();
    // let expressions: Vec<Expression> = expressions
    //     .iter()
    //     .map(|e| match e {
    //         Expression::Application(application) => {
    //             if application.len() == 1 {
    //                 application.first().unwrap().clone()
    //             } else {
    //                 e.clone()
    //             }
    //         }
    //         e => e.clone(),
    //     })
    //     .collect();
    // if expressions.len() == 1 {
    //     Ok(expressions.first().unwrap().clone())
    // } else {
    //     let res = Expression::Application(expressions);
    //     Ok(res)
    // }
}
