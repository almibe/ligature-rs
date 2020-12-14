/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import cats.effect.Resource
import monix.eval.Task
import monix.reactive.Observable
import dev.ligature.iris.IRI

final case class LocalName(name: String)
final case class BlankNode(identifier: Long)
object DefaultGraph

sealed trait Literal
final case class LangLiteral(value: String, langTag: String) extends Literal
final case class StringLiteral(value: String) extends Literal
final case class BooleanLiteral(value: Boolean) extends Literal
final case class LongLiteral(value: Long) extends Literal
final case class DoubleLiteral(value: Double) extends Literal
final case class UnknownLiteral(value: String, `type`: IRI) extends Literal

sealed trait Range
final case class LangLiteralRange(start: LangLiteral, stop: LangLiteral) extends Range
final case class StringLiteralRange(start: StringLiteral, stop: StringLiteral) extends Range
final case class LongLiteralRange(start: LongLiteral, stop: LongLiteral) extends Range
final case class DoubleLiteralRange(start: DoubleLiteral, stop: DoubleLiteral) extends Range

type Subject = IRI | LocalName | BlankNode | DefaultGraph.type
type Graph = IRI | LocalName | BlankNode | DefaultGraph.type
type Object = Subject | Literal

final case class Statement(subject: Subject, predicate: IRI, `object`: Object)
final case class PersistedStatement(dataset: IRI, statement: Statement, graph: Graph)

object Ligature {
  val a: IRI = IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").getOrElse(???)
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
  def datasets: Observable[IRI]
  def datasets(prefix: String): Observable[IRI]
  def datasets(from: String, to: String): Observable[IRI]
  def allStatements(dataset: IRI): Observable[PersistedStatement]
  def matchStatements(dataset: IRI,
                      subject: Option[Subject] = None,
                      predicate: Option[IRI] = None,
                      `object`: Option[Object] = None,
                      graph: Option[Graph] = None): Observable[PersistedStatement]
  def matchStatements(dataset: IRI,
                      subject: Option[Subject],
                      predicate: Option[IRI],
                      graph: Option[Graph],
                      range: Range): Observable[PersistedStatement]
  def statementByContext(dataset: IRI, context: BlankNode): Task[Option[PersistedStatement]]
}

trait LigatureWriteTx {
  def createDataset(dataset: IRI): Task[IRI]
  def deleteDataset(dataset: IRI): Task[IRI]
  def newNode(dataset: IRI): Task[BlankNode]
  def addStatement(dataset: IRI, statement: Statement, graph: Graph = DefaultGraph): Task[PersistedStatement]
  def removeStatement(dataset: IRI, statement: Statement, graph: Graph = DefaultGraph): Task[Statement]
  def cancel(): Unit
}
