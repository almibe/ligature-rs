/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.turtle

import org.almibe.ligature.IRI
import org.almibe.ligature.Triple
import org.almibe.ligature.parser.TurtleBaseVisitor
import org.almibe.ligature.parser.TurtleLexer
import org.almibe.ligature.parser.TurtleParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream

class Turtle {
    fun parseTurtle(text: String) : List<Triple> {
        val parser = TurtleParserInstance()
        return parser.parseTurtle(text)
    }
}

private class TurtleParserInstance {
    private var base: String = ""
    private val prefixes: MutableMap<String, String> = mutableMapOf()
    private val triples: MutableList<Triple> = mutableListOf()

    fun parseTurtle(text: String) : List<Triple>  {
        val stream = CharStreams.fromString(text)
        val lexer = TurtleLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = TurtleParser(tokens)

        val turtleDocVisitor = TurtleDocVisitor()
        return turtleDocVisitor.visit(parser.turtleDoc())
    }

    inner class TurtleDocVisitor : TurtleBaseVisitor<List<Triple>>() {
        override fun visitTurtleDoc(ctx: TurtleParser.TurtleDocContext): List<Triple> {
            ctx.statement().forEach { statementContext ->
                if (statementContext.directive() != null) {
                    handleDirective(statementContext.directive())//directives mutate state so they don't need a visitor
                } else if (statementContext.triples() != null) {
                    val triplesVisitor = TriplesVisitor()
                    val resultTriples = triplesVisitor.visit(statementContext.triples())
                    triples.addAll(resultTriples)
                } else {
                    throw RuntimeException("Unexpected statement type.")
                }
            }
            return triples
        }
    }

    fun handleDirective(ctx: TurtleParser.DirectiveContext) {
        if (ctx.base() != null) {
            this.base = ctx.base().IRIREF().text
        } else if (ctx.prefixID() != null) {
            this.prefixes[ctx.prefixID().PNAME_NS().text] = ctx.prefixID().IRIREF().text
        } else if (ctx.sparqlBase() != null) {
            this.base = ctx.sparqlBase().IRIREF().text
        } else if (ctx.sparqlPrefix() != null) {
            this.prefixes[ctx.sparqlPrefix().PNAME_NS().text] = ctx.sparqlPrefix().IRIREF().text
        } else {
            throw RuntimeException("Unexpected directive type.")
        }
    }

    inner class TriplesVisitor : TurtleBaseVisitor<List<Triple>>() {
        override fun visitStatement(ctx: TurtleParser.StatementContext): List<Triple> {
            return listOf(Triple(IRI(""), IRI(""), IRI("")))
//        val subject = SubjectVisitor().visitSubject(ctx.subject())
//        val predicate = PredicateVisitor().visitPredicate(ctx.predicate())
//        val `object` = ObjectVisitor().visitObject(ctx.`object`())
//        return Triple(subject, predicate, `object`)
        }
    }
}
