/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

data class Dataset(val name: String)

sealed class Object
sealed class Node: Object()
data class NamedNode(val name: String): Node()
data class AnonymousNode(val identifier: Long): Node()

sealed class Literal: Object()
data class LangLiteral(val value: String, val langTag: String): Literal()
data class StringLiteral(val value: String): Literal()
data class BooleanLiteral(val value: Boolean): Literal()
data class LongLiteral(val value: Long): Literal()
data class DoubleLiteral(val value: Double): Literal()

sealed class Range
data class LangLiteralRange(val start: LangLiteral, val stop: LangLiteral): Range()
data class StringLiteralRange(val start: StringLiteral, val stop: StringLiteral): Range()
data class LongLiteralRange(val start: LongLiteral, val stop: LongLiteral): Range()
data class DoubleLiteralRange(val start: DoubleLiteral, val stop: DoubleLiteral): Range()

data class Statement(val subject: Node, val predicate: NamedNode, val `object`: Object)
data class PersistedStatement(val dataset: Dataset, val statement: Statement, val context: AnonymousNode)

val a: NamedNode = NamedNode("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
fun validDataset(dataset: Dataset): Boolean =
  "[a-z_]+(/[a-z_]+)*".r.matches(dataset.name)

fun validNamedNode(node: NamedNode): Boolean =
  "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(node.name)

fun validLangTag(langTag: String): Boolean =
  "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)

interface Ligature {
  fun instance: Resource[IO, LigatureInstance]
}

interface LigatureInstance {
  fun read: Resource[IO, LigatureReadTx]
  fun write: Resource[IO, LigatureWriteTx]
}

interface LigatureReadTx {
  fun datasets: Stream[IO, Dataset]
  fun datasets(prefix: Dataset): Stream[IO, Dataset]
  fun datasets(from: Dataset, to: Dataset): Stream[IO, Dataset]
  fun allStatements(dataset: Dataset): Stream[IO, PersistedStatement]
  fun matchStatements(dataset: Dataset,
                      subject: Option[Node] = None,
                      predicate: Option[NamedNode] = None,
                      `object`: Option[Object] = None): Stream[IO, PersistedStatement]
  fun matchStatements(dataset: Dataset,
                      subject: Option[Node],
                      predicate: Option[NamedNode],
                      range: Range): Stream[IO, PersistedStatement]
  fun statementByContext(dataset: Dataset, context: AnonymousNode): IO[Option[PersistedStatement]]
}

interface LigatureWriteTx {
  fun createDataset(dataset: Dataset): IO[Dataset]
  fun deleteDataset(dataset: Dataset): IO[Dataset]
  fun newNode(dataset: Dataset): IO[AnonymousNode]
  fun addStatement(dataset: Dataset, statement: Statement): IO[PersistedStatement]
  fun removeStatement(dataset: Dataset, statement: Statement): IO[Statement]
  fun cancel(): Unit
}
