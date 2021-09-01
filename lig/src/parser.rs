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
    save_point: Option<usize>,
}

impl Parser<'_> {
    /// Creates a new Parser with the given input.
    pub fn new(input: &str) -> Parser {
        Parser {
            input,
            location: 0,
            line: 0,
            location_in_line: 0,
            save_point: None,
        }
    }

    /// Read the current location.
    pub fn location(&self) -> usize {
        self.location
    }

    /// Read the current line.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Read the current location in the current line.
    pub fn location_in_line(&self) -> usize {
        self.location_in_line
    }

    /// Creates a save point within the parser for back tracking.
    pub fn save(&mut self) {
        self.save_point = Some(self.location);
    }

    /// Removes the current save point without reverting.
    pub fn clear_save(&mut self) {
        self.save_point = None
    }

    /// Back tracks to the last save point.
    pub fn rollback(&mut self) {
        match self.save_point {
            None => { /* do nothing */ }
            Some(loc) => {
                self.location = loc;
                self.save_point = None
            }
        }
    }

    /// Returns the next char, but doesn't affect the current Parser location.
    /// Returns None if there is no more text.
    pub fn peek(&self) -> Option<char> {
        if self.input.len() < self.location {
            None
        } else {
            let x = self.input.as_bytes()[self.location] as char; //TODO rewrite
            Some(x)
        }
    }

    /// Increases the current Parser location 1 space and returns the next char.
    /// Returns None if there is no more text.
    pub fn next(&mut self) -> Option<char> {
        if self.input.len() < self.location {
            None
        } else {
            let x = self.input.as_bytes()[self.location] as char; //TODO rewrite
            self.location = self.location + 1;
            Some(x)
        }
    }

    /// Attempts to match an entire string.
    /// Returns true if it succeeds and also bumps the location.
    /// Returns false if it fails for any reason and the location remains unchanged.
    pub fn take(&mut self, input: &str) -> bool {
        let start_pos = self.location;
        let mut offset = 0usize;
        let chars = input.as_bytes(); //TODO use unicode-segmentation for this eventually
        loop {
            if offset == chars.len() {
                return true;
            }
            let current = self.next();
            let test = chars[offset] as char;
            match current {
                None => {
                    self.location = start_pos;
                    return false;
                }
                Some(c) => {
                    if c != test {
                        self.location = start_pos;
                        return false;
                    }
                    offset += 1;
                }
            }
        }
    }

    /// Takes until a sentinel character is found.
    /// Returns the values that were found before final token.
    pub fn take_until(&mut self, sentinel: char) -> Option<String> {
        let mut res = String::new();
        let start_point = self.location;
        loop {
            let peek = self.peek();
            match peek {
                None => {
                    self.location = start_point;
                    return None;
                }
                Some(c) => {
                    if c == sentinel {
                        self.next();
                        return Some(res);
                    } else {
                        self.next();
                        res.push(c);
                        continue;
                    }
                }
            }
        }
    }

    /// Skips all of the chars passed and bumps the location accordingly.
    pub fn ignore_all(&mut self, chars: &HashSet<char>) {
        loop {
            let current = self.peek();
            match current {
                None => return,
                Some(c) => {
                    if chars.contains(&c) {
                        self.next();
                        continue;
                    } else {
                        return;
                    }
                }
            }
        }
    }

    /// Skips all white space.
    pub fn ignore_ws(&mut self) {
        let mut ws: HashSet<char> = HashSet::new(); //TODO make static
        ws.insert(' ');
        ws.insert('\t');
        ws.insert('\r');
        ws.insert('\n');

        self.ignore_all(&ws)
    }
}
