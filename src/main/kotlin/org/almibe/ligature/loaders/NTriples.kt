/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import com.google.common.graph.ImmutableNetwork
import com.google.common.graph.MutableNetwork
import com.google.common.graph.NetworkBuilder
import org.almibe.ligature.*
import org.almibe.ligature.parser.ntriples.NTriplesBaseListener
import org.almibe.ligature.parser.ntriples.NTriplesLexer
import org.almibe.ligature.parser.ntriples.NTriplesParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.tree.ErrorNode
import org.antlr.v4.runtime.tree.ParseTreeWalker

class NTriples {
    fun loadNTriples(text: String): ImmutableNetwork<Node, Predicate> {
        val stream = CharStreams.fromString(text)
        val lexer = NTriplesLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = NTriplesParser(tokens)
        val walker = ParseTreeWalker()
        val listener = TriplesNTripleListener()
        walker.walk(listener, parser.ntriplesDoc())
        return ImmutableNetwork.copyOf(listener.subgraph)
    }
}

private class TriplesNTripleListener : NTriplesBaseListener() {
    val subgraph: MutableNetwork<Node, Predicate> = NetworkBuilder.directed().allowsParallelEdges(true)
            .allowsSelfLoops(true).build<Node, Predicate>()
    lateinit var currentTriple: TempTriple
    val blankNodes = HashMap<String, BlankNode>()

    override fun enterTriple(ctx: NTriplesParser.TripleContext) {
        currentTriple = TempTriple()
    }

    override fun exitSubject(ctx: NTriplesParser.SubjectContext) {
        currentTriple.subject = when {
            ctx.IRIREF() != null -> handleIRI(ctx.IRIREF().text)
            ctx.BLANK_NODE_LABEL() != null -> handleBlankNode(ctx.BLANK_NODE_LABEL().text)
            else -> throw RuntimeException("Unexpected Subject Type")
        }
    }

    override fun exitPredicate(ctx: NTriplesParser.PredicateContext) {
        currentTriple.predicate = handleIRI(ctx.IRIREF().text)
    }

    override fun exitObject(ctx: NTriplesParser.ObjectContext) {
        when {
            ctx.IRIREF() != null -> handleObject(handleIRI(ctx.IRIREF().text))
            ctx.BLANK_NODE_LABEL() != null -> handleObject(handleBlankNode(ctx.BLANK_NODE_LABEL().text))
            ctx.literal() != null -> handleLiteral(ctx.literal())
            else -> throw RuntimeException("Unexpected Object Type")
        }
    }

    override fun visitErrorNode(node: ErrorNode) {
        throw RuntimeException(node.toString()) //TODO do I need this or will ANTLR throw its own RTE?
    }

    internal fun handleIRI(iriRef: String): IRI {
        if (iriRef.length > 2) {
            val iri = IRI(iriRef.substring(1, (iriRef.length-1)))
            return iri
        } else {
            throw RuntimeException("Invalid iriRef - $iriRef")
        }
    }

    internal fun handleLiteral(literal: NTriplesParser.LiteralContext) {
        //TODO figure out how to store literals, does OrientDB handle maps in properties
        //TODO or do I need to use an embedded doc/vertx?
        val value = if (literal.STRING_LITERAL_QUOTE().text.length >= 2) {
            literal.STRING_LITERAL_QUOTE().text.substring(1, literal.STRING_LITERAL_QUOTE().text.length-1)
        } else {
            throw RuntimeException("Invalid literal.")
        }

        if (literal.LANGTAG() != null) {
            //LangLiteral(value, literal.LANGTAG().text.substring(1))
            //TODO persist literal
        } else if (literal.IRIREF() != null) {
            //TypedLiteral(value, handleIRI(literal.IRIREF().text))
            //TODO persist literal
        } else {
            //TypedLiteral(value)
            //TODO persist literal
        }
    }

    fun handleBlankNode(blankNode: String): BlankNode {
        if (blankNode.length > 2) {
            val blankNodeLabel = blankNode.substring(2)
            if (blankNodes.containsKey(blankNodeLabel)) {
                return blankNodes[blankNodeLabel]!!
            } else {
                TODO("create new blank node and return after adding to blankNodes map")
            }
        } else {
            throw RuntimeException("Invalid blank node label - $blankNode")
        }
    }

    fun handleObject(objectVertx: Object) {
        subgraph.addEdge(currentTriple.subject, objectVertx, currentTriple.predicate)
    }

    internal class TempTriple {
        lateinit var subject: Subject
        lateinit var predicate: Predicate
    }
}
