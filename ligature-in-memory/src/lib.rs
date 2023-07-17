// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of Ligature that uses only
//! in-memory persistent data structures for storing data.

use std::{collections::{BTreeMap, BTreeSet}, sync::RwLock, rc::Rc};

use ligature::{Statement, LigatureError};
use wander::{bindings::Bindings, NativeFunction, WanderValue};

pub struct LigatureInMemory {
    datasets: Rc<RwLock<BTreeMap<String, BTreeSet<Statement>>>>,
}

impl LigatureInMemory {
    pub fn new() -> Self {
        Self {
            datasets: Rc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn add_bindings(&self, bindings: &mut Bindings) {
        bindings.bind_native_function(
            String::from("datasets"),
            Rc::new(DatasetsFunction {
                lim: self.datasets.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("addDataset"),
            Rc::new(AddDatasetFunction {
                lim: self.datasets.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("removeDataset"),
            Rc::new(RemoveDatasetFunction {
                lim: self.datasets.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("statements"),
            Rc::new(StatementsFunction {
                lim: self.datasets.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("addStatements"),
            Rc::new(AddStatementsFunction {
                lim: self.datasets.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("removeStatements"),
            Rc::new(RemoveStatementsFunction {
                lim: self.datasets.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("query"),
            Rc::new(QueryFunction {
                lim: self.datasets.clone(),
            }),
        );
    }
}

struct DatasetsFunction {
    lim: Rc<RwLock<BTreeMap<String, BTreeSet<Statement>>>>,
}
impl NativeFunction for DatasetsFunction {
    fn run(
        &self,
        arguments: &Vec<WanderValue>,
    ) -> Result<WanderValue, LigatureError> {
        if arguments.is_empty() {
            let x = self.lim.read().unwrap();
            let x = x.keys().map(|e| WanderValue::String(e.to_owned())).collect();
            Ok(WanderValue::List(x))
        } else {
            Err(LigatureError("`datasets` function requires no arguments.".to_owned()))
        }
    }
}

struct AddDatasetFunction {
    lim: Rc<RwLock<BTreeMap<String, BTreeSet<Statement>>>>,
}
impl NativeFunction for AddDatasetFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, ligature::LigatureError> {
        match &arguments[..] {
            [WanderValue::String(name)] => {
                let mut instance = self.lim.write().unwrap();
                if instance.contains_key(name) {
                    Ok(WanderValue::Nothing) //do nothing
                } else {
                    instance.insert(name.to_owned(), BTreeSet::new());
                    Ok(WanderValue::Nothing)
                }
            },
            _ => Err(LigatureError("`addDataset` function requires one string parameter.".to_owned()))
        }
    }
}

struct RemoveDatasetFunction {
    lim: Rc<RwLock<BTreeMap<String, BTreeSet<Statement>>>>,
}
impl NativeFunction for RemoveDatasetFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, ligature::LigatureError> {
        match &arguments[..] {
            [WanderValue::String(name)] => {
                let mut instance = self.lim.write().unwrap();
                if instance.contains_key(name) {
                    instance.remove(name);
                    Ok(WanderValue::Nothing)
                } else {
                    Ok(WanderValue::Nothing) // do nothing
                }
            },
            _ => Err(LigatureError("`removeDataset` function requires one string parameter.".to_owned()))
        }
    }
}

struct StatementsFunction {
    lim: Rc<RwLock<BTreeMap<String, BTreeSet<Statement>>>>,
}
impl NativeFunction for StatementsFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, ligature::LigatureError> {
        match &arguments[..] {
            [WanderValue::String(name)] => {
                let instance = self.lim.read().unwrap();
                match instance.get(name) {
                    Some(statements) => {
                        let mut results = vec![];
                        for statement in statements {
                            let entity = WanderValue::Identifier(statement.entity.clone());
                            let attribute = WanderValue::Identifier(statement.attribute.clone());
                            let value = match statement.value.clone() {
                                ligature::Value::Identifier(value) => WanderValue::Identifier(value),
                                ligature::Value::StringLiteral(value) => WanderValue::String(value),
                                ligature::Value::IntegerLiteral(value) => WanderValue::Int(value),
                                ligature::Value::BytesLiteral(value) => todo!(),
                            };
                            results.push(WanderValue::List(vec![entity, attribute, value]));
                        }
                        Ok(WanderValue::List(results))
                    },
                    _ => Ok(WanderValue::Nothing) // do nothing
                }
            },
            _ => Err(LigatureError("`removeDataset` function requires one string parameter.".to_owned()))
        }
    }
}

struct AddStatementsFunction {
    lim: Rc<RwLock<BTreeMap<String, BTreeSet<Statement>>>>,
}
impl NativeFunction for AddStatementsFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, ligature::LigatureError> {
        todo!()
    }
}

struct RemoveStatementsFunction {
    lim: Rc<RwLock<BTreeMap<String, BTreeSet<Statement>>>>,
}
impl NativeFunction for RemoveStatementsFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, ligature::LigatureError> {
        todo!()
    }
}

struct QueryFunction {
    lim: Rc<RwLock<BTreeMap<String, BTreeSet<Statement>>>>,
}
impl NativeFunction for QueryFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, ligature::LigatureError> {
        todo!()
    }
}
