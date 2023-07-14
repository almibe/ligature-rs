// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the Ligature-SQLite project.
//! It implements the traits supplied by Ligature and persists data via SQLite3.

#![deny(missing_docs)]

use std::{cell::RefCell, path::PathBuf, rc::Rc};

use ligature::LigatureError;
use rusqlite::{Connection, Error, params};
use wander::{bindings::BindingsProvider, NativeFunction, WanderValue};

/// The main struct used for working with the SQLite stored version of Ligature.
pub struct LigatureSQLite {
    connection: Rc<RefCell<Connection>>,
}

impl LigatureSQLite {
    /// Create a LigatureSQLite instance by opening the given file,
    /// or creating a new instance if that file doesn't exist.
    pub fn create_or_open_file(path: PathBuf) -> LigatureSQLite {
        todo!()
    }

    /// Create a new instance of LigatureSQLite that is stored in-memory only.
    pub fn new_memory_store() -> Result<LigatureSQLite, rusqlite::Error> {
        let instance = LigatureSQLite {
            connection: Rc::new(RefCell::new(Connection::open_in_memory()?)),
        };
        instance.setup()?;
        Ok(instance)
    }

    fn setup(&self) -> Result<(), rusqlite::Error> {
        self.connection.borrow().execute(
            r#"
            create table dataset(
                id integer primary key, 
                name text not null
            );
        "#,
            (),
        )?;
        self.connection.borrow().execute(
            r#"
            create table statement(
                id integer primary key, 
                dataset_id integer not null, 
                entity text not null,
                attribute text not null,
                value_int integer,
                value_string text,
                value_identifier text
            );
        "#,
            (),
        )?;
        Ok(())
    }
}

impl BindingsProvider for LigatureSQLite {
    fn add_bindings(&self, bindings: &mut wander::bindings::Bindings) {
        bindings.bind_native_function(
            String::from("datasets"),
            Rc::new(DatasetsFunction {
                connection: self.connection.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("addDataset"),
            Rc::new(AddDatasetFunction {
                connection: self.connection.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("removeDataset"),
            Rc::new(RemoveDatasetFunction {
                connection: self.connection.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("statements"),
            Rc::new(StatementsFunction {
                connection: self.connection.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("addStatements"),
            Rc::new(AddStatementsFunction {
                connection: self.connection.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("removeStatements"),
            Rc::new(RemoveStatementsFunction {
                connection: self.connection.clone(),
            }),
        );
        bindings.bind_native_function(
            String::from("match"),
            Rc::new(MatchFunction {
                connection: self.connection.clone(),
            }),
        );
    }
}

struct DatasetsFunction {
    connection: Rc<RefCell<Connection>>,
}
impl NativeFunction for DatasetsFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, LigatureError> {
        if arguments.is_empty() {
            let connection = self.connection.borrow();
            let mut stmt = connection.prepare("select name from dataset").unwrap();
            let iter = stmt
                .query_map([], |row| {
                    let x: String = row.get(0)?;
                    Ok(x)
                })
                .unwrap();
            let mut results: Vec<WanderValue> = vec![];
            for name in iter {
                results.push(WanderValue::String(name.unwrap()));
            }
            Ok(WanderValue::List(results))
        } else {
            Err(LigatureError(
                "`datasets` function take no arguments.".to_owned(),
            ))
        }
    }
}

struct AddDatasetFunction {
    connection: Rc<RefCell<Connection>>,
}
impl NativeFunction for AddDatasetFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::String(name)] => {
                self.connection
                    .borrow()
                    .execute("insert into dataset (name) values (?1)", [name])
                    .unwrap();
                Ok(WanderValue::Nothing)
            }
            _ => todo!(),
        }
    }
}

struct RemoveDatasetFunction {
    connection: Rc<RefCell<Connection>>,
}
impl NativeFunction for RemoveDatasetFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::String(name)] => {
                self.connection
                    .borrow()
                    .execute("delete from dataset where name =?1", [name])
                    .unwrap();
                Ok(WanderValue::Nothing)
            }
            _ => todo!(),
        }
    }
}

struct StatementsFunction {
    connection: Rc<RefCell<Connection>>,
}
impl NativeFunction for StatementsFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::String(name)] => {
                let connection = self.connection.borrow();
                let mut stmt = connection.prepare(
                    "select entity, attribute, value_identifier from statement inner join dataset on statement.dataset_id = dataset.id").unwrap();
                let iter = stmt
                    .query_map([], |row| {
                        let entity: String = row.get(0)?;
                        let attribute = row.get(1)?;
                        let value = row.get(2)?;
                        Ok(WanderValue::List(vec![
                            WanderValue::String(entity),
                            WanderValue::String(attribute),
                            WanderValue::String(value),
                        ]))
                    })
                    .unwrap();
                let mut results: Vec<WanderValue> = vec![];
                for statement in iter {
                    results.push(statement.unwrap());
                }
                Ok(WanderValue::List(results))
            }
            _ => todo!(),
        }
    }
}

