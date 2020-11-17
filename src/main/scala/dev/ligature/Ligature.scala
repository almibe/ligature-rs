/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import cats.effect.{IO, Resource}
import fs2.Stream

final case class Dataset(name: String)

sealed trait Object
final case class Subject(identifier: Long) extends Object
final case class Predicate(identifier: String)

sealed trait Literal extends Object
final case class LangLiteral(value: String, langTag: String) extends Literal
final case class StringLiteral(value: String) extends Literal
final case class BooleanLiteral(value: Boolean) extends Literal
final case class LongLiteral(value: Long) extends Literal
final case class DoubleLiteral(value: Double) extends Literal

sealed trait Range
final case class LangLiteralRange(start: LangLiteral, stop: LangLiteral) extends Range
final case class StringLiteralRange(start: StringLiteral, stop: StringLiteral) extends Range
final case class LongLiteralRange(start: LongLiteral, stop: LongLiteral) extends Range
final case class DoubleLiteralRange(start: DoubleLiteral, stop: DoubleLiteral) extends Range

final case class Statement(subject: Subject, predicate: Predicate, `object`: Object)
final case class PersistedStatement(dataset: Dataset, statement: Statement, context: Subject)

object Ligature {
  def validDataset(dataset: Dataset): Boolean = {
    "[a-z_]+(/[a-z_]+)*".r.matches(dataset.name)
  }
  def validPredicate(predicate: Predicate): Boolean = {
    "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(predicate.identifier)
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
                      subject: Option[Subject] = None,
                      predicate: Option[Predicate] = None,
                      `object`: Option[Object] = None): Stream[IO, PersistedStatement]
  def matchStatements(dataset: Dataset,
                      subject: Option[Subject],
                      predicate: Option[Predicate],
                      range: Range): Stream[IO, PersistedStatement]
  def statementByContext(dataset: Dataset, context: Subject): IO[Option[PersistedStatement]]
}

trait LigatureWriteTx {
  def createDataset(dataset: Dataset): IO[Dataset]
  def deleteDataset(dataset: Dataset): IO[Dataset]
  def newSubject(dataset: Dataset): IO[Subject]
  def addStatement(dataset: Dataset, statement: Statement): IO[PersistedStatement]
  def removeStatement(dataset: Dataset, statement: Statement): IO[Statement]
  def cancel(): Unit
}
