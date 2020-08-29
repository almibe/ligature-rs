/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import arrow.core.None
import arrow.core.Option
import kotlinx.coroutines.flow.Flow

sealed class Element
sealed class Subject: Element()
data class NamedElement(val identifier: String): Subject()
data class AnonymousElement(val identifier: Long): Subject()
sealed class Literal: Element()
data class LangLiteral(val value: String, val langTag: String): Literal()
data class StringLiteral(val value: String): Literal()
data class BooleanLiteral(val value: Boolean): Literal()
data class LongLiteral(val value: Long): Literal()
data class DoubleLiteral(val value: Double): Literal()

val a: NamedElement = NamedElement("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
fun validNamedElement(identifier: String): Boolean {
  return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
}

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
fun validLangTag(langTag: String): Boolean {
  return "[a-zA-Z]+(-[a-zA-Z0-9]+)*".toRegex().matches(langTag)
}

data class Statement(val subject: Subject, val predicate: NamedElement, val `object`: Element)
data class PersistedStatement(val collection: NamedElement, val statement: Statement, val context: AnonymousElement)

interface Ligature {
  suspend fun <T>read(fn: (ReadTx)->T): T
  suspend fun write(fn: (WriteTx)->Unit)
}

interface ReadTx {
  /**
   * Returns a Iterable of all existing collections.
   */
  suspend fun collections(): Flow<NamedElement>

  /**
   * Returns a Iterable of all existing collections that start with the given prefix.
   */
  suspend fun collections(prefix: NamedElement): Flow<NamedElement>

  /**
   * Returns a Iterable of all existing collections that are within the given range.
   * `from` is inclusive and `to` is exclusive.
   */
  suspend fun collections(from: NamedElement, to: NamedElement): Flow<NamedElement>

  /**
   * Accepts nothing but returns a Iterable of all Statements in the Collection.
   */
  suspend fun allStatements(collection: NamedElement): Flow<PersistedStatement>

  /**
   * Is passed a pattern and returns a seq with all matching Statements.
   */
  suspend fun matchStatements(collection: NamedElement,
                              subject: Option<Subject> = None,
                              predicate: Option<NamedElement> = None,
                              `object`: Option<Element> = None): Flow<PersistedStatement>

//  /**
//   * Is passed a pattern and returns a seq with all matching Statements.
//   */
//  fun matchStatements(collection: NamedEntity,
//                      subject: Option<Entity>,
//                      predicate: Option<Predicate>,
//                      range: ClosedRange<RangeLiteral>): Any, Throwable, Iterable<PersistedStatement>>

  /**
   * Returns the Statement with the given context.
   * Returns None if the context doesn't exist.
   */
  suspend fun statementByContext(collection: NamedElement, context: AnonymousElement): Option<PersistedStatement>
}

interface WriteTx {
  /**
   * Creates a collection with the given name or does nothing if the collection already exists.
   * Only useful for creating an empty collection.
   */
  suspend fun createCollection(collection: NamedElement): NamedElement

  /**
   * Deletes the collection of the name given and does nothing if the collection doesn't exist.
   */
  suspend fun deleteCollection(collection: NamedElement): NamedElement

  /**
   * Returns a new, unique to this collection, AnonymousEntity
   */
  suspend fun newEntity(collection: NamedElement): AnonymousElement
  suspend fun addStatement(collection: NamedElement, statement: Statement): PersistedStatement
//  Commenting out the below as part of #125
//  fun removeStatement(collection: NamedEntity, statement: Statement): Any, Throwable, Statement>>
//  fun removeEntity(collection: NamedEntity, entity: Entity): Any, Throwable, Entity>>
//  fun removePredicate(collection: NamedEntity, predicate: Predicate): Any, Throwable, Predicate>>

  /**
   * Cancels this transaction.
   */
  suspend fun cancel()
}
