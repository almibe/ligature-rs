// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the Ligature project.
//! It represents to common types and traits used by Ligature.

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;

use std::fmt::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Check if a given name is valid.
pub fn validate_element(id: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9-._~:/?#\[\]@!$&'()*+,;%=\x{00A0}-\x{D7FF}\x{F900}-\x{FDCF}\x{FDF0}-\x{FFEF}\x{10000}-\x{1FFFD}\x{20000}-\x{2FFFD}\x{30000}-\x{3FFFD}\x{40000}-\x{4FFFD}\x{50000}-\x{5FFFD}\x{60000}-\x{6FFFD}\x{70000}-\x{7FFFD}\x{80000}-\x{8FFFD}\x{90000}-\x{9FFFD}\x{A0000}-\x{AFFFD}\x{B0000}-\x{BFFFD}\x{C0000}-\x{CFFFD}\x{D0000}-\x{DFFFD}\x{E1000}-\x{EFFFD}]+$").unwrap();
    }

    RE.is_match(id)
}

/// Check if a given str only contains valid characters.
/// This is a duplicate of valid_name for now but the two could deviate potentially.
pub fn validate_element_characters(id: &str) -> bool {
    validate_element(id)
}

/// An Entity that is identified by a unique String id.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Element(pub String);

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A single entry in a Network.
pub enum Entry {
    /// A relationship between two elements.
    Role {
        /// The Entity of a Statement
        first: Element,
        /// The Attribute of a Statement
        second: Element,
        /// The Value of a Statement
        role: Element,
    },
    /// Represens an element extending a concept.
    Extension {
        /// The element
        element: Element,
        /// The concept
        concept: Element
    },
    /// Represens an element not extending a concept.
    NotExtension {
        /// The element
        element: Element,
        /// The concept
        concept: Element
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Role { first, second, role } => 
                write!(f, "{} {} {}", first, role, second),
            Entry::Extension { element, concept } =>
                write!(f, "{} : {}", element, concept),
            Entry::NotExtension { element, concept } => 
                write!(f, "{} ¬: {}", element, concept),
        }
    }
}

/// A general struct for representing errors involving Ligature.
/// TODO should probably be an enum with a bunch of specific cases
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LigatureError(pub String);
