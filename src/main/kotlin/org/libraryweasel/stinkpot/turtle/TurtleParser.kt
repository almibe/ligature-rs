/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.turtle

import org.libraryweasel.stinkpot.*

class TurtleParser(lexer: TurtleLexer, val handler: (Triple) -> Unit) : Parser<TurtleTokenType>(lexer) {
    val prefixes = mutableMapOf<String, String>()
    var base = ""

    fun start() : Unit {
        while (lookAhead.tokenType != TurtleTokenType.EOF) {
            statement()
        }
    }

    fun statement() : Unit {
        if (isDirective(lookAhead)) {
            checkForDirectives()
        } else {
            triples()
        }
    }

    fun isDirective(token: Token<TurtleTokenType>): Boolean =
            (token.tokenType == TurtleTokenType.BASE || token.tokenType == TurtleTokenType.PREFIX
                    || (token.tokenType == TurtleTokenType.CHARACTER_TOKEN && token.text.toLowerCase() == "base")
                    || (token.tokenType == TurtleTokenType.CHARACTER_TOKEN && token.text.toLowerCase() == "prefix"))

    fun triples() {
        val subject = subject()
        var predicate = predicate()
        var `object` = `object`()
        while (lookAhead.tokenType == TurtleTokenType.COMMA) {
            match(TurtleTokenType.COMMA)
            handler(Triple(subject, predicate, `object`))
            `object` = `object`()
        }
        while (lookAhead.tokenType == TurtleTokenType.SEMICOLON) {
            match(TurtleTokenType.SEMICOLON)
            handler(Triple(subject, predicate, `object`))
            predicate = predicate()
            `object` = `object`()
            while (lookAhead.tokenType == TurtleTokenType.COMMA) {
                match(TurtleTokenType.COMMA)
                handler(Triple(subject, predicate, `object`))
                `object` = `object`()
            }
        }
        match(TurtleTokenType.PERIOD)
        handler(Triple(subject, predicate, `object`))
    }

    fun checkForDirectives() {
        when (lookAhead.tokenType) {
            TurtleTokenType.BASE -> {
                match(TurtleTokenType.BASE)
                val iriToken = match(TurtleTokenType.IRIREF)
                base = iriToken.text
                match(TurtleTokenType.PERIOD)
            }
            TurtleTokenType.PREFIX -> {
                match(TurtleTokenType.PREFIX)
                val nameToken = match(TurtleTokenType.CHARACTER_TOKEN)
                when (lookAhead.tokenType) {
                    TurtleTokenType.IRIREF -> {
                        val iriToken = match(TurtleTokenType.IRIREF)
                        prefixes.put(nameToken.text, iriToken.text)
                    }
                    TurtleTokenType.RELATIVE_IRI -> {
                        val iriToken = match(TurtleTokenType.RELATIVE_IRI)
                        prefixes.put(nameToken.text, base + iriToken.text)
                    }
                }
                match(TurtleTokenType.PERIOD)
            }
            else -> {
                val tokenType = match(TurtleTokenType.CHARACTER_TOKEN).text
                if (tokenType.toLowerCase() == "base") {
                    val iriToken = match(TurtleTokenType.IRIREF)
                    base = iriToken.text
                } else { //we can safely assume this is a prefix
                    val nameToken = match(TurtleTokenType.CHARACTER_TOKEN)
                    val iriToken = match(TurtleTokenType.IRIREF)
                    prefixes.put(nameToken.text, iriToken.text)
                }
            }
        }
    }

    fun subject() : Subject {
        when (lookAhead.tokenType) {
            TurtleTokenType.IRIREF -> {
                val token = match(TurtleTokenType.IRIREF)
                return IRI(token.text)
            }
            TurtleTokenType.RELATIVE_IRI -> {
                val token = match(TurtleTokenType.RELATIVE_IRI)
                return IRI(base + token.text)
            }
            TurtleTokenType.BLANK_NODE_LABEL -> {
                val token = match(TurtleTokenType.BLANK_NODE_LABEL)
                return BlankNode(token.text)
            }
            TurtleTokenType.CHARACTER_TOKEN -> {
                val token = match(TurtleTokenType.CHARACTER_TOKEN)
                return IRI(handlePrefix(token.text))
            }
            else -> throw RuntimeException("Error Parsing Subject -- must be IRI or Blank Node not ${lookAhead.tokenType} -- ${lookAhead.text}")
        }
    }

    fun predicate() : Predicate {
        when (lookAhead.tokenType) {
            TurtleTokenType.IRIREF -> {
                val token = match(TurtleTokenType.IRIREF)
                return IRI(token.text)
            }
            TurtleTokenType.RELATIVE_IRI -> {
                val token = match(TurtleTokenType.RELATIVE_IRI)
                return IRI(base + token.text)
            }
            TurtleTokenType.CHARACTER_TOKEN -> {
                val token = match(TurtleTokenType.CHARACTER_TOKEN)
                if (token.text == "a") {
                    return IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
                } else {
                    return IRI(handlePrefix(token.text))
                }
            }
            else -> throw RuntimeException("Error Parsing Subject -- must be IRI not ${lookAhead.tokenType} -- ${lookAhead.text}")
        }
    }

    fun `object`() : Object {
        when (lookAhead.tokenType) {
            TurtleTokenType.IRIREF -> {
                val token = match(TurtleTokenType.IRIREF)
                return IRI(token.text)
            }
            TurtleTokenType.RELATIVE_IRI -> {
                val token = match(TurtleTokenType.RELATIVE_IRI)
                return IRI(base + token.text)
            }
            TurtleTokenType.BLANK_NODE_LABEL -> {
                val token = match(TurtleTokenType.BLANK_NODE_LABEL)
                return BlankNode(token.text)
            }
            TurtleTokenType.STRING_LITERAL_QUOTE -> {
                return literal()
            }
            TurtleTokenType.CHARACTER_TOKEN -> {
                val token = match(TurtleTokenType.CHARACTER_TOKEN)
                return IRI(handlePrefix(token.text))
            }
            else -> throw RuntimeException("Error Parsing Object -- must be IRI, Blank Node, or Literal not ${lookAhead.tokenType}")
        }
    }

    fun handlePrefix(text: String) : String {
        val parts = text.split(':')
        assert(parts.size == 2 && prefixes.containsKey(parts[0] + ":")) {
            "Error Handling Prefix -- $text"
        }
        val prefix = prefixes[parts[0] + ":"]
        return prefix + parts[1]
    }

    fun literal() : Literal {
        val token = match(TurtleTokenType.STRING_LITERAL_QUOTE)
        when (lookAhead.tokenType) {
            TurtleTokenType.PERIOD, TurtleTokenType.COMMA -> return PlainLiteral(token.text)
            TurtleTokenType.LANGTAG -> {
                val lang = match(TurtleTokenType.LANGTAG)
                return LangLiteral(token.text, lang.text)
            }
            TurtleTokenType.IRIREF -> {
                val iri = match(TurtleTokenType.IRIREF)
                return TypedLiteral(token.text, IRI(iri.text))
            }

            else -> throw RuntimeException("Error Parsing Literal -- must be PERIOD, COMMA, LANGTAG, or IRIREF not ${lookAhead.tokenType}")
        }
    }
}
