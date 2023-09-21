// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the Ligature-SQLite project.
//! It implements the traits supplied by Ligature and persists data via SQLite3.

#![deny(missing_docs)]

use std::{
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
};

use dirs::data_local_dir;
use ligature::{Dataset, Identifier, Ligature, LigatureError, Statement, Value};
use rusqlite::{params, Connection, Error, Transaction};
use sql_builder::{quote, SqlBuilder};
use wander::{
    bindings::{Bindings, BindingsProvider},
    NativeFunction, WanderError, WanderType, WanderValue,
};

#[derive(Clone)]
/// The main struct used for working with the SQLite stored version of Ligature.
pub struct LigatureSQLite {
    connection: Arc<Mutex<Connection>>,
}

impl LigatureSQLite {
    /// Create a LigatureSQLite instance by opening the given file,
    /// or creating a new instance if that file doesn't exist.
    pub fn create_or_open_file(path: PathBuf) -> LigatureSQLite {
        let connection = Arc::new(Mutex::new(Connection::open(path).unwrap()));
        let instance = LigatureSQLite { connection };
        instance.setup().unwrap();
        instance
    }

    /// Create a new instance of LigatureSQLite that is stored in-memory only.
    pub fn new_memory_store() -> Result<LigatureSQLite, rusqlite::Error> {
        let instance = LigatureSQLite {
            connection: Arc::new(Mutex::new(Connection::open_in_memory()?)),
        };
        instance.setup()?;
        Ok(instance)
    }

