// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A simple HTTP server for Ligature using Axum.

use axum::{http::StatusCode, routing::post, Json, Router, extract::State};
use ligature::LigatureError;
use ligature_sqlite::LigatureSQLite;
use std::{net::SocketAddr, sync::Arc};
use wander::{bindings::BindingsProvider, preludes::common, run, ScriptValue};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let instance = Arc::new(LigatureSQLite::default());
    let app = Router::new()
        .route("/wander", post(handler))
        .with_state(instance);
    let addr = SocketAddr::from(([127, 0, 0, 1], 4200));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(State(instance): State<Arc<LigatureSQLite>>, query: String) -> (StatusCode, Json<Result<ScriptValue, LigatureError>>) {
    println!("Received {}", query);
    let mut bindings = common();
    instance.add_bindings(&mut bindings);
    match run(&query, &mut bindings) {
        Ok(value) => (StatusCode::OK, Json(Ok(value))),
        Err(err) => (StatusCode::BAD_REQUEST, Json(Err(err))),
    }
}
