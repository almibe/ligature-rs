/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import spock.lang.Specification

class TurtleSpec  extends Specification {
    Stinkpot stinkpot = new Stinkpot()

    def spidermanEnemy = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))

    def spidermanName = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://xmlns.com/foaf/0.1/name"), new PlainLiteral("Spiderman"))

    def spidermanNameRu = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://xmlns.com/foaf/0.1/name"), new LangLiteral("Человек-паук", "ru"))

    def 'support basic IRI triple'() {
        given:
        def expectedResult = spidermanEnemy
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/basicTriple.ttl').text)
        then:
        results.size() == 1
        results.first() == expectedResult
    }

    def 'support predicate lists'() {
        given:
        def expectedResults = [spidermanEnemy, spidermanName]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/predicateList.ttl').text)
        then:
        results.size() == 2
        results == expectedResults
    }

    def 'support object lists'() {
        given:
        def expectedResults = [spidermanName, spidermanNameRu]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/objectList.ttl').text)
        then:
        results.size() == 2
        results == expectedResults
    }

    def base = "http://one.example/"
    def base2 = "http://one.example2/"
    def base3 = "http://another.example/"

    def 'turtle iri parsing'() {
        given:
        def expectedResults = [
            new Triple(new IRI("http://one.example/subject1"), new IRI("http://one.example/predicate1"), new IRI("http://one.example/object1")),
            new Triple(new IRI("$base/subject2"), new IRI("$base/predicate2"), new IRI("$base/object2")),
            new Triple(new IRI("$base2/subject2"), new IRI("$base2/predicate2"), new IRI("$base2/object2")),
            new Triple(new IRI("$base/subject3"), new IRI("$base/predicate3"), new IRI("$base/object3")),
            new Triple(new IRI("$base2/subject3"), new IRI("$base2/predicate3"), new IRI("$base2/object3")),
            new Triple(new IRI("$base/path/subject4"), new IRI("$base/path/predicate4"), new IRI("$base/path/object4")),
            new Triple(new IRI("$base3/subject5"), new IRI("$base3/predicate5"), new IRI("$base3/object5")),
            new Triple(new IRI("$base3/subject6"), new IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), new IRI("$base3/subject7")),
            new Triple(new IRI("http://伝言.example/?user=أكرم&amp;channel=R%26D"), new IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), new IRI("$base3/subject8"))
        ]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/comprehensivePrefixBaseExample.ttl').text)
        then:
        results.size() == 2
        results == expectedResults
    }
}
