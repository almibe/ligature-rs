/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

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
type Predicate = IRI | LocalName
type Graph = IRI | LocalName | BlankNode | DefaultGraph.type
type Object = Subject | Literal

final case class Statement(subject: Subject, predicate: Predicate, `object`: Object)
final case class PersistedStatement(dataset: LocalName, statement: Statement, graph: Graph)

object Ligature {
  val a: IRI = IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").getOrElse(???)
  def validLangTag(langTag: String): Boolean =
    "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)
}

trait Ligature {
   def datasets(): Observable[LocalName]
   def datasets(prefix: String): Observable[LocalName]
   def datasets(from: String, to: String): Observable[LocalName]
   def createDataset(dataset: LocalName): Task[LocalName]
   def deleteDataset(dataset: LocalName): Task[LocalName]
   def query(): QueryTx
   def addStatements(dataset: LocalName, statements: Iterator<Statement>)
   def removeStatements(dataset: LocalName, statements: Iterator<Statement>)
}

trait QueryTx {
  def allStatements(dataset: LocalName): Observable[PersistedStatement]
  def matchStatements(dataset: LocalName,
                      subject: Option[Subject] = None,
                      predicate: Option[Predicate] = None,
                      `object`: Option[Object] = None,
                      graph: Option[Graph] = None): Observable[PersistedStatement]
  def matchStatements(dataset: LocalName,
                      subject: Option[Subject],
                      predicate: Option[Predicate],
                      graph: Option[Graph],
                      range: Range): Observable[PersistedStatement]
}

trait WriteTx {
  def newNode(dataset: LocalName): Task[BlankNode]
  def addStatement(dataset: LocalName, statement: Statement, graph: Graph = DefaultGraph): Task[PersistedStatement]
  def removeStatement(dataset: LocalName, statement: Statement, graph: Graph = DefaultGraph): Task[Statement]
  def cancel(): Unit
}
