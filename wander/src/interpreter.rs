// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::LigatureError;

use crate::parser::Element;
use crate::WanderValue;

pub fn eval(script: Vec<Element>) -> Result<WanderValue, LigatureError> {
    match script.last() {
        Some(Element::Boolean(value)) => Ok(WanderValue::Boolean(*value)),
        Some(Element::Int(value)) => Ok(WanderValue::Int(*value)),
        Some(Element::String(value)) => Ok(WanderValue::String(value.to_string())),
        _ => todo!(),
    }
}
