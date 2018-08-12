/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import io.kotlintest.shouldBe
import io.kotlintest.specs.StringSpec
import kotlin.Pair
import org.almibe.ligature.*

class NTriplesSpec : StringSpec() {
    override fun isInstancePerTest() = true
    
    init {
        val model = InMemoryModel()
        val ligature = Ligature(model)
        val stringIRI = IRI("http://www.w3.org/2001/XMLSchema#string")
        val spiderMan = IRI ("http://example.org/#spiderman")
        val greenGoblin = IRI ("http://example.org/#green-goblin")
        val enemyOf = IRI ("http://www.perceive.net/schemas/relationship/enemyOf")
        val thatSeventiesShow = IRI ("http://example.org/show/218")
        val helium = IRI ("http://en.wikipedia.org/wiki/Helium")
        val label = IRI ("http://www.w3.org/2000/01/rdf-schema#label")

        "support basic IRI triple" {
            ligature.loadNTriples(readText("/ntriples/01-basicTriple.nt"))
            
            ligature.statementsFor(spiderMan) shouldBe setOf(Pair < Predicate, Object > (enemyOf, greenGoblin))
            ligature.getSubjects().size shouldBe 2
            model.getIRIs().size shouldBe 3
        }

        "support multiple IRI triples" {
            ligature.loadNTriples(readText("/ntriples/02-multipleIRITriples.nt"))
            
            ligature.statementsFor(spiderMan) shouldBe [Pair (enemyOf, greenGoblin), Pair(enemyOf, IRI("http://example.org/#black-cat"))].toSet()
            ligature.getSubjects().size shouldBe 3
            model.getObjects().size shouldBe 2
            model.getPredicates().size shouldBe 1
        }

        "support beginning of line and end of line comments" {
            
            ligature.loadNTriples(this.class. getResource ("/ntriples/03-comments.nt").text)
            
            ligature.statementsFor(spiderMan) shouldBe [Pair (enemyOf, greenGoblin)].toSet()
            ligature.getSubjects().size shouldBe 2
            model.getIRIs().size shouldBe 3
        }

        "support literals with languages and types" {
            
            ligature.loadNTriples(this.class. getResource ("/ntriples/04-literals.nt").text)
            
            ligature.statementsFor(thatSeventiesShow) shouldBe [
                Pair (label, TypedLiteral ("That Seventies Show", stringIRI)),
            Pair (label, TypedLiteral("That Seventies Show", stringIRI)),
            Pair (IRI ("http://example.org/show/localName"), LangLiteral("That Seventies Show", "en")),
            Pair (IRI ("http://example.org/show/localName"), LangLiteral("Cette Série des Années Septante", "fr-be"))].toSet()
            ligature.statementsFor(spiderMan) shouldBe [
                Pair (IRI ("http://example.org/text"), TypedLiteral ("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI))].toSet()
            ligature.statementsFor(helium) shouldBe [
                Pair (IRI ("http://example.org/elements/atomicNumber"), TypedLiteral ("2", IRI ("http://www.w3.org/2001/XMLSchema#integer"))),
            Pair (IRI ("http://example.org/elements/specificGravity"), TypedLiteral("1.663E-4", IRI("http://www.w3.org/2001/XMLSchema#double")))].toSet()
            model.literals shouldBe [TypedLiteral ("That Seventies Show", stringIRI),
            LangLiteral ("That Seventies Show", "en"),
            LangLiteral ("Cette Série des Années Septante", "fr-be"),
            TypedLiteral ("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI),
            TypedLiteral ("2", IRI("http://www.w3.org/2001/XMLSchema#integer")),
            TypedLiteral ("1.663E-4", IRI("http://www.w3.org/2001/XMLSchema#double"))].toSet()
        }

        "support blank nodes" {
            
            def nTriples = NTriples()
            def model = nTriples . loadNTriples (this.class. getResource ("/ntriples/05-blankNodes.nt").text)
            
            model.getSubjects().size shouldBe 2
            model.getPredicates().size shouldBe 1
            model.getObjects().size shouldBe 2
            model.statementsFor(BlankNode ("bob")) shouldBe [
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("alice"))].toSet()
            model.statementsFor(BlankNode ("alice")) shouldBe [
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("bob"))].toSet()
        }

        "make sure blank nodes are unique across document loads" {
            
            ligature.loadNTriples(this.class. getResource ("/ntriples/05-blankNodes.nt").text)
            ligature.loadNTriples(this.class. getResource ("/ntriples/05-blankNodes.nt").text)
            
            ligature.getSubjects().size shouldBe 4
            model.getPredicates().size shouldBe 1
            ligature.model.getObjects().size shouldBe 4
            ligature.statementsFor(BlankNode ("bob_1")) shouldBe [
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("alice_2"))].toSet()
            ligature.statementsFor(BlankNode ("alice_2")) shouldBe [
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("bob_1"))].toSet()
            ligature.statementsFor(BlankNode ("bob_3")) shouldBe [
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("alice_4"))].toSet()
            ligature.statementsFor(BlankNode ("alice_4")) shouldBe [
                Pair (IRI ("http://xmlns.com/foaf/0.1/knows"), BlankNode ("bob_3"))].toSet()
        }
    }
}