struct AddStatementsFunction {
    connection: Rc<RefCell<Connection>>,
}
impl NativeFunction for AddStatementsFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::String(name), WanderValue::List(statements)] => {
                let mut connection = self.connection.borrow_mut();
                let mut tx = connection.transaction().unwrap();
                let id = tx
                    .query_row_and_then("select id from dataset where name = ?1", [name], |row| {
                        let id: i64 = row.get(0).unwrap();
                        Ok::<i64, Error>(id)
                    })
                    .unwrap();
                statements.iter().for_each(|statement| {
                    if let WanderValue::List(contents) = statement {
                        match &contents[..] {
                            [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), x] => {
                                let entity = entity.id();
                                let attribute = attribute.id();
                                let value = if let WanderValue::Identifier(value) = x {
                                    value.id()
                                } else {
                                    todo!();
                                };
                                tx.execute("insert into statement (dataset_id, entity, attribute, value_identifier) values (?1, ?2, ?3, ?4)", params![id, entity, attribute, value]).unwrap();
                            },
                            _ => todo!()
                        }
                    } else {
                        todo!()
                    }
                });
                tx.commit().unwrap();
                Ok(WanderValue::Nothing)
            }
            _ => todo!(),
        }
    }
}

struct RemoveStatementsFunction {
    connection: Rc<RefCell<Connection>>,
}
impl NativeFunction for RemoveStatementsFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, LigatureError> {
        match &arguments[..] {
            [WanderValue::String(name), WanderValue::List(statements)] => {
                let mut connection = self.connection.borrow_mut();
                let mut tx = connection.transaction().unwrap();
                let id = tx
                    .query_row_and_then("select id from dataset where name = ?1", [name], |row| {
                        let id: i64 = row.get(0).unwrap();
                        Ok::<i64, Error>(id)
                    })
                    .unwrap();
                statements.iter().for_each(|statement| {
                    if let WanderValue::List(contents) = statement {
                        match &contents[..] {
                            [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), x] => {
                                let entity = entity.id();
                                let attribute = attribute.id();
                                let value = if let WanderValue::Identifier(value) = x {
                                    value.id()
                                } else {
                                    todo!();
                                };
                                tx.execute("delete from statement where dataset_id = ?1 and entity = ?2 and attribute = ?3 and value_identifier = ?4", params![id, entity, attribute, value]).unwrap();
                            },
                            _ => todo!()
                        }
                    } else {
                        todo!()
                    }
                });
                tx.commit().unwrap();
                Ok(WanderValue::Nothing)
            }
            _ => todo!(),
        }
    }
}

struct MatchFunction {
    connection: Rc<RefCell<Connection>>,
}
impl NativeFunction for MatchFunction {
    fn run(
        &self,
        arguments: &Vec<wander::WanderValue>,
    ) -> Result<wander::WanderValue, LigatureError> {
        todo!()
    }
}
