/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.IRI
import org.libraryweasel.stinkpot.ntriples.Triple
import spock.lang.Specification

public class StinkpotSpec extends Specification {
    Stinkpot stinkpot = new Stinkpot()

    def 'support basic IRI triple'() {
        given:
        def expectedResult = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        when:
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/basicTriple.nt').text)
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
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/multipleIRITriples.nt').text)
        then:
        results.size() == 2
        results.first() == expectedResult1
        results.last() == expectedResult2
    }

    def 'support begging of line and end of line comments'() {
        given:
        def expectedResult = new Triple(new IRI("http://example.org/#spiderman"),
                new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        when:
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/comments.nt').text)
        then:
        results.size() == 1
        results.first() == expectedResult
    }
}
