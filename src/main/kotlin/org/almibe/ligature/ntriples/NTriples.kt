/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.Triple
import org.almibe.ligature.parser.NTriplesBaseVisitor
import org.almibe.ligature.parser.NTriplesLexer
import org.almibe.ligature.parser.NTriplesParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream

class DocumentVisitor : NTriplesBaseVisitor<List<Triple>>() {
    override fun visitDocument(ctx: NTriplesParser.DocumentContext): List<Triple> {
        val tripleVisitor = TripleVisitor()
        return ctx.triple().map {
            it.accept(tripleVisitor)
        }
    }
}

class TripleVisitor : NTriplesBaseVisitor<Triple>() {
    override fun visitTriple(ctx: NTriplesParser.TripleContext?): Triple {
        TODO()
    }
}

class NTriples {
    fun parseNTriples(text: String) : List<Triple>  {
        val stream = CharStreams.fromString(text)
        val lexer = NTriplesLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = NTriplesParser(tokens)

        val documentVisitor = DocumentVisitor()
        return documentVisitor.visit(parser.document())
    }
}
