/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

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

data class Quad(val subject: Subject, val predicate: Predicate, val `object`: Object, val graph: IRI? = null)

interface Store {
    fun getDataSetNames(): Stream<String>
    fun getDataSet(name: String): DataSet
    fun deleteDataSet(name: String)
}

interface DataSet {
    fun getDataSetName(): String
    fun execute(sparql: SparqlCommand): SparqlResult
    fun addStatements(statements: Collection<Quad>)
    fun removeStatements(statements: Collection<Quad>)
    fun findAll(subject: Subject? = null, predicate: Predicate? = null,
                `object`: Object? = null, graph: IRI? = null): Stream<Quad>
}
