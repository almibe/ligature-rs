// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A server for Ligature using ZeroMQ servers.

use axum::{http::StatusCode, routing::post, Json, Router};
use ligature::LigatureError;
use std::net::SocketAddr;
use wander::{bindings::BindingsProvider, preludes::common, run, ScriptValue};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/wander", post(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 4200));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(query: String) -> (StatusCode, Json<Result<ScriptValue, LigatureError>>) {
    println!("Received {}", query);
    let instance = ligature_sqlite::LigatureSQLite::new_memory_store().unwrap();
    let mut bindings = common();
    instance.add_bindings(&mut bindings);
    match run(&query, &mut bindings) {
        Ok(value) => (StatusCode::OK, Json(Ok(value))),
        Err(err) => (StatusCode::BAD_REQUEST, Json(Err(err))),
    }
}
