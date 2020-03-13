/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.ligature

import kotlinx.coroutines.flow.Flow

sealed class Node
data class Entity(val identifier: String): Node()
sealed class Literal: Node()
data class LangLiteral(val value: String, val langTag: String): Literal()
data class StringLiteral(val value: String): Literal()
data class BooleanLiteral(val value: Boolean): Literal()
data class LongLiteral(val value: Long): Literal()
data class DoubleLiteral(val value: Double): Literal()
data class ListLiteral(val value: List<Literal>): Literal()
data class BagLiteral(val value: List<Literal>): Literal()
data class AltLiteral(val value: List<Literal>): Literal()

data class Statement(val subject: Node, val predicate: Predicate, val `object`: Node, val graph: Entity)

data class Rule(val subject: Node, val predicate: Predicate, val `object`: Node)

data class Range(val start: Literal, val end: Literal)

data class Predicate(val name: Entity)

interface LigatureStore {
    /**
     * Returns a collection based on the name passed.
     * Calling this function will not create a new collection, it just binds a Store and Collection name.
     */
    fun collection(collectionName: String): LigatureCollection

    /**
     * Creates a new collection or does nothing if collection already exists.
     * Regardless the collection is returned.
     */
    fun createCollection(collectionName: String): LigatureCollection

    /**
     * Deletes the collection of the name given and does nothing if the collection doesn't exist.
     */
    fun deleteCollection(collectionName: String)

    /**
     * Returns a Flow of all existing collections.
     */
    fun allCollections(): Flow<LigatureCollection>

    /**
     * Close connection with the Store.
     */
    fun close()

    /**
     * Returns an implementation specific map of details about this Store useful for debugging.
     */
    fun details(): Map<String, String>
}

/**
 * Manages a collection of Statements and Rules, supports ontologies, and querying.
 */
interface LigatureCollection {
    val collectionName: String
    fun readTx(): ReadTx
    fun writeTx(): WriteTx
}

interface ReadTx {
    /**
     * Accepts nothing but returns a Flow of all Statements in the Collection.
     */
    fun allStatements(): Flow<Statement>

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fun matchStatements(subject: Node? = null, predicate: Predicate? = null, `object`: Node? = null, graph: Entity? = null): Flow<Statement>

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fun matchStatements(subject: Node? = null, predicate: Predicate? = null, `object`: Range, graph: Entity? = null): Flow<Statement>

    /**
     * Accepts nothing but returns a seq of all Rules in the Collection.
     */
    fun allRules(): Flow<Rule>

    /**
     * Is passed a pattern and returns a seq with all matching rules.
     */
    fun matchRules(subject: Entity?, predicate: Predicate?, `object`: Node?): Flow<Rule>

    /**
     * Cancels this transaction.
     */
    fun cancel()
}

interface WriteTx: ReadTx {
    /**
     * Returns a new, unique to this collection identifier in the form _:NUMBER"
     */
    fun newIdentifier(): Entity
    fun addStatement(statement: Statement)
    fun removeStatement(statement: Statement)
    fun addRule(rule: Rule)
    fun removeRule(rule: Rule)
    fun commit()
}

/**
 * Accepts a String representing an identifier and returns true or false depending on if it is valid.
 */
fun validIdentifier(identifier: String): Boolean {
    return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
}

/**
 * Accepts a String representing a lang tag and returns true or false depending on if it is valid.
 */
fun validLangTag(langTag: String): Boolean {
    return "[a-zA-Z]+(-[a-zA-Z0-9]+)*".toRegex().matches(langTag)
}
