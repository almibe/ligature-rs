/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.parsers

import org.almibe.ligature.*
import org.almibe.ligature.parser.turtle.ModalTurtleLexer
import org.almibe.ligature.parser.turtle.Turtle
import org.almibe.ligature.parser.turtle.TurtleListener
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.ParserRuleContext
import org.antlr.v4.runtime.tree.ErrorNode
import org.antlr.v4.runtime.tree.ParseTreeWalker
import org.antlr.v4.runtime.tree.TerminalNode

class Turtle {
    fun parseTurtle(text: String) : List<Triple> {
        val parser = TurtleInstance()
        return parser.parseTurtle(text)
    }
}

private class TurtleInstance {
    fun parseTurtle(text: String): List<Triple> {
        val stream = CharStreams.fromString(text)
        val lexer = ModalTurtleLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = Turtle(tokens)

        val walker = ParseTreeWalker()
        val listener = TriplesTurtleListener()
        walker.walk(listener, parser.turtleDoc())
        return listener.triples
    }
}

private class TurtleStatement {
    val subjects = mutableListOf<Subject>()
    val blankNodePropertyList = mutableListOf<Pair<IRI, MutableList<Object>>>()
    val predicateObjectList = mutableListOf<Pair<IRI, MutableList<Object>>>()

    fun computeTriples(): List<Triple> {
        if (subjects.size == 1) {
            return predicateObjectList.map { Triple(subjects.first(), it.first, it.second.first()) }
        } else {
            TODO("finish")
        }
    }
}

private class TriplesTurtleListener : TurtleListener {
    val triples = mutableListOf<Triple>()
    val prefixes: MutableMap<String, String> = mutableMapOf()
    lateinit var base: String
    var currentStatement: TurtleStatement = TurtleStatement()

    override fun exitSubject(ctx: Turtle.SubjectContext) {
        //TODO handle all subject logic here
        if (ctx.iri() != null) {
            currentStatement.subjects.add(handleTurtleIRI(ctx.iri()))
        } else if (ctx.collection() != null) {
            TODO()
        } else if (ctx.blankNode() != null) {
            TODO()
        } else {
            throw RuntimeException("Unexpected subject.")
        }
    }

    override fun exitVerbObjectList(ctx: Turtle.VerbObjectListContext) {
        val iri = if (ctx.verb().text != null && !ctx.verb().text.equals("a")) {
            handleTurtleIRI(ctx.verb().predicate().iri())
        } else {
            IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
        }
        ctx.objectList().`object`().forEach {
            val `object`: Object = handleObject(it)
            currentStatement.predicateObjectList.add(Pair(iri, mutableListOf(`object`)))
        }
    }

    override fun exitBlankNodePropertyList(ctx: Turtle.BlankNodePropertyListContext) {
        //TODO handle all blankNodePropertyList logic here
    }

    override fun exitTriples(ctx: Turtle.TriplesContext) {
        triples.addAll(currentStatement.computeTriples())
    }

    override fun enterTriples(ctx: Turtle.TriplesContext) {
        currentStatement = TurtleStatement()
    }

    override fun exitBase(ctx: Turtle.BaseContext) {
//        if (ctx.iriRef().text.length >= 2) {
//            this.base = ctx.iriRef().text.trim('<', '>')
//        } else {
//            throw RuntimeException("Unexpected base ${ctx.iriRef().text}.")
//        }
    }

    override fun exitSparqlBase(ctx: Turtle.SparqlBaseContext) {
//        if (ctx.iriRef().text.length >= 2) {
//            this.base = ctx.iriRef().text.trim('<', '>')
//        } else {
//            throw RuntimeException("Unexpected sparql base ${ctx.iriRef().text}.")
//        }
    }

    override fun exitPrefixID(ctx: Turtle.PrefixIDContext) {
//        if (ctx.PNAME_NS() != null)  {
//            this.prefixes[ctx.PNAME_NS().text.trimEnd(':')] = handleIRIRef(ctx.iriRef())
//        } else {
//            throw RuntimeException("Unexpected prefix ${ctx.text}")
//        }
    }

    override fun exitSparqlPrefix(ctx: Turtle.SparqlPrefixContext) {
//        if (ctx.iriRef().text.length >= 2) {
//           this.prefixes[ctx.PNAME_NS().text.trimEnd(':')] = handleIRIRef(ctx.iriRef())
//        } else {
//            throw RuntimeException("Unexpected sparql base ${ctx.iriRef().text}.")
//        }
    }

    fun handleTurtleIRI(ctx: Turtle.IriContext): IRI {
        return if (ctx.PREFIXED_NAME() != null) {
            val prefix = ctx.PREFIXED_NAME().text.split(":")
            if (prefix.size == 1) {
                IRI(prefixes[""] + prefix[0])
            } else if (prefix.size == 2) {
                IRI(prefixes[prefix[0]] + prefix[1])
            } else {
                throw RuntimeException("Unexpected IRI prefix value ${ctx.PREFIXED_NAME().text}")
            }
        } else if (ctx.ABSOLUTE_IRI() != null) {
            IRI(ctx.ABSOLUTE_IRI().text)
        } else if (ctx.RELATIVE_IRI() != null) {
            IRI(base + ctx.RELATIVE_IRI().text)
        } else {
            throw RuntimeException("Unexpected IRI type")
        }
    }

