/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.parsers

import org.almibe.ligature.*
import spock.lang.Specification

class NTriplesSpec extends Specification {
    def ligature = new NTriples()

    def stringIRI = new IRI("http://www.w3.org/2001/XMLSchema#string")

    def "support basic IRI triple"() {
        given:
        def expectedResult = new Triple( new IRI("http://example.org/#spiderman"),
                new IRI("http://www.perceive.net/schemas/relationship/enemyOf"),  new IRI("http://example.org/#green-goblin"))
        def results = ligature.parseNTriples(this.class.getResource("/ntriples/01-basicTriple.nt").text)
        expect:
        results.size() == 1
        results == [expectedResult]
    }

    def "support multiple IRI triples"() {
        given:
        def expectedResult1 =  new Triple(new IRI("http://example.org/#spiderman"),
                new IRI("http://www.perceive.net/schemas/relationship/enemyOf"),  new IRI("http://example.org/#green-goblin"))
        def expectedResult2 =  new Triple(new IRI("http://example.org/#spiderman"),
                new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#black-cat"))
        def results = ligature.parseNTriples(this.class.getResource("/ntriples/02-multipleIRITriples.nt").text)
        expect:
        results == [expectedResult1, expectedResult2]
    }

    def "support beginning of line and end of line comments"() {
        given:
        def expectedResult =  new Triple(new IRI("http://example.org/#spiderman"),
                new IRI("http://www.perceive.net/schemas/relationship/enemyOf"),  new IRI("http://example.org/#green-goblin"))
        def results = ligature.parseNTriples(this.class.getResource("/ntriples/03-comments.nt").text)
        expect:
        results.size() == 1
        results.first() == expectedResult
    }

    def "support literals with languages and types"() {
        given:
        def expectedResults = [
            (new Triple(new IRI("http://example.org/show/218"), new IRI("http://www.w3.org/2000/01/rdf-schema#label"), new TypedLiteral("That Seventies Show", stringIRI))),
            (new Triple(new IRI("http://example.org/show/218"), new IRI("http://www.w3.org/2000/01/rdf-schema#label"), new TypedLiteral("That Seventies Show", stringIRI))),
            (new Triple(new IRI("http://example.org/show/218"), new IRI("http://example.org/show/localName"), new LangLiteral("That Seventies Show", "en"))),
            (new Triple(new IRI("http://example.org/show/218"), new IRI("http://example.org/show/localName"), new LangLiteral("Cette Série des Années Septante", "fr-be"))),
            (new Triple(new IRI("http://example.org/#spiderman"), new IRI("http://example.org/text"), new TypedLiteral("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI))),
            (new Triple(new IRI("http://en.wikipedia.org/wiki/Helium"), new IRI("http://example.org/elements/atomicNumber"), new TypedLiteral("2", new IRI("http://www.w3.org/2001/XMLSchema#integer")))),
            (new Triple(new IRI("http://en.wikipedia.org/wiki/Helium"), new IRI("http://example.org/elements/specificGravity"), new TypedLiteral("1.663E-4", new IRI("http://www.w3.org/2001/XMLSchema#double"))))
        ]
        def results = ligature.parseNTriples(this.class.getResource("/ntriples/04-literals.nt").text)
        expect:
        results == expectedResults
    }

    def "support blank nodes"() {
        given:
        def expectedResult1 = new Triple(new LabeledBlankNode("alice"), new IRI("http://xmlns.com/foaf/0.1/knows"), new LabeledBlankNode("bob"))
        def expectedResult2 = new Triple(new LabeledBlankNode("bob"), new IRI("http://xmlns.com/foaf/0.1/knows"), new LabeledBlankNode("alice"))
        def results = ligature.parseNTriples(this.class.getResource("/ntriples/05-blankNodes.nt").text)
        expect:
        results == [expectedResult1, expectedResult2]
    }
}
