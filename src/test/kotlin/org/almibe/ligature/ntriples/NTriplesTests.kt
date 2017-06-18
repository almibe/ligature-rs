/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.*
import org.testng.Assert
import org.testng.annotations.Test
import java.nio.file.Paths

class NTriplesTests {
    val ligature = NTriples()

    val stringIRI = IRI("http://www.w3.org/2001/XMLSchema#string")

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
        val results = ligature.parseNTriples(Paths.get(this.javaClass.getResource("/ntriples/literals.nt").toURI()))
        Assert.assertEquals(results, expectedResults)
    }

    @Test fun supportBlankNodes() {
        val expectedResult1 =  Triple( BlankNode("alice"),  IRI("http://xmlns.com/foaf/0.1/knows"),  BlankNode("bob"))
        val expectedResult2 =  Triple( BlankNode("bob"),  IRI("http://xmlns.com/foaf/0.1/knows"),  BlankNode("alice"))
        val results = ligature.parseNTriples(this.javaClass.getResource("/ntriples/blankNodes.nt").readText())
        Assert.assertEquals(results, listOf(expectedResult1, expectedResult2))
    }
}
