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
    private val prefixes: Map<String, String> = mutableMapOf()
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
                    val directiveVisitor = DirectiveVisitor()
                    directiveVisitor.visit(statementContext.directive())
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

    inner class DirectiveVisitor : TurtleBaseVisitor<Any?>() {
        override fun visitDirective(ctx: TurtleParser.DirectiveContext): Any? {
            if (ctx.base() != null) {
                TODO("Complete")
            } else if (ctx.prefixID() != null) {
                TODO("Complete")
            } else if (ctx.sparqlBase() != null) {
                TODO("Complete")
            } else if (ctx.sparqlPrefix() != null) {
                TODO("Complete")
            } else {
                throw RuntimeException("Unexpected directive type.")
            }
            return null //directives just manipulate parser state so they return null
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
