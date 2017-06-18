/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.turtle

class TurtleTests {
//    val stinkpot = Stinkpot()
//    val xsd = "http://www.w3.org/2001/XMLSchema#"
//    val foafKnows = IRI("http://xmlns.com/foaf/0.1/knows")
//    val rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#"
//
//    val spidermanEnemy = Triple(IRI("http://example.org/#spiderman"),
//    IRI("http://www.perceive.net/schemas/relationship/enemyOf"), IRI("http://example.org/#green-goblin"))
//
//    val spidermanName = Triple(IRI("http://example.org/#spiderman"),
//    IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Spiderman", IRI("http://www.w3.org/2001/XMLSchema#string")))
//
//    val spidermanNameRu = Triple(IRI("http://example.org/#spiderman"),
//    IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Человек-паук", "ru"))
//
//    @Test fun supportBasicIRITriple() {
//        val expectedResult = spidermanEnemy
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/basicTriple.ttl").readText())
//        results.size == 1
//        results.first() == expectedResult
//    }
//
//    @Test fun supportPredicateLists() {
//        val expectedResults = listOf(spidermanEnemy, spidermanName)
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/predicateList.ttl").readText())
//        results.size == 2
//        results == expectedResults
//    }
//
//    @Test fun supportObjectLists() {
//        val expectedResults = listOf(spidermanName, spidermanNameRu)
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/objectList.ttl").readText())
//        results.size == 2
//        results == expectedResults
//    }
//
//    @Test fun supportComments() {
//        val expectedResults = listOf(spidermanEnemy, spidermanName)
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/comments.ttl").readText())
//        results.size == 2
//        results == expectedResults
//    }
//
//    @Test fun supportMultilineTriples() {
//        val expectedResults = listOf(spidermanEnemy)
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/multilineTriple.ttl").readText())
//        results.size == 1
//        results == expectedResults
//    }
//
//    val base = "http://one.example/"
//    val base2 = "http://one.example2/"
//    val baseTwo = "http://two.example/"
//    val baseTwo2 = "http://two.example2/"
//
//    val base3 = "http://another.example/"
//
//    @Test fun turtleIRIParsing() {
//        val expectedResults = listOf(
//        Triple(IRI("http://one.example/subject1"), IRI("http://one.example/predicate1"), IRI("http://one.example/object1")),
//        Triple(IRI("${base}subject2"), IRI("${base}predicate2"), IRI("${base}object2")),
//        Triple(IRI("${base2}subject2"), IRI("${base2}predicate2"), IRI("${base2}object2")),
//        Triple(IRI("${baseTwo}subject3"), IRI("${baseTwo}predicate3"), IRI("${baseTwo}object3")),
//        Triple(IRI("${baseTwo2}subject3"), IRI("${baseTwo2}predicate3"), IRI("${baseTwo2}object3")),
//        Triple(IRI("${base2}path/subject4"), IRI("${base2}path/predicate4"), IRI("${base2}path/object4")),
//        Triple(IRI("${base3}subject5"), IRI("${base3}predicate5"), IRI("${base3}object5")),
//        Triple(IRI("${base3}subject6"), IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), IRI("${base3}subject7")),
//        Triple(IRI("http://伝言.example/?user=أكرم&amp;channel=R%26D"), IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), IRI("${base3}subject8"))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/comprehensivePrefixBaseExample.ttl").readText())
//        Assert.assertEquals(results, expectedResults)
//    }
//
//    @Test fun supportLanguageLiterals() {
//        val expectedResults = listOf(spidermanNameRu)
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/literalWithLanguage.ttl").readText())
//        Assert.assertEquals(results, expectedResults)
//    }
//
//    @Test fun supportQuotedLiterals() {
//        val base = "http://www.libraryweasel.org/fake/madeup#"
//        val show = IRI("http://example.org/vocab/show/218")
//        val show219 = IRI("http://example.org/vocab/show/219")
//        val label = IRI("http://www.w3.org/2000/01/rdf-schema#label")
//        val localName = IRI("http://example.org/vocab/show/localName")
//        val blurb = IRI("http://example.org/vocab/show/blurb")
//        val multilineText = "This is a multi-line\n" +
//            "literal with many quotes (\"\"\"\"\")\n" +
//            "and up to two sequential apostrophes ('')."
//        val multilineText2 = "Another\n" +
//            "multiline string with' 'a' \"custom datatype\"\\\"."
//        val expectedResults = listOf(
//            Triple(show, label, TypedLiteral("That Seventies Show")),
//            Triple(show, label, TypedLiteral("That Seventies Show")),
//            Triple(show, label, TypedLiteral("That Seventies Show")),
//            Triple(show, IRI("${base}pred"), TypedLiteral("That Seventies Show", IRI("${base}string"))),
//            Triple(show, localName, LangLiteral("That Seventies Show", "en")),
//            Triple(show, localName, LangLiteral("Cette Série des Années Soixante-dix", "fr")),
//            Triple(show, localName, LangLiteral("Cette Série des Années Septante", "fr-be")),
//            Triple(show, blurb, TypedLiteral(multilineText)),
//            Triple(show219, blurb, TypedLiteral(multilineText2, IRI("${base}long-string"))),
//            Triple(show219, blurb, TypedLiteral("")),
//            Triple(show219, blurb, TypedLiteral("")),
//            Triple(show219, blurb, TypedLiteral(""))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/quotedLiterals.ttl").readText())
//        Assert.assertEquals(results, expectedResults)
//    }
//
//    @Test fun supportNumbers() {
//        val helium = "http://en.wikipedia.org/wiki/Helium"
//        val prefix = "http://example.org/elements"
//        val expectedResults = listOf(
//            Triple(IRI(helium), IRI("${prefix}atomicNumber"), TypedLiteral("2", IRI("${xsd}integer"))),
//            Triple(IRI(helium), IRI("${prefix}atomicMass"), TypedLiteral("4.002602", IRI("${xsd}float"))),
//            Triple(IRI(helium), IRI("${prefix}specificGravity"), TypedLiteral("1.663E-4", IRI("${xsd}double")))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/numbers.ttl").readText())
//        Assert.assertEquals(results, expectedResults)
//    }
//
//    @Test fun supportBooleans() {
//        val expectedResults = listOf(
//            Triple(IRI("http://somecountry.example/census2007"), IRI("http://example.org/stats/isLandlocked"),
//                    TypedLiteral("false", IRI("${xsd}boolean")))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/booleans.ttl").readText())
//        Assert.assertEquals(results, expectedResults)
//    }
//
//    @Test fun supportBlankNodes() {
//        val expectedResults = listOf(
//            Triple(BlankNode("alice"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("bob")),
//            Triple(BlankNode("bob"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("alice"))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/blankNodes.ttl").readText())
//        Assert.assertEquals(results, expectedResults)
//    }
//
//    @Test fun unlabeledBlankNodes() {
//        val expectedResults = listOf(
//            Triple(IRI("http://example.com/person/bob"), foafKnows, IRI("http://example.com/person/george")),
//            Triple(BlankNode("ANON0"), foafKnows, IRI("http://example.com/person/george")),
//            Triple(IRI("http://example.com/person/bob"), foafKnows, BlankNode("ANON1")),
//            Triple(BlankNode("ANON2"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("ANON3"))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/unlabeledBlankNodes.ttl").readText())
//        Assert.assertEquals(results, expectedResults)
//    }
//
//    @Test fun nestedUnlabeledBlankNodes() {
//        val expectedResults = listOf(
//            Triple(BlankNode("ANON1"), IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Bob")),
//            Triple(BlankNode("ANON0"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("ANON1"))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/nestedUnlabeledBlankNodes.ttl").readText())
//        Assert.assertEquals(results, expectedResults)
//    }
//
//    @Test fun complexUnlabeledBlankNodes() {
//        val expectedResults = listOf(
//            Triple(BlankNode("ANON0"), IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Alice")),
//            Triple(BlankNode("ANON1"), IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Bob")),
//            Triple(BlankNode("ANON0"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("ANON1")),
//            Triple(BlankNode("ANON2"), IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Eve")),
//            Triple(BlankNode("ANON1"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("ANON2")),
//            Triple(BlankNode("ANON1"), IRI("http://xmlns.com/foaf/0.1/mbox"), IRI("http://bob@example.com"))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/complexUnlabeledBlankNodes.ttl").readText())
//        val c = Comparator<Triple> { f, s -> f.toString().compareTo(s.toString()) }
//        Assert.assertEquals(results.sortedWith(c), expectedResults.sortedWith(c))
//    }
//
//    @Test fun supportCollections() {
//        val expectedResults = listOf(
//                Triple(IRI("http://example.org/foo/subject"), IRI("http://example.org/foo/predicate"), BlankNode("ANON0")),
//                Triple(BlankNode("ANON0"), IRI("${rdf}first"), IRI("http://example.org/foo/a")),
//                Triple(BlankNode("ANON0"), IRI("${rdf}rest"), BlankNode("ANON1")),
//                Triple(BlankNode("ANON1"), IRI("${rdf}first"), IRI("http://example.org/foo/b")),
//                Triple(BlankNode("ANON1"), IRI("${rdf}rest"), BlankNode("ANON2")),
//                Triple(BlankNode("ANON2"), IRI("${rdf}first"), IRI("http://example.org/foo/c")),
//                Triple(BlankNode("ANON2"), IRI("${rdf}rest"), IRI("${rdf}nil")),
//                Triple(IRI("http://example.org/foo/subject"), IRI("http://example.org/foo/predicate2"), IRI("${rdf}nil"))
//        )
//        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/collections.ttl").readText())
//        val c = Comparator<Triple> { f, s -> f.toString().compareTo(s.toString()) }
//        Assert.assertEquals(results.sortedWith(c), expectedResults.sortedWith(c))
//    }
////
////    //TODO examples 19-26 and wordnetStinkpot.ttl
////    @Test fun wordnetTest() {
////        val expectedResults = listOf(
////                Triple(IRI(""),IRI(""),IRI(""))
////        )
////        val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/wordnetStinkpot.ttl").readText())
////        Assert.assertEquals(results, expectedResults)
////    }
//
//    @Test fun malformedQuotedLiterals() {
//        try {
//            val results = stinkpot.parseTurtle(this.javaClass.getResource("/turtle/malformed/quotedLiterals.ttl").readText())
//        } catch (exception: RuntimeException) {
//            return
//        }
//        throw RuntimeException("Test failed")
//    }
}
