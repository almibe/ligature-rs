// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{bindings::Bindings, NativeFunction};

struct AndFunction {}
impl NativeFunction for AndFunction {
    fn run(&self) -> Result<crate::WanderValue, ligature::LigatureError> {
        Ok(crate::WanderValue::Boolean(true))
    }
}

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common() -> Bindings {
    let mut bindings = Bindings::new();
    bindings.bind_native_function(String::from("and"), Box::new(AndFunction {}));
    bindings
}
