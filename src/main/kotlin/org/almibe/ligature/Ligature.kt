/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import org.almibe.ligature.loaders.NTriples
import org.almibe.ligature.loaders.Turtle
import java.io.Reader

class Ligature(val model: Graph): Graph by model {
    private val nTriples = NTriples()
    private val turtle = Turtle()

    fun loadNTriples(reader: Reader) {
        val loadedModel = nTriples.loadNTriples(reader)
        model.addModel(loadedModel)
    }
    fun loadTurtle(reader: Reader) {
        val loadedModel = turtle.loadTurtle(reader)
        model.addModel(loadedModel)
    }
}
