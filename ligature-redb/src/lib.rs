// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of Ligature that uses
//! [redb](https://redb.org) for storing data.

use home::home_dir;

use rand::distributions::{Alphanumeric, DistString};
use redb::{Database, Error, ReadableTable, StorageError, Table, TableError, TransactionError};
use std::{path::PathBuf, rc::Rc};
use tables::IdTypes;
use wander::{
    HostFunction, WanderValue, WanderError,
};

mod tables {
    use redb::TableDefinition;

    pub const IDS_TABLE: TableDefinition<&str, u64> = TableDefinition::new("IDS");

    pub const DATASETS_ID_TABLE: TableDefinition<&str, u64> = TableDefinition::new("DATASETS_ID");
    pub const ID_DATASETS_TABLE: TableDefinition<u64, &str> = TableDefinition::new("ID_DATASETS");

    pub const IDENTIFIER_ID_TABLE: TableDefinition<&str, u64> =
        TableDefinition::new("IDENTIFIER_ID");
    pub const ID_IDENTIFIER_TABLE: TableDefinition<&str, u64> =
        TableDefinition::new("ID_IDENTIFIER");

    pub const EAV_TABLE: TableDefinition<&[u8], ()> = TableDefinition::new("EAV");
    // pub const EVA_TABLE: TableDefinition<&[u8], ()> = TableDefinition::new("EVA");
    // pub const AEV_TABLE: TableDefinition<&[u8], ()> = TableDefinition::new("AEV");
    // pub const AVE_TABLE: TableDefinition<&[u8], ()> = TableDefinition::new("AVE");
    // pub const VEA_TABLE: TableDefinition<&[u8], ()> = TableDefinition::new("VEA");
    // pub const VAE_TABLE: TableDefinition<&[u8], ()> = TableDefinition::new("VAE");

    pub enum IdTypes {
        Datasets,
        Identifiers,
        Strings,
    }
}

pub struct LigatureRedb {
    db: Rc<Database>,
    config: Config,
}

//#[derive(Clone)]
pub struct Config {
    pub location: PathBuf,
}

impl Default for LigatureRedb {
    fn default() -> Self {
        match home_dir() {
            Some(mut path) => {
                path.push(".ligature");
                path.push("redb");
                std::fs::create_dir_all(&path).unwrap();
                path.push("ligature.redb");
                match Self::create(Config { location: path }) {
                    Ok(inst) => inst,
                    Err(err) => panic!(
                        "Could not create LigatureRedb instance in default location.\n{}",
                        err.to_string()
                    ),
                }
            }
            None => panic!("Could not create LigatureRedb instance in default location."),
        }
    }
}

impl LigatureRedb {
    pub fn temp() -> Result<Self, Error> {
        let suffix = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

        match home_dir() {
            Some(mut path) => {
                path.push(".ligature");
                path.push("redb");
                std::fs::create_dir_all(&path).unwrap();
                path.push(format!("ligature-{suffix}.redb"));
                match Self::create(Config { location: path }) {
                    Ok(inst) => Ok(inst),
                    Err(_) => panic!("Could not create LigatureRedb instance in default location."),
                }
            }
            None => panic!("Could not create LigatureRedb instance in default location."),
        }
    }

    pub fn create(config: Config) -> Result<Self, Error> {
        let db = Database::create(&config.location)?;
        {
            let tx = db.begin_write()?;
            tx.open_table(tables::IDS_TABLE)?;
            tx.open_table(tables::DATASETS_ID_TABLE)?;
            tx.open_table(tables::ID_DATASETS_TABLE)?;
            tx.open_table(tables::IDENTIFIER_ID_TABLE)?;
            tx.open_table(tables::ID_IDENTIFIER_TABLE)?;

            tx.open_table(tables::EAV_TABLE)?;
            // tx.open_table(tables::EVA_TABLE)?;
            // tx.open_table(tables::AEV_TABLE)?;
            // tx.open_table(tables::AVE_TABLE)?;
            // tx.open_table(tables::VEA_TABLE)?;
            // tx.open_table(tables::VAE_TABLE)?;

            tx.commit()?;
        }
        let instance = Self {
            config,
            db: Rc::new(db),
        };
        Ok(instance)
    }
}

// impl BindingsProvider for LigatureRedb {
//     fn add_bindings(&self, bindings: &mut Bindings) {
//         bindings.bind_native_function(
//             "Ligature".to_owned(),
//             String::from("datasets"),
//             Rc::new(DatasetsFunction {
//                 db: self.db.clone(),
//             }),
//         );
//         bindings.bind_native_function(
//             "Ligature".to_owned(),
//             String::from("addDataset"),
//             Rc::new(AddDatasetFunction {
//                 db: self.db.clone(),
//             }),
//         );
//         bindings.bind_native_function(
//             "Ligature".to_owned(),
//             String::from("removeDataset"),
//             Rc::new(RemoveDatasetFunction {
//                 db: self.db.clone(),
//             }),
//         );
//         bindings.bind_native_function(
//             "Ligature".to_owned(),
//             String::from("statements"),
//             Rc::new(StatementsFunction {
//                 db: self.db.clone(),
//             }),
//         );
//     }
// }

