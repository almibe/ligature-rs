/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.*
import org.almibe.ligature.parser.ntriples.NTriplesBaseListener
import org.almibe.ligature.parser.ntriples.NTriplesParser
import org.almibe.ligature.store.InMemoryStore
import org.antlr.v4.runtime.tree.ErrorNode
import java.io.Reader
import java.io.Writer

class NTriples: Parser {
    override fun import(reader: Reader, store: Store, defaultGraph: IRI?) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun export(writer: Writer, graphs: Collection<Graph>) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

}
//{
//    fun loadNTriples(reader: Reader): Graph {
//        val stream = CharStreams.fromReader(reader)
//        val lexer = NTriplesLexer(stream)
//        val tokens = CommonTokenStream(lexer)
//        val parser = NTriplesParser(tokens)
//        val walker = ParseTreeWalker()
//        val listener = TriplesNTripleListener()
//        walker.walk(listener, parser.ntriplesDoc())
//        return listener.model
//    }
//}

private class TriplesNTripleListener : NTriplesBaseListener() {
    val model = InMemoryStore()
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
            return IRI(iriRef.substring(1, (iriRef.length-1)))
        } else {
            throw RuntimeException("Invalid iriRef - $iriRef")
        }
    }

    internal fun handleLiteral(literal: NTriplesParser.LiteralContext) {
        val value = if (literal.STRING_LITERAL_QUOTE().text.length >= 2) {
            literal.STRING_LITERAL_QUOTE().text.substring(1, literal.STRING_LITERAL_QUOTE().text.length-1)
        } else {
            throw RuntimeException("Invalid literal.")
        }
        val result = when {
            literal.LANGTAG() != null -> LangLiteral(value, literal.LANGTAG().text.substring(1))
            literal.IRIREF() != null -> TypedLiteral(value, handleIRI(literal.IRIREF().text))
            else -> TypedLiteral(value)
        }
        model.addStatement(currentTriple.subject, currentTriple.predicate, result)
    }

    fun handleBlankNode(blankNode: String): BlankNode {
        return if (blankNode.length > 2) {
            val blankNodeLabel = blankNode.substring(2)
            if (blankNodes.containsKey(blankNodeLabel)) {
                blankNodes[blankNodeLabel]!!
            } else {
                val newBlankNode = BlankNode(blankNodeLabel)
                blankNodes[blankNodeLabel] = newBlankNode
                newBlankNode
            }
        } else {
            throw RuntimeException("Invalid blank node label - $blankNode")
        }
    }

    fun handleObject(objectVertex: Object) {
        model.addStatement(currentTriple.subject, currentTriple.predicate, objectVertex)
    }

    internal class TempTriple {
        lateinit var subject: Subject
        lateinit var predicate: Predicate
    }
}
