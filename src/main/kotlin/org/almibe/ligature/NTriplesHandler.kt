/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

import org.almibe.ligature.parser.NTriplesListener
import org.almibe.ligature.parser.NTriplesParser
import org.antlr.v4.runtime.ParserRuleContext
import org.antlr.v4.runtime.tree.ErrorNode
import org.antlr.v4.runtime.tree.TerminalNode

class NTriplesHandler : NTriplesListener {
    override fun enterDocument(ctx: NTriplesParser.DocumentContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitDocument(ctx: NTriplesParser.DocumentContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterTriple(ctx: NTriplesParser.TripleContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitTriple(ctx: NTriplesParser.TripleContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterSubject(ctx: NTriplesParser.SubjectContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitSubject(ctx: NTriplesParser.SubjectContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterPredicate(ctx: NTriplesParser.PredicateContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitPredicate(ctx: NTriplesParser.PredicateContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterObject(ctx: NTriplesParser.ObjectContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitObject(ctx: NTriplesParser.ObjectContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterIri(ctx: NTriplesParser.IriContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitIri(ctx: NTriplesParser.IriContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterLiteral(ctx: NTriplesParser.LiteralContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitLiteral(ctx: NTriplesParser.LiteralContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterTypedLiteral(ctx: NTriplesParser.TypedLiteralContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitTypedLiteral(ctx: NTriplesParser.TypedLiteralContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterLanguageLiteral(ctx: NTriplesParser.LanguageLiteralContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitLanguageLiteral(ctx: NTriplesParser.LanguageLiteralContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterBlankNode(ctx: NTriplesParser.BlankNodeContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitBlankNode(ctx: NTriplesParser.BlankNodeContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun visitErrorNode(node: ErrorNode?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun visitTerminal(node: TerminalNode?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun enterEveryRule(ctx: ParserRuleContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }

    override fun exitEveryRule(ctx: ParserRuleContext?) {
        //TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
    }
}
