/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.store

import org.almibe.ligature.Graph
import org.almibe.ligature.Store

class InMemoryStore: Store {
    override fun execute(sparql: String) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun defaultGraph(): Graph {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun namedGraph(name: String): Graph? {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }
}
