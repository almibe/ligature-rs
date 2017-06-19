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
    override fun visitDocument(ctx: NTriplesParser.DocumentContext): List<Triple> {
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
            ctx.iri() != null -> IRIVisitor().visitIri(ctx.iri())
            ctx.blankNode() != null -> BlankNodeVisitor().visitBlankNode(ctx.blankNode())
            else -> throw RuntimeException("Unexpected Subject Type")
        }
    }
}

class PredicateVisitor : NTriplesBaseVisitor<Predicate>() {
    override fun visitPredicate(ctx: NTriplesParser.PredicateContext): Predicate {
        return when {
            ctx.iri() != null -> IRIVisitor().visitIri(ctx.iri())
            else -> throw RuntimeException("Unexpected Predicate Type")
        }
    }
}

class ObjectVisitor : NTriplesBaseVisitor<Object>() {
    override fun visitObject(ctx: NTriplesParser.ObjectContext): Object {
        return when {
            ctx.iri() != null -> IRIVisitor().visitIri(ctx.iri())
            ctx.blankNode() != null -> BlankNodeVisitor().visitBlankNode(ctx.blankNode())
            ctx.literal() != null -> LiteralVisitor().visitLiteral(ctx.literal())
            ctx.langLiteral() != null -> LangLiteralVisitor().visitLangLiteral(ctx.langLiteral())
            ctx.typedLiteral() != null -> TypedLiteralVisitor().visitTypedLiteral(ctx.typedLiteral())
            else -> throw RuntimeException("Unexpected Object Type")
        }
    }
}

class IRIVisitor : NTriplesBaseVisitor<IRI>() {
    override fun visitIri(ctx: NTriplesParser.IriContext): IRI {
        TODO(ctx.text)
    }
}

class BlankNodeVisitor : NTriplesBaseVisitor<BlankNode>() {
    override fun visitBlankNode(ctx: NTriplesParser.BlankNodeContext): BlankNode {
        TODO()
    }
}

class LiteralVisitor : NTriplesBaseVisitor<Literal>() {
    override fun visitLiteral(ctx: NTriplesParser.LiteralContext): Literal {
        TODO()
    }
}

class TypedLiteralVisitor : NTriplesBaseVisitor<TypedLiteral>() {
    override fun visitTypedLiteral(ctx: NTriplesParser.TypedLiteralContext): TypedLiteral {
        TODO()
    }
}

class LangLiteralVisitor : NTriplesBaseVisitor<LangLiteral>() {
    override fun visitLangLiteral(ctx: NTriplesParser.LangLiteralContext): LangLiteral {
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

    fun parseSubject(text: String) : Subject {
        val stream = CharStreams.fromString(text)
        val lexer = NTriplesLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = NTriplesParser(tokens)

        val documentVisitor = SubjectVisitor()
        return documentVisitor.visit(parser.subject())
    }

    fun parsePredicate(text: String) : Predicate {
        TODO()
    }

    fun parseObject(text: String) : Object {
        TODO()
    }
}
