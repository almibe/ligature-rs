/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use futures::stream::Stream;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Entity {
    pub identifier: String,
}

pub enum Object {
    Entity(Entity),
    Literal(Literal),
}

pub enum Literal {
    LangLiteral(LangLiteral),
    StringLiteral(String),
    BooleanLiteral(bool),
    LongLiteral(i64),
    DoubleLiteral(f64),
}

pub struct LangLiteral {
    pub value: String,
    pub lang_tag: String,
}

pub struct Predicate {
    pub predicate: String,
}

pub enum Range {
    LangLiteralRange(LangLiteral, LangLiteral),
    StringLiteralRange(String, String),
    LongLiteralRange(i64, i64),
    DoubleLiteralRange(f64, f64),
}

pub struct Statement {
    pub subject: Entity,
    pub predicate: Predicate,
    pub object: Object,
    pub context: Entity,
}

pub struct CollectionName {
    pub name: String,
}

pub trait LigatureStore {
    fn read_tx(&self) -> &dyn ReadTx;
    fn write_tx(&self) -> &dyn WriteTx;

    /**
     * Close connection with the Store.
     */
    fn close(&mut self);

    fn is_open(&self) -> bool;
}

pub trait ReadTx {
    /**
     * Returns a Stream of all existing collections.
     */
    fn collections(&self) -> &dyn Stream<Item = CollectionName>;

    /**
     * Returns a Stream of all existing collections that start with the given prefix.
     */
    fn collections_prefix(&self, prefix: CollectionName) -> &dyn Stream<Item = CollectionName>;

    /**
     * Returns a Stream of all existing collections that are within the given range.
     * `from` is inclusive and `to` is exclusive.
     */
    fn collections_range(
        &self,
        from: CollectionName,
        to: CollectionName,
    ) -> &dyn Stream<Item = CollectionName>;

    /**
     * Accepts nothing but returns a Stream of all Statements in the Collection.
     */
    fn all_statements(&self, collection: CollectionName) -> &dyn Stream<Item = Statement>;

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn match_statements(
        &self,
        collection: CollectionName,
        subject: Option<Entity>,
        predicate: Option<Predicate>,
        object: Option<Object>,
        context: Option<Entity>,
    ) -> &dyn Stream<Item = Statement>;

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn match_statements_range(
        &self,
        collection: CollectionName,
        subject: Option<Entity>,
        predicate: Option<Predicate>,
        range: Option<Range>,
        context: Option<Entity>,
    ) -> &dyn Stream<Item = Statement>;

    /**
     * Cancels this transaction.
     */
    fn cancel(&self);

    fn is_open(&self) -> bool;
}

pub trait WriteTx {
    /**
     * Creates a collection with the given name or does nothing if the collection already exists.
     * Only useful for creating an empty collection.
     */
    fn create_collection(&self, collection: CollectionName);

    /**
     * Deletes the collection of the name given and does nothing if the collection doesn't exist.
     */
    fn delete_collection(&self, collection: CollectionName);

    /**
     * Returns a new, unique to this collection identifier in the form _:NUMBER
     */
    fn new_entity(&self, collection: CollectionName) -> Entity;
    fn remove_entity(&self, collection: CollectionName, entity: Entity);
    fn add_statement(&self, collection: CollectionName, statement: Statement);
    fn remove_statement(&self, collection: CollectionName, statement: Statement);

    /**
     * Commits this transaction.
     */
    fn commit(&self);

    /**
     * Cancels this transaction.
     */
    fn cancel(&self);

    fn is_open(&self) -> bool;
}

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
pub fn valid_predicate(identifier: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("^[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*$").unwrap();
    }
    RE.is_match(identifier)
}

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
pub fn valid_lang_tag(lang_tag: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[a-zA-Z]+(-[a-zA-Z0-9]+)*$").unwrap();
    }
    RE.is_match(lang_tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_identifier_tests() {
        assert_eq!(valid_predicate(""), false);
        assert_eq!(valid_predicate("http://localhost/people/7"), true);
        assert_eq!(valid_predicate("http://localhost(/people/7"), false);
        assert_eq!(valid_predicate("http://localhost /people/7"), false);
        assert_eq!(valid_predicate("hello"), true);
        assert_eq!(valid_predicate("_:"), true);
        assert_eq!(valid_predicate("_:valid"), true);
        assert_eq!(valid_predicate("_:1"), true);
        assert_eq!(valid_predicate("_:1344"), true);
    }

    #[test]
    fn valid_lang_tag_tests() {
        assert_eq!(valid_lang_tag(""), false);
        assert_eq!(valid_lang_tag("en"), true);
        assert_eq!(valid_lang_tag("en-"), false);
        assert_eq!(valid_lang_tag("en-fr"), true);
        assert_eq!(valid_lang_tag("en-fr-"), false);
        assert_eq!(valid_lang_tag("en-fr-sp"), true);
        assert_eq!(
            valid_lang_tag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj"),
            true
        );
        assert_eq!(valid_lang_tag("en-fr-ef "), false);
    }
}
