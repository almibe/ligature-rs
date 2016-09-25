/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.turtle

import org.libraryweasel.stinkpot.*

class TurtleParser(lexer: TurtleLexer, val handler: (Triple) -> Unit) : Parser<TurtleTokenType>(lexer) {
    val prefixes = mutableMapOf<String, String>()
    var base = ""
    var unlabeledBlankNodeCount = 0

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
            TurtleTokenType.UNLABELED_BLANK_NODE_OPEN -> {
                consume()
                return handleUnlabeledBlankNode()
            }
            TurtleTokenType.CHARACTER_TOKEN -> {
                val token = match(TurtleTokenType.CHARACTER_TOKEN)
                return handlePrefix(token.text)
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
                    return handlePrefix(token.text)
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
            TurtleTokenType.UNLABELED_BLANK_NODE_OPEN -> {
                consume()
                return handleUnlabeledBlankNode()
            }
            TurtleTokenType.STRING_LITERAL_QUOTE -> {
                return literal()
            }
            TurtleTokenType.CHARACTER_TOKEN -> {
                val token = match(TurtleTokenType.CHARACTER_TOKEN)
                return handleCharacterToken(token.text)
            }
            else -> throw RuntimeException("Error Parsing Object -- must be IRI, Blank Node, or Literal not ${lookAhead.tokenType}")
        }
    }

    fun handleUnlabeledBlankNode(): BlankNode {
        if (lookAhead.tokenType == TurtleTokenType.UNLABELED_BLANK_NODE_CLOSE) {
            consume()
            return BlankNode("ANON${unlabeledBlankNodeCount++}")
        } else {
            val subject = BlankNode("ANON${unlabeledBlankNodeCount++}")
            //TODO create inner triples
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
            match(TurtleTokenType.UNLABELED_BLANK_NODE_CLOSE)
            handler(Triple(subject, predicate, `object`))
            return subject
        }
    }

    fun handleCharacterToken(text: String) : Object {
        val numberType = isNumberType(text)
        return if (text.split(':').size == 2) {
            handlePrefix(text)
        } else if (numberType != null) {
            TypedLiteral(text, IRI(numberType.url))
        } else if (listOf("true", "false").contains(text.toLowerCase())) {
            TypedLiteral(text.toLowerCase(), IRI("http://www.w3.org/2001/XMLSchema#boolean"))
        } else {
            throw RuntimeException("Could not parse character token -- ${text}")
        }
    }

    fun handlePrefix(text: String) : IRI {
        val parts = text.split(':')
        assert(parts.size == 2 && prefixes.containsKey(parts[0] + ":")) {
            "Error Handling Prefix -- $text"
        }
        val prefix = prefixes[parts[0] + ":"]
        return IRI(prefix + parts[1])
    }

    enum class NumberType(val url: String) {
        integer(url = "http://www.w3.org/2001/XMLSchema#integer"),
        double(url = "http://www.w3.org/2001/XMLSchema#double"),
        float(url = "http://www.w3.org/2001/XMLSchema#float")
    }

    fun isNumberType(str: String?): NumberType? {
        var numberType = NumberType.integer
        if (str == null) return null
        val data = str.toCharArray()
        if (data.size <= 0) return null
        var index = 0
        if (data[0] == '-' && data.size > 1) index = 1
        while (index < data.size) {
            if (data[index] == '.') {
                if (numberType == NumberType.float || numberType == NumberType.double) return null
                else numberType = NumberType.float
            } else if (data[index] == 'e' || data[index] == 'E') {
                if (numberType == NumberType.double) return null
                else {
                    index++
                    if (data[index] == '-' && data.size > (index + 1)) index++
                    numberType = NumberType.double
                }
            } else if (!Character.isDigit(data[index])) {
                return null
            }
            index++
        }
        return numberType
    }

    fun literal() : Literal {
        val token = match(TurtleTokenType.STRING_LITERAL_QUOTE)
        when (lookAhead.tokenType) {
            TurtleTokenType.PERIOD, TurtleTokenType.SEMICOLON, TurtleTokenType.COMMA, TurtleTokenType.UNLABELED_BLANK_NODE_CLOSE -> return TypedLiteral(token.text)
            TurtleTokenType.LANGTAG -> {
                val lang = match(TurtleTokenType.LANGTAG)
                return LangLiteral(token.text, lang.text)
            }
            TurtleTokenType.TYPE_TAG -> {
                consume()
                when (lookAhead.tokenType) {
                    TurtleTokenType.IRIREF -> {
                        val iri = match(TurtleTokenType.IRIREF)
                        return TypedLiteral(token.text, IRI(iri.text))
                    }
                    TurtleTokenType.RELATIVE_IRI -> {
                        val iri = match(TurtleTokenType.RELATIVE_IRI)
                        return TypedLiteral(token.text, IRI("$base${iri.text}"))
                    }
                    TurtleTokenType.CHARACTER_TOKEN -> {
                        val iri = match(TurtleTokenType.CHARACTER_TOKEN)
                        return TypedLiteral(token.text, handlePrefix(iri.text))
                    }
                    else -> throw RuntimeException("Error Parsing Typed Literal -- must be IRIREF, RELATIVE_IRI, or CHARACTER_TOKEN not ${lookAhead.tokenType}")
                }
            }
            else -> throw RuntimeException("Error Parsing Literal -- must be PERIOD, COMMA, SEMICOLON, UNLABELED_BLANK_NODE_CLOSE, LANGTAG, or TYPE_TAG not ${lookAhead.tokenType}")
        }
    }
}
