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
    // sealed class Range<T>(open val start: T, open val end: T)
    // data class LangLiteralRange(override val start: LangLiteral, override val end: LangLiteral): Range<LangLiteral>(start, end)
    // data class StringLiteralRange(override val start: String, override val end: String): Range<String>(start, end)
    // data class LongLiteralRange(override val start: Long, override val end: Long): Range<Long>(start, end)
    // data class DoubleLiteralRange(override val start: Double, override val end: Double): Range<Double>(start, end)
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
// if (readTx.isOpen()) {
// readTx.cancel()
// }
// }
//}

// fn write(fn: suspend (WriteTx) -> Unit) {
// val writeTx = this.writeTx()
// try {
// return fn(writeTx)
// } finally {
// if (writeTx.isOpen()) {
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
    fn collections(prefix: CollectionName) -> Flow<CollectionName>

    /**
     * Returns a Flow of all existing collections that are within the given range.
     * `from` is inclusive and `to` is exclusive.
     */
    fn collections(from: CollectionName, to: CollectionName) -> Flow<CollectionName>

    /**
     * Accepts nothing but returns a Flow of all Statements in the Collection.
     */
    fn allStatements(collection: CollectionName) -> Flow<Statement>

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn matchStatements(collection: CollectionName, subject: Entity? = null, predicate: Predicate? = null, `object`: Object? = null, context: Entity? = null) -> Flow<Statement>

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn matchStatements(collection: CollectionName, subject: Entity? = null, predicate: Predicate? = null, range: Range<*>, context: Entity? = null) -> Flow<Statement>

    /**
     * Cancels this transaction.
     */
    fn cancel();

    fn isOpen() -> bool;
}

trait WriteTx {
    /**
     * Creates a collection with the given name or does nothing if the collection already exists.
     * Only useful for creating an empty collection.
     */
    fn createCollection(collection: CollectionName)

    /**
     * Deletes the collection of the name given and does nothing if the collection doesn't exist.
     */
    fn deleteCollection(collection: CollectionName)

    /**
     * Returns a new, unique to this collection identifier in the form _:NUMBER
     */
    fn newEntity(collection: CollectionName) -> Entity
    fn addStatement(collection: CollectionName, statement: Statement)
    fn removeStatement(collection: CollectionName, statement: Statement)

    /**
     * Commits this transaction.
     */
    fn commit()

    /**
     * Cancels this transaction.
     */
    fn cancel()

    fn isOpen() -> bool
}

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
fn validPredicate(identifier: String) -> bool {
    return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
}

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
fn validLangTag(langTag: String) -> bool {
    return "[a-zA-Z]+(-[a-zA-Z0-9]+)*".toRegex().matches(langTag)
}

///////TESTS

class LigatureSpec : StringSpec({
"validIdentifier tests" {
validPredicate("") shouldBe false
validPredicate("http://localhost/people/7") shouldBe true
validPredicate("http://localhost(/people/7") shouldBe false
validPredicate("http://localhost /people/7") shouldBe false
validPredicate("hello") shouldBe true
validPredicate("_:") shouldBe true
validPredicate("_:valid") shouldBe true
validPredicate("_:1") shouldBe true
validPredicate("_:1344") shouldBe true
}

"validLangTag tests" {
validLangTag("") shouldBe false
validLangTag("en") shouldBe true
validLangTag("en-") shouldBe false
validLangTag("en-fr") shouldBe true
validLangTag("en-fr-") shouldBe false
validLangTag("en-fr-sp") shouldBe true
validLangTag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj") shouldBe true
validLangTag("en-fr-ef ") shouldBe false
}
})
