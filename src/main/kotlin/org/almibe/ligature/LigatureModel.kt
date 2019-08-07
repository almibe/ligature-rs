/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import java.io.Closeable
import java.util.stream.Stream

sealed class Element
data class Symbol(val value: String) : Element()
sealed class Literal: Element()

data class LangLiteral(val value: String, val langTag: String) : Literal()
data class LongLiteral(val value: Long) : Literal()
//other literals...

data class Statement(val subject: Element, val predicate: Symbol, val `object`: Element, val graph: Graph = DefaultGraph)

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
