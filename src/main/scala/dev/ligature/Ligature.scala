/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import monix.eval.Task
import monix.reactive.Observable

import scala.util.Try

sealed trait Object
sealed trait Entity extends Object
case class NamedEntity(identifier: String) extends Entity
case class AnonymousEntity(identifier: Long) extends Entity
case class Predicate(identifier: String)
sealed trait Literal extends Object
sealed trait RangeLiteral extends Literal
case class Range[T <: RangeLiteral, U <: RangeLiteral](start: T, end: U)(implicit ev: T =:= U)
case class LangLiteral(value: String, langTag: String) extends RangeLiteral
case class StringLiteral(value: String) extends RangeLiteral
case class BooleanLiteral(value: Boolean) extends Literal
case class LongLiteral(value: Long) extends RangeLiteral
case class DoubleLiteral(value: Double) extends RangeLiteral

object Ligature {
  val a: Predicate = Predicate("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")

  /**
   * Accepts a String representing an identifier and returns true or false depending on if it is valid.
   */
  def validNamedEntity(identifier: String): Boolean =
    "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(identifier)

  /**
   * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
   */
  def validLangTag(langTag: String): Boolean =
    "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)
}

case class Statement(subject: Entity, predicate: Predicate, `object`: Object)
case class PersistedStatement(collection: NamedEntity, statement: Statement, context: AnonymousEntity)

trait Ligature {
  def start(): Observable[LigatureSession]
}

trait LigatureSession {
  def compute: Observable[ReadTx]
  def write: Observable[WriteTx]
}

trait ReadTx {
  /**
   * Returns a Iterable of all existing collections.
   */
  def collections: Observable[NamedEntity]

  /**
   * Returns a Iterable of all existing collections that start with the given prefix.
   */
  def collections(prefix: NamedEntity): Observable[NamedEntity]

  /**
   * Returns a Iterable of all existing collections that are within the given range.
   * `from` is inclusive and `to` is exclusive.
   */
  def collections(from: NamedEntity, to: NamedEntity): Observable[NamedEntity]

  /**
   * Accepts nothing but returns a Iterable of all Statements in the Collection.
   */
  def allStatements(collection: NamedEntity): Observable[PersistedStatement]

  /**
   * Is passed a pattern and returns a seq with all matching Statements.
   */
  def matchStatements(collection: NamedEntity,
                      subject: Option[Entity] = None,
                      predicate: Option[Predicate] = None,
                      `object`: Option[Object] = None): Observable[PersistedStatement]

//  /**
//   * Is passed a pattern and returns a seq with all matching Statements.
//   */
//  def matchStatements(collection: NamedEntity,
//                      subject: Option[Entity],
//                      predicate: Option[Predicate],
//                      range: Range[_, _]): Task[Any, Throwable, Iterable[PersistedStatement]]

  /**
   * Returns the Statement with the given context.
   * Returns None if the context doesn't exist.
   */
  def statementByContext(collection: NamedEntity, context: AnonymousEntity): Task[Option[PersistedStatement]]

  def isOpen: Boolean
}

trait WriteTx {
  /**
   * Creates a collection with the given name or does nothing if the collection already exists.
   * Only useful for creating an empty collection.
   */
  def createCollection(collection: NamedEntity): Task[Try[NamedEntity]]

  /**
   * Deletes the collection of the name given and does nothing if the collection doesn't exist.
   */
  def deleteCollection(collection: NamedEntity): Task[Try[NamedEntity]]

  /**
   * Returns a new, unique to this collection, AnonymousEntity
   */
  def newEntity(collection: NamedEntity): Task[Try[AnonymousEntity]]
  def addStatement(collection: NamedEntity, statement: Statement): Task[Try[PersistedStatement]]
//  Commenting out the below as part of #125
//  def removeStatement(collection: NamedEntity, statement: Statement): Task[Any, Throwable, Try[Statement]]
//  def removeEntity(collection: NamedEntity, entity: Entity): Task[Any, Throwable, Try[Entity]]
//  def removePredicate(collection: NamedEntity, predicate: Predicate): Task[Any, Throwable, Try[Predicate]]

  /**
   * Cancels this transaction.
   */
  def cancel(): Unit

  def isOpen: Boolean
}
