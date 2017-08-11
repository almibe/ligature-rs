/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import org.almibe.ligature.loaders.NTriples
import org.almibe.ligature.loaders.Turtle

class Ligature(val model: Model): Model by model {
    private val nTriples = NTriples()
    private val turtle = Turtle()
    fun loadNTriples(text: String) {
        val loadedModel = nTriples.loadNTriples(text)
        model.addModel(loadedModel)
    }
    fun loadTurtle(text: String) {
        val loadedModel = turtle.loadTurtle(text)
        model.addModel(loadedModel)
    }
}
