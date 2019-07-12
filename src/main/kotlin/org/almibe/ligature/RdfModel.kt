/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import java.io.Closeable
import java.util.stream.Stream

interface Subject
interface Predicate
interface Object

data class IRI(val value: String) : Subject, Predicate, Object
data class BlankNode(val label: String) : Subject, Object

sealed class Literal : Object
data class LangLiteral(val value: String, val langTag: String) : Literal()
data class TypedLiteral(val value: String,
                        val datatypeIRI: IRI = IRI("http://www.w3.org/2001/XMLSchema#string")) : Literal()

data class Quad(val subject: Subject, val predicate: Predicate, val `object`: Object, val graph: Graph = DefaultGraph)

sealed class Graph
object DefaultGraph: Graph()
data class NamedGraph(val iri: IRI): Graph()

data class SparqlResultField(val name: String, val value: Object)

interface Store: Closeable {
    fun getDatasetNames(): Stream<String>
    fun getDataset(name: String): Dataset
    fun deleteDataset(name: String)
    override fun close()
}

interface Dataset {
    fun getDatasetName(): String
    fun executeSparql(sparql: String): Stream<List<SparqlResultField>>
    fun addStatements(statements: Collection<Quad>)
    fun removeStatements(statements: Collection<Quad>)
    fun findAll(subject: Subject? = null, predicate: Predicate? = null,
                `object`: Object? = null, graph: Graph = DefaultGraph): Stream<Quad>
}
