// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A simple HTTP server for Ligature using Axum.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::post,
    Router,
};
//use lig::load_lig_from_str;
use ligature::Ligature;
//use ligature_sqlite::LigatureSQLite;
use std::{net::SocketAddr, sync::Arc};
use wander::{prelude::common, run, WanderValue};

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    // let instance = Arc::new(LigatureSQLite::default());
    // let app = Router::new()
    //     // .route("/wander", post(handler))
    //     // .route("/lig/:dataset", post(lig_handler))
    //     .with_state(instance);
    // let addr = SocketAddr::from(([127, 0, 0, 1], 4200));
    // println!("Ligature HTTP is listening on {addr}");
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

// async fn lig_handler(
//     Path(dataset): Path<String>,
//     State(instance): State<Arc<LigatureSQLite>>,
//     lig: String,
// ) -> (StatusCode, String) {
//     let mut bindings = common();
//     instance.add_bindings(&mut bindings);
//     match Dataset::new(&dataset) {
//         Ok(dataset) => {
//             let ligature: &dyn Ligature = instance.as_ref();
//             match load_lig_from_str(dataset, &lig, ligature) {
//                 Ok(_) => (StatusCode::OK, WanderValue::Nothing.to_string()),
//                 Err(err) => (StatusCode::BAD_REQUEST, err.0),
//             }
//         }
//         Err(err) => (StatusCode::BAD_REQUEST, err.0),
//     }
// }

// async fn handler(
//     State(instance): State<Arc<LigatureSQLite>>,
//     query: String,
// ) -> (StatusCode, String) {
//     let mut bindings = common();
//     instance.add_bindings(&mut bindings);
//     match run(&query, &mut bindings) {
//         Ok(value) => (StatusCode::OK, value.to_string()),
//         Err(err) => (StatusCode::BAD_REQUEST, err.0),
//     }
// }
