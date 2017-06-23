/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.*
import org.almibe.ligature.parser.NTriplesLexer
import org.almibe.ligature.parser.NTriplesParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.testng.Assert
import org.testng.annotations.Test

fun parseSubject(text: String) : Subject {
    val stream = CharStreams.fromString(text)
    val lexer = NTriplesLexer(stream)
    val tokens = CommonTokenStream(lexer)
    val parser = NTriplesParser(tokens)

    val subjectVisitor = SubjectVisitor()
    return subjectVisitor.visit(parser.subject())
}

fun parsePredicate(text: String) : Predicate {
    val stream = CharStreams.fromString(text)
    val lexer = NTriplesLexer(stream)
    val tokens = CommonTokenStream(lexer)
    val parser = NTriplesParser(tokens)

    val predicateVisitor = PredicateVisitor()
    return predicateVisitor.visit(parser.subject())
}

fun parseObject(text: String) : Object {
    val stream = CharStreams.fromString(text)
    val lexer = NTriplesLexer(stream)
    val tokens = CommonTokenStream(lexer)
    val parser = NTriplesParser(tokens)

    val objectVisitor = ObjectVisitor()
    return objectVisitor.visit(parser.subject())
}

class NTriplesTests {
    val ligature = NTriples()

    val stringIRI = IRI("http://www.w3.org/2001/XMLSchema#string")

    @Test fun testParsingSubjects() {
        val resultIRI = parseSubject("<http://www.w3.org/2001/XMLSchema#string>")
        val expectedResultIRI = IRI("http://www.w3.org/2001/XMLSchema#string")

        val resultBlankNode = parseSubject("_:ludo")
        val expectedResultBlankNode = BlankNode("ludo")

        Assert.assertEquals(resultIRI, expectedResultIRI)
        Assert.assertEquals(resultBlankNode, expectedResultBlankNode)
    }

    @Test fun testParsingPredicates() {
        val resultIRI = parsePredicate("<http://www.w3.org/2001/XMLSchema#string>")
        val expectedResultIRI = IRI("http://www.w3.org/2001/XMLSchema#string")

        Assert.assertEquals(resultIRI, expectedResultIRI)
    }

    @Test fun testParsingObjects() {
        val resultIRI = parseObject("<http://www.w3.org/2001/XMLSchema#string>")
        val expectedResultIRI = IRI("http://www.w3.org/2001/XMLSchema#string")

        val resultBlankNode = parseObject("_:ludo")
        val expectedResultBlankNode = BlankNode("ludo")

        val resultLiteral = parseObject("\"Test\"")
        val expectedResultLiteral = TypedLiteral("Test")

        val resultTypedLiteral = parseObject("\"Test\"^^<Type>")
        val expectedResultTypedLiteral = TypedLiteral("Test", IRI("Type"))

        val resultLangLiteral = parseObject("\"Test\"@EN")
        val expectedResultLangLiteral = LangLiteral("Test", "EN")

        Assert.assertEquals(resultIRI, expectedResultIRI)
        Assert.assertEquals(resultBlankNode, expectedResultBlankNode)
        Assert.assertEquals(resultLiteral, expectedResultLiteral)
        Assert.assertEquals(resultTypedLiteral, expectedResultTypedLiteral)
        Assert.assertEquals(resultLangLiteral, expectedResultLangLiteral)
    }

    @Test fun supportBasicIRITriple() {
        val expectedResult = Triple(IRI("http://example.org/#spiderman"),
                 IRI("http://www.perceive.net/schemas/relationship/enemyOf"),  IRI("http://example.org/#green-goblin"))
        val results = ligature.parseNTriples(this.javaClass.getResource("/ntriples/basicTriple.nt").readText())
        Assert.assertEquals(results.size, 1)
        Assert.assertEquals(results, listOf(expectedResult))
    }

    @Test fun supportMultipleIRITriples() {
        val expectedResult1 =  Triple( IRI("http://example.org/#spiderman"),
                 IRI("http://www.perceive.net/schemas/relationship/enemyOf"),  IRI("http://example.org/#green-goblin"))
        val expectedResult2 =  Triple( IRI("http://example.org/#spiderman"),
                 IRI("http://www.perceive.net/schemas/relationship/enemyOf"),  IRI("http://example.org/#black-cat"))
        val results = ligature.parseNTriples(this.javaClass.getResource("/ntriples/multipleIRITriples.nt").readText())
        Assert.assertEquals(results, listOf(expectedResult1, expectedResult2))
    }

