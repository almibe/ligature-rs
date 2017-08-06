package org.almibe.ligature

import com.orientechnologies.orient.core.db.OrientDB
import com.orientechnologies.orient.core.db.OrientDBConfig
import org.almibe.ligature.loaders.NTriples

class Ligature {
    private lateinit var orientDB: OrientDB
    private val nTriples = NTriples(orientDB)

    constructor(orientDB: OrientDB) {
        this.orientDB = orientDB
    }

    constructor() {
        this.orientDB = OrientDB("memory:ligature", OrientDBConfig.defaultConfig())
    }

    fun loadNTriples(text: String) {
        nTriples.loadNTriples(text)
    }
}
