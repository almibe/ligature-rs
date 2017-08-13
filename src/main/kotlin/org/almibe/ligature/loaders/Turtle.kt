/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import org.almibe.ligature.*
import org.almibe.ligature.parser.turtle.ModalTurtleLexer
import org.almibe.ligature.parser.turtle.Turtle
import org.almibe.ligature.parser.turtle.TurtleBaseVisitor
import org.almibe.ligature.parser.turtle.TurtleListener
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.ParserRuleContext
import org.antlr.v4.runtime.tree.ErrorNode
import org.antlr.v4.runtime.tree.TerminalNode
import java.util.*

class Turtle {
    fun loadTurtle(text: String): ReadOnlyModel {
        val stream = CharStreams.fromString(text)
        val lexer = ModalTurtleLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = Turtle(tokens)
        return TurtleDocVisitor().visitTurtleDoc(parser.turtleDoc())
    }
}

val integerIRI = IRI("http://www.w3.org/2001/XMLSchema#integer")
val doubleIRI = IRI("http://www.w3.org/2001/XMLSchema#double")
val decimalIRI = IRI("http://www.w3.org/2001/XMLSchema#float")
val typeIRI = IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
val booleanIRI = IRI("http://www.w3.org/2001/XMLSchema#boolean")

/** Temporary class used to hold data while parsing that will eventually be used to create RdfModel classes */
private class TurtleStatement {
    val subjects = mutableListOf<Subject>()
    val predicateObjectList = mutableListOf<Pair<IRI, MutableList<Object>>>()

    fun computeTriples(): List<Triple<Subject, Predicate, Object>> {
        if (subjects.size == 1) {
            return predicateObjectList.map {
                Triple(subjects.first(), it.first, it.second.first())
            }
        } else {
            TODO("finish")
        }
    }
}

private class TurtleDocVisitor: TurtleBaseVisitor<Model>() {
    val model = InMemoryModel()
    val prefixes: MutableMap<String, String> = mutableMapOf()
    lateinit var base: String
    var anonymousCounter = 0
    val blankNodes = HashMap<String, BlankNode>()

    override fun visitTurtleDoc(ctx: Turtle.TurtleDocContext): Model {
        ctx.statement().forEach { statement ->
            when {
                statement.directive() != null -> handleDirective(statement.directive())
                statement.triples() != null -> handleTriples(statement.triples())
            }
        }
        return model
    }

    fun handleDirective(directiveContext: Turtle.DirectiveContext) {
        when {
            directiveContext.base() != null -> handleBase(directiveContext.base())
            directiveContext.prefixID() != null -> handlePrefixID(directiveContext.prefixID())
            directiveContext.sparqlBase() != null -> handleSparqlBase(directiveContext.sparqlBase())
            directiveContext.sparqlPrefix() != null -> handleSparqlPrefix(directiveContext.sparqlPrefix())
        }
    }

    fun handleBase(ctx: Turtle.BaseContext) {
        if (ctx.iriRef().text.length >= 2) {
            this.base = ctx.iriRef().text.trim('<', '>')
        } else {
            throw RuntimeException("Unexpected base ${ctx.iriRef().text}.")
        }
    }

    fun handleSparqlBase(ctx: Turtle.SparqlBaseContext) {
        if (ctx.iriRef().text.length >= 2) {
            this.base = ctx.iriRef().text.trim('<', '>')
        } else {
            throw RuntimeException("Unexpected sparql base ${ctx.iriRef().text}.")
        }
    }

    fun handlePrefixID(ctx: Turtle.PrefixIDContext) {
        if (ctx.PNAME_NS() != null)  {
            this.prefixes[ctx.PNAME_NS().text.trimEnd(':')] = handleTurtleIRIRef(ctx.iriRef())
        } else {
            throw RuntimeException("Unexpected prefix ${ctx.text}")
        }
    }

    fun handleSparqlPrefix(ctx: Turtle.SparqlPrefixContext) {
        if (ctx.iriRef().text.length >= 2) {
            this.prefixes[ctx.PNAME_NS().text.trimEnd(':')] = handleTurtleIRIRef(ctx.iriRef())
        } else {
            throw RuntimeException("Unexpected sparql base ${ctx.iriRef().text}.")
        }
    }

