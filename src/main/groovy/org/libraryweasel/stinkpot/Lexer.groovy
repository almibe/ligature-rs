/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

//TODO add a match method to this class to clean up Lexer and make error handling work better
abstract class Lexer {
    String input
    int pos = 0
    char c
    final char EOF = -1

    public Lexer(String input) {
        this.input = input
        c = input[pos]
    }

    void match(char c) {
        if (this.c == c) consume()
        else throw new RuntimeException("Error Parsing - Expected '$c' Found '${this.c}'")
    }

    void consume() {
        pos++
        if (pos >= input.length()) c = EOF
        else c = input[pos]
    }

    abstract Token nextToken()
}
