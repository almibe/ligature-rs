/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import java.io.Closeable
import java.math.BigDecimal
import java.util.*
import java.util.stream.Stream

sealed class Element
data class Symbol(val value: String) : Element()
sealed class Literal: Element()

data class LangLiteral(val value: String, val langTag: String) : Literal()
data class StringLiteral(val value: String) : Literal()
data class BooleanLiteral(val value: Boolean): Literal()
data class LongLiteral(val value: Long) : Literal()
data class DecimalLiteral(val value: BigDecimal) : Literal()

sealed class Graph: Element()
object DefaultGraph: Graph()
data class NamedGraph(val symbol: Symbol): Graph()
class AnonymousGraph: Graph() {
    private val uuid = UUID.randomUUID()

    override fun equals(other: Any?): Boolean {
        return when (other) {
            is AnonymousGraph -> return other.uuid == uuid
            else -> false
        }
    }

    override fun hashCode(): Int {
        return uuid.hashCode()
    }
}

data class Statement(val subject: Element,
                     val predicate: Symbol,
                     val `object`: Element,
                     val graph: Graph = DefaultGraph)

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
