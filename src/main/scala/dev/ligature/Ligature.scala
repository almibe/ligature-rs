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
case class AnonymousNode(identifier: Long) extends Node

sealed trait Literal extends Object
case class LangLiteral(value: String, langTag: String) extends Literal
case class StringLiteral(value: String) extends Literal
case class BooleanLiteral(value: Boolean) extends Literal
case class LongLiteral(value: Long) extends Literal
case class DoubleLiteral(value: Double) extends Literal
case class UnknownLiteral(value: String, `type`: NamedNode) extends Literal

sealed trait Range
case class LangLiteralRange(start: LangLiteral, stop: LangLiteral) extends Range
case class StringLiteralRange(start: StringLiteral, stop: StringLiteral) extends Range
case class LongLiteralRange(start: LongLiteral, stop: LongLiteral) extends Range
case class DoubleLiteralRange(start: DoubleLiteral, stop: DoubleLiteral) extends Range

case class Statement(subject: Node, predicate: NamedNode, `object`: Object)
case class PersistedStatement(dataset: NamedNode, statement: Statement, context: AnonymousNode)

object Ligature {
  val a: NamedNode = NamedNode("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
  def validNamedNode(node: NamedNode): Boolean = {
    "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(node.name)
  }
  def validLangTag(langTag: String): Boolean =
    "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)
}

trait Ligature {
  def instance: Resource[Task, LigatureInstance]
}

trait LigatureInstance {
  def read: Resource[Task, LigatureReadTx]
  def write: Resource[Task, LigatureWriteTx]
}

trait LigatureReadTx {
  def datasets: Observable[NamedNode]
  def datasets(prefix: NamedNode): Observable[NamedNode]
  def datasets(from: NamedNode, to: NamedNode): Observable[NamedNode]
  def allStatements(dataset: NamedNode): Observable[PersistedStatement]
  def matchStatements(dataset: NamedNode,
                      subject: Option[Node] = None,
                      predicate: Option[NamedNode] = None,
                      `object`: Option[Object] = None): Observable[PersistedStatement]
  def matchStatements(dataset: NamedNode,
                      subject: Option[Node],
                      predicate: Option[NamedNode],
                      range: Range): Observable[PersistedStatement]
  def statementByContext(dataset: NamedNode, context: AnonymousNode): Task[Option[PersistedStatement]]
}

trait LigatureWriteTx {
  def createDataset(dataset: NamedNode): Task[NamedNode]
  def deleteDataset(dataset: NamedNode): Task[NamedNode]
  def newNode(dataset: NamedNode): Task[AnonymousNode]
  def addStatement(dataset: NamedNode, statement: Statement): Task[PersistedStatement]
  def removeStatement(dataset: NamedNode, statement: Statement): Task[Statement]
  def cancel(): Unit
}
