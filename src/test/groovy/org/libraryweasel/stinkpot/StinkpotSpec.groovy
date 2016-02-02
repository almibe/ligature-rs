/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import spock.lang.Specification

public class StinkpotSpec extends Specification {
    Stinkpot stinkpot = new Stinkpot()

    def 'support basic triple, example 2'() {
        given:
        def expectedResult = new Triple('http://example.org/#spiderman', 'http://www.perceive.net/schemas/relationship/enemyOf',
            'http://example.org/#green-goblin')
        when:
        Triple result = stinkpot.parseText(this.getClass().getResource('/example2.ttl').text)
        then:
        result == expectedResult
    }

    def 'allow multiple triples one per line, example 4'() {
        given:
        def expectedResult = [
            new Triple('http://example.org/#spiderman', 'http://www.perceive.net/schemas/relationship/enemyOf',
                'http://example.org/#green-goblin'),
            new Triple('http://example.org/#spiderman', 'http://xmlns.com/foaf/0.1/name',
                    'Spiderman')
        ]
        when:
        List<Triple> result = stinkpot.parseText(this.getClass().getResource('/example2.ttl').text)
        then:
        result == expectedResult
    }
}
