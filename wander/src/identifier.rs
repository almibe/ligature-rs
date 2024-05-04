// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Contains the defition of Identifiers.
//! This code is mostly taken from Ligature,
//! but I don't want to add a dependency on Ligature at the moment or vice-versa.

use crate::WanderError;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Check if a given identifier is valid.
pub fn validate_identifier(id: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9-._~:/?#\[\]@!$&'()*+,;%=\x{00A0}-\x{D7FF}\x{F900}-\x{FDCF}\x{FDF0}-\x{FFEF}\x{10000}-\x{1FFFD}\x{20000}-\x{2FFFD}\x{30000}-\x{3FFFD}\x{40000}-\x{4FFFD}\x{50000}-\x{5FFFD}\x{60000}-\x{6FFFD}\x{70000}-\x{7FFFD}\x{80000}-\x{8FFFD}\x{90000}-\x{9FFFD}\x{A0000}-\x{AFFFD}\x{B0000}-\x{BFFFD}\x{C0000}-\x{CFFFD}\x{D0000}-\x{DFFFD}\x{E1000}-\x{EFFFD}]+$").unwrap();
    }
    RE.is_match(id)
}

/// Check if a given str only contains valid characters.
/// This is a duplicate of valid_identifier for now but the two could deviate potentially.
pub fn validate_identifier_characters(id: &str) -> bool {
    validate_identifier(id)
}

/// An Entity that is identified by a unique String id.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a new Identifier and returns a Result based on if it is valid or not.
    pub fn new(name: &str) -> Result<Self, WanderError> {
        if validate_identifier(name) {
            Ok(Self(name.to_string()))
        } else {
            Err(WanderError(format!("Invalid Entity id {}", name)))
        }
    }

    /// Returns the name of the given Identifier.
    pub fn id(&self) -> &str {
        &self.0
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}
