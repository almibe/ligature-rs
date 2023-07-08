// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A client for Ligature ZeroMQ servers.

use std::fmt::Error;

use ligature::LigatureError;
use wander::WanderValue;
use zmq::{Context, Socket};

pub struct LigatureZeroMQClient {
    ctx: Context,
    socket: Socket,
}

impl LigatureZeroMQClient {
    pub fn create(port: u32) -> Result<Self, Error> {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::REQ).unwrap();
        socket
            .connect(format!("tcp://127.0.0.1:{}", port.to_string()).as_str())
            .unwrap();
        Ok(LigatureZeroMQClient { ctx, socket })
    }

    //TODO this should eventually return WanderValue not String
    pub fn run_wander(&self, input: &str) -> Result<String, LigatureError> {
        let send_res = self.socket.send(input, 0).unwrap();
        let message = self.socket.recv_string(0).unwrap().unwrap();
        Ok(message)
    }
}
