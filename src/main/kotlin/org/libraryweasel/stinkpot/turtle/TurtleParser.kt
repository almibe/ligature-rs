/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.turtle

import org.libraryweasel.stinkpot.Parser
import org.libraryweasel.stinkpot.Triple

class TurtleParser(lexer: TurtleLexer, val handler: (Triple) -> Unit) : Parser<TurtleTokenType>(lexer) {

}
