/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import cats.effect.Resource
import monix.eval.Task
import monix.reactive.Observable

sealed trait Object
sealed trait Node extends Object
case class NamedNode(name: String) extends Node
case class AnonymousNode(identifier: Long) extends Node()
sealed trait Literal extends Object
case class LangLiteral(value: String, langTag: String) extends Literal()
case class StringLiteral(value: String) extends Literal()
case class BooleanLiteral(value: Boolean) extends Literal()
case class LongLiteral(value: Long) extends Literal()
case class DoubleLiteral(value: Double) extends Literal()

object Ligature {
  val a: NamedNode = NamedNode("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")

  /**
   * Accepts a String representing an identifier and returns true or false depending on if it is valid.
   */
  def validNamedNode(node: NamedNode): Boolean = {
    "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(node.name)
  }

  /**
   * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
   */
  def validLangTag(langTag: String): Boolean =
    "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)
}

case class Statement(subject: Node, predicate: NamedNode, `object`: Object)
case class PersistedStatement(collection: NamedNode, statement: Statement, context: AnonymousNode)

trait Ligature {
  def session(): Resource[Task, LigatureSession]
}

trait LigatureSession {
  def read(): Resource[Task, ReadTx]
  def write(): Resource[Task, WriteTx]
}

trait ReadTx {
  /**
   * Returns a Iterable of all existing collections.
   */
  def collections(): Observable[NamedNode]

  /**
   * Returns a Iterable of all existing collections that start with the given prefix.
   */
  def collections(prefix: NamedNode): Observable[NamedNode]

  /**
   * Returns a Iterable of all existing collections that are within the given range.
   * `from` is inclusive and `to` is exclusive.
   */
  def collections(from: NamedNode, to: NamedNode): Observable[NamedNode]

  /**
   * Accepts nothing but returns a Iterable of all Statements in the Collection.
   */
  def allStatements(collection: NamedNode): Observable[PersistedStatement]

  /**
   * Is passed a pattern and returns a seq with all matching Statements.
   */
  def matchStatements(collection: NamedNode,
                      subject: Option[Node] = None,
                      predicate: Option[NamedNode] = None,
                      `object`: Option[Object] = None): Observable[PersistedStatement]

  //  /**
  //   * Is passed a pattern and returns a seq with all matching Statements.
  //   */
  //  fun matchStatements(collection: NamedNode,
  //                      subject: Option[Node],
  //                      predicate: Option[Predicate],
  //                      range: ClosedRange[RangeLiteral]): Any, Throwable, Observable[PersistedStatement]

  /**
   * Returns the Statement with the given context.
   * Returns None if the context doesn't exist.
   */
  def statementByContext(collection: NamedNode, context: AnonymousNode): Task[Option[PersistedStatement]]
}

trait WriteTx {
  /**
   * Creates a collection with the given name or does nothing if the collection already exists.
   * Only useful for creating an empty collection.
   */
  def createCollection(collection: NamedNode): Task[NamedNode]

  /**
   * Deletes the collection of the name given and does nothing if the collection doesn't exist.
   */
  def deleteCollection(collection: NamedNode): Task[NamedNode]

  /**
   * Returns a new, unique to this collection, AnonymousNode
   */
  def newNode(collection: NamedNode): Task[AnonymousNode]

  def addStatement(collection: NamedNode, statement: Statement): Task[PersistedStatement]

  //  Commenting out the below as part of #125
  //  fun removeStatement(collection: NamedNode, statement: Statement): Any, Throwable, Statement>>
  //  fun removeNode(collection: NamedNode, node: Node): Any, Throwable, Node>>
  //  fun removePredicate(collection: NamedNode, predicate: Predicate): Any, Throwable, Predicate>>

  /**
   * Cancels this transaction.
   */
  def cancel(): Unit
}
