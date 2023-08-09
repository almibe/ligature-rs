// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::LigatureError;
use std::rc::Rc;

use crate::{bindings::Bindings, NativeFunction, TokenTransformer, WanderValue};

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
}

struct EntityFunction {}
impl NativeFunction for EntityFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::List(value)] = &arguments[..] {
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
}

struct AttributeFunction {}
impl NativeFunction for AttributeFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::List(value)] = &arguments[..] {
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
}

struct ValueFunction {}
impl NativeFunction for ValueFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::List(value)] = &arguments[..] {
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
}

struct AtFunction {}
impl NativeFunction for AtFunction {
    fn run(&self, arguments: &[WanderValue]) -> Result<WanderValue, LigatureError> {
        if let [WanderValue::Int(index), WanderValue::List(value)] = &arguments[..] {
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
}

struct GraphTransformer {}
impl TokenTransformer for GraphTransformer {
    fn transform(
        &self,
        input: &[crate::lexer::Token],
    ) -> Result<Vec<crate::lexer::Token>, LigatureError> {
        todo!()
    }
}

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common() -> Bindings {
    let mut bindings = Bindings::new();
    bindings.bind_native_function(String::from("and"), Rc::new(AndFunction {}));
    bindings.bind_native_function(String::from("not"), Rc::new(NotFunction {}));
    bindings.bind_native_function(String::from("entity"), Rc::new(EntityFunction {}));
    bindings.bind_native_function(String::from("attribute"), Rc::new(AttributeFunction {}));
    bindings.bind_native_function(String::from("value"), Rc::new(ValueFunction {}));
    bindings.bind_native_function(String::from("at"), Rc::new(AtFunction {}));
    bindings.bind_token_transformer(String::from("graph"), Rc::new(GraphTransformer {}));
    bindings
}
