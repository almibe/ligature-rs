// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module contains Ligature's data model in Rust.

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// An Element that is identified by a unique String value.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Element(pub String);

/// A single entry in a Network.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Entry {
    /// A relationship between two Elements.
    Role {
        /// The first Element.
        first: Element,
        /// The second Element.
        second: Element,
        /// The Role between the two Elements.
        role: Element,
    },
    /// Represens an Element extending a Concept.
    Extends {
        /// The Element.
        element: Element,
        /// The Concept the Element extends.
        concept: Element,
    },
    /// Represens an Element not extending a Concept.
    NotExtends {
        /// The Element.
        element: Element,
        /// The Concept the Element does not extend.
        concept: Element,
    },
}