    fun handleTriples(triplesContext: Turtle.TriplesContext) {
        if (triplesContext.subject() != null) {
            val subject = handleTurtleIRI(triplesContext.subject().iri())
            val predicateObjectList = mutableListOf<Pair<IRI, MutableList<Object>>>()
            triplesContext.predicateObjectList().verbObjectList().forEach { verbObjectList ->
                predicateObjectList.addAll(handleVerbObjectList(verbObjectList))
            }
            predicateObjectList.forEach { (predicate, objects) ->
                objects.forEach { `object` ->
                    model.addStatement(subject, predicate, `object`)
                }
            }
        } else if (triplesContext.blankNodePropertyList() != null) {
            TODO()
        } else {
            throw RuntimeException("Unexpected triples values ${triplesContext.text}.")
        }
    }

    fun handleVerbObjectList(ctx: Turtle.VerbObjectListContext): List<Pair<IRI, MutableList<Object>>> {
        val result = mutableListOf<Pair<IRI, MutableList<Object>>>()
        val iri = if (ctx.verb().text != null && !ctx.verb().text.equals("a")) {
            handleTurtleIRI(ctx.verb().predicate().iri())
        } else {
            typeIRI
        }
        ctx.objectList().`object`().forEach {
            val objects = handleObject(it)
            result.add(Pair(iri, objects))
        }
        return result
    }

    private fun handleObject(ctx: Turtle.ObjectContext): MutableList<Object> { //TODO make this return a collection of Objects or do something else?
        return when {
            ctx.literal() != null -> mutableListOf(handleTurtleLiteral(ctx.literal()))
            ctx.blankNode() != null -> mutableListOf(handleTurtleBlankNode(ctx.blankNode()))
            ctx.iri() != null -> mutableListOf(handleTurtleIRI(ctx.iri()))
            ctx.blankNodePropertyList() != null -> mutableListOf(handleBlankNodePropertyList(ctx.blankNodePropertyList()))
            ctx.collection() != null -> TODO()
            else -> throw RuntimeException("Unexpected object")
        }
    }

    private fun handleTurtleLiteral(ctx: Turtle.LiteralContext): Literal {
        return when {
            ctx.booleanLiteral() != null -> handleBooleanLiteral(ctx.booleanLiteral())
            ctx.numericLiteral() != null  -> handleNumericLiteral(ctx.numericLiteral())
            ctx.rdfLiteral() != null  -> handleRdfLiteral(ctx.rdfLiteral())
            else -> throw RuntimeException("Unexpected literal")
        }
    }

    private fun handleBooleanLiteral(ctx: Turtle.BooleanLiteralContext): Literal {
        return TypedLiteral(ctx.text, booleanIRI)
    }

    private fun handleNumericLiteral(ctx: Turtle.NumericLiteralContext): Literal {
        return if (ctx.DECIMAL() != null) {
            TypedLiteral(ctx.DECIMAL().text, decimalIRI)
        } else if (ctx.DOUBLE() != null) {
            TypedLiteral(ctx.DOUBLE().text, doubleIRI)
        } else if (ctx.INTEGER() != null) {
            TypedLiteral(ctx.INTEGER().text, integerIRI)
        } else {
            throw RuntimeException("Unexpected Numeric type")
        }
    }

    private fun handleRdfLiteral(ctx: Turtle.RdfLiteralContext): Literal {
        val value = extractStringLiteralValue(ctx.string())
        return when {
            ctx.LANGTAG() != null -> LangLiteral(value, ctx.LANGTAG().text.substring(1))
            ctx.iri() != null -> TypedLiteral(value, handleTurtleIRI(ctx.iri()))
            else -> TypedLiteral(value)
        }
    }

    private fun extractStringLiteralValue(ctx: Turtle.StringContext): String {
        return when {
            ctx.START_SINGLE_QUOTE() != null -> ctx.STRING_CONTENT_SINGLE_QUOTE()
            ctx.START_DOUBLE_QUOTE() != null -> ctx.STRING_CONTENT_DOUBLE_QUOTE()
            ctx.START_TRIPLE_SINGLE_QUOTE() != null -> ctx.STRING_CONTENT_TRIPLE_SINGLE_QUOTE()
            ctx.START_TRIPLE_DOUBLE_QUOTE() != null -> ctx.STRING_CONTENT_TRIPLE_DOUBLE_QUOTE()
            else -> throw RuntimeException("Unexpected String type")
        }?.text ?: ""
    }

