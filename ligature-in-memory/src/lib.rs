// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of Ligature that uses only
//! in-memory persistent data structures for storing data.

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    sync::RwLock,
};

use ligature::{Dataset, Ligature, LigatureError, Query, Statement, Value};
use wander::{bindings::Bindings, HostFunction, WanderError, WanderType, WanderValue};

#[derive(Default)]
pub struct LigatureInMemory {
    datasets: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}

impl Ligature for LigatureInMemory {
    fn datasets(&self) -> Result<Vec<Dataset>, LigatureError> {
        todo!()
    }

    fn add_dataset(&mut self, _dataset: &Dataset) -> Result<(), LigatureError> {
        todo!()
    }

    fn remove_dataset(&mut self, _dataset: &Dataset) -> Result<(), LigatureError> {
        todo!()
    }

    fn statements(&self, _dataset: &Dataset) -> Result<Vec<Statement>, LigatureError> {
        todo!()
    }

    fn add_statements(
        &self,
        _dataset: &Dataset,
        _statements: Vec<Statement>,
    ) -> Result<(), LigatureError> {
        todo!()
    }

    fn remove_statements(
        &self,
        _dataset: &Dataset,
        _statements: Vec<Statement>,
    ) -> Result<(), LigatureError> {
        todo!()
    }

    fn query(&self) -> Result<Box<dyn Query>, LigatureError> {
        todo!()
    }
}

impl LigatureInMemory {
    pub fn new() -> Self {
        Self {
            datasets: Rc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn add_bindings(&self, bindings: &mut Bindings) {
        bindings.bind_host_function(Rc::new(DatasetsFunction {
            lim: self.datasets.clone(),
        }));
        bindings.bind_host_function(Rc::new(AddDatasetFunction {
            lim: self.datasets.clone(),
        }));
        bindings.bind_host_function(Rc::new(RemoveDatasetFunction {
            lim: self.datasets.clone(),
        }));
        bindings.bind_host_function(Rc::new(StatementsFunction {
            lim: self.datasets.clone(),
        }));
        bindings.bind_host_function(Rc::new(AddStatementsFunction {
            lim: self.datasets.clone(),
        }));
        bindings.bind_host_function(Rc::new(RemoveStatementsFunction {
            lim: self.datasets.clone(),
        }));
        bindings.bind_host_function(Rc::new(QueryFunction {
            lim: self.datasets.clone(),
        }));
    }
}

struct DatasetsFunction {
    lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}
impl HostFunction for DatasetsFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        if arguments.is_empty() {
            let datasets = self.lim.read().unwrap();
            let datasets = datasets
                .keys()
                .map(|name| WanderValue::String(name.to_owned()))
                .collect();
            Ok(WanderValue::List(datasets))
        } else {
            Err(WanderError(
                "`datasets` function requires no arguments.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<WanderType> {
        todo!()
    }

    fn returns(&self) -> WanderType {
        todo!()
    }

    fn name(&self) -> String {
        "Ligature.datasets".to_owned()
    }
}

struct AddDatasetFunction {
    lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}
impl HostFunction for AddDatasetFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<wander::WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name)] => {
                let mut instance = self.lim.write().unwrap();
                if instance.contains_key(name) {
                    Ok(WanderValue::Nothing) //do nothing
                } else {
                    let instance = instance.borrow_mut();
                    instance.insert(name.to_owned(), RefCell::new(BTreeSet::new()));
                    Ok(WanderValue::Nothing)
                }
            }
            _ => Err(WanderError(
                "`addDataset` function requires one string parameter.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<WanderType> {
        todo!()
    }

    fn returns(&self) -> WanderType {
        todo!()
    }

    fn name(&self) -> String {
        "Ligature.addDataset".to_owned()
    }
}

struct RemoveDatasetFunction {
    lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}
impl HostFunction for RemoveDatasetFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<wander::WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name)] => {
                let mut instance = self.lim.write().unwrap();
                if instance.contains_key(name) {
                    instance.remove(name);
                    Ok(WanderValue::Nothing)
                } else {
                    Ok(WanderValue::Nothing) // do nothing
                }
            }
            _ => Err(WanderError(
                "`removeDataset` function requires one string parameter.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<WanderType> {
        todo!()
    }

    fn returns(&self) -> WanderType {
        todo!()
    }

    fn name(&self) -> String {
        "Ligature.removeDataset".to_owned()
    }
}

