/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

sealed class Object
data class Entity(val identifier: Long): Object()
sealed class Literal: Object()
data class LangLiteral(val value: String, val langTag: String): Literal() {
init {
require(validLangTag(langTag)) {
"Invalid lang tag: $langTag"
}
}
}
data class StringLiteral(val value: String): Literal()
data class BooleanLiteral(val value: Boolean): Literal()
data class LongLiteral(val value: Long): Literal()
data class DoubleLiteral(val value: Double): Literal()

data class Predicate(val identifier: String) {
init {
require(validPredicate(identifier)) {
"Invalid Predicate: $identifier"
}
}
}

val a = Predicate("_a")
val default = Entity(0)

data class Statement(val subject: Entity, val predicate: Predicate, val `object`: Object, val context: Entity = default)

sealed class Range<T>(open val start: T, open val end: T)
data class LangLiteralRange(override val start: LangLiteral, override val end: LangLiteral): Range<LangLiteral>(start, end)
data class StringLiteralRange(override val start: String, override val end: String): Range<String>(start, end)
data class LongLiteralRange(override val start: Long, override val end: Long): Range<Long>(start, end)
data class DoubleLiteralRange(override val start: Double, override val end: Double): Range<Double>(start, end)

data class CollectionName(val name: String) {
init {
require(validPredicate(name)) {
"Invalid Collection Name: $name"
}
}
}

interface LigatureStore {
suspend fun readTx(): ReadTx
suspend fun writeTx(): WriteTx
suspend fun <T>compute(fn: suspend (ReadTx) -> T): T {
val readTx = this.readTx()
try {
return fn(readTx)
} finally {
if (readTx.isOpen()) {
readTx.cancel()
}
}
}

suspend fun write(fn: suspend (WriteTx) -> Unit) {
val writeTx = this.writeTx()
try {
return fn(writeTx)
} finally {
if (writeTx.isOpen()) {
writeTx.commit()
}
}
}

/**
 * Close connection with the Store.
 */
suspend fun close()

suspend fun isOpen(): Boolean
}

interface ReadTx {
/**
 * Returns a Flow of all existing collections.
 */
suspend fun collections(): Flow<CollectionName>

/**
 * Returns a Flow of all existing collections that start with the given prefix.
 */
suspend fun collections(prefix: CollectionName): Flow<CollectionName>

/**
 * Returns a Flow of all existing collections that are within the given range.
 * `from` is inclusive and `to` is exclusive.
 */
suspend fun collections(from: CollectionName, to: CollectionName): Flow<CollectionName>

/**
 * Accepts nothing but returns a Flow of all Statements in the Collection.
 */
suspend fun allStatements(collection: CollectionName): Flow<Statement>

/**
 * Is passed a pattern and returns a seq with all matching Statements.
 */
suspend fun matchStatements(collection: CollectionName, subject: Entity? = null, predicate: Predicate? = null, `object`: Object? = null, context: Entity? = null): Flow<Statement>

/**
 * Is passed a pattern and returns a seq with all matching Statements.
 */
suspend fun matchStatements(collection: CollectionName, subject: Entity? = null, predicate: Predicate? = null, range: Range<*>, context: Entity? = null): Flow<Statement>

/**
 * Cancels this transaction.
 */
suspend fun cancel()

suspend fun isOpen(): Boolean
}

interface WriteTx {
/**
 * Creates a collection with the given name or does nothing if the collection already exists.
 * Only useful for creating an empty collection.
 */
suspend fun createCollection(collection: CollectionName)

/**
 * Deletes the collection of the name given and does nothing if the collection doesn't exist.
 */
suspend fun deleteCollection(collection: CollectionName)

/**
 * Returns a new, unique to this collection identifier in the form _:NUMBER
 */
suspend fun newEntity(collection: CollectionName): Entity
suspend fun addStatement(collection: CollectionName, statement: Statement)
suspend fun removeStatement(collection: CollectionName, statement: Statement)

/**
 * Commits this transaction.
 */
suspend fun commit()

/**
 * Cancels this transaction.
 */
suspend fun cancel()

suspend fun isOpen(): Boolean
}

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
fun validPredicate(identifier: String): Boolean {
return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
}

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
fun validLangTag(langTag: String): Boolean {
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
