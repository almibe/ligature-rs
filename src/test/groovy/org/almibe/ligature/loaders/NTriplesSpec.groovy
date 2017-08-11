/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import kotlin.Pair
import org.almibe.ligature.*
import spock.lang.Specification

class NTriplesSpec extends Specification {
    def ligature = new Ligature(new InMemoryModel())
    final stringIRI = new IRI("http://www.w3.org/2001/XMLSchema#string")
    static final spiderMan = new IRI("http://example.org/#spiderman")
    static final greenGoblin = new IRI("http://example.org/#green-goblin")
    static final enemyOf = new IRI("http://www.perceive.net/schemas/relationship/enemyOf")
    static final thatSeventiesShow = new IRI("http://example.org/show/218")
    static final helium = new IRI("http://en.wikipedia.org/wiki/Helium")
    static final label = new IRI("http://www.w3.org/2000/01/rdf-schema#label")

    def "support basic IRI triple"() {
        given:
        ligature.loadNTriples(this.class.getResource("/ntriples/01-basicTriple.nt").text)
        expect:
        ligature.statementsFor(spiderMan) == [new Pair<Predicate, Object>(enemyOf, greenGoblin)].toSet()
        ligature.subjects.size() == 1
        ligature.IRIs.size() == 3
    }

    def "support multiple IRI triples"() {
        given:
        ligature.loadNTriples(this.class.getResource("/ntriples/02-multipleIRITriples.nt").text)
        expect:
        ligature.statementsFor(spiderMan) == [new Pair(enemyOf, greenGoblin), new Pair(enemyOf, new IRI("http://example.org/#black-cat"))].toSet()
        ligature.subjects.size() == 1
        ligature.objects.size() == 2
        ligature.predicates.size() == 1
    }

    def "support beginning of line and end of line comments"() {
        given:
        ligature.loadNTriples(this.class.getResource("/ntriples/03-comments.nt").text)
        expect:
        ligature.statementsFor(spiderMan) == [new Pair(enemyOf, greenGoblin)].toSet()
        ligature.subjects.size() == 1
        ligature.IRIs.size() == 3
    }

    def "support literals with languages and types"() {
        given:
        ligature.loadNTriples(this.class.getResource("/ntriples/04-literals.nt").text)
        expect:
        ligature.statementsFor(thatSeventiesShow) == [
                new Pair(label, new TypedLiteral("That Seventies Show", stringIRI)),
                new Pair(label, new TypedLiteral("That Seventies Show", stringIRI)),
                new Pair(new IRI("http://example.org/show/localName"), new LangLiteral("That Seventies Show", "en")),
                new Pair(new IRI("http://example.org/show/localName"), new LangLiteral("Cette Série des Années Septante", "fr-be"))].toSet()
        ligature.statementsFor(spiderMan) == [
                new Pair(new IRI("http://example.org/text"), new TypedLiteral("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI))].toSet()
        ligature.statementsFor(helium) == [
                new Pair(new IRI("http://example.org/elements/atomicNumber"), new TypedLiteral("2", new IRI("http://www.w3.org/2001/XMLSchema#integer"))),
                new Pair(new IRI("http://example.org/elements/specificGravity"), new TypedLiteral("1.663E-4", new IRI("http://www.w3.org/2001/XMLSchema#double")))].toSet()
        ligature.literals == [new TypedLiteral("That Seventies Show", stringIRI),
                new LangLiteral("That Seventies Show", "en"),
                new LangLiteral("Cette Série des Années Septante", "fr-be"),
                new TypedLiteral("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI),
                new TypedLiteral("2", new IRI("http://www.w3.org/2001/XMLSchema#integer")),
                new TypedLiteral("1.663E-4", new IRI("http://www.w3.org/2001/XMLSchema#double"))].toSet()
    }

    def "support blank nodes"() {
        given:
        def nTriples = new NTriples()
        def model = nTriples.loadNTriples(this.class.getResource("/ntriples/05-blankNodes.nt").text)
        expect:
        model.subjects.size() == 2
        model.predicates.size() == 1
        model.objects.size() == 2
        model.statementsFor(new BlankNode("bob")) == [
                new Pair(new IRI("http://xmlns.com/foaf/0.1/knows"), new BlankNode("alice"))].toSet()
        model.statementsFor(new BlankNode("alice")) == [
                new Pair(new IRI("http://xmlns.com/foaf/0.1/knows"), new BlankNode("bob"))].toSet()
    }

    def "make sure blank nodes are unique across document loads"() {
        given:
        ligature.loadNTriples(this.class.getResource("/ntriples/05-blankNodes.nt").text)
        ligature.loadNTriples(this.class.getResource("/ntriples/05-blankNodes.nt").text)
        expect:
        ligature.subjects.size() == 4
        ligature.predicates.size() == 1
        ligature.objects.size() == 4
        ligature.statementsFor(new BlankNode("bob_1")) == [new Pair(new IRI("http://xmlns.com/foaf/0.1/knows"), new BlankNode("alice_2"))].toSet()
        ligature.statementsFor(new BlankNode("alice_2")) == [new Pair(new IRI("http://xmlns.com/foaf/0.1/knows"), new BlankNode("bob_1"))].toSet()
        ligature.statementsFor(new BlankNode("bob_3")) == [new Pair(new IRI("http://xmlns.com/foaf/0.1/knows"), new BlankNode("alice_4"))].toSet()
        ligature.statementsFor(new BlankNode("alice_4")) == [new Pair(new IRI("http://xmlns.com/foaf/0.1/knows"), new BlankNode("bob_3"))].toSet()
    }
}