struct StatementsFunction {
    lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}
impl HostFunction for StatementsFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<wander::WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name)] => {
                let instance = self.lim.read().unwrap();
                match instance.get(name) {
                    Some(statements) => {
                        let mut results = vec![];
                        let statements = statements.borrow();
                        for statement in statements.iter() {
                            let entity = WanderValue::Identifier(statement.entity.clone());
                            let attribute = WanderValue::Identifier(statement.attribute.clone());
                            let value = match statement.value.clone() {
                                ligature::Value::Identifier(value) => {
                                    WanderValue::Identifier(value)
                                }
                                ligature::Value::String(value) => WanderValue::String(value),
                                ligature::Value::Integer(value) => WanderValue::Int(value),
                                ligature::Value::Bytes(_) => todo!(),
                            };
                            results.push(WanderValue::List(vec![entity, attribute, value]));
                        }
                        Ok(WanderValue::List(results))
                    }
                    _ => Ok(WanderValue::Nothing), // do nothing
                }
            }
            _ => Err(WanderError(
                "`removeDataset` function requires one string parameter.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<WanderType> {
        todo!()
    }

    fn returns(&self) -> WanderType {
        todo!()
    }

    fn name(&self) -> String {
        "Ligature.statements".to_owned()
    }
}

struct AddStatementsFunction {
    lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}
impl HostFunction for AddStatementsFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<wander::WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name), WanderValue::List(statements)] => {
                let instance = self.lim.write().unwrap();
                match instance.get(name) {
                    Some(ds_statements) => {
                        //ds_statements.insert( Statement { entity: Identifier::new("test").unwrap(), attribute: todo!(), value: todo!() } );
                        for statement in statements {
                            match statement {
                                WanderValue::List(statement) => match &statement[..] {
                                    [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), value] =>
                                    {
                                        let value: Value = match value {
                                            WanderValue::Int(value) => {
                                                Value::Integer(value.to_owned())
                                            }
                                            WanderValue::String(value) => {
                                                Value::String(value.to_owned())
                                            }
                                            WanderValue::Identifier(value) => {
                                                Value::Identifier(value.to_owned())
                                            }
                                            _ => {
                                                return Err(WanderError(
                                                    "Invalid Statement".to_owned(),
                                                ))
                                            }
                                        };
                                        let statement = Statement {
                                            entity: entity.to_owned(),
                                            attribute: attribute.to_owned(),
                                            value,
                                        };
                                        let mut ds_statements = ds_statements.borrow_mut();
                                        ds_statements.insert(statement);
                                    }
                                    _ => todo!(),
                                },
                                _ => todo!(),
                            }
                        }
                        Ok(WanderValue::Nothing)
                    }
                    _ => Ok(WanderValue::Nothing), // do nothing
                }
            }
            _ => Err(WanderError(
                "`addStatements` function requires one string parameter and a list of Statements."
                    .to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<WanderType> {
        todo!()
    }

    fn returns(&self) -> WanderType {
        todo!()
    }

    fn name(&self) -> String {
        "Ligature.addStatements".to_owned()
    }
}

fn wander_value_to_value(value: &WanderValue) -> Result<Value, WanderError> {
    match value {
        WanderValue::Int(value) => Ok(Value::Integer(value.to_owned())),
        WanderValue::String(value) => Ok(Value::String(value.to_owned())),
        WanderValue::Identifier(value) => Ok(Value::Identifier(value.to_owned())),
        _ => Err(WanderError("Invalid Statement".to_owned())),
    }
}

fn value_to_wander_value(value: &Value) -> WanderValue {
    match value {
        Value::Identifier(value) => WanderValue::Identifier(value.to_owned()),
        Value::String(value) => WanderValue::String(value.to_owned()),
        Value::Integer(value) => WanderValue::Int(value.to_owned()),
        Value::Bytes(_) => todo!(),
    }
}

struct RemoveStatementsFunction {
    lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}
impl HostFunction for RemoveStatementsFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<wander::WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name), WanderValue::List(statements)] => {
                let instance = self.lim.write().unwrap();
                match instance.get(name) {
                    Some(ds_statements) => {
                        for statement in statements {
                            match statement {
                                WanderValue::List(statement) => match &statement[..] {
                                    [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), value] =>
                                    {
                                        let value: Value = wander_value_to_value(value)?;
                                        let statement = Statement {
                                            entity: entity.to_owned(),
                                            attribute: attribute.to_owned(),
                                            value,
                                        };
                                        let mut ds_statements = ds_statements.borrow_mut();
                                        ds_statements.remove(&statement);
                                        return Ok(WanderValue::Nothing);
                                    }
                                    _ => todo!(),
                                },
                                _ => todo!(),
                            }
                        }
                        Ok(WanderValue::Nothing)
                    }
                    _ => Ok(WanderValue::Nothing), // do nothing
                }
            }
            _ => Err(WanderError(
                "`removeStatements` function requires one string parameter and a list of Statements.".to_owned(),
            )),
        }
    }

    fn doc(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<WanderType> {
        todo!()
    }

    fn returns(&self) -> WanderType {
        todo!()
    }

    fn name(&self) -> String {
        "Ligature.removeStatements".to_owned()
    }
}

