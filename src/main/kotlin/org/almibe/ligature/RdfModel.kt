/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import java.util.*
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicInteger
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

interface ReadOnlyModel {
    fun statementsFor(subject: Subject): Set<Pair<Predicate, Object>>
    fun getSubjects(): Stream<Subject>
}

interface Model: ReadOnlyModel {
    /**
     * Adds the passed in model to this current one.  IRIs are merged and all blank nodes from the new model are
     * given unique names in the current model.  Literals are not merged.
     */
    fun addModel(model: ReadOnlyModel)
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

class InMemoryModel: Model {
    //for now just using a single ConcurrentHashMap for this, later it might be better to create a few different maps
    val statements: ConcurrentHashMap<Subject, MutableSet<Pair<Predicate, Object>>> = ConcurrentHashMap()
    val blankNodeCounter = AtomicInteger()

    /**
     * Adds the contents of the passed in model to this model.  Every blank node from the model that is passed in
     * is given a unique name and no blank node merging is attempted.
     */
    override fun addModel(model: ReadOnlyModel) {
        val blankNodeMap = mutableMapOf<BlankNode, BlankNode>()

        model.getSubjects().forEach { subject ->
            val finalSubject = when (subject) {
                is BlankNode -> createUniqueBlankNode(subject, blankNodeMap)
                else -> subject
            }

            model.statementsFor(subject).forEach {
                val finalObject = when (it.second) {
                    is BlankNode -> createUniqueBlankNode(it.second as BlankNode, blankNodeMap)
                    else -> it.second
                }
                addStatement(finalSubject, it.first, finalObject)
            }
        }
    }

    private fun createUniqueBlankNode(subject: BlankNode, blankNodeMap: MutableMap<BlankNode, BlankNode>): BlankNode {
        if (blankNodeMap.containsKey(subject)) {
            return blankNodeMap[subject]!!
        }
        while (true) {
            val tempBlankNode = BlankNode("${subject.label}_${blankNodeCounter.incrementAndGet()}")
            if (!statements.containsKey(tempBlankNode)) {
                blankNodeMap[subject] = tempBlankNode
                return tempBlankNode
            }
        }
    }

    /**
     * Add a specified statement to the current model.  Blank nodes that are added will use the name that is given.
     */
    override fun addStatement(subject: Subject, predicate: Predicate, `object`: Object) {
        if (statements.containsKey(subject)) {
            statements[subject]!!.add(Pair(predicate, `object`))
        } else {
            val newSet = Collections.newSetFromMap(ConcurrentHashMap<Pair<Predicate, Object>, Boolean>())
            val pair = Pair(predicate, `object`)
            newSet.add(pair)
            //add new value OR if value has been set since last checked add new pair
            statements.putIfAbsent(subject, newSet)?.add(pair)
        }
    }

    override fun statementsFor(subject: Subject): Set<Pair<Predicate, Object>> {
        return statements[subject] ?: setOf()
    }

    override fun addSubject(subject: Subject) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun removeSubject(subject: Subject) {
        statements.remove(subject)
    }

    override fun removeStatement(subject: Subject, predicate: Predicate, `object`: Object) {
        statements[subject]?.remove(Pair(predicate, `object`))
    }

    fun getPredicates(): Set<Predicate> {
        val results = mutableSetOf<Predicate>()
        statements.forEach {
            it.value.forEach { (predicate) ->
                results.add(predicate)
            }
        }
        return results
    }

    override fun getSubjects(): Stream<Subject> {
        return statements.keys.parallelStream()
    }

    fun getObjects(): Set<Object> {
        val results = mutableSetOf<Object>()
        statements.forEach {
            it.value.forEach { (_, `object`) ->
                results.add(`object`)
            }
        }
        return results
    }

    fun getIRIs(): Set<IRI> {
        val results = mutableSetOf<IRI>()
        statements.forEach {
            if (it.key is IRI) {
                results.add(it.key as IRI)
            }
            it.value.forEach { (predicate, `object`) ->
                results.add(predicate as IRI)
                if (`object` is IRI) {
                    results.add(`object`)
                }
            }
        }
        return results
    }

    fun getLiterals(): Set<Literal> {
        val results = mutableSetOf<Literal>()
        statements.forEach {
            it.value.forEach { (_, `object`) ->
                if (`object` is Literal) {
                    results.add(`object`)
                }
            }
        }
        return results
    }
}
