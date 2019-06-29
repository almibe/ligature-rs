/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import org.almibe.ligature.Graph
import org.almibe.ligature.IRI
import org.almibe.ligature.Store
import java.io.Reader
import java.io.Writer

interface Loader {
    fun import(reader: Reader, store: Store, defaultGraph: IRI? = null)
    fun export(writer: Writer, graphs: Collection<Graph>)
}