    fn setup(&self) -> Result<(), rusqlite::Error> {
        self.connection.lock().unwrap().execute(
            r#"
            create table if not exists dataset(
                id integer primary key, 
                name text not null
            );
        "#,
            (),
        )?;
        self.connection.lock().unwrap().execute(
            r#"
            create table if not exists statement(
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

impl Default for LigatureSQLite {
    fn default() -> Self {
        let mut path = data_local_dir().unwrap();
        path.push("ligature");
        path.push("sqlite");
        LigatureSQLite::create_or_open_file(path)
    }
}

impl Ligature for LigatureSQLite {
    fn datasets(&self) -> Result<Vec<Dataset>, LigatureError> {
        let connection = self.connection.lock().unwrap();
        let mut stmt = connection.prepare("select name from dataset").unwrap();
        let iter = stmt
            .query_map([], |row| {
                let x: String = row.get(0)?;
                Ok(x)
            })
            .unwrap();
        let mut results: Vec<Dataset> = vec![];
        for name in iter {
            results.push(Dataset::new(&name.unwrap()).unwrap());
        }
        Ok(results)
    }

    fn add_dataset(&mut self, dataset: &ligature::Dataset) -> Result<(), LigatureError> {
        self.connection
            .lock()
            .unwrap()
            .execute("insert into dataset (name) values (?1)", [dataset.name()])
            .unwrap();
        Ok(())
    }

    fn remove_dataset(&mut self, dataset: &ligature::Dataset) -> Result<(), LigatureError> {
        self.connection
            .lock()
            .unwrap()
            .execute("delete from dataset where name = ?1", [dataset.name()])
            .unwrap();
        Ok(())
    }

    fn statements(
        &self,
        dataset: &ligature::Dataset,
    ) -> Result<Vec<ligature::Statement>, LigatureError> {
        let connection = self.connection.lock().unwrap();
        let mut stmt = connection.prepare(
            "select entity, attribute, value_identifier, value_int, value_string from statement inner join dataset on statement.dataset_id = dataset.id where dataset.name = ?1").unwrap();
        let iter = stmt
            .query_map([dataset.name()], |row| {
                let entity: String = row.get(0)?;
                let attribute: String = row.get(1)?;
                let value_id: Option<String> = row.get(2)?;
                let value_int: Option<i64> = row.get(3)?;
                let value_str: Option<String> = row.get(4)?;
                let value = if let Some(value) = value_id {
                    Value::Identifier(Identifier::new(&value).unwrap())
                } else if let Some(value) = value_int {
                    Value::Integer(value)
                } else if let Some(value) = value_str {
                    Value::String(value)
                } else {
                    todo!()
                };
                Ok(Statement {
                    entity: Identifier::new(&entity).unwrap(),
                    attribute: Identifier::new(&attribute).unwrap(),
                    value,
                })
            })
            .unwrap();
        let mut results: Vec<Statement> = vec![];
        for statement in iter {
            results.push(statement.unwrap());
        }
        Ok(results)
    }

    fn add_statements(
        &self,
        dataset: &ligature::Dataset,
        statements: Vec<ligature::Statement>,
    ) -> Result<(), LigatureError> {
        let mut connection = self.connection.lock().unwrap();
        let tx = connection.transaction().unwrap();
        let id = tx
            .query_row_and_then(
                "select id from dataset where name = ?1",
                [dataset.name()],
                |row| {
                    let id: i64 = row.get(0).unwrap();
                    Ok::<i64, Error>(id)
                },
            )
            .unwrap();
        statements.iter().for_each(|statement| {
                        let entity = statement.entity.id();
                        let attribute = statement.attribute.id();
                        match &statement.value {
                            Value::Integer(value) => {
                                tx.execute("insert into statement (dataset_id, entity, attribute, value_int) values (?1, ?2, ?3, ?4)", params![id, entity, attribute, value]).unwrap();
                            },
                            Value::String(value) => {
                                tx.execute("insert into statement (dataset_id, entity, attribute, value_string) values (?1, ?2, ?3, ?4)", params![id, entity, attribute, value]).unwrap();
                            },
                            Value::Identifier(value) => {
                                tx.execute("insert into statement (dataset_id, entity, attribute, value_identifier) values (?1, ?2, ?3, ?4)", params![id, entity, attribute, value.id()]).unwrap();
                            },
                            Value::Bytes(_) => todo!(),
                        }
                });
        tx.commit().unwrap();
        Ok(())
    }

    fn remove_statements(
        &self,
        dataset: &ligature::Dataset,
        statements: Vec<ligature::Statement>,
    ) -> Result<(), LigatureError> {
        let mut connection = self.connection.lock().unwrap();
        let tx = connection.transaction().unwrap();
        let id = tx
            .query_row_and_then(
                "select id from dataset where name = ?1",
                [dataset.name()],
                |row| {
                    let id: i64 = row.get(0).unwrap();
                    Ok::<i64, Error>(id)
                },
            )
            .unwrap();
        statements.iter().for_each(|statement| {
                        let entity = statement.entity.id();
                        let attribute = statement.attribute.id();
                        match &statement.value {
                            Value::Identifier(value) => {
                                tx.execute("delete from statement where dataset_id = ?1 and entity = ?2 and attribute = ?3 and value_identifier = ?4", params![id, entity, attribute, value.id()]).unwrap();
                            },
                            Value::String(value) => {
                                tx.execute("delete from statement where dataset_id = ?1 and entity = ?2 and attribute = ?3 and value_string = ?4", params![id, entity, attribute, value]).unwrap();
                            },
                            Value::Integer(value) => {
                                tx.execute("delete from statement where dataset_id = ?1 and entity = ?2 and attribute = ?3 and value_int = ?4", params![id, entity, attribute, value]).unwrap();
                            },
                            Value::Bytes(_) => todo!(),
                        };
        });
        tx.commit().unwrap();
        Ok(())
    }

    fn query(&self) -> Result<Box<dyn ligature::Query>, LigatureError> {
        todo!()
    }
}

impl BindingsProvider for LigatureSQLite {
    fn add_bindings(&self, bindings: &mut Bindings) {
        bindings.bind_native_function(Rc::new(DatasetsFunction {
            instance: Arc::new(Mutex::new(self.clone())),
        }));
        bindings.bind_native_function(Rc::new(AddDatasetFunction {
            instance: Arc::new(Mutex::new(self.clone())),
        }));
        bindings.bind_native_function(Rc::new(RemoveDatasetFunction {
            instance: Arc::new(Mutex::new(self.clone())),
        }));
        bindings.bind_native_function(Rc::new(StatementsFunction {
            instance: Arc::new(Mutex::new(self.clone())),
        }));
        bindings.bind_native_function(Rc::new(AddStatementsFunction {
            instance: Arc::new(Mutex::new(self.clone())),
        }));
        bindings.bind_native_function(Rc::new(RemoveStatementsFunction {
            instance: Arc::new(Mutex::new(self.clone())),
        }));
        // bindings.bind_native_function(
        //     "Ligature".to_owned(),
        //     String::from("query"),
        //     Rc::new(QueryFunction {
        //         instance: Arc::new(Mutex::new(self.clone())),
        //         connection: self.connection.clone(),
        //     }),
        // );
    }
}

struct DatasetsFunction {
    instance: Arc<Mutex<dyn Ligature>>,
}
impl NativeFunction for DatasetsFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        if arguments.is_empty() {
            let ds = self.instance.lock().unwrap().datasets().unwrap();
            let mut results = vec![];
            for name in ds {
                results.push(WanderValue::String(name.name().to_owned()));
            }
            Ok(WanderValue::List(results))
        } else {
            Err(WanderError(
                "`datasets` function take no arguments.".to_owned(),
            ))
        }
    }

    fn doc(&self) -> String {
        "Get a list of all Datasets.".to_owned()
    }

    fn params(&self) -> Vec<WanderType> {
        vec![]
    }

    fn returns(&self) -> WanderType {
        WanderType::List
    }

    fn name(&self) -> String {
        "Ligature.datasets".to_owned()
    }
}

struct AddDatasetFunction {
    instance: Arc<Mutex<LigatureSQLite>>,
}
impl NativeFunction for AddDatasetFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name)] => {
                self.instance
                    .lock()
                    .unwrap()
                    .add_dataset(&Dataset::new(name).unwrap())
                    .map_err(|e| WanderError(e.0))?;
                Ok(WanderValue::Nothing)
            }
            _ => todo!(),
        }
    }

    fn doc(&self) -> String {
        "Add a new Dataset.".to_owned()
    }

    fn params(&self) -> Vec<WanderType> {
        vec![WanderType::String]
    }

    fn returns(&self) -> WanderType {
        WanderType::Nothing
    }

    fn name(&self) -> String {
        "Ligature.addDataset".to_owned()
    }
}