struct QueryFunction {
    lim: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}
impl HostFunction for QueryFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<wander::WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name), entity, attribute, value] => {
                let instance = self.lim.read().unwrap();
                match instance.get(name) {
                    Some(ds_statements) => {
                        let res: Vec<WanderValue> = ds_statements
                            .borrow()
                            .iter()
                            .filter(|statement| {
                                if let WanderValue::Identifier(id) = entity {
                                    if statement.entity == *id {
                                        //do nothing
                                    } else {
                                        return false;
                                    }
                                } else if let WanderValue::Nothing = entity {
                                    //do nothing
                                } else {
                                    return false;
                                }

                                if let WanderValue::Identifier(id) = attribute {
                                    if statement.attribute == *id {
                                        //do nothing
                                    } else {
                                        return false;
                                    }
                                } else if let WanderValue::Nothing = entity {
                                    //do nothing
                                } else {
                                    return false;
                                }

                                match value {
                                    WanderValue::Boolean(_) => false,
                                    WanderValue::Int(ovalue) => {
                                        if let Value::Integer(ivalue) = &statement.value {
                                            ovalue == ivalue
                                        } else {
                                            false
                                        }
                                    }
                                    WanderValue::String(ovalue) => {
                                        if let Value::String(ivalue) = &statement.value {
                                            ovalue == ivalue
                                        } else {
                                            false
                                        }
                                    }
                                    WanderValue::Identifier(ovalue) => {
                                        if let Value::Identifier(ivalue) = &statement.value {
                                            ovalue == ivalue
                                        } else {
                                            false
                                        }
                                    }
                                    WanderValue::Nothing => true,
                                    WanderValue::NativeFunction(_) => false,
                                    WanderValue::Lambda(_, _) => false,
                                    WanderValue::List(_) => false,
                                    WanderValue::Graph(_) => false,
                                    WanderValue::Tuple(_) => false,
                                    WanderValue::Record(_) => todo!(),
                                }
                            })
                            .map(|statement| {
                                let entity = WanderValue::Identifier(statement.entity.to_owned());
                                let attribute =
                                    WanderValue::Identifier(statement.attribute.to_owned());
                                let value = value_to_wander_value(&statement.value);
                                WanderValue::List(vec![entity, attribute, value])
                            })
                            .collect();
                        Ok(WanderValue::List(res))
                    }
                    _ => Ok(WanderValue::Nothing), // do nothing
                }
            }
            _ => Err(WanderError("Error calling `query` function.".to_owned())),
        }
    }

    fn doc(&self) -> String {
        "Query Dataset.".to_owned()
    }

    fn params(&self) -> Vec<wander::WanderType> {
        vec![
            WanderType::String,
            WanderType::Optional(Box::new(WanderType::Identifier)),
            WanderType::Optional(Box::new(WanderType::Identifier)),
            WanderType::Optional(Box::new(WanderType::Value)),
        ]
    }

    fn returns(&self) -> wander::WanderType {
        WanderType::Nothing
    }

    fn name(&self) -> String {
        "Ligature.query".to_owned()
    }
}
