/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.turtle

import org.almibe.ligature.Subject
import org.almibe.ligature.Triple
import org.almibe.ligature.ntriples.TempTriple
import org.almibe.ligature.parser.TurtleLexer
import org.almibe.ligature.parser.TurtleListener
import org.almibe.ligature.parser.TurtleParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.ParserRuleContext
import org.antlr.v4.runtime.tree.ErrorNode
import org.antlr.v4.runtime.tree.ParseTreeWalker
import org.antlr.v4.runtime.tree.TerminalNode

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

    fun parseTurtle(text: String): List<Triple> {
        val stream = CharStreams.fromString(text)
        val lexer = TurtleLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = TurtleParser(tokens)

        val walker = ParseTreeWalker()
        val listener = TriplesTurtleListener()
        walker.walk(listener, parser.turtleDoc())
        return listener.triples
    }
}

private class TriplesTurtleListener : TurtleListener {
    val triples = mutableListOf<Triple>()
    val prefixes: MutableMap<String, String> = mutableMapOf()
    lateinit var base: String
    lateinit var currentTriple: TempTriple
    lateinit var currentSubject: Subject

    override fun enterTurtleDoc(ctx: TurtleParser.TurtleDocContext) {
        //do nothing
    }

    override fun exitNumericLiteral(ctx: TurtleParser.NumericLiteralContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitSubject(ctx: TurtleParser.SubjectContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitBlankNode(ctx: TurtleParser.BlankNodeContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitLiteral(ctx: TurtleParser.LiteralContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitCollection(ctx: TurtleParser.CollectionContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitPredicate(ctx: TurtleParser.PredicateContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterBlankNode(ctx: TurtleParser.BlankNodeContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitSparqlBase(ctx: TurtleParser.SparqlBaseContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterBooleanLiteral(ctx: TurtleParser.BooleanLiteralContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterBlankNodePropertyList(ctx: TurtleParser.BlankNodePropertyListContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterRdfLiteral(ctx: TurtleParser.RdfLiteralContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun visitErrorNode(node: ErrorNode?) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitVerb(ctx: TurtleParser.VerbContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterObject(ctx: TurtleParser.ObjectContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterCollection(ctx: TurtleParser.CollectionContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitIri(ctx: TurtleParser.IriContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitBooleanLiteral(ctx: TurtleParser.BooleanLiteralContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitObject(ctx: TurtleParser.ObjectContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterLiteral(ctx: TurtleParser.LiteralContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitBlankNodePropertyList(ctx: TurtleParser.BlankNodePropertyListContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterEveryRule(ctx: ParserRuleContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitEveryRule(ctx: ParserRuleContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterPredicateObjectList(ctx: TurtleParser.PredicateObjectListContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterSparqlBase(ctx: TurtleParser.SparqlBaseContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterPrefixedName(ctx: TurtleParser.PrefixedNameContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterPrefixID(ctx: TurtleParser.PrefixIDContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitTurtleDoc(ctx: TurtleParser.TurtleDocContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitRdfLiteral(ctx: TurtleParser.RdfLiteralContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterBase(ctx: TurtleParser.BaseContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterNumericLiteral(ctx: TurtleParser.NumericLiteralContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitString(ctx: TurtleParser.StringContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterStatement(ctx: TurtleParser.StatementContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitStatement(ctx: TurtleParser.StatementContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterTriples(ctx: TurtleParser.TriplesContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterIri(ctx: TurtleParser.IriContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitDirective(ctx: TurtleParser.DirectiveContext) {
        //TODO rewrite this to use individual exit methods instead of having an if check here
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

    override fun enterDirective(ctx: TurtleParser.DirectiveContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitTriples(ctx: TurtleParser.TriplesContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitObjectList(ctx: TurtleParser.ObjectListContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterSubject(ctx: TurtleParser.SubjectContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitPrefixedName(ctx: TurtleParser.PrefixedNameContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun visitTerminal(node: TerminalNode?) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterString(ctx: TurtleParser.StringContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitPrefixID(ctx: TurtleParser.PrefixIDContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitSparqlPrefix(ctx: TurtleParser.SparqlPrefixContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitBase(ctx: TurtleParser.BaseContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitPredicateObjectList(ctx: TurtleParser.PredicateObjectListContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterObjectList(ctx: TurtleParser.ObjectListContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterVerb(ctx: TurtleParser.VerbContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterSparqlPrefix(ctx: TurtleParser.SparqlPrefixContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterPredicate(ctx: TurtleParser.PredicateContext) {
        TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }
}
