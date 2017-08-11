/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import java.util.*
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicInteger

interface Subject
interface Predicate
interface Object

data class IRI(val value: String) : Subject, Predicate, Object
data class BlankNode(val label: String) : Subject, Object

interface Literal : Object { val value: String}
data class LangLiteral(override val value: String, val langTag: String) : Literal
data class TypedLiteral(override val value: String,
                        val datatypeIRI: IRI = IRI("http://www.w3.org/2001/XMLSchema#string")) : Literal

//TODO there should probably be a read only Model
interface Model {
    fun addModel(model: Model)
    fun addStatement(subject: Subject, predicate: Predicate, `object`: Object)
    fun statementsFor(subject: Subject): Set<Pair<Predicate, Object>>
    fun getPredicates(): Set<Predicate>
    fun getSubjects(): Set<Subject>
    fun getObjects(): Set<Object>
    fun getIRIs(): Set<IRI>
    fun getLiterals(): Set<Literal>
}

class InMemoryModel: Model {
    //for now just using a single ConcurrentHashMap for this, later it might be better to create a few different maps
    val statements: ConcurrentHashMap<Subject, MutableSet<Pair<Predicate, Object>>> = ConcurrentHashMap()
    val blankNodeCounter = AtomicInteger()

    /**
     * Adds the contents of the passed in model to this model.  Blank nodes from the model that is passed in
     * are given unique names and no blank node merging is attempted.
     */
    override fun addModel(model: Model) {
        TODO()
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

    override fun getPredicates(): Set<Predicate> {
        val results = mutableSetOf<Predicate>()
        statements.forEach {
            it.value.forEach { (predicate) ->
                results.add(predicate)
            }
        }
        return results
    }

    override fun getSubjects(): Set<Subject> {
        return statements.keys
    }

    override fun getObjects(): Set<Object> {
        val results = mutableSetOf<Object>()
        statements.forEach {
            it.value.forEach { (_, `object`) ->
                results.add(`object`)
            }
        }
        return results
    }

    override fun getIRIs(): Set<IRI> {
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

    override fun getLiterals(): Set<Literal> {
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
