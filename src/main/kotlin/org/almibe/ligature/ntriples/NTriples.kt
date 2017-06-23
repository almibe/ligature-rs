/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.*
import org.almibe.ligature.parser.NTriplesBaseVisitor
import org.almibe.ligature.parser.NTriplesLexer
import org.almibe.ligature.parser.NTriplesParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream

class DocumentVisitor : NTriplesBaseVisitor<List<Triple>>() {
    override fun visitNtriplesDoc(ctx: NTriplesParser.NtriplesDocContext): List<Triple> {
        val tripleVisitor = TripleVisitor()
        return ctx.triple().map {
            it.accept(tripleVisitor)
        }
    }
}

class TripleVisitor : NTriplesBaseVisitor<Triple>() {
    override fun visitTriple(ctx: NTriplesParser.TripleContext): Triple {
        val subject = SubjectVisitor().visitSubject(ctx.subject())
        val predicate = PredicateVisitor().visitPredicate(ctx.predicate())
        val `object` = ObjectVisitor().visitObject(ctx.`object`())
        return Triple(subject, predicate, `object`)
    }
}

class SubjectVisitor : NTriplesBaseVisitor<Subject>() {
    override fun visitSubject(ctx: NTriplesParser.SubjectContext): Subject {
        return when {
            ctx.IRIREF() != null -> handleIRI(ctx.IRIREF().text)
            ctx.BLANK_NODE_LABEL() != null -> handleBlankNode(ctx.BLANK_NODE_LABEL().text)
            else -> throw RuntimeException("Unexpected Subject Type")
        }
    }
}

class PredicateVisitor : NTriplesBaseVisitor<Predicate>() {
    override fun visitPredicate(ctx: NTriplesParser.PredicateContext): Predicate {
        return when {
            ctx.IRIREF() != null -> handleIRI(ctx.IRIREF().text)
            else -> throw RuntimeException("Unexpected Predicate Type")
        }
    }
}

class ObjectVisitor : NTriplesBaseVisitor<Object>() {
    override fun visitObject(ctx: NTriplesParser.ObjectContext): Object {
        return when {
            ctx.IRIREF() != null -> handleIRI(ctx.IRIREF().text)
            ctx.BLANK_NODE_LABEL() != null -> handleBlankNode(ctx.BLANK_NODE_LABEL().text)
            ctx.literal() != null -> LiteralVisitor().visitLiteral(ctx.literal())
            else -> throw RuntimeException("Unexpected Object Type")
        }
    }
}

class LiteralVisitor : NTriplesBaseVisitor<Literal>() {
    override fun visitLiteral(ctx: NTriplesParser.LiteralContext): Literal {
        return when {
            ctx.STRING_LITERAL_QUOTE() != null -> handleLiteral(ctx)
            else -> throw RuntimeException("Unexpected Literal Type")
        }
    }
}

class NTriples {
    fun parseNTriples(text: String) : List<Triple>  {
        val stream = CharStreams.fromString(text)
        val lexer = NTriplesLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = NTriplesParser(tokens)

        val documentVisitor = DocumentVisitor()
        return documentVisitor.visit(parser.ntriplesDoc())
    }
}

fun handleIRI(iriRef: String): IRI {
    if (iriRef.length > 2) {
        return IRI(iriRef.substring(1, (iriRef.length-1)))
    } else {
        throw RuntimeException("Invalid iriRef - $iriRef")
    }
}

fun handleBlankNode(blankNode: String): BlankNode {
    if (blankNode.length > 2) {
        return BlankNode(blankNode.substring(2))
    } else {
        throw RuntimeException("Invalid blank node label - $blankNode")
    }
}

fun handleLiteral(literal: NTriplesParser.LiteralContext): Literal {
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
