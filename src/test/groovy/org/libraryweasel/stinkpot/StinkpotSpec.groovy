/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import spock.lang.Specification

public class StinkpotSpec extends Specification {
    Stinkpot stinkpot = new Stinkpot()

    def 'support basic triple, example 2'() {
        when:
        Triple result = stinkpot.parseText(this.getClass().getResource('/example2.ttl').text)
        then:
        result == new Triple('http://example.org/#spiderman', 'http://www.perceive.net/schemas/relationship/enemyOf',
                'http://example.org/#green-goblin')
    }
}
