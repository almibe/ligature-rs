// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use wander::{
    interpreter::eval, interpreter::Expression, preludes::common, HostFunction,
    HostFunctionBinding, HostValue, WanderError, WanderValue, Location,
};

struct SayHello {}
impl HostFunction<String> for SayHello {
    fn run(
        &self,
        _arguments: &[WanderValue<String>],
        _bindings: &wander::environment::Environment<String>,
    ) -> Result<WanderValue<String>, WanderError> {
        Ok(WanderValue::HostValue(HostValue {
            value: "hello!".to_owned(),
        }))
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            name: "hello".to_owned(),
            parameters: vec![],
            result: None,
            doc_string: "Say hello!".to_owned(),
        }
    }
}

//#[test]
fn eval_host_value() {
    let mut bindings = common::<String>();
    bindings.bind_host_function(Rc::new(SayHello {}));
    let input = Location(Expression::Nothing, 0);
    let res = eval(&input, &mut bindings);
    let expected = Ok(WanderValue::HostValue(HostValue {
        value: "hello!".to_owned(),
    }));
    assert_eq!(res, expected);
}
