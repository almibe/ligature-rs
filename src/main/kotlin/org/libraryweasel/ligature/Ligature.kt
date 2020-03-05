/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.ligature

import java.util.stream.Stream

const val a = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
sealed class Object
data class Identifier(val identifier: String): Object()
sealed class Literal: Object()
data class PlainLiteral(val value: String, val langTag: String): Literal()
data class TypedLiteral(val value: String, val type: Identifier): Literal()
data class Statement(val subject: Identifier, val predicate: Identifier, val `object`: Object, val graph: Identifier)
data class Rule(val subject: Identifier, val predicate: Identifier, val `object`: Object)

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
     * Returns a Stream of all existing collections.
     */
    fun allCollections(): Stream<LigatureCollection>

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
     * Accepts nothing but returns a Stream of all Statements in the Collection.
     */
    fun allStatements(): Stream<Statement>

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fun matchStatements(subject: Identifier?, predicate: Identifier?, `object`: Object?, graph: Identifier?): Stream<Statement>

    /**
     * Accepts nothing but returns a seq of all Rules in the Collection.
     */
    fun allRules(): Stream<Rule>

    /**
     * Is passed a pattern and returns a seq with all matching rules.
     */
    fun matchRules(subject: Identifier?, predicate: Identifier?, `object`: Object?): Stream<Rule>

    /**
     * Cancels this transaction.
     */
    fun cancel()

    /**
     * Runs a SPARQL query in this transaction.
     * If a write is attempted in a read-only transaction and error will occur.
     * TODO shouldn't return Any?
     */
    fun sparqlQuery(query: String): Any?

    /**
     * Runs a Wander query in this transaction.
     * If a write is attempted in a read-only transaction and error will occur.
     * TODO shouldn't return Any?
     */
    fun wanderQuery(): Any?
}

interface WriteTx: ReadTx {
    /**
     * Returns a new, unique to this collection identifier in the form _:NUMBER"
     */
    fun newIdentifier(): Identifier
    fun addStatement(statement: Statement)
    fun removeStatement(statement: Statement)
    fun addRule(rule: Rule)
    fun removeRule(rule: Rule)
    fun commit()
}

(defn identifier?
"Accepts a String representing an identifier and returns true or false depending on if it is valid."
[identifier]
(and
(string? identifier)
(not (nil?(re-matches #"[a-zA-Z_][^\s\(\)\[\]\{\}\'\"`<>\\]*" identifier)))))

(defn lang-tag?
"Accepts a String representing a lang tag and returns true or false depending on if it is valid."
[lang]
(and
(string? lang)
(not (nil?(re-matches #"[a-zA-Z]+(-[a-zA-Z0-9]+)*" lang)))))

(defn plain-literal?
"Accepts a Map and returns true or false depending on if it is a valid lang literal.
A lang literal should contain a :value key with a valid string literal and a :lang key with a valid lang code."
[literal]
(and
(map? literal)
(= (set (keys literal)) #{:lang :value})
(lang-tag? (:lang literal))
(string? (:value literal))))

(defn typed-literal?
"Accepts a Map and returns true or false depending on if it is a valid typed literal.
A typed literal should contain a :valud key with a valid string literal and a :type key with a valid identifier."
[literal]
(and
(map? literal)
(= (set (keys literal)) #{:type :value})
(identifier? (:type literal))
(string? (:value literal))))

(defn literal?
"Accepts a String or Map representing a literal and returns true or false depending on if it is valid."
[literal]
(or (plain-literal? literal) (typed-literal? literal)))

(defn subject?
"Accepts a String representing a subject and returns true or false depending of
whether or not that String is a valid identifier."
[subject]
(identifier? subject))

(defn predicate?
"Accepts a String representing a predicate and returns true or false depending on if it is valid."
[predicate]
(identifier? predicate))

(defn object?
"Accepts a String or Map representing an object and returns true or false depending on if it is valid."
[object]
(or (identifier? object) (literal? object)))

(defn graph?
"Checks that a passed String value is either a valid identifier."
[graph]
(identifier? graph))

(defn subject
"Accepts a Statement tuple and returns the Subject."
[statement]
(get statement 0))

(defn predicate
"Accepts a Statement tuple and returns the Predicate."
[statement]
(get statement 1))

(defn object
"Accepts a Statement tuple and returns the Object."
[statement]
(get statement 2))

(defn graph
"Accepts a Statement tuple and returns the Graph."
[statement]
(get statement 3))
