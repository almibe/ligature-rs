/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import spock.lang.Specification

import java.nio.file.Paths

public class NTriplesSpec extends Specification {
    Stinkpot stinkpot = new Stinkpot()

    def stringIRI = new IRI("http://www.w3.org/2001/XMLSchema#string")

    def 'support basic IRI triple'() {
        given:
        def expectedResult = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        when:
        List<Triple> results = stinkpot.parseNTriples(this.getClass().getResource('/ntriples/basicTriple.nt').text)
        then:
        results.size() == 1
        results.first() == expectedResult
    }

    def 'support multiple IRI triples'() {
        given:
        def expectedResult1 = new Triple(new IRI("http://example.org/#spiderman"),
                    new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        def expectedResult2 = new Triple(new IRI("http://example.org/#spiderman"),
                    new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#black-cat"))
        when:
        List<Triple> results = stinkpot.parseNTriples(this.getClass().getResource('/ntriples/multipleIRITriples.nt').text)
        then:
        results.size() == 2
        results.first() == expectedResult1
        results.last() == expectedResult2
    }

    def 'support beginning of line and end of line comments'() {
        given:
        def expectedResult = new Triple(new IRI("http://example.org/#spiderman"),
                new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        when:
        List<Triple> results = stinkpot.parseNTriples(this.getClass().getResource('/ntriples/comments.nt').text)
        then:
        results.size() == 1
        results.first() == expectedResult
    }

    def 'support literals with languages and types'() {
        given:
        def expectedResults = []
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://www.w3.org/2000/01/rdf-schema#label"), new TypedLiteral("That Seventies Show", stringIRI)))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://www.w3.org/2000/01/rdf-schema#label"), new TypedLiteral("That Seventies Show", stringIRI)))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://example.org/show/localName"), new LangLiteral("That Seventies Show", "en")))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://example.org/show/localName"), new LangLiteral("Cette Série des Années Septante", "fr-be")))
        expectedResults.add(new Triple(new IRI("http://example.org/#spiderman"), new IRI("http://example.org/text"), new TypedLiteral("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI)))
        expectedResults.add(new Triple(new IRI("http://en.wikipedia.org/wiki/Helium"), new IRI("http://example.org/elements/atomicNumber"), new TypedLiteral("2", new IRI("http://www.w3.org/2001/XMLSchema#integer"))))
        expectedResults.add(new Triple(new IRI("http://en.wikipedia.org/wiki/Helium"), new IRI("http://example.org/elements/specificGravity"), new TypedLiteral("1.663E-4", new IRI("http://www.w3.org/2001/XMLSchema#double"))))
        when:
        List<Triple> results = stinkpot.parseNTriples(this.getClass().getResource('/ntriples/literals.nt').text)
        then:
        results[0] == expectedResults[0]
        results[1] == expectedResults[1]
        results[2] == expectedResults[2]
        results[3] == expectedResults[3]
        results[4] == expectedResults[4]
        results[5] == expectedResults[5]
        results[6] == expectedResults[6]
    }

    def 'support literals with languages and types passing a path instead of a String'() {
        given:
        def expectedResults = []
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://www.w3.org/2000/01/rdf-schema#label"), new TypedLiteral("That Seventies Show", stringIRI)))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://www.w3.org/2000/01/rdf-schema#label"), new TypedLiteral("That Seventies Show", stringIRI)))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://example.org/show/localName"), new LangLiteral("That Seventies Show", "en")))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://example.org/show/localName"), new LangLiteral("Cette Série des Années Septante", "fr-be")))
        expectedResults.add(new Triple(new IRI("http://example.org/#spiderman"), new IRI("http://example.org/text"), new TypedLiteral("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI)))
        expectedResults.add(new Triple(new IRI("http://en.wikipedia.org/wiki/Helium"), new IRI("http://example.org/elements/atomicNumber"), new TypedLiteral("2", new IRI("http://www.w3.org/2001/XMLSchema#integer"))))
        expectedResults.add(new Triple(new IRI("http://en.wikipedia.org/wiki/Helium"), new IRI("http://example.org/elements/specificGravity"), new TypedLiteral("1.663E-4", new IRI("http://www.w3.org/2001/XMLSchema#double"))))
        when:
        List<Triple> results = stinkpot.parseNTriples(Paths.get(this.getClass().getResource('/ntriples/literals.nt').toURI()))
        then:
        results[0] == expectedResults[0]
        results[1] == expectedResults[1]
        results[2] == expectedResults[2]
        results[3] == expectedResults[3]
        results[4] == expectedResults[4]
        results[5] == expectedResults[5]
        results[6] == expectedResults[6]
    }

    def 'support blank nodes'() {
        given:
        def expectedResult1 = new Triple(new BlankNode('alice'), new IRI('http://xmlns.com/foaf/0.1/knows'), new BlankNode('bob'))
        def expectedResult2 = new Triple(new BlankNode('bob'), new IRI('http://xmlns.com/foaf/0.1/knows'), new BlankNode('alice'))
        when:
        List<Triple> results = stinkpot.parseNTriples(this.getClass().getResource('/ntriples/blankNodes.nt').text)
        then:
        results.size() == 2
        results.first() == expectedResult1
        results.last() == expectedResult2
    }
}
