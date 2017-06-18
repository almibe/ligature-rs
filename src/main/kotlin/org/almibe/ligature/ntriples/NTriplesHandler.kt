/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.Triple
import org.almibe.ligature.parser.NTriplesBaseVisitor

class NTriplesHandler : NTriplesBaseVisitor<List<Triple>>()
