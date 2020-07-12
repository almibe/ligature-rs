/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import scala.util.Try
import cats.effect.{IO, Resource}

sealed trait Object
sealed trait Entity extends Object
case class NamedEntity(identifier: String) extends Entity
case class AnonymousEntity(identifier: Long) extends Entity
case class Predicate(identifier: String)
sealed trait Literal extends Object
case class LangLiteral(value: String, langTag: String) extends Literal
case class StringLiteral(value: String) extends Literal
case class BooleanLiteral(value: Boolean) extends Literal
case class LongLiteral(value: Long) extends Literal
case class DoubleLiteral(value: Double) extends Literal

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

sealed class Range[T](val start: T, val end: T)
case class LangLiteralRange(override val start: LangLiteral, override val end: LangLiteral) extends Range[LangLiteral](start, end)
case class StringLiteralRange(override val start: String, override val end: String) extends Range[String](start, end)
case class LongLiteralRange(override val start: Long, override val end: Long) extends Range[Long](start, end)
case class DoubleLiteralRange(override val start: Double, override val end: Double) extends Range[Double](start, end)

trait LigatureStore {
  def compute(): Resource[IO, ReadTx]
  def write(): Resource[IO, WriteTx]

  /**
  * Close connection with the Store.
  */
  def close(): Unit

  def isOpen: Boolean
}

trait ReadTx {
  /**
   * Returns a Iterable of all existing collections.
   */
  def collections(): Iterable[NamedEntity]

  /**
   * Returns a Iterable of all existing collections that start with the given prefix.
   */
  def collections(prefix: NamedEntity): Iterable[NamedEntity]

  /**
   * Returns a Iterable of all existing collections that are within the given range.
   * `from` is inclusive and `to` is exclusive.
   */
  def collections(from: NamedEntity, to: NamedEntity): Iterable[NamedEntity]

  /**
   * Accepts nothing but returns a Iterable of all Statements in the Collection.
   */
  def allStatements(collection: NamedEntity): Iterable[PersistedStatement]

  /**
   * Is passed a pattern and returns a seq with all matching Statements.
   */
  def matchStatements(collection: NamedEntity,
                      subject: Option[Entity] = None,
                      predicate: Option[Predicate] = None,
                      `object`: Option[Object] = None): Iterable[PersistedStatement]

  /**
   * Is passed a pattern and returns a seq with all matching Statements.
   */
  def matchStatements(collection: NamedEntity,
                      subject: Option[Entity],
                      predicate: Option[Predicate],
                      range: Range[_]): Iterable[PersistedStatement]

  /**
   * Returns the Statement with the given context.
   * Returns None if the context doesn't exist.
   */
  def statementByContext(collection: NamedEntity, context: AnonymousEntity): Option[PersistedStatement]

  /**
   * Cancels this transaction.
   */
  def cancel(): Unit

  def isOpen: Boolean
}

trait WriteTx {
  /**
   * Creates a collection with the given name or does nothing if the collection already exists.
   * Only useful for creating an empty collection.
   */
  def createCollection(collection: NamedEntity): Try[NamedEntity]

  /**
   * Deletes the collection of the name given and does nothing if the collection doesn't exist.
   */
  def deleteCollection(collection: NamedEntity): Try[NamedEntity]

  /**
   * Returns a new, unique to this collection, AnonymousEntity
   */
  def newEntity(collection: NamedEntity): Try[AnonymousEntity]
  def addStatement(collection: NamedEntity, statement: Statement): Try[PersistedStatement]
  def removeStatement(collection: NamedEntity, statement: Statement): Try[Statement]
  def removeEntity(collection: NamedEntity, entity: Entity): Try[Entity]
  def removePredicate(collection: NamedEntity, predicate: Predicate): Try[Predicate]

  /**
   * Commits this transaction.
   */
  def commit(): Try[Unit]

  /**
   * Cancels this transaction.
   */
  def cancel(): Unit

  def isOpen: Boolean
}
