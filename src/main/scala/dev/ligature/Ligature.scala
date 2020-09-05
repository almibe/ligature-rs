/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import cats.effect.{IO, Resource}
import fs2.Stream

sealed trait Object
sealed trait Node extends Object
sealed trait NamedNode extends Node
case class LocalNode(identifier: String) extends NamedNode
case class IRINode(iri: String) extends NamedNode
case class AnonymousNode(val identifier: Long) extends Node()
sealed trait Literal extends Object
case class LangLiteral(val value: String, val langTag: String) extends Literal()
case class StringLiteral(val value: String) extends Literal()
case class BooleanLiteral(val value: Boolean) extends Literal()
case class LongLiteral(val value: Long) extends Literal()
case class DoubleLiteral(val value: Double) extends Literal()

val a: IRINode = IRINode("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
def validNamedNode(namedNode: NamedNode): Boolean =
  namedNode match
    case local: LocalNode => "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(local.identifier)
    case iri: IRINode => ???

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
def validLangTag(langTag: String): Boolean =
  "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)

case class Statement(val subject: Node, val predicate: NamedNode, val `object`: Object)
case class PersistedStatement(val collection: LocalNode, val statement: Statement, val context: AnonymousNode)

trait Ligature:
  def store(): Resource[IO, LigatureStore]

trait LigatureStore:
  def read(): Resource[IO, ReadTx]
  def write(): Resource[IO, WriteTx]

trait ReadTx:
  /**
   * Returns a Iterable of all existing collections.
   */
  def collections(): Stream[IO, LocalNode]

  /**
   * Returns a Iterable of all existing collections that start with the given prefix.
   */
  def collections(prefix: LocalNode): Stream[IO, LocalNode]

  /**
   * Returns a Iterable of all existing collections that are within the given range.
   * `from` is inclusive and `to` is exclusive.
   */
  def collections(from: LocalNode, to: LocalNode): Stream[IO, LocalNode]

  /**
   * Accepts nothing but returns a Iterable of all Statements in the Collection.
   */
  def allStatements(collection: LocalNode): Stream[IO, PersistedStatement]

  /**
   * Is passed a pattern and returns a seq with all matching Statements.
   */
  def matchStatements(collection: LocalNode,
    subject: Option[Node] = None,
    predicate: Option[NamedNode] = None,
    `object`: Option[Object] = None): Stream[IO, PersistedStatement]

//  /**
//   * Is passed a pattern and returns a seq with all matching Statements.
//   */
//  fun matchStatements(collection: NamedEntity,
//                      subject: Option[Entity],
//                      predicate: Option[Predicate],
//                      range: ClosedRange[RangeLiteral]): Any, Throwable, Stream[IO, PersistedStatement]

  /**
   * Returns the Statement with the given context.
   * Returns None if the context doesn't exist.
   */
  def statementByContext(collection: LocalNode, context: AnonymousNode): IO[Option[PersistedStatement]]

trait WriteTx:
  /**
   * Creates a collection with the given name or does nothing if the collection already exists.
   * Only useful for creating an empty collection.
   */
  def createCollection(collection: LocalNode): IO[LocalNode]

  /**
   * Deletes the collection of the name given and does nothing if the collection doesn't exist.
   */
  def deleteCollection(collection: LocalNode): IO[LocalNode]

  /**
   * Returns a new, unique to this collection, AnonymousEntity
   */
  def newEntity(collection: LocalNode): IO[AnonymousNode]
  def addStatement(collection: LocalNode, statement: Statement): IO[PersistedStatement]
  //  Commenting out the below as part of #125
  //  fun removeStatement(collection: NamedEntity, statement: Statement): Any, Throwable, Statement>>
  //  fun removeEntity(collection: NamedEntity, entity: Entity): Any, Throwable, Entity>>
  //  fun removePredicate(collection: NamedEntity, predicate: Predicate): Any, Throwable, Predicate>>

  /**
   * Cancels this transaction.
   */
  def cancel(): Unit
