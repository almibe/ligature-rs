/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

sealed trait Object
sealed trait Node extends Object
sealed trait NamedNode extends Node
case class LocalNode(name: String) extends NamedNode
case class IRINode(iri: String) extends NamedNode
case class AnonymousNode(val identifier: Long) extends Node()
sealed trait Literal extends Object
case class LangLiteral(val value: String, val langTag: String) extends Literal()
case class StringLiteral(val value: String) extends Literal()
case class BooleanLiteral(val value: Boolean) extends Literal()
case class LongLiteral(val value: Long) extends Literal()
case class DoubleLiteral(val value: Double) extends Literal()

val a: IRIElement = IRIElement("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
def validNamedElement(identifier: NamedElement): Boolean =
  "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(identifier)

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
def validLangTag(langTag: String): Boolean =
  "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)

case class Statement(val subject: Node, val predicate: NamedNode, val `object`: Object)
case class PersistedStatement(val collection: LocalNode, val statement: Statement, val context: AnonymousNode)

//trait Ligature
//  def use(fn: (LigatureStore) -> Unit)
//
//trait LigatureStore {
//suspend fun <T>read(fn: (ReadTx)->T): T
//suspend fun write(fn: (WriteTx)->Unit)
//}
//
//trait ReadTx {
///**
// * Returns a Iterable of all existing collections.
// */
//suspend fun collections(): Flow<NamedElement>
//
///**
// * Returns a Iterable of all existing collections that start with the given prefix.
// */
//suspend fun collections(prefix: NamedElement): Flow<NamedElement>
//
///**
// * Returns a Iterable of all existing collections that are within the given range.
// * `from` is inclusive and `to` is exclusive.
// */
//suspend fun collections(from: NamedElement, to: NamedElement): Flow<NamedElement>
//
///**
// * Accepts nothing but returns a Iterable of all Statements in the Collection.
// */
//suspend fun allStatements(collection: NamedElement): Flow<PersistedStatement>
//
///**
// * Is passed a pattern and returns a seq with all matching Statements.
// */
//suspend fun matchStatements(collection: NamedElement,
//subject: Option<Subject> = None,
//predicate: Option<NamedElement> = None,
//`object`: Option<Element> = None): Flow<PersistedStatement>
//
////  /**
////   * Is passed a pattern and returns a seq with all matching Statements.
////   */
////  fun matchStatements(collection: NamedEntity,
////                      subject: Option<Entity>,
////                      predicate: Option<Predicate>,
////                      range: ClosedRange<RangeLiteral>): Any, Throwable, Iterable<PersistedStatement>>
//
///**
// * Returns the Statement with the given context.
// * Returns None if the context doesn't exist.
// */
//suspend fun statementByContext(collection: NamedElement, context: AnonymousElement): Option<PersistedStatement>
//}
//
//trait WriteTx {
///**
// * Creates a collection with the given name or does nothing if the collection already exists.
// * Only useful for creating an empty collection.
// */
//suspend fun createCollection(collection: NamedElement): NamedElement
//
///**
// * Deletes the collection of the name given and does nothing if the collection doesn't exist.
// */
//suspend fun deleteCollection(collection: NamedElement): NamedElement
//
///**
// * Returns a new, unique to this collection, AnonymousEntity
// */
//suspend fun newEntity(collection: NamedElement): AnonymousElement
//suspend fun addStatement(collection: NamedElement, statement: Statement): PersistedStatement
////  Commenting out the below as part of #125
////  fun removeStatement(collection: NamedEntity, statement: Statement): Any, Throwable, Statement>>
////  fun removeEntity(collection: NamedEntity, entity: Entity): Any, Throwable, Entity>>
////  fun removePredicate(collection: NamedEntity, predicate: Predicate): Any, Throwable, Predicate>>
//
///**
// * Cancels this transaction.
// */
//suspend fun cancel()
//}
