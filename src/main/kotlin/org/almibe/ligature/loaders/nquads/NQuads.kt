/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders.nquads

import org.almibe.ligature.Graph
import org.almibe.ligature.IRI
import org.almibe.ligature.Store
import org.almibe.ligature.loaders.Loader
import java.io.Reader
import java.io.Writer

class NQuads: Loader {
    override fun import(reader: Reader, store: Store, defaultGraph: IRI?) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun export(writer: Writer, graphs: Collection<Graph>) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

}
