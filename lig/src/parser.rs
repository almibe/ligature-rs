// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module contains a parsing utility used for parsing Lig.

use std::collections::HashSet;

pub struct Parser<'a> {
    input: &'a str,
    location: usize,
    line: usize,
    location_in_line: usize,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        Parser {
            input,
            location: 0,
            line: 0,
            location_in_line: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.input.len() < self.location {
            None
        } else {
            let x = self.input.as_bytes()[self.location] as char; //TODO rewrite
            Some(x)
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.input.len() < self.location {
            None
        } else {
            let x = self.input.as_bytes()[self.location] as char; //TODO rewrite
            self.location = self.location + 1;
            Some(x)
        }
    }

    pub fn take(&mut self, input: &str) -> Option<&str> {
        todo!()
    }

    /// Skips all of the chars passed.
    pub fn ignore_all(&mut self, chars: HashSet<char>) {
        todo!()
    }
}