    //non ANTRL member methods
    internal fun handleObject(ctx: Turtle.ObjectContext): Object {
        return when {
            ctx.literal() != null -> handleTurtleLiteral(ctx.literal())
            ctx.blankNode() != null -> handleBlankNode(ctx.blankNode().text)
            ctx.iri() != null -> handleTurtleIRI(ctx.iri())
            ctx.blankNodePropertyList() != null -> TODO()
            ctx.collection() != null -> TODO()
            else -> throw RuntimeException("Unexpected object")
        }
    }

    internal fun handleTurtleLiteral(ctx: Turtle.LiteralContext): Literal {
        return when {
            ctx.booleanLiteral() != null -> handleBooleanLiteral(ctx.booleanLiteral())
            ctx.numericLiteral() != null  -> handleNumericLiteral(ctx.numericLiteral())
            ctx.rdfLiteral() != null  -> handleRdfLiteral(ctx.rdfLiteral())
            else -> throw RuntimeException("Unexpected literal")
        }
    }

    fun  handleBooleanLiteral(ctx: Turtle.BooleanLiteralContext): Literal {
        TODO()
    }

    fun  handleNumericLiteral(ctx: Turtle.NumericLiteralContext): Literal {
        TODO()
    }

    internal fun handleRdfLiteral(ctx: Turtle.RdfLiteralContext): Literal {
        val value = if (ctx.string().text.length >= 2) {
            ctx.string().text.trim('"', '\'')
        } else {
            throw RuntimeException("Invalid literal.")
        }
        return when {
            ctx.LANGTAG() != null -> LangLiteral(value, ctx.LANGTAG().text.substring(1))
            ctx.iri() != null -> TypedLiteral(value, handleTurtleIRI(ctx.iri()))
            else -> TypedLiteral(value)
        }
    }

    //ANTRL methods that aren't being used currently / will be removed when switching to ABC
    override fun exitBlankNode(ctx: Turtle.BlankNodeContext) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitLiteral(ctx: Turtle.LiteralContext) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitCollection(ctx: Turtle.CollectionContext) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun visitErrorNode(node: ErrorNode?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitVerb(ctx: Turtle.VerbContext) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitBooleanLiteral(ctx: Turtle.BooleanLiteralContext) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitObject(ctx: Turtle.ObjectContext) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitTurtleDoc(ctx: Turtle.TurtleDocContext) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitRdfLiteral(ctx: Turtle.RdfLiteralContext) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun visitTerminal(node: TerminalNode?) { /* do nothing */ }
    override fun enterString(ctx: Turtle.StringContext) { /* do nothing */ }
    override fun exitPredicate(ctx: Turtle.PredicateContext) { /* do nothing */ }
    override fun enterDirective(ctx: Turtle.DirectiveContext) { /* do nothing */ }
    override fun exitPredicateObjectList(ctx: Turtle.PredicateObjectListContext) { /* do nothing */ }
    override fun enterPredicate(ctx: Turtle.PredicateContext) { /* do nothing */ }
    override fun exitIri(ctx: Turtle.IriContext) { /* do nothing */ }
    override fun enterObjectList(ctx: Turtle.ObjectListContext) { /* do nothing */ }
    override fun enterVerbObjectList(ctx: Turtle.VerbObjectListContext) { /* do nothing */}
    override fun enterVerb(ctx: Turtle.VerbContext) { /* do nothing */ }
    override fun enterSparqlPrefix(ctx: Turtle.SparqlPrefixContext) { /* do nothing */ }
    override fun enterBlankNode(ctx: Turtle.BlankNodeContext) { /* do nothing */ }
    override fun enterBooleanLiteral(ctx: Turtle.BooleanLiteralContext) { /* do nothing */ }
    override fun enterBlankNodePropertyList(ctx: Turtle.BlankNodePropertyListContext) { /* do nothing */ }
    override fun enterRdfLiteral(ctx: Turtle.RdfLiteralContext) { /* do nothing */ }
    override fun enterObject(ctx: Turtle.ObjectContext) { /* do nothing */ }
    override fun enterCollection(ctx: Turtle.CollectionContext) { /* do nothing */ }
    override fun enterLiteral(ctx: Turtle.LiteralContext) { /* do nothing */ }
    override fun enterEveryRule(ctx: ParserRuleContext) { /* do nothing */ }
    override fun enterPredicateObjectList(ctx: Turtle.PredicateObjectListContext) { /* do nothing */ }
    override fun enterSparqlBase(ctx: Turtle.SparqlBaseContext) { /* do nothing */ }
    override fun enterPrefixID(ctx: Turtle.PrefixIDContext) { /* do nothing */ }
    override fun enterBase(ctx: Turtle.BaseContext) { /* do nothing */ }
    override fun enterNumericLiteral(ctx: Turtle.NumericLiteralContext) { /* do nothing */ }
    override fun enterTurtleDoc(ctx: Turtle.TurtleDocContext) { /* do nothing */ }
    override fun enterIri(ctx: Turtle.IriContext) { /* do nothing */ }
    override fun enterSubject(ctx: Turtle.SubjectContext) { /* do nothing */ }
    override fun enterStatement(ctx: Turtle.StatementContext) { /* do nothing */ }
    override fun exitStatement(ctx: Turtle.StatementContext) { /* do nothing */ }
    override fun exitString(ctx: Turtle.StringContext) { /* do nothing */ }
    override fun exitObjectList(ctx: Turtle.ObjectListContext) { /* do nothing */ }
    override fun exitNumericLiteral(ctx: Turtle.NumericLiteralContext) { /* do nothing */ }
    override fun exitEveryRule(ctx: ParserRuleContext) { /* do nothing */ }
    override fun exitDirective(ctx: Turtle.DirectiveContext) { /* do nothing */ }
}
