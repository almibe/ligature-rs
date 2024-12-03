// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A server for Ligature using ZeroMQ servers.

use std::fmt::{Debug, Display};

use wander::run;
use zmq::{Context, Message, SocketType::REP};

fn main() {
    let ctx = Context::new();
    let responder = ctx.socket(REP).unwrap();
    responder.bind("tcp://127.0.0.1:4200").unwrap();
    let mut msg = Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        let query = msg.as_str().unwrap();
        println!("Received {}", query);
        match run(query, &wander::prelude::common(), &mut ligature_graph::LigatureGraph::new()) {
            Ok(res) => responder.send(&res.to_string(), 0).unwrap(),
            Err(_) => todo!()
        }
    }
}
