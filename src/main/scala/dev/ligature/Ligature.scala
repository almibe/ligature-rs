/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import cats.effect.{IO, Resource}

import scala.util.Try

case class CollectionName(name: String)

sealed trait Vertex
case class Node(identifier: Literal) extends Vertex
case class AnonymousNode(identifier: Long) extends Vertex
sealed trait Literal extends Vertex
sealed trait RangeLiteral extends Literal
case class Range[T <: RangeLiteral, U <: RangeLiteral](start: T, end: U)(implicit ev: T =:= U)
case class LangLiteral(value: String, langTag: String) extends RangeLiteral
case class StringLiteral(value: String) extends RangeLiteral
case class BooleanLiteral(value: Boolean) extends Literal
case class LongLiteral(value: Long) extends RangeLiteral
case class DoubleLiteral(value: Double) extends RangeLiteral
case class Context(identifier: Long) extends Vertex

case class Edge(label: String)

object Ligature {
  /**
   * Accepts a String representing an identifier and returns true or false depending on if it is valid.
   */
  def validLabel(label: String): Boolean =
    "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(label)
}

case class Triple(source: Vertex, edge: Edge, destination: Vertex)
case class PersistedTriple(collection: CollectionName, triple: Triple, context: Node)

trait Ligature {
  def start(): Resource[IO, LigatureSession]
}

trait LigatureSession {
  def compute: Resource[IO, ReadTx]
  def write: Resource[IO, WriteTx]
}

trait ReadTx {
  /**
   * Returns a Iterable of all existing collections.
   */
  def collections: IO[Iterator[CollectionName]]

  /**
   * Returns a Iterable of all existing collections that start with the given prefix.
   */
  def collections(prefix: CollectionName): IO[Iterator[CollectionName]]

  /**
   * Returns a Iterable of all existing collections that are within the given range.
   * `from` is inclusive and `to` is exclusive.
   */
  def collections(from: CollectionName, to: CollectionName): IO[Iterator[CollectionName]]

  /**
   * Accepts nothing but returns a Iterable of all Statements in the Collection.
   */
  def allTriples(collection: CollectionName): IO[Iterator[PersistedTriple]]

  /**
   * Is passed a pattern and returns a seq with all matching Triples.
   */
  def matchTriples(collection: CollectionName,
                      subject: Option[Vertex] = None,
                      predicate: Option[Edge] = None,
                      `object`: Option[Vertex] = None): IO[Iterator[PersistedTriple]]

//  /**
//   * Is passed a pattern and returns a seq with all matching Triples.
//   */
//  def matchTriples(collection: NamedEntity,
//                      subject: Option[Entity],
//                      predicate: Option[Predicate],
//                      range: Range[_, _]): IO[Any, Throwable, Iterable[PersistedTriple]]

  /**
   * Returns the Triple with the given context.
   * Returns None if the context doesn't exist.
   */
  def statementByContext(collection: CollectionName, context: Node): IO[Option[PersistedTriple]]

  def isOpen: Boolean
}

trait WriteTx {
  /**
   * Creates a collection with the given name or does nothing if the collection already exists.
   * Only useful for creating an empty collection.
   */
  def createCollection(collection: CollectionName): IO[Try[CollectionName]]

  /**
   * Deletes the collection of the name given and does nothing if the collection doesn't exist.
   */
  def deleteCollection(collection: CollectionName): IO[Try[CollectionName]]

  /**
   * Returns a new, unique to this collection, AnonymousEntity
   */
  def newEntity(collection: CollectionName): IO[Try[AnonymousNode]]
  def addTriple(collection: CollectionName, statement: Triple): IO[Try[PersistedTriple]]
//  Commenting out the below as part of #125
//  def removeTriple(collection: NamedEntity, statement: Triple): IO[Any, Throwable, Try[Triple]]
//  def removeEntity(collection: NamedEntity, entity: Entity): IO[Any, Throwable, Try[Entity]]
//  def removePredicate(collection: NamedEntity, predicate: Predicate): IO[Any, Throwable, Try[Predicate]]

  /**
   * Cancels this transaction.
   */
  def cancel(): Unit

  def isOpen: Boolean
}
