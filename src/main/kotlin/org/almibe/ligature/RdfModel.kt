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

interface Literal : Object { val value: String}
data class LangLiteral(override val value: String, val langTag: String) : Literal
data class TypedLiteral(override val value: String,
                        val datatypeIRI: IRI = IRI("http://www.w3.org/2001/XMLSchema#string")) : Literal

interface Store {
    fun execute(sparql: String) //TODO return queryResult or JSON or something else?
    fun defaultGraph(): Graph
    fun namedGraph(name: String): Graph?
}

interface Graph {
    fun statementsFor(subject: Subject): Stream<Pair<Predicate, Object>>
    fun getSubjects(): Stream<Subject>

    /**
     * Adds the passed in model to this current one.  IRIs are merged and all blank nodes from the new model are
     * given unique names in the current model.  Literals are not merged.
     */
    fun addModel(graph: Graph)
    /**
     * Add a given statement to this model.  The subject will be added if it doesn't already exist, a new predicate
     * is always added to this subject, and the object is always created if it is a Literal otherwise a new object
     * is only created if it doesn't already exist.
     */
    fun addStatement(subject: Subject, predicate: Predicate, `object`: Object)
    /**
     * Remove a given statement from this model.  The subject is not removed, the predicate always is removed,
     * and the object is removed only if it is a Literal.
     */
    fun removeStatement(subject: Subject, predicate: Predicate, `object`: Object)
    /**
     * Remove a subject from this model.  All predicates related to this object are removed and all objects of those
     * statements are removed if they are Literals, BlankNodes and IRIs are left in the model.  This method does
     * nothing if the subject doesn't exist.
     */
    fun removeSubject(subject: Subject)
    /**
     * Adds a new subject to this model or does nothing if this subject already exists.
     */
    fun addSubject(subject: Subject)
}
