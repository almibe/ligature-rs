/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

sealed class Element
sealed class Subject: Element()
data class NamedElement(val identifier: String): Subject()
data class AnonymousElement(val identifier: Long): Subject()
sealed class Literal: Element()
sealed class RangeLiteral: Literal()
data class LangLiteral(val value: String, val langTag: String): RangeLiteral()
data class StringLiteral(val value: String): RangeLiteral()
data class BooleanLiteral(val value: Boolean): Literal()
data class LongLiteral(val value: Long): RangeLiteral()
data class DoubleLiteral(val value: Double): RangeLiteral()

val a: NamedElement = NamedElement("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
fun validNamedElement(identifier: String): Boolean =
  "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(identifier)

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
fun validLangTag(langTag: String): Boolean =
  "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)

data class Statement(val subject: Subject, val predicate: NamedElement, val `object`: Element)
data class PersistedStatement(val collection: NamedElement, val statement: Statement, val context: AnonymousElement)

interface Ligature {
  fun compute: Resource[IO, ReadTx]
  fun write: Resource[IO, WriteTx]
}

interface ReadTx {
  /**
   * Returns a Iterable of all existing collections.
   */
  fun collections: IO[Iterator[NamedElement]]

  /**
   * Returns a Iterable of all existing collections that start with the given prefix.
   */
  fun collections(prefix: NamedElement): IO[Iterator[NamedElement]]

  /**
   * Returns a Iterable of all existing collections that are within the given range.
   * `from` is inclusive and `to` is exclusive.
   */
  fun collections(from: NamedElement, to: NamedElement): IO[Iterator[NamedElement]]

  /**
   * Accepts nothing but returns a Iterable of all Statements in the Collection.
   */
  fun allStatements(collection: NamedElement): IO[Iterator[PersistedStatement]]

  /**
   * Is passed a pattern and returns a seq with all matching Statements.
   */
  fun matchStatements(collection: NamedElement,
                      subject: Option[Subject] = None,
                      predicate: Option[NamedElement] = None,
                      `object`: Option[Element] = None): IO[Iterator[PersistedStatement]]

//  /**
//   * Is passed a pattern and returns a seq with all matching Statements.
//   */
//  fun matchStatements(collection: NamedEntity,
//                      subject: Option[Entity],
//                      predicate: Option[Predicate],
//                      range: ClosedRange<RangeLiteral>): IO[Any, Throwable, Iterable[PersistedStatement]]

  /**
   * Returns the Statement with the given context.
   * Returns None if the context doesn't exist.
   */
  fun statementByContext(collection: NamedElement, context: AnonymousElement): IO[Option[PersistedStatement]]

  fun isOpen: Boolean
}

interface WriteTx {
  /**
   * Creates a collection with the given name or does nothing if the collection already exists.
   * Only useful for creating an empty collection.
   */
  fun createCollection(collection: NamedElement): IO[Try[NamedElement]]

  /**
   * Deletes the collection of the name given and does nothing if the collection doesn't exist.
   */
  fun deleteCollection(collection: NamedElement): IO[Try[NamedElement]]

  /**
   * Returns a new, unique to this collection, AnonymousEntity
   */
  fun newEntity(collection: NamedElement): IO[Try[AnonymousElement]]
  fun addStatement(collection: NamedElement, statement: Statement): IO[Try[PersistedStatement]]
//  Commenting out the below as part of #125
//  fun removeStatement(collection: NamedEntity, statement: Statement): IO[Any, Throwable, Try[Statement]]
//  fun removeEntity(collection: NamedEntity, entity: Entity): IO[Any, Throwable, Try[Entity]]
//  fun removePredicate(collection: NamedEntity, predicate: Predicate): IO[Any, Throwable, Try[Predicate]]

  /**
   * Cancels this transaction.
   */
  fun cancel(): Unit

  fun isOpen(): Boolean
}
