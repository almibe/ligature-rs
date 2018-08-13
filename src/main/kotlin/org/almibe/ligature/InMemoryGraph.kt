/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import java.util.*
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicInteger
import java.util.concurrent.locks.ReentrantReadWriteLock
import kotlin.collections.HashMap
import kotlin.concurrent.read
import kotlin.concurrent.write

class InMemoryGraph: Graph {
    //TODO replace ConcurrentHashMap with Lock/sync and multiple collections
    private val lock = ReentrantReadWriteLock()
    private val statements: MutableMap<Subject, MutableSet<Pair<Predicate, Object>>> = HashMap()
    private val blankNodeCounter = AtomicInteger()

    /**
     * Adds the contents of the passed in model to this model.  Every blank node from the model that is passed in
     * is given a unique name and no blank node merging is attempted.
     */
    override fun addModel(graph: Graph) {
        lock.write {
            val blankNodeMap = mutableMapOf<BlankNode, BlankNode>()

            graph.getSubjects().forEach { subject ->
                val finalSubject = when (subject) {
                    is BlankNode -> createUniqueBlankNode(subject, blankNodeMap)
                    else -> subject
                }

                graph.statementsFor(subject).forEach {
                    val finalObject = when (it.second) {
                        is BlankNode -> createUniqueBlankNode(it.second as BlankNode, blankNodeMap)
                        else -> it.second
                    }
                    addStatement(finalSubject, it.first, finalObject)
                }
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
        lock.write {
            if (statements.containsKey(subject)) {
                statements[subject]!!.add(Pair(predicate, `object`))
            } else {
                val newSet = Collections.newSetFromMap(ConcurrentHashMap<Pair<Predicate, Object>, Boolean>())
                val pair = Pair(predicate, `object`)
                newSet.add(pair)
                //add new value OR if value has been set since last checked add new pair
                statements.putIfAbsent(subject, newSet)?.add(pair)
            }
            //make sure that object is persisted as a subject if it is one
            if (`object` is Subject) {
                addSubject(`object`)
            }
        }
    }

    override fun statementsFor(subject: Subject): Set<Pair<Predicate, Object>> {
        return lock.read { statements[subject] ?: setOf() }
    }

    override fun addSubject(subject: Subject) {
        lock.write {
            val newSet = Collections.newSetFromMap(ConcurrentHashMap<Pair<Predicate, Object>, Boolean>())
            statements.putIfAbsent(subject, newSet)
        }
    }

    override fun removeSubject(subject: Subject) {
        lock.write {
            statements.remove(subject)
            //remove all statements that have this subject as the object
            statements.keys.forEach { currentSubject ->
                statements[currentSubject]?.forEach { statement ->
                    if (statement.second == subject) {
                        statements[currentSubject]?.remove(statement)
                    }
                }
            }
        }
    }

    override fun removeStatement(subject: Subject, predicate: Predicate, `object`: Object) {
        lock.write {
            statements[subject]?.remove(Pair(predicate, `object`))
        }
    }

    fun getPredicates(): Set<Predicate> {
        val results = mutableSetOf<Predicate>()
        lock.read {
            statements.forEach {
                it.value.forEach { (predicate) ->
                    results.add(predicate)
                }
            }
        }
        return results
    }

    override fun getSubjects(): Set<Subject> {
        return lock.read { statements.keys }
    }

    fun getObjects(): Set<Object> {
        val results = mutableSetOf<Object>()
        lock.read {
            statements.forEach {
                it.value.forEach { (_, `object`) ->
                    results.add(`object`)
                }
            }
        }
        return results
    }

    fun getIRIs(): Set<IRI> {
        val results = mutableSetOf<IRI>()
        lock.read {
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
        }
        return results
    }

    fun getLiterals(): Set<Literal> {
        val results = mutableSetOf<Literal>()
        lock.read {
            statements.forEach {
                it.value.forEach { (_, `object`) ->
                    if (`object` is Literal) {
                        results.add(`object`)
                    }
                }
            }
        }
        return results
    }
}
