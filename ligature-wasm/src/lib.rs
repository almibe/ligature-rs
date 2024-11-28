// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This project exposes functionality from the Rust implementation of Wander to WASM and JS runtimes thanks to wasm-bindgen and wasm-pack.

mod utils;
use std::collections::{self, BTreeSet, HashMap};

use ligature::{Entry, Ligature};
use ligature_graph::LigatureGraph;
use serde::Serialize;
use wander::{WanderError, WanderValue};
use wasm_bindgen::prelude::*;

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphologyResult {
    nodes: BTreeSet<NodeResult>,
    edges: BTreeSet<EdgeResult>,
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeResult {
    key: String,
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeResult {
    key: String,
    source: String,
    target: String,
}

fn graph_to_graphology(graph: BTreeSet<Entry>) -> GraphologyResult {
    let mut nodes = BTreeSet::new();
    let mut edges = BTreeSet::new();
    for entry in graph {
        match entry {
            Entry::Extends { element, concept } => {
                nodes.insert(NodeResult {
                    key: element.clone().0,
                });
                nodes.insert(NodeResult {
                    key: concept.clone().0,
                });
                edges.insert(EdgeResult {
                    key: ":".to_owned(),
                    source: element.0,
                    target: concept.0,
                });
            }
            Entry::NotExtends { element, concept } => {
                nodes.insert(NodeResult {
                    key: element.clone().0,
                });
                nodes.insert(NodeResult {
                    key: concept.clone().0,
                });
                edges.insert(EdgeResult {
                    key: "¬:".to_owned(),
                    source: element.0,
                    target: concept.0,
                });
            }
            Entry::Role {
                first,
                second,
                role,
            } => {
                nodes.insert(NodeResult {
                    key: first.clone().0,
                });
                nodes.insert(NodeResult {
                    key: second.clone().0,
                });
                edges.insert(EdgeResult {
                    key: role.0,
                    source: first.0,
                    target: second.0,
                });
            }
        }
    }
    GraphologyResult {
        nodes: nodes,
        edges: edges,
    }
}

pub fn ligature_to_graphology(ligature: &dyn Ligature) -> JsValue {
    let mut res = HashMap::new();
    match ligature.collections() {
        Ok(collections) => {
            for collection in collections {
                match ligature.entries(&collection) {
                    Ok(entries) => {
                        res.insert(collection.0, graph_to_graphology(entries));
                    }
                    Err(_) => todo!(),
                }
            }
        }
        Err(err) => todo!(),
    }
    serde_wasm_bindgen::to_value(&res).unwrap()
}

#[wasm_bindgen]
pub fn run(script: String) -> JsValue {
    let mut state = LigatureGraph::new();
    let mut bindings = wander::prelude::common();
    match wander::run(&script, &bindings, &mut state) {
        Ok(_) => ligature_to_graphology(&state),
        Err(err) => serde_wasm_bindgen::to_value(&err).unwrap(),
    }
}
