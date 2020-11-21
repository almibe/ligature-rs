/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import cats.effect.{IO, Resource}
import fs2.Stream

final case class Dataset(name: String)

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

sealed trait Range
case class LangLiteralRange(start: LangLiteral, stop: LangLiteral) extends Range
case class StringLiteralRange(start: StringLiteral, stop: StringLiteral) extends Range
case class LongLiteralRange(start: LongLiteral, stop: LongLiteral) extends Range
case class DoubleLiteralRange(start: DoubleLiteral, stop: DoubleLiteral) extends Range

case class Statement(subject: Node, predicate: NamedNode, `object`: Object)
case class PersistedStatement(dataset: Dataset, statement: Statement, context: AnonymousNode)

object Ligature {
  val a: NamedNode = NamedNode("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
  def validDataset(dataset: Dataset): Boolean = {
    "[a-z_]+(/[a-z_]+)*".r.matches(dataset.name)
  }
  def validNamedNode(node: NamedNode): Boolean = {
    "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(node.name)
  }
  def validLangTag(langTag: String): Boolean =
    "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)
}

trait Ligature {
  def instance: Resource[IO, LigatureInstance]
}

trait LigatureInstance {
  def read: Resource[IO, LigatureReadTx]
  def write: Resource[IO, LigatureWriteTx]
}

trait LigatureReadTx {
  def datasets: Stream[IO, Dataset]
  def datasets(prefix: Dataset): Stream[IO, Dataset]
  def datasets(from: Dataset, to: Dataset): Stream[IO, Dataset]
  def allStatements(dataset: Dataset): Stream[IO, PersistedStatement]
  def matchStatements(dataset: Dataset,
                      subject: Option[Node] = None,
                      predicate: Option[NamedNode] = None,
                      `object`: Option[Object] = None): Stream[IO, PersistedStatement]
  def matchStatements(dataset: Dataset,
                      subject: Option[Node],
                      predicate: Option[NamedNode],
                      range: Range): Stream[IO, PersistedStatement]
  def statementByContext(dataset: Dataset, context: AnonymousNode): IO[Option[PersistedStatement]]
}

trait LigatureWriteTx {
  def createDataset(dataset: Dataset): IO[Dataset]
  def deleteDataset(dataset: Dataset): IO[Dataset]
  def newNode(dataset: Dataset): IO[AnonymousNode]
  def addStatement(dataset: Dataset, statement: Statement): IO[PersistedStatement]
  def removeStatement(dataset: Dataset, statement: Statement): IO[Statement]
  def cancel(): Unit
}