    private fun handleTurtleBlankNode(ctx: Turtle.BlankNodeContext): BlankNode {
        return if (ctx.ANON() != null) {
            handleBlankNode("ANON${++anonymousCounter}")
        } else if (ctx.BLANK_NODE_LABEL() != null) {
            if (ctx.BLANK_NODE_LABEL().text.length > 2) {
                handleBlankNode(ctx.BLANK_NODE_LABEL().text.substring(2))
            } else {
                throw RuntimeException("Invalid blank node label - ${ctx.BLANK_NODE_LABEL().text}")
            }
        } else {
            throw RuntimeException("Unexpected blank node - ${ctx.text}")
        }
    }

    private fun handleBlankNode(blankNode: String): BlankNode {
        if (blankNodes.containsKey(blankNode)) {
            return blankNodes[blankNode]!!
        } else {
            val newBlankNode = BlankNode(blankNode)
            blankNodes[blankNode] = newBlankNode
            return newBlankNode
        }
    }

    private fun handleTurtleIRI(ctx: Turtle.IriContext): IRI {
        return if (ctx.PREFIXED_NAME() != null) {
            val prefix = ctx.PREFIXED_NAME().text.split(":")
            if (prefix.size == 1) {
                IRI(prefixes[""] + prefix[0])
            } else if (prefix.size == 2) {
                IRI(prefixes[prefix[0]] + prefix[1])
            } else {
                throw RuntimeException("Unexpected IRI prefix value ${ctx.PREFIXED_NAME().text}")
            }
        } else {
            IRI(handleTurtleIRIRef(ctx.iriRef()))
        }
    }

    private fun handleTurtleIRIRef(ctx: Turtle.IriRefContext): String {
        return if (ctx.ABSOLUTE_IRI() != null) {
            ctx.ABSOLUTE_IRI().text
        } else if (ctx.RELATIVE_IRI() != null) {
            base + ctx.RELATIVE_IRI().text
        } else {
            throw RuntimeException("Unexpected IRI type")
        }
    }

    private fun handleBlankNodePropertyList(ctx: Turtle.BlankNodePropertyListContext): Object {
//        val statement = TurtleStatement()
//        currentStatement.push(statement)
        val node = handleBlankNode("ANON${++anonymousCounter}")
//        statement.subjects.add(node)
//        ctx.predicateObjectList().verbObjectList().forEach { verbObjectList ->
//            val predicate = handleTurtleIRI(verbObjectList.verb().predicate().iri())
//            verbObjectList.objectList().`object`().forEach { objectContext ->
//                handleObject(objectContext)//.forEach { `object` ->
//                //model.addStatement(node, predicate, `object`)
//                //}
//            }
//        }
//        currentStatement.peek().computeTriples().forEach { (subject, predicate, `object`) ->
//            model.addStatement(subject, predicate, `object`)
//        }
//        currentStatement.pop()
        return node
    }
}

//TODO ************************************** eventually remove this class *********************************************
private class TriplesTurtleListener : TurtleListener {
    val model = InMemoryModel()
    val prefixes: MutableMap<String, String> = mutableMapOf()
    lateinit var base: String
    var currentStatement: Stack<TurtleStatement> = Stack()
    var anonymousCounter = 0
    val blankNodes = HashMap<String, BlankNode>()

    override fun exitSubject(ctx: Turtle.SubjectContext) {
        if (ctx.iri() != null) {
            currentStatement.peek().subjects.add(handleTurtleIRI(ctx.iri()))
        } else if (ctx.collection() != null) {
            TODO()
        } else if (ctx.blankNode() != null) {
            currentStatement.peek().subjects.add(handleTurtleBlankNode(ctx.blankNode()))
        } else {
            throw RuntimeException("Unexpected subject.")
        }
    }

    override fun exitVerbObjectList(ctx: Turtle.VerbObjectListContext) {
        val iri = if (ctx.verb().text != null && !ctx.verb().text.equals("a")) {
            handleTurtleIRI(ctx.verb().predicate().iri())
        } else {
            typeIRI
        }
        ctx.objectList().`object`().forEach {
            val objects = handleObject(it)
            currentStatement.peek().predicateObjectList.add(Pair(iri, objects)) //TODO this method shouldn't be called here since we might be working on a blank node and not the current statement
        }
    }

    override fun exitBlankNodePropertyList(ctx: Turtle.BlankNodePropertyListContext) {
    }

