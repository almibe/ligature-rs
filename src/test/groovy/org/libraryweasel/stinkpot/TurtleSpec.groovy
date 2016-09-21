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
}
