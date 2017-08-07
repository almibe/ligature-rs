/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import com.orientechnologies.orient.core.db.ODatabasePool
import com.orientechnologies.orient.core.db.OrientDB
import com.orientechnologies.orient.core.db.OrientDBConfig
import com.orientechnologies.orient.core.id.ORID
import org.almibe.ligature.loaders.NTriples

class Ligature {
    lateinit var dbPool: ODatabasePool
        private set
    private val nTriples = NTriples(dbPool)

    constructor(dbPool: ODatabasePool) {
        this.dbPool = dbPool
    }

    constructor() {
        this.dbPool = ODatabasePool(
                OrientDB("memory:ligature", OrientDBConfig.defaultConfig()), "ligature", "admin", "admin")
    }

    fun loadNTriples(text: String): Set<ORID> = nTriples.loadNTriples(text)
}