    override fun exitTriples(ctx: Turtle.TriplesContext) {
        currentStatement.peek().computeTriples().forEach { (subject, predicate, `object`) ->
            model.addStatement(subject, predicate, `object`)
        }
        currentStatement.pop()
    }

    override fun enterTriples(ctx: Turtle.TriplesContext) {
        currentStatement.push(TurtleStatement())
    }

    override fun exitBase(ctx: Turtle.BaseContext) {
        if (ctx.iriRef().text.length >= 2) {
            this.base = ctx.iriRef().text.trim('<', '>')
        } else {
            throw RuntimeException("Unexpected base ${ctx.iriRef().text}.")
        }
    }

    override fun exitSparqlBase(ctx: Turtle.SparqlBaseContext) {
        if (ctx.iriRef().text.length >= 2) {
            this.base = ctx.iriRef().text.trim('<', '>')
        } else {
            throw RuntimeException("Unexpected sparql base ${ctx.iriRef().text}.")
        }
    }

    override fun exitPrefixID(ctx: Turtle.PrefixIDContext) {
        if (ctx.PNAME_NS() != null)  {
            this.prefixes[ctx.PNAME_NS().text.trimEnd(':')] = handleTurtleIRIRef(ctx.iriRef())
        } else {
            throw RuntimeException("Unexpected prefix ${ctx.text}")
        }
    }

    override fun exitSparqlPrefix(ctx: Turtle.SparqlPrefixContext) {
        if (ctx.iriRef().text.length >= 2) {
           this.prefixes[ctx.PNAME_NS().text.trimEnd(':')] = handleTurtleIRIRef(ctx.iriRef())
        } else {
            throw RuntimeException("Unexpected sparql base ${ctx.iriRef().text}.")
        }
    }

    private fun handleTurtleIRI(ctx: Turtle.IriContext): IRI {
        return if (ctx.PREFIXED_NAME() != null) {
            val prefix = ctx.PREFIXED_NAME().text.split(":")
            if (prefix.size == 1) {
                IRI(prefixes[""] + prefix[0])
            } else if (prefix.size == 2) {
                IRI(prefixes[prefix[0]] + prefix[1])
            } else {
                throw RuntimeException("Unexpected IRI prefix value ${ctx.PREFIXED_NAME().text}")
            }
        } else {
            IRI(handleTurtleIRIRef(ctx.iriRef()))
        }
    }

    private fun handleTurtleIRIRef(ctx: Turtle.IriRefContext): String {
        return if (ctx.ABSOLUTE_IRI() != null) {
            ctx.ABSOLUTE_IRI().text
        } else if (ctx.RELATIVE_IRI() != null) {
            base + ctx.RELATIVE_IRI().text
        } else {
            throw RuntimeException("Unexpected IRI type")
        }
    }

    private fun handleObject(ctx: Turtle.ObjectContext): MutableList<Object> { //TODO make this return a collection of Objects or do something else?
        return when {
            ctx.literal() != null -> mutableListOf(handleTurtleLiteral(ctx.literal()))
            ctx.blankNode() != null -> mutableListOf(handleTurtleBlankNode(ctx.blankNode()))
            ctx.iri() != null -> mutableListOf(handleTurtleIRI(ctx.iri()))
            ctx.blankNodePropertyList() != null -> mutableListOf(handleBlankNodePropertyList(ctx.blankNodePropertyList()))
            ctx.collection() != null -> TODO()
            else -> throw RuntimeException("Unexpected object")
        }
    }

    private fun handleTurtleLiteral(ctx: Turtle.LiteralContext): Literal {
        return when {
            ctx.booleanLiteral() != null -> handleBooleanLiteral(ctx.booleanLiteral())
            ctx.numericLiteral() != null  -> handleNumericLiteral(ctx.numericLiteral())
            ctx.rdfLiteral() != null  -> handleRdfLiteral(ctx.rdfLiteral())
            else -> throw RuntimeException("Unexpected literal")
        }
    }

    private fun handleBooleanLiteral(ctx: Turtle.BooleanLiteralContext): Literal {
        return TypedLiteral(ctx.text, booleanIRI)
    }

    private fun handleNumericLiteral(ctx: Turtle.NumericLiteralContext): Literal {
        return if (ctx.DECIMAL() != null) {
            TypedLiteral(ctx.DECIMAL().text, decimalIRI)
        } else if (ctx.DOUBLE() != null) {
            TypedLiteral(ctx.DOUBLE().text, doubleIRI)
        } else if (ctx.INTEGER() != null) {
            TypedLiteral(ctx.INTEGER().text, integerIRI)
        } else {
            throw RuntimeException("Unexpected Numeric type")
        }
    }

