// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{parser::Element, WanderError};

pub fn translate(elements: Vec<Element>) -> Result<Vec<Element>, WanderError> {
    process_forwards(elements)
}

fn process_forwards(elements: Vec<Element>) -> Result<Vec<Element>, WanderError> {
    let mut index = 0;
    let mut results: Vec<Element> = vec![];
    while let Some(element) = elements.get(index) {
        if element == &Element::Forward {
            let prev = results.pop().unwrap(); //elements.get(index - 1).unwrap();
            index += 1;
            if let Some(Element::FunctionCall(name, arguments)) = elements.get(index) {
                let mut arguments = arguments.clone();
                arguments.push(prev.clone());
                results.push(Element::FunctionCall(name.to_owned(), arguments));
            } else {
                return Err(WanderError("Error handling forward operator.".to_owned()));
            }
        } else {
            results.push(element.clone());
        }
        index += 1;
    }
    Ok(results)
}
