/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

interface Subject {}
interface Object {}
interface Predicate {}

data class IRI(val value: String) : Subject, Predicate, Object
data class BlankNode(val label: String) : Subject, Object
data class Triple(val subject: Subject, val predicate: Predicate, val `object`: Object)

interface Literal : Object { val value: String}
data class PlainLiteral (override val value: String) : Literal
data class LangLiteral(override val value: String, val langTag: String) : Literal
data class TypedLiteral(override val value: String, val datatypeIRI: IRI) : Literal
