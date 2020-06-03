/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature


sealed trait Entity
final case class NamedEntity(val name: String)
final case class AnonymousEntity(val id: Long)

final case class LangLiteral(val value: String, val langTag: String)

type Literal = String | Boolean | Long | Double | LangLiteral

type Object = Entity | Literal

val a = NamedEntity("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
val default = NamedEntity("_")

final case class Statement(val subject: Entity, val predicate: NamedEntity, val `object`: Object, val context: Entity = default)

// type Range<T>(open val start: T, open val end: T)
// final case class LangLiteralRange(override val start: LangLiteral, override val end: LangLiteral): Range<LangLiteral>(start, end)
// final case class StringLiteralRange(override val start: String, override val end: String): Range<String>(start, end)
// final case class LongLiteralRange(override val start: Long, override val end: Long): Range<Long>(start, end)
// final case class DoubleLiteralRange(override val start: Double, override val end: Double): Range<Double>(start, end)

final case class CollectionName(val name: String)

trait LigatureStore {
  def readTx(): ReadTx
  def writeTx(): WriteTx
//   def <T>compute(fn: suspend (ReadTx) -> T): T {
//     val readTx = this.readTx()
//     try {
//       return fn(readTx)
//     } finally {
//       if (readTx.isOpen()) {
//         readTx.cancel()
//       }
//     }
//   }
 
//   def write(fn: suspend (WriteTx) -> Unit) {
//     val writeTx = this.writeTx()
//     try {
//       return fn(writeTx)
//     } finally {
//       if (writeTx.isOpen()) {
//         writeTx.commit()
//       }
//     }
//   }
 
  /**
   * Close connection with the Store.
   */
  def close(): Unit
 
  def isOpen(): Boolean
}
 
trait ReadTx {
  /**
   * Returns a Flow of all existing collections.
   */
  def collections(): Flow<CollectionName>

  /**
   * Returns a Flow of all existing collections that start with the given prefix.
   */
  def collections(prefix: CollectionName): Flow<CollectionName>

  /**
   * Returns a Flow of all existing collections that are within the given range.
   * `from` is inclusive and `to` is exclusive.
   */
  def collections(from: CollectionName, to: CollectionName): Flow<CollectionName>

  /**
   * Accepts nothing but returns a Flow of all Statements in the Collection.
   */
  def allStatements(collection: CollectionName): Flow<Statement>

  /**
   * Is passed a pattern and returns a seq with all matching Statements.
   */
  def matchStatements(collection: CollectionName, subject: Entity | null = null, predicate: NamedEntity | null = null, `object`: Object? = null, context: Entity? = null): Flow<Statement>

//   /**
//    * Is passed a pattern and returns a seq with all matching Statements.
//    */
//   def matchStatements(collection: CollectionName, subject: Entity? = null, predicate: Predicate? = null, range: Range<*>, context: Entity? = null): Flow<Statement>

  /**
   * Cancels this transaction.
   */
  def cancel(): Unit

  def isOpen(): Boolean
 }

trait WriteTx {
  /**
   * Creates a collection with the given name or does nothing if the collection already exists.
   * Only useful for creating an empty collection.
   */
  def createCollection(collection: CollectionName)

  /**
   * Deletes the collection of the name given and does nothing if the collection doesn't exist.
   */
  def deleteCollection(collection: CollectionName)

  /**
   * Returns a new, unique to this collection identifier in the form _:NUMBER
   */
  def newEntity(collection: CollectionName): Entity
  def addStatement(collection: CollectionName, statement: Statement)
  def removeStatement(collection: CollectionName, statement: Statement)
  def removeEntity(entity: Entity)

  /**
   * Commits this transaction.
   */
  def commit(): Unit

  /**
   * Cancels this transaction.
   */
  def cancel(): Unit

  def isOpen(): Boolean
}

// /**
//  * Accepts a String representing an identifier and returns true or false depending on if it is valid.
//  */
// def validPredicate(identifier: String): Boolean {
//   return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
// }

// /**
//  * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
//  */
// def validLangTag(langTag: String): Boolean {
//   return "[a-zA-Z]+(-[a-zA-Z0-9]+)*".toRegex().matches(langTag)
// }
