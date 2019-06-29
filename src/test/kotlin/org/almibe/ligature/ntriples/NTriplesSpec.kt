/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import io.kotlintest.specs.StringSpec
import org.almibe.ligature.*
import org.almibe.ligature.parsers.*

class NTriplesSpec : StringSpec() {
    override fun isInstancePerTest() = true
    
    init {
        val model = InMemoryGraph()
        val ligature = Ligature(model)

        "support basic IRI triple" {
            ligature.loadNTriples(readText("/ntriples/01-basicTriple.nt"))
            
            ligature.statementsFor(spiderMan) shouldBe setOf(Pair < Predicate, Object > (enemyOf, greenGoblin))
            ligature.getSubjects().size shouldBe 2
            model.getIRIs().size shouldBe 3
        }

        "support multiple IRI triples" {
            ligature.loadNTriples(readText("/ntriples/02-multipleIRITriples.nt"))
            
            ligature.statementsFor(spiderMan) shouldBe setOf(Pair (enemyOf, greenGoblin), Pair(enemyOf, IRI("http://example.org/#black-cat")))
            ligature.getSubjects().size shouldBe 3
            model.getObjects().size shouldBe 2
            model.getPredicates().size shouldBe 1
        }

        "support beginning of line and end of line comments" {
            
            ligature.loadNTriples(readText("/ntriples/03-comments.nt"))
            
            ligature.statementsFor(spiderMan) shouldBe setOf(Pair (enemyOf, greenGoblin))
            ligature.getSubjects().size shouldBe 2
            model.getIRIs().size shouldBe 3
        }

        "support literals with languages and types" {
            
            ligature.loadNTriples(readText("/ntriples/04-literals.nt"))

            ligature.statementsFor(thatSeventiesShow) shouldBe setOf(
                Pair (label, TypedLiteral ("That Seventies Show", stringIRI)),
            Pair (label, TypedLiteral("That Seventies Show", stringIRI)),
            Pair (IRI ("http://example.org/show/localName"), LangLiteral("That Seventies Show", "en")),
            Pair (IRI ("http://example.org/show/localName"), LangLiteral("Cette Série des Années Septante", "fr-be")))
            ligature.statementsFor(spiderMan) shouldBe setOf(
                Pair (IRI ("http://example.org/text"), TypedLiteral ("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI)))
            ligature.statementsFor(helium) shouldBe setOf(
                Pair (IRI ("http://example.org/elements/atomicNumber"), TypedLiteral ("2", IRI ("http://www.w3.org/2001/XMLSchema#integer"))),
            Pair (IRI ("http://example.org/elements/specificGravity"), TypedLiteral("1.663E-4", IRI("http://www.w3.org/2001/XMLSchema#double"))))
            model.getLiterals() shouldBe setOf(TypedLiteral ("That Seventies Show", stringIRI),
                LangLiteral ("That Seventies Show", "en"),
                LangLiteral ("Cette Série des Années Septante", "fr-be"),
                TypedLiteral ("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI),
                TypedLiteral ("2", IRI("http://www.w3.org/2001/XMLSchema#integer")),
                TypedLiteral ("1.663E-4", IRI("http://www.w3.org/2001/XMLSchema#double")))
        }

        "support blank nodes" {
            
            val nTriples = NTriples()
            val model = nTriples.loadNTriples(readText("/ntriples/05-blankNodes.nt"))
            model.getSubjects().size shouldBe 2
            model.statementsFor(BlankNode ("bob")) shouldBe setOf(
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("alice")))
            model.statementsFor(BlankNode ("alice")) shouldBe setOf(
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("bob")))
        }

        "make sure blank nodes are unique across document loads" {
            
            ligature.loadNTriples(readText("/ntriples/05-blankNodes.nt"))
            ligature.loadNTriples(readText("/ntriples/05-blankNodes.nt"))
            
            ligature.getSubjects().size shouldBe 4
            model.getPredicates().size shouldBe 1
            model.getObjects().size shouldBe 4
            ligature.statementsFor(BlankNode ("bob_1")) shouldBe setOf(
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("alice_2")))
            ligature.statementsFor(BlankNode ("alice_2")) shouldBe setOf(
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("bob_1")))
            ligature.statementsFor(BlankNode ("bob_3")) shouldBe setOf(
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("alice_4")))
            ligature.statementsFor(BlankNode ("alice_4")) shouldBe setOf(
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("bob_3")))
        }
    }
}
