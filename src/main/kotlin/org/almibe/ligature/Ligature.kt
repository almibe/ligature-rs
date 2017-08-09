/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import com.google.common.graph.ImmutableNetwork
import org.almibe.ligature.loaders.NTriples

class Ligature {
    private val nTriples = NTriples()
    fun loadNTriples(text: String): ImmutableNetwork<Node, Predicate> = nTriples.loadNTriples(text)
}
