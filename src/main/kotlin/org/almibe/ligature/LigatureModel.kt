/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import java.io.Closeable
import java.util.stream.Stream

interface Subject
interface Predicate
interface Object

data class Symbol(val value: String) : Subject, Predicate, Object

enum class DataType

sealed class Literal : Object
data class LangLiteral(val value: String, val langTag: String) : Literal()
data class TypedLiteral(val value: String,
                        val datatype: DataType) : Literal()

data class Statement(val subject: Subject, val predicate: Predicate, val `object`: Object, val graph: Graph = DefaultGraph)

sealed class Graph
object DefaultGraph: Graph()
data class NamedGraph(val symbol: Symbol): Graph()

interface Store: Closeable {
    fun getDatasetNames(): Stream<String>
    fun getDataset(name: String): Dataset
    fun deleteDataset(name: String)
    override fun close()
}

interface Dataset {
    fun getDatasetName(): String
    fun addStatements(statements: Collection<Statement>)
    fun removeStatements(statements: Collection<Statement>)
    fun allStatements(): Stream<Statement>
}