    @Test fun supportBeginningOfLineAndEndOfLineComments() {
        val expectedResult =  Triple( IRI("http://example.org/#spiderman"),
                 IRI("http://www.perceive.net/schemas/relationship/enemyOf"),  IRI("http://example.org/#green-goblin"))
        val results = ligature.parseNTriples(this.javaClass.getResource("/ntriples/comments.nt").readText())
        Assert.assertEquals(results.size, 1)
        Assert.assertEquals(results.first(), expectedResult)
    }

    @Test fun supportLiteralsWithLanguagesAndTypes() {
        val expectedResults = mutableListOf<Triple>()
        expectedResults.add( Triple( IRI("http://example.org/show/218"),  IRI("http://www.w3.org/2000/01/rdf-schema#label"),  TypedLiteral("That Seventies Show", stringIRI)))
        expectedResults.add( Triple( IRI("http://example.org/show/218"),  IRI("http://www.w3.org/2000/01/rdf-schema#label"),  TypedLiteral("That Seventies Show", stringIRI)))
        expectedResults.add( Triple( IRI("http://example.org/show/218"),  IRI("http://example.org/show/localName"),  LangLiteral("That Seventies Show", "en")))
        expectedResults.add( Triple( IRI("http://example.org/show/218"),  IRI("http://example.org/show/localName"),  LangLiteral("Cette Série des Années Septante", "fr-be")))
        expectedResults.add( Triple( IRI("http://example.org/#spiderman"),  IRI("http://example.org/text"),  TypedLiteral("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI)))
        expectedResults.add( Triple( IRI("http://en.wikipedia.org/wiki/Helium"),  IRI("http://example.org/elements/atomicNumber"),  TypedLiteral("2",  IRI("http://www.w3.org/2001/XMLSchema#integer"))))
        expectedResults.add( Triple( IRI("http://en.wikipedia.org/wiki/Helium"),  IRI("http://example.org/elements/specificGravity"),  TypedLiteral("1.663E-4",  IRI("http://www.w3.org/2001/XMLSchema#double"))))
        val results = ligature.parseNTriples(this.javaClass.getResource("/ntriples/literals.nt").readText())
        Assert.assertEquals(results, expectedResults)
    }

    @Test fun supportLiteralsWithLanguagesAndTypesPassingAPathInsteadOfAString() {
        val expectedResults = mutableListOf<Triple>()
        expectedResults.add( Triple( IRI("http://example.org/show/218"),  IRI("http://www.w3.org/2000/01/rdf-schema#label"),  TypedLiteral("That Seventies Show", stringIRI)))
        expectedResults.add( Triple( IRI("http://example.org/show/218"),  IRI("http://www.w3.org/2000/01/rdf-schema#label"),  TypedLiteral("That Seventies Show", stringIRI)))
        expectedResults.add( Triple( IRI("http://example.org/show/218"),  IRI("http://example.org/show/localName"),  LangLiteral("That Seventies Show", "en")))
        expectedResults.add( Triple( IRI("http://example.org/show/218"),  IRI("http://example.org/show/localName"),  LangLiteral("Cette Série des Années Septante", "fr-be")))
        expectedResults.add( Triple( IRI("http://example.org/#spiderman"),  IRI("http://example.org/text"),  TypedLiteral("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').", stringIRI)))
        expectedResults.add( Triple( IRI("http://en.wikipedia.org/wiki/Helium"),  IRI("http://example.org/elements/atomicNumber"),  TypedLiteral("2",  IRI("http://www.w3.org/2001/XMLSchema#integer"))))
        expectedResults.add( Triple( IRI("http://en.wikipedia.org/wiki/Helium"),  IRI("http://example.org/elements/specificGravity"),  TypedLiteral("1.663E-4",  IRI("http://www.w3.org/2001/XMLSchema#double"))))
        val results = ligature.parseNTriples(this.javaClass.getResource("/ntriples/literals.nt").readText())
        Assert.assertEquals(results, expectedResults)
    }

    @Test fun supportBlankNodes() {
        val expectedResult1 =  Triple( BlankNode("alice"),  IRI("http://xmlns.com/foaf/0.1/knows"),  BlankNode("bob"))
        val expectedResult2 =  Triple( BlankNode("bob"),  IRI("http://xmlns.com/foaf/0.1/knows"),  BlankNode("alice"))
        val results = ligature.parseNTriples(this.javaClass.getResource("/ntriples/blankNodes.nt").readText())
        Assert.assertEquals(results, listOf(expectedResult1, expectedResult2))
    }
}