    private fun handleRdfLiteral(ctx: Turtle.RdfLiteralContext): Literal {
        val value = extractStringLiteralValue(ctx.string())
        return when {
            ctx.LANGTAG() != null -> LangLiteral(value, ctx.LANGTAG().text.substring(1))
            ctx.iri() != null -> TypedLiteral(value, handleTurtleIRI(ctx.iri()))
            else -> TypedLiteral(value)
        }
    }

    private fun handleTurtleBlankNode(ctx: Turtle.BlankNodeContext): BlankNode {
        return if (ctx.ANON() != null) {
            handleBlankNode("ANON${++anonymousCounter}")
        } else if (ctx.BLANK_NODE_LABEL() != null) {
            if (ctx.BLANK_NODE_LABEL().text.length > 2) {
                handleBlankNode(ctx.BLANK_NODE_LABEL().text.substring(2))
            } else {
                throw RuntimeException("Invalid blank node label - ${ctx.BLANK_NODE_LABEL().text}")
            }
        } else {
            throw RuntimeException("Unexpected blank node - ${ctx.text}")
        }
    }

    private fun handleBlankNode(blankNode: String): BlankNode {
        if (blankNodes.containsKey(blankNode)) {
            return blankNodes[blankNode]!!
        } else {
            val newBlankNode = BlankNode(blankNode)
            blankNodes[blankNode] = newBlankNode
            return newBlankNode
        }
    }

    private fun handleBlankNodePropertyList(ctx: Turtle.BlankNodePropertyListContext): Object {
        val statement = TurtleStatement()
        currentStatement.push(statement)
        val node = handleBlankNode("ANON${++anonymousCounter}")
        statement.subjects.add(node)
        ctx.predicateObjectList().verbObjectList().forEach { verbObjectList ->
            val predicate = handleTurtleIRI(verbObjectList.verb().predicate().iri())
            verbObjectList.objectList().`object`().forEach { objectContext ->
                handleObject(objectContext)//.forEach { `object` ->
                    //model.addStatement(node, predicate, `object`)
                //}
            }
        }
        currentStatement.peek().computeTriples().forEach { (subject, predicate, `object`) ->
            model.addStatement(subject, predicate, `object`)
        }
        currentStatement.pop()
        return node
    }

    private fun extractStringLiteralValue(ctx: Turtle.StringContext): String {
        return when {
            ctx.START_SINGLE_QUOTE() != null -> ctx.STRING_CONTENT_SINGLE_QUOTE()
            ctx.START_DOUBLE_QUOTE() != null -> ctx.STRING_CONTENT_DOUBLE_QUOTE()
            ctx.START_TRIPLE_SINGLE_QUOTE() != null -> ctx.STRING_CONTENT_TRIPLE_SINGLE_QUOTE()
            ctx.START_TRIPLE_DOUBLE_QUOTE() != null -> ctx.STRING_CONTENT_TRIPLE_DOUBLE_QUOTE()
            else -> throw RuntimeException("Unexpected String type")
        }?.text ?: ""
    }

    //TODO ANTRL listener methods that aren't being used currently / will be removed when switching to ABC
    override fun exitBlankNode(ctx: Turtle.BlankNodeContext) { /* do nothing */ }
    override fun exitLiteral(ctx: Turtle.LiteralContext) { /* do nothing */ }
    override fun exitCollection(ctx: Turtle.CollectionContext) { /* do nothing */ }
    override fun visitErrorNode(node: ErrorNode?) { /* do nothing */ }
    override fun exitVerb(ctx: Turtle.VerbContext) { /* do nothing */ }
    override fun exitBooleanLiteral(ctx: Turtle.BooleanLiteralContext) { /* do nothing */ }
    override fun exitObject(ctx: Turtle.ObjectContext) { /* do nothing */ }
    override fun exitTurtleDoc(ctx: Turtle.TurtleDocContext) { /* do nothing */ }
    override fun exitRdfLiteral(ctx: Turtle.RdfLiteralContext) { /* do nothing */ }
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
    override fun exitIriRef(p0: Turtle.IriRefContext?) { /* do nothing */ }
    override fun enterIriRef(p0: Turtle.IriRefContext?) { /* do nothing */ }
}
