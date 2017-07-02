/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.parsers

import org.almibe.ligature.*
import org.almibe.ligature.parser.NTriplesBaseListener
import org.almibe.ligature.parser.NTriplesLexer
import org.almibe.ligature.parser.NTriplesParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.tree.ErrorNode
import org.antlr.v4.runtime.tree.ParseTreeWalker

class NTriples {
    fun parseNTriples(text: String) : List<Triple>  {
        val stream = CharStreams.fromString(text)
        val lexer = NTriplesLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = NTriplesParser(tokens)

        val walker = ParseTreeWalker()
        val listener = TriplesNTripleListener()
        walker.walk(listener, parser.ntriplesDoc())
        return listener.triples
    }
}

private class TriplesNTripleListener : NTriplesBaseListener() {
    val triples: MutableList<Triple> = mutableListOf()

    lateinit var currentTriple: TempTriple

    override fun enterTriple(ctx: NTriplesParser.TripleContext) {
        currentTriple = TempTriple()
    }

    override fun exitTriple(ctx: NTriplesParser.TripleContext) {
        triples.add(Triple(currentTriple.subject, currentTriple.predicate, currentTriple.`object`))
    }

    override fun exitSubject(ctx: NTriplesParser.SubjectContext) {
        val subject: Subject = when {
            ctx.IRIREF() != null -> handleIRI(ctx.IRIREF().text)
            ctx.BLANK_NODE_LABEL() != null -> handleBlankNode(ctx.BLANK_NODE_LABEL().text)
            else -> throw RuntimeException("Unexpected Subject Type")
        }
        currentTriple.subject = subject
    }

    override fun exitPredicate(ctx: NTriplesParser.PredicateContext) {
        val predicate: Predicate = when {
            ctx.IRIREF() != null -> handleIRI(ctx.IRIREF().text)
            else -> throw RuntimeException("Unexpected Predicate Type")
        }
        currentTriple.predicate = predicate
    }

    override fun exitObject(ctx: NTriplesParser.ObjectContext) {
        val `object`: Object = when {
            ctx.IRIREF() != null -> handleIRI(ctx.IRIREF().text)
            ctx.BLANK_NODE_LABEL() != null -> handleBlankNode(ctx.BLANK_NODE_LABEL().text)
            ctx.literal() != null -> handleLiteral(ctx.literal())
            else -> throw RuntimeException("Unexpected Object Type")
        }
        currentTriple.`object` = `object`
    }

    override fun visitErrorNode(node: ErrorNode) {
        throw RuntimeException(node.toString()) //TODO do I need this or will ANTLR throw its own RTE?
    }
}

internal class TempTriple {
    lateinit var subject: Subject
    lateinit var predicate: Predicate
    lateinit var `object`: Object
}

internal fun handleIRI(iriRef: String): IRI {
    if (iriRef.length > 2) {
        return IRI(iriRef.substring(1, (iriRef.length-1)))
    } else {
        throw RuntimeException("Invalid iriRef - $iriRef")
    }
}

internal fun handleBlankNode(blankNode: String): BlankNode {
    if (blankNode.length > 2) {
        return BlankNode(blankNode.substring(2))
    } else {
        throw RuntimeException("Invalid blank node label - $blankNode")
    }
}

internal fun handleLiteral(literal: NTriplesParser.LiteralContext): Literal {
    val value = if (literal.STRING_LITERAL_QUOTE().text.length >= 2) {
        literal.STRING_LITERAL_QUOTE().text.substring(1, literal.STRING_LITERAL_QUOTE().text.length-1)
    } else {
        throw RuntimeException("Invalid literal.")
    }

    return when {
        literal.LANGTAG() != null -> LangLiteral(value, literal.LANGTAG().text.substring(1))
        literal.IRIREF() != null -> TypedLiteral(value, handleIRI(literal.IRIREF().text))
        else -> TypedLiteral(value)
    }
}