/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use futures::stream::Stream;

pub struct Entity {
    identifier: u64
}

pub enum Object<'a> {
    Entity(Entity),
    Literal(Literal<'a>)
}

pub enum Literal<'a> {
    LangLiteral(LangLiteral<'a>),
    StringLiteral(&'a str),
    BooleanLiteral(bool),
    LongLiteral(i64),
    DoubleLiteral(f64),
}

pub struct LangLiteral<'a> {
    value: &'a str,
    lang_tag: &'a str,
}

pub struct Predicate<'a> {
    predicate: &'a str,
}

pub enum Range<'a> {
    LangLiteralRange(LangLiteral<'a>, LangLiteral<'a>),
    StringLiteralRange(&'a str, &'a str),
    LongLiteralRange(i64, i64),
    DoubleLiteralRange(f64, f64),
}

pub struct Statement<'a> {
    subject: Entity,
    predicate: Predicate<'a>,
    object: Object<'a>,
    context: Entity,
}

pub const A: Predicate = Predicate { predicate: "_a" };
pub const DEFAULT: Entity = Entity { identifier: 0 };

pub struct CollectionName<'a> {
    name: &'a str
}

pub trait LigatureStore<R, W> where R: ReadTxTrait, W: WriteTxTrait {
    fn read_tx() -> ReadTx<R>;
    fn write_tx() -> WriteTx<W>;
//     fn <T>compute(fun: suspend (ReadTx) -> T): T {
// val readTx = this.readTx()
// try {
// return fn(readTx)
// } finally {
// if (readTx.is_open()) {
// readTx.cancel()
// }
// }
//}

// fn write(fn: suspend (WriteTx) -> Unit) {
// val writeTx = this.writeTx()
// try {
// return fn(writeTx)
// } finally {
// if (writeTx.is_open()) {
// writeTx.commit()
// }
// }
// }

    /**
     * Close connection with the Store.
     */
    fn close();

    fn is_open() -> bool;
}

pub struct ReadTx<T> where T: ReadTxTrait {
    pub read_tx: T,
}

pub struct WriteTx<T> where T: WriteTxTrait {
    pub write_tx: T,
}

pub trait ReadTxTrait {
    /**
     * Returns a Stream of all existing collections.
     */
    fn collections<'a>() -> dyn Stream<Item = CollectionName<'a>>;

    /**
     * Returns a Stream of all existing collections that start with the given prefix.
     */
    fn collections_prefix<'a>(prefix: CollectionName) -> dyn Stream<Item = CollectionName<'a>>;

    /**
     * Returns a Stream of all existing collections that are within the given range.
     * `from` is inclusive and `to` is exclusive.
     */
    fn collections_range<'a>(from: CollectionName, to: CollectionName) -> dyn Stream<Item = CollectionName<'a>>;

    /**
     * Accepts nothing but returns a Stream of all Statements in the Collection.
     */
    fn all_statements<'a>(collection: CollectionName) -> dyn Stream<Item = Statement<'a>>;

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn match_statements<'a>(collection: CollectionName, subject: Option<Entity>, predicate: Option<Predicate>, object: Option<Object>, context: Option<Entity>) -> dyn Stream<Item = Statement<'a>>;

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn match_statements_range<'a>(collection: CollectionName, subject: Option<Entity>, predicate: Option<Predicate>, range: Option<Range>, context: Option<Entity>) -> dyn Stream<Item = Statement<'a>>;

    /**
     * Cancels this transaction.
     */
    fn cancel();

    fn is_open() -> bool;
}

pub trait WriteTxTrait {
    /**
     * Creates a collection with the given name or does nothing if the collection already exists.
     * Only useful for creating an empty collection.
     */
    fn create_collection(collection: CollectionName);

    /**
     * Deletes the collection of the name given and does nothing if the collection doesn't exist.
     */
    fn delete_collection(collection: CollectionName);

    /**
     * Returns a new, unique to this collection identifier in the form _:NUMBER
     */
    fn new_entity(collection: CollectionName) -> Entity;
    fn add_statement(collection: CollectionName, statement: Statement);
    fn remove_statement(collection: CollectionName, statement: Statement);

    /**
     * Commits this transaction.
     */
    fn commit();

    /**
     * Cancels this transaction.
     */
    fn cancel();

    fn is_open() -> bool;
}

// /**
//  * Accepts a String representing an identifier and returns true or false depending on if it is valid.
//  */
// fn valid_predicate(identifier: &str) -> bool {
//     return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
// }

// /**
//  * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
//  */
// fn valid_lang_tag(lang_tag: &str) -> bool {
//     return "[a-zA-Z]+(-[a-zA-Z0-9]+)*".toRegex().matches(lang_tag)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn valid_identifier_tests() {
//         assert_eq!(valid_predicate(""), false);
//         assert_eq!(valid_predicate("http://localhost/people/7"), true);
//         assert_eq!(valid_predicate("http://localhost(/people/7"), false);
//         assert_eq!(valid_predicate("http://localhost /people/7"), false);
//         assert_eq!(valid_predicate("hello"), true);
//         assert_eq!(valid_predicate("_:"), true);
//         assert_eq!(valid_predicate("_:valid"), true);
//         assert_eq!(valid_predicate("_:1"), true);
//         assert_eq!(valid_predicate("_:1344"), true);
//     }
//
//     #[test]
//     fn valid_lang_tag_tests() {
//         assert_eq!(valid_lang_tag(""), false);
//         assert_eq!(valid_lang_tag("en"), true);
//         assert_eq!(valid_lang_tag("en-"), false);
//         assert_eq!(valid_lang_tag("en-fr"), true);
//         assert_eq!(valid_lang_tag("en-fr-"), false);
//         assert_eq!(valid_lang_tag("en-fr-sp"), true);
//         assert_eq!(valid_lang_tag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj"), true);
//         assert_eq!(valid_lang_tag("en-fr-ef "), false);
//     }
// }
