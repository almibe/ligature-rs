// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::LigatureError;
use std::rc::Rc;

use crate::{bindings::Bindings, parser::Element, NativeFunction};

struct AndFunction {}
impl NativeFunction for AndFunction {
    fn run(
        &self,
        arguments: &Vec<Element>,
        _bindings: &mut Bindings,
    ) -> Result<crate::WanderValue, ligature::LigatureError> {
        if let [Element::Boolean(left), Element::Boolean(right)] = arguments[..] {
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
        arguments: &Vec<Element>,
        _bindings: &mut Bindings,
    ) -> Result<crate::WanderValue, ligature::LigatureError> {
        if let [Element::Boolean(value)] = arguments[..] {
            Ok(crate::WanderValue::Boolean(!value))
        } else {
            Err(LigatureError(
                "`not` function requires one boolean parameter.".to_owned(),
            ))
        }
    }
}

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common() -> Bindings {
    let mut bindings = Bindings::new();
    bindings.bind_native_function(String::from("and"), Rc::new(AndFunction {}));
    bindings.bind_native_function(String::from("not"), Rc::new(NotFunction {}));
    bindings
}
