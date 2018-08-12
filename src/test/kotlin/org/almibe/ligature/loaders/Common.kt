/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

fun readText(resourcePath: String): String {
    return NTriplesSpec::class.java.getResource(resourcePath).readText()
}
