/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.turtle

import org.libraryweasel.stinkpot.TokenType

enum class TurtleTokenType : TokenType {
    //Below are tokens used for ntriples and turtle
    LANGTAG,
    EOL,
    IRIREF,
    STRING_LITERAL_QUOTE,
    BLANK_NODE_LABEL,
    UCHAR,
    ECHAR,
    PN_CHARS_BASE,
    PN_CHARS_U,
    PN_CHARS,
    HEX,
    EOF,
    PERIOD,

    //Below are tokens only used by turtle
    SEMICOLON
}
