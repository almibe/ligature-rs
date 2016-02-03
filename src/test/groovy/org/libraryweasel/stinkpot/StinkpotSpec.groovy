/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.IRI
import org.libraryweasel.stinkpot.ntriples.Triple
import spock.lang.Specification

public class StinkpotSpec extends Specification {
    Stinkpot stinkpot = new Stinkpot()

    def 'support basic triple, example 2'() {
        given:
        def expectedResult = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        when:
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/specificationExamples/example2.nt').text)
        then:
        results.size() == 1
        results.first() == expectedResult
    }
}
