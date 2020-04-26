/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

enum Object<'a> {
    Entity(u64),
    Literal(Literal<'a>)
}

enum Literal<'a> {
    LangLiteral(&'a str, LangTag<'a>),
    StringLiteral(&'a str),
    BooleanLiteral(bool),
    LongLiteral(i64),
    DoubleLiteral(f64),
}

struct LangTag<'a> {
    langTag: &'a str
}

struct Predicate<'a> {
    predicate: &'a str,
}

enum Range {
    LangLiteralRange(Literal::LangLiteral, Literal::LangLiteral),
    StringLiteralRange(Literal::StringLiteral, Literal::StringLiteral),
    LongLiteralRange(Literal::LongLiteral, Literal::LongLiteral),
    DoubleLiteralRange(Literal::DoubleLiteral, Literal::DoubleLiteral),
}

struct Statement<'a> {
    subject: Entity,
    predicate: Predicate<'a>,
    object: Object<'a>,
    context: Entity,
}

pub const A: Predicate = Predicate("_a");
pub const DEFAULT: Entity = Object::Entity(0);

struct CollectionName<'a> {
    name: &'a str
}

trait LigatureStore {
    fn readTx() -> ReadTx;
    fn writeTx() -> WriteTx;
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

    fn isOpen() -> bool;
}

trait ReadTx {
    /**
     * Returns a Flow of all existing collections.
     */
    fn collections() -> Stream<CollectionName>;

    /**
     * Returns a Flow of all existing collections that start with the given prefix.
     */
    fn collections_prefix(prefix: CollectionName) -> Flow<CollectionName>

    /**
     * Returns a Flow of all existing collections that are within the given range.
     * `from` is inclusive and `to` is exclusive.
     */
    fn collections_range(from: CollectionName, to: CollectionName) -> Flow<CollectionName>

    /**
     * Accepts nothing but returns a Flow of all Statements in the Collection.
     */
    fn all_statements(collection: CollectionName) -> Flow<Statement>

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn match_statements(collection: CollectionName, subject: Entity? = null, predicate: Predicate? = null, `object`: Object? = null, context: Entity? = null) -> Flow<Statement>

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn match_statements_range(collection: CollectionName, subject: Entity? = null, predicate: Predicate? = null, range: Range<*>, context: Entity? = null) -> Flow<Statement>

    /**
     * Cancels this transaction.
     */
    fn cancel();

    fn is_open() -> bool;
}

trait WriteTx {
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

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
fn valid_predicate(identifier: String) -> bool {
    return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
}

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
fn valid_lang_tag(langTag: String) -> bool {
    return "[a-zA-Z]+(-[a-zA-Z0-9]+)*".toRegex().matches(langTag)
}

#[cfg(test)]
mod tests {
    #[test]
    fn valid_identifier_tests() {
        assert_eq(valid_predicate(""), false);
        assert_eq(valid_predicate("http://localhost/people/7"), true);
        assert_eq(valid_predicate("http://localhost(/people/7"), false);
        assert_eq(valid_predicate("http://localhost /people/7"), false);
        assert_eq(valid_predicate("hello"), true);
        assert_eq(valid_predicate("_:"), true);
        assert_eq(valid_predicate("_:valid"), true);
        assert_eq(valid_predicate("_:1"), true);
        assert_eq(valid_predicate("_:1344"), true);
    }

    #[test]
    fn valid_lang_tag_tests() {
        assert_eq(valid_lang_tag(""), false);
        assert_eq(valid_lang_tag("en"), true);
        assert_eq(valid_lang_tag("en-"), false);
        assert_eq(valid_lang_tag("en-fr"), true);
        assert_eq(valid_lang_tag("en-fr-"), false);
        assert_eq(valid_lang_tag("en-fr-sp"), true);
        assert_eq(valid_lang_tag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj"), true);
        assert_eq(valid_lang_tag("en-fr-ef "), false);
    }
}