struct RemoveDatasetFunction {
    instance: Arc<Mutex<LigatureSQLite>>,
}
impl NativeFunction for RemoveDatasetFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name)] => {
                self.instance
                    .lock()
                    .unwrap()
                    .remove_dataset(&Dataset::new(name).unwrap())
                    .unwrap();
                Ok(WanderValue::Nothing)
            }
            _ => todo!(),
        }
    }

    fn doc(&self) -> String {
        "Remove a Dataset.".to_owned()
    }

    fn params(&self) -> Vec<WanderType> {
        vec![WanderType::String]
    }

    fn returns(&self) -> WanderType {
        WanderType::Nothing
    }

    fn name(&self) -> String {
        "Ligature.removeDataset".to_owned()
    }
}

struct StatementsFunction {
    instance: Arc<Mutex<LigatureSQLite>>,
}
impl NativeFunction for StatementsFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name)] => {
                let statements = self
                    .instance
                    .lock()
                    .unwrap()
                    .statements(&Dataset::new(name).unwrap())
                    .unwrap();
                let mut results: Vec<WanderValue> = vec![];
                for statement in statements {
                    let mut result = vec![];
                    result.push(WanderValue::Identifier(statement.entity));
                    result.push(WanderValue::Identifier(statement.attribute));
                    let value = match statement.value {
                        Value::Identifier(value) => WanderValue::Identifier(value),
                        Value::String(value) => WanderValue::String(value),
                        Value::Integer(value) => WanderValue::Int(value),
                        Value::Bytes(_value) => todo!(),
                    };
                    result.push(value);
                    results.push(WanderValue::List(result));
                }
                Ok(WanderValue::List(results))
            }
            _ => panic!("Should not reach."),
        }
    }

    fn doc(&self) -> String {
        "Get all of the Statements in a Dataset.".to_owned()
    }

    fn params(&self) -> Vec<WanderType> {
        vec![WanderType::String]
    }

    fn returns(&self) -> WanderType {
        WanderType::List
    }

    fn name(&self) -> String {
        "Ligature.statements".to_owned()
    }
}

fn wander_value_to_statement(values: &Vec<WanderValue>) -> Result<Vec<Statement>, WanderError> {
    let mut results = vec![];
    for value in values {
        match value {
            WanderValue::List(contents) => match &contents[..] {
                [WanderValue::Identifier(entity), WanderValue::Identifier(attribute), value] => {
                    let value = match value {
                        WanderValue::Int(value) => Value::Integer(*value),
                        WanderValue::String(value) => Value::String(value.to_string()),
                        WanderValue::Identifier(value) => Value::Identifier(value.clone()),
                        _ => todo!(),
                    };
                    let statement = Statement {
                        entity: entity.clone(),
                        attribute: attribute.clone(),
                        value,
                    };
                    results.push(statement);
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
    Ok(results)
}

struct AddStatementsFunction {
    instance: Arc<Mutex<LigatureSQLite>>,
}
impl NativeFunction for AddStatementsFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name), WanderValue::List(statements)] => {
                let dataset = Dataset::new(name).unwrap();
                let statements = wander_value_to_statement(statements)?;
                self.instance
                    .lock()
                    .unwrap()
                    .add_statements(&dataset, statements)
                    .map_err(|e| WanderError(e.0))?;
                Ok(WanderValue::Nothing)
            }
            _ => todo!(),
        }
    }

    fn doc(&self) -> String {
        "Add Statements to Dataset.".to_owned()
    }

    fn params(&self) -> Vec<WanderType> {
        vec![WanderType::String, WanderType::List]
    }

    fn returns(&self) -> WanderType {
        WanderType::Nothing
    }

    fn name(&self) -> String {
        "Ligature.addStatements".to_owned()
    }
}

struct RemoveStatementsFunction {
    instance: Arc<Mutex<dyn Ligature>>,
}
impl NativeFunction for RemoveStatementsFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(name), WanderValue::List(statements)] => {
                let dataset = Dataset::new(name).map_err(|e| WanderError(e.0))?;
                let statements = wander_value_to_statement(statements)?;
                self.instance
                    .lock()
                    .unwrap()
                    .remove_statements(&dataset, statements)
                    .map_err(|e| WanderError(e.0))?;
                Ok(WanderValue::Nothing)
            }
            _ => todo!(),
        }
    }

    fn doc(&self) -> String {
        "Remove Statements from the given Dataset.".to_owned()
    }

    fn params(&self) -> Vec<WanderType> {
        vec![WanderType::String, WanderType::List]
    }

    fn returns(&self) -> WanderType {
        WanderType::Nothing
    }

    fn name(&self) -> String {
        "Ligature.removeStatements".to_owned()
    }
}

fn fetch_dataset_id(dataset_name: &str, tx: &Transaction) -> Result<Option<u64>, Error> {
    let x = tx.query_row_and_then(
        "select id from dataset where name = ?1",
        [dataset_name],
        |row| {
            let id: u64 = row.get(0)?;
            Ok::<u64, Error>(id)
        },
    );
    match x {
        Ok(dataset_id) => Ok(Some(dataset_id)),
        Err(_) => Ok(None), //TODO just returning None for now, eventually I should match on the error (some errors should return Err others None)
    }
}

struct QueryFunction {
    _instance: Arc<Mutex<LigatureSQLite>>,
    connection: Arc<Mutex<Connection>>,
}
impl NativeFunction for QueryFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        _bindings: &Bindings,
    ) -> Result<WanderValue, WanderError> {
        match arguments {
            [WanderValue::String(dataset), entity, attribute, value] => {
                let mut connection = self.connection.lock().unwrap();
                let tx = connection.transaction().unwrap();
                let dataset_id = fetch_dataset_id(dataset, &tx).unwrap().unwrap();

                let mut builder = SqlBuilder::select_from("statement");
                builder
                    .field("entity")
                    .field("attribute")
                    .field("value_identifier")
                    .field("value_int")
                    .field("value_string")
                    .and_where_eq("dataset_id", dataset_id);

                if let WanderValue::Identifier(entity) = entity {
                    builder.and_where_eq("entity", &quote(entity.id()));
                } else if let WanderValue::Nothing = entity {
                    //do nothing
                } else {
                    return Err(WanderError(
                        "Invalid argument in Entity position in call to `query`.".to_owned(),
                    ));
                }

                if let WanderValue::Identifier(attribute) = attribute {
                    builder.and_where_eq("attribute", &quote(attribute.id()));
                } else if let WanderValue::Nothing = attribute {
                    //do nothing
                } else {
                    return Err(WanderError(
                        "Invalid argument in Attribute position in call to `query`.".to_owned(),
                    ));
                }

                match value {
                    WanderValue::Int(value) => {
                        builder.and_where_eq("value_int", value);
                    }
                    WanderValue::String(value) => {
                        builder.and_where_eq("value_string", &quote(value));
                    }
                    WanderValue::Identifier(value) => {
                        builder.and_where_eq("value_identifier", &quote(value.id()));
                    }
                    WanderValue::Nothing => (), //do nothing
                    _ => {
                        return Err(WanderError(
                            "Invalid argument in Value position in call to `query`.".to_owned(),
                        ))
                    }
                }
                let stmt = builder.sql().unwrap();
                let mut stmt = tx.prepare(&stmt).unwrap();
                let x = stmt
                    .query_map([], |e| {
                        let entity: String = e.get(0).unwrap();
                        let attribute: String = e.get(1).unwrap();
                        let value_identifier: Option<String> = e.get(2).unwrap();
                        let value_int: Option<i64> = e.get(3).unwrap();
                        let value_string: Option<String> = e.get(4).unwrap();

                        let entity = WanderValue::Identifier(Identifier::new(&entity).unwrap());
                        let attribute =
                            WanderValue::Identifier(Identifier::new(&attribute).unwrap());
                        let value = if let Some(value) = value_identifier {
                            WanderValue::Identifier(Identifier::new(&value).unwrap())
                        } else if let Some(value) = value_int {
                            WanderValue::Int(value)
                        } else if let Some(value) = value_string {
                            WanderValue::String(value)
                        } else {
                            todo!("err")
                            //return Err(WanderError("Invalid argument in Value position in call to `query`.".to_owned()))
                        };
                        Ok(WanderValue::List(vec![entity, attribute, value]))
                    })
                    .unwrap();
                let mut results = vec![];
                for y in x {
                    results.push(y.unwrap());
                }
                Ok(WanderValue::List(results))
            }
            _ => Err(WanderError("Incorrect arguments.".to_owned())),
        }
    }

    fn doc(&self) -> String {
        "Run a query against a Dataset.".to_owned()
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
        WanderType::List
    }

    fn name(&self) -> String {
        todo!()
    }
}