fn id_type_str(id_type: IdTypes) -> &'static str {
    match id_type {
        IdTypes::Datasets => "Datasets",
        IdTypes::Identifiers => "Identifiers",
        IdTypes::Strings => "Strings",
    }
}

fn next_id(id_type: IdTypes, table: &mut Table<'_, '_, &str, u64>) -> Result<u64, Error> {
    let key = id_type_str(id_type);
    let value = {
        match table.get(key)? {
            None => None,
            Some(v) => Some(v.value()),
        }
    };
    match value {
        Some(value) => {
            let id = value + 1;
            table.insert(key, id)?;
            Ok(id)
        }
        None => {
            table.insert(key, 0)?;
            Ok(0)
        }
    }
}

fn tx_err(err: TransactionError) -> WanderError {
    WanderError(format!("Redb Error - {}", err.to_string()))
}

fn tbl_err(err: TableError) -> WanderError {
    WanderError(format!("Redb Error - {}", err.to_string()))
}

fn stor_err(err: StorageError) -> WanderError {
    WanderError(format!("Redb Error - {}", err.to_string()))
}

// struct DatasetsFunction {
//     db: Rc<Database>,
// }
// impl NativeFunction for DatasetsFunction {
//     fn run(&self, arguments: &[WanderValue], bindings: &Bindings) -> Result<WanderValue, WanderError> {
//         if arguments.is_empty() {
//             let mut datasets = vec![];
//             {
//                 let tx = self.db.begin_read().map_err(&tx_err)?;
//                 let t = tx.open_table(tables::DATASETS_ID_TABLE).map_err(&tbl_err)?;
//                 let itr = t.iter().map_err(&stor_err)?;
//                 for i in itr {
//                     let ag = i.map_err(stor_err)?;
//                     let name = ag.0.value();
//                     datasets.push(WanderValue::String(name.to_owned()));
//                 }
//             }
//             Ok(WanderValue::List(datasets))
//         } else {
//             Err(WanderError(
//                 "`datasets` function have no arguments.".to_owned(),
//             ))
//         }
//     }

//     // fn doc(&self) -> String {
//     //     todo!()
//     // }

//     // fn params(&self) -> Vec<WanderType> {
//     //     todo!()
//     // }

//     // fn returns(&self) -> WanderType {
//     //     todo!()
//     // }
// }

// struct AddDatasetFunction {
//     db: Rc<Database>,
// }
// impl NativeFunction for AddDatasetFunction {
//     fn run(&self, arguments: &[WanderValue], bindings: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::String(name)] => {
//                 let tx = self.db.begin_write().map_err(tx_err)?;
//                 let mut ids = tx.open_table(tables::IDS_TABLE).map_err(tbl_err)?;
//                 let mut datasets = tx.open_table(tables::DATASETS_ID_TABLE).map_err(tbl_err)?;
//                 let mut ids_datasets = tx.open_table(tables::ID_DATASETS_TABLE).map_err(tbl_err)?;
//                 let exists = match datasets.get(name.as_str()) {
//                     Ok(res) => res.is_some(),
//                     Err(err) => todo!(),
//                 };
//                 if exists {
//                     Ok(WanderValue::Nothing)
//                 } else {
//                     let new_id = next_id(IdTypes::Datasets, &mut ids).unwrap();
//                     datasets.insert(name.as_str(), new_id).unwrap();
//                     ids_datasets.insert(new_id, name.as_str()).unwrap();
//                     drop(ids);
//                     drop(datasets);
//                     drop(ids_datasets);
//                     tx.commit().unwrap();
//                     Ok(WanderValue::Nothing)
//                 }
//             }
//             _ => Err(WanderError(
//                 "`addDatasets` requires a single string argument.".to_owned(),
//             )),
//         }
//     }

//     // fn doc(&self) -> String {
//     //     "Add Dataset.".to_owned()
//     // }

//     // fn params(&self) -> Vec<wander::WanderType> {
//     //     vec![WanderType::String]
//     // }

//     // fn returns(&self) -> wander::WanderType {
//     //     WanderType::Nothing
//     // }
// }

