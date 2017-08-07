/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import com.orientechnologies.orient.core.db.ODatabasePool
import com.orientechnologies.orient.core.id.ORID
import com.orientechnologies.orient.core.record.OVertex
import org.almibe.ligature.*
import org.almibe.ligature.parser.ntriples.NTriplesBaseListener
import org.almibe.ligature.parser.ntriples.NTriplesLexer
import org.almibe.ligature.parser.ntriples.NTriplesParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.tree.ErrorNode
import org.antlr.v4.runtime.tree.ParseTreeWalker

class NTriples(val dbPool: ODatabasePool) {
    fun loadNTriples(text: String): Set<ORID> {
        val stream = CharStreams.fromString(text)
        val lexer = NTriplesLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = NTriplesParser(tokens)
        val walker = ParseTreeWalker()
        val listener = TriplesNTripleListener(dbPool)
        walker.walk(listener, parser.ntriplesDoc())
        return listener.orids
    }
}

private class TriplesNTripleListener(val dbPool: ODatabasePool) : NTriplesBaseListener() {
    val orids = HashSet<ORID>()
    lateinit var currentTriple: TempTriple
    val blankNodes = HashMap<String, OVertex>()

    override fun enterTriple(ctx: NTriplesParser.TripleContext) {
        currentTriple = TempTriple()
    }

    override fun exitSubject(ctx: NTriplesParser.SubjectContext) {
        currentTriple.subject = when {
            ctx.IRIREF() != null -> handleIRI(ctx.IRIREF().text)
            ctx.BLANK_NODE_LABEL() != null -> handleBlankNode(ctx.BLANK_NODE_LABEL().text)
            else -> throw RuntimeException("Unexpected Subject Type")
        }
        orids.add(currentTriple.subject.identity)
    }

    override fun exitPredicate(ctx: NTriplesParser.PredicateContext) {
        val predicate: String = when {
            ctx.IRIREF() != null -> ctx.IRIREF().text
            else -> throw RuntimeException("Unexpected Predicate Type")
        }
        currentTriple.predicate = predicate
    }

    override fun exitObject(ctx: NTriplesParser.ObjectContext) {
        if (ctx.IRIREF() != null) {
            handleIRI(ctx.IRIREF().text)
        } else if (ctx.BLANK_NODE_LABEL() != null) {
            handleBlankNode(ctx.BLANK_NODE_LABEL().text)
        } else if (ctx.literal() != null) {
            handleLiteral(ctx.literal())
        } else {
            throw RuntimeException("Unexpected Object Type")
        }
    }

    override fun visitErrorNode(node: ErrorNode) {
        throw RuntimeException(node.toString()) //TODO do I need this or will ANTLR throw its own RTE?
    }

    internal fun handleIRI(iriRef: String): OVertex {
        if (iriRef.length > 2) {
            val iri = IRI(iriRef.substring(1, (iriRef.length-1)))
            //TODO if iri exists return existing OVertex
            //TODO if not then persist iri in this method and return new OVertx
        } else {
            throw RuntimeException("Invalid iriRef - $iriRef")
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

    fun handleBlankNode(blankNode: String): OVertex {
        if (blankNode.length > 2) {
            val blankNodeLabel = blankNode.substring(2)
            if (blankNodes.containsKey(blankNodeLabel)) {
                return blankNodes[blankNodeLabel]!!
            } else {
                //TODO create new blank node and return after adding to blankNodes map
            }
        } else {
            throw RuntimeException("Invalid blank node label - $blankNode")
        }
    }

    internal class TempTriple {
        lateinit var subject: OVertex
        lateinit var predicate: String
    }
}
