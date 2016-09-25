/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

interface Node {}
interface Subject: Node {}
interface Object {}
interface Predicate: Node {}

data class IRI(val value: String) : Subject, Predicate, Object
data class BlankNode(val label: String) : Subject, Object
data class Triple(val subject: Subject, val predicate: Predicate, val `object`: Object)

interface Literal : Object { val value: String}
data class LangLiteral(override val value: String, val langTag: String) : Literal
data class TypedLiteral(override val value: String,
                        val datatypeIRI: IRI = IRI("http://www.w3.org/2001/XMLSchema#string")) : Literal