// struct RemoveDatasetFunction {
//     db: Rc<Database>,
// }
// impl NativeFunction for RemoveDatasetFunction {
//     fn run(&self, arguments: &[WanderValue], bindings: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::String(name)] => {
//                 let tx = self.db.begin_write().map_err(tx_err)?;
//                 let mut ids = tx.open_table(tables::IDS_TABLE).map_err(tbl_err)?;
//                 let mut datasets = tx.open_table(tables::DATASETS_ID_TABLE).map_err(tbl_err)?;
//                 let mut ids_datasets = tx.open_table(tables::ID_DATASETS_TABLE).map_err(tbl_err)?;
//                 let id = match datasets.get(name.as_str()) {
//                     Ok(Some(res)) => Some(res.value()),
//                     Ok(None) => None,
//                     Err(err) => todo!(),
//                 };
//                 match id {
//                     Some(id) => {
//                         datasets.remove(name.as_str()).unwrap();
//                         ids_datasets.remove(id).unwrap();
//                         drop(ids);
//                         drop(datasets);
//                         drop(ids_datasets);
//                         tx.commit().unwrap();
//                         Ok(WanderValue::Nothing)
//                     }
//                     None => Ok(WanderValue::Nothing), //doesn't exist
//                 }
//             }
//             _ => Err(WanderError(
//                 "`addDatasets` requires a single string argument.".to_owned(),
//             )),
//         }
//     }

//     // fn doc(&self) -> String {
//     //     "Remove a Dataset.".to_owned()
//     // }

//     // fn params(&self) -> Vec<WanderType> {
//     //     vec![WanderType::String]
//     // }

//     // fn returns(&self) -> WanderType {
//     //     WanderType::Nothing
//     // }
// }

// struct StatementsFunction {
//     db: Rc<Database>,
// }
// impl NativeFunction for StatementsFunction {
//     fn run(&self, arguments: &[WanderValue], bindings: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::String(name)] => {
//                 let tx = self.db.begin_write().map_err(tx_err)?;
//                 let mut datasets = tx.open_table(tables::DATASETS_ID_TABLE).map_err(tbl_err)?;
//                 let mut eav = tx.open_table(tables::EAV_TABLE).map_err(tbl_err)?;
//                 let exists = match datasets.get(name.as_str()) {
//                     Ok(Some(res)) => Some(res.value()),
//                     Ok(None) => None,
//                     Err(err) => todo!(),
//                 };
//                 if let Some(id) = exists {
//                     //look up entries for the given dataset in EAV table
//                     //let range = eav.range(3...&4).unwrap();
//                     //let mut result = vec![];
//                     // range.for_each(|x| {
//                     //     match x {
//                     //         Ok((x, y)) => {
//                     //             let y = x.value().to_owned();
//                     //             result.push(y);
//                     //         },
//                     //         Err(_) => todo!(),
//                     //     };
//                     // });
//                     todo!()
//                 } else {
//                     Err(WanderError(format!("Dataset `{name}` doesn't exist.")))
//                 }
//             }
//             _ => Err(WanderError(
//                 "`addDatasets` requires a single string argument.".to_owned(),
//             )),
//         }
//     }

//     fn doc(&self) -> String {
//         "".to_owned()
//     }

//     fn params(&self) -> Vec<WanderType> {
//         todo!()
//     }

//     fn returns(&self) -> WanderType {
//         todo!()
//     }
// }

// struct AddStatementsFunction {
//     db: Rc<Database>,
// }
// impl NativeFunction for AddStatementsFunction {
//     fn run(&self, arguments: &[WanderValue], bindings: &Bindings) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::String(name)] => {
//                 let tx = self.db.begin_write().map_err(tx_err)?;

//                 let mut datasets = tx.open_table(tables::DATASETS_ID_TABLE).map_err(tbl_err)?;
//                 let mut eav = tx.open_table(tables::EAV_TABLE).map_err(tbl_err)?;
//                 // let mut eva = tx.open_table(tables::EVA_TABLE).map_err(tbl_err)?;
//                 // let mut aev = tx.open_table(tables::AEV_TABLE).map_err(tbl_err)?;
//                 // let mut ave = tx.open_table(tables::AVE_TABLE).map_err(tbl_err)?;
//                 // let mut vea = tx.open_table(tables::VEA_TABLE).map_err(tbl_err)?;
//                 // let mut vae = tx.open_table(tables::VAE_TABLE).map_err(tbl_err)?;

//                 let exists = match datasets.get(name.as_str()) {
//                     Ok(Some(res)) => Some(res.value()),
//                     Ok(None) => None,
//                     Err(err) => todo!(),
//                 };
//                 if let Some(id) = exists {
//                     //
//                     //let x: Range<&[u8], ()> = &[]..&[];
//                     //look up entries for the given dataset in EAV table
//                     //eav.range(&[]..&[]).unwrap();
//                     todo!()
//                 } else {
//                     Err(WanderError(format!("Dataset `{name}` doesn't exist.")))
//                 }
//             }
//             _ => Err(WanderError(
//                 "`addDatasets` requires a single string argument.".to_owned(),
//             )),
//         }
//     }

//     fn doc(&self) -> String {
//         todo!()
//     }

//     fn params(&self) -> Vec<WanderType> {
//         todo!()
//     }

//     fn returns(&self) -> WanderType {
//         todo!()
//     }
// }
