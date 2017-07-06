/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.parsers

import org.almibe.ligature.IRI
import org.almibe.ligature.LangLiteral
import org.almibe.ligature.Triple
import org.almibe.ligature.TypedLiteral
import spock.lang.Specification

class TurtleSpec extends Specification {
    final def turtle = new Turtle()
    final def xsd = "http://www.w3.org/2001/XMLSchema#"
    final def foafKnows = new IRI("http://xmlns.com/foaf/0.1/knows")
    final def rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#"

    final def spidermanEnemy = new Triple(new IRI("http://example.org/#spiderman"),
        new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))

    final def spidermanName = new Triple(new IRI("http://example.org/#spiderman"),
        new IRI("http://xmlns.com/foaf/0.1/name"), new TypedLiteral("Spiderman", new IRI("http://www.w3.org/2001/XMLSchema#string")))

    final def spidermanNameRu = new Triple(new IRI("http://example.org/#spiderman"),
        new IRI("http://xmlns.com/foaf/0.1/name"), new LangLiteral("Человек-паук", "ru"))

    final def "support basic IRI triple"() {
        given:
        final def expectedResult = spidermanEnemy
        final def results = turtle.parseTurtle(this.class.getResource("/turtle/01-basicTriple.ttl").text)
        expect:
        results.size() == 1
        results.first() == expectedResult
    }

    final def "support predicate lists"() {
        given:
        final def expectedResults = [spidermanEnemy, spidermanName]
        final def results = turtle.parseTurtle(this.class.getResource("/turtle/02-predicateList.ttl").text)
        expect:
        results.size() == 2
        results == expectedResults
    }

    final def "support object lists"() {
        given:
        final def expectedResults = [spidermanName, spidermanNameRu]
        final def results = turtle.parseTurtle(this.class.getResource("/turtle/03-objectList.ttl").text)
        expect:
        results.size() == 2
        results == expectedResults
    }
}

//
//    final def supportComments() {
//        final def expectedResults = [spidermanEnemy, spidermanName)
//        final def results = turtle.parseTurtle(this.class.getResource("/turtle/comments.ttl").text)
//        results.size, 2)
//        results, expectedResults)
//    }
//
//    final def supportMultilineTriples() {
//        final def expectedResults = [spidermanEnemy)
//        final def results = turtle.parseTurtle(this.class.getResource("/turtle/multilineTriple.ttl").text)
//        results.size, 1)
//        results, expectedResults)
//    }
////
////    final def base = "http://one.example/"
////    final def base2 = "http://one.example2/"
////    final def baseTwo = "http://two.example/"
////    final def baseTwo2 = "http://two.example2/"
////
////    final def base3 = "http://another.example/"
////
////    final def turtleIRIParsing() {
////        final def expectedResults = [
////        Triple(IRI("http://one.example/subject1"), IRI("http://one.example/predicate1"), IRI("http://one.example/object1")),
////        Triple(IRI("${base}subject2"), IRI("${base}predicate2"), IRI("${base}object2")),
////        Triple(IRI("${base2}subject2"), IRI("${base2}predicate2"), IRI("${base2}object2")),
////        Triple(IRI("${baseTwo}subject3"), IRI("${baseTwo}predicate3"), IRI("${baseTwo}object3")),
////        Triple(IRI("${baseTwo2}subject3"), IRI("${baseTwo2}predicate3"), IRI("${baseTwo2}object3")),
////        Triple(IRI("${base2}path/subject4"), IRI("${base2}path/predicate4"), IRI("${base2}path/object4")),
////        Triple(IRI("${base3}subject5"), IRI("${base3}predicate5"), IRI("${base3}object5")),
////        Triple(IRI("${base3}subject6"), IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), IRI("${base3}subject7")),
////        Triple(IRI("http://伝言.example/?user=أكرم&amp;channel=R%26D"), IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), IRI("${base3}subject8"))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/comprehensivePrefixBaseExample.ttl").text)
////        results, expectedResults)
////    }
////
////    final def supportLanguageLiterals() {
////        final def expectedResults = [spidermanNameRu)
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/literalWithLanguage.ttl").text)
////        results, expectedResults)
////    }
////
////    final def supportQuotedLiterals() {
////        final def base = "http://www.libraryweasel.org/fake/madeup#"
////        final def show = IRI("http://example.org/vocab/show/218")
////        final def show219 = IRI("http://example.org/vocab/show/219")
////        final def label = IRI("http://www.w3.org/2000/01/rdf-schema#label")
////        final def localName = IRI("http://example.org/vocab/show/localName")
////        final def blurb = IRI("http://example.org/vocab/show/blurb")
////        final def multilineText = "This is a multi-line\n" +
////            "literal with many quotes (\"\"\"\"\")\n" +
////            "and up to two sequential apostrophes ('')."
////        final def multilineText2 = "Another\n" +
////            "multiline string with' 'a' \"custom datatype\"\\\"."
////        final def expectedResults = [
////            Triple(show, label, TypedLiteral("That Seventies Show")),
////            Triple(show, label, TypedLiteral("That Seventies Show")),
////            Triple(show, label, TypedLiteral("That Seventies Show")),
////            Triple(show, IRI("${base}pred"), TypedLiteral("That Seventies Show", IRI("${base}string"))),
////            Triple(show, localName, LangLiteral("That Seventies Show", "en")),
////            Triple(show, localName, LangLiteral("Cette Série des Années Soixante-dix", "fr")),
////            Triple(show, localName, LangLiteral("Cette Série des Années Septante", "fr-be")),
////            Triple(show, blurb, TypedLiteral(multilineText)),
////            Triple(show219, blurb, TypedLiteral(multilineText2, IRI("${base}long-string"))),
////            Triple(show219, blurb, TypedLiteral("")),
////            Triple(show219, blurb, TypedLiteral("")),
////            Triple(show219, blurb, TypedLiteral(""))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/quotedLiterals.ttl").text)
////        results, expectedResults)
////    }
////
////    final def supportNumbers() {
////        final def helium = "http://en.wikipedia.org/wiki/Helium"
////        final def prefix = "http://example.org/elements"
////        final def expectedResults = [
////            Triple(IRI(helium), IRI("${prefix}atomicNumber"), TypedLiteral("2", IRI("${xsd}integer"))),
////            Triple(IRI(helium), IRI("${prefix}atomicMass"), TypedLiteral("4.002602", IRI("${xsd}float"))),
////            Triple(IRI(helium), IRI("${prefix}specificGravity"), TypedLiteral("1.663E-4", IRI("${xsd}double")))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/numbers.ttl").text)
////        results, expectedResults)
////    }
////
////    final def supportBooleans() {
////        final def expectedResults = [
////            Triple(IRI("http://somecountry.example/census2007"), IRI("http://example.org/stats/isLandlocked"),
////                    TypedLiteral("false", IRI("${xsd}boolean")))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/booleans.ttl").text)
////        results, expectedResults)
////    }
////
////    final def supportBlankNodes() {
////        final def expectedResults = [
////            Triple(BlankNode("alice"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("bob")),
////            Triple(BlankNode("bob"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("alice"))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/blankNodes.ttl").text)
////        results, expectedResults)
////    }
////
////    final def unlabeledBlankNodes() {
////        final def expectedResults = [
////            Triple(IRI("http://example.com/person/bob"), foafKnows, IRI("http://example.com/person/george")),
////            Triple(BlankNode("ANON0"), foafKnows, IRI("http://example.com/person/george")),
////            Triple(IRI("http://example.com/person/bob"), foafKnows, BlankNode("ANON1")),
////            Triple(BlankNode("ANON2"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("ANON3"))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/unlabeledBlankNodes.ttl").text)
////        results, expectedResults)
////    }
////
////    final def nestedUnlabeledBlankNodes() {
////        final def expectedResults = [
////            Triple(BlankNode("ANON1"), IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Bob")),
////            Triple(BlankNode("ANON0"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("ANON1"))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/nestedUnlabeledBlankNodes.ttl").text)
////        results, expectedResults)
////    }
////
////    final def complexUnlabeledBlankNodes() {
////        final def expectedResults = [
////            Triple(BlankNode("ANON0"), IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Alice")),
////            Triple(BlankNode("ANON1"), IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Bob")),
////            Triple(BlankNode("ANON0"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("ANON1")),
////            Triple(BlankNode("ANON2"), IRI("http://xmlns.com/foaf/0.1/name"), TypedLiteral("Eve")),
////            Triple(BlankNode("ANON1"), IRI("http://xmlns.com/foaf/0.1/knows"), BlankNode("ANON2")),
////            Triple(BlankNode("ANON1"), IRI("http://xmlns.com/foaf/0.1/mbox"), IRI("http://bob@example.com"))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/complexUnlabeledBlankNodes.ttl").text)
////        final def c = Comparator<Triple> { f, s -> f.toString().compareTo(s.toString()) }
////        results.sortedWith(c), expectedResults.sortedWith(c))
////    }
////
////    final def supportCollections() {
////        final def expectedResults = [
////                Triple(IRI("http://example.org/foo/subject"), IRI("http://example.org/foo/predicate"), BlankNode("ANON0")),
////                Triple(BlankNode("ANON0"), IRI("${rdf}first"), IRI("http://example.org/foo/a")),
////                Triple(BlankNode("ANON0"), IRI("${rdf}rest"), BlankNode("ANON1")),
////                Triple(BlankNode("ANON1"), IRI("${rdf}first"), IRI("http://example.org/foo/b")),
////                Triple(BlankNode("ANON1"), IRI("${rdf}rest"), BlankNode("ANON2")),
////                Triple(BlankNode("ANON2"), IRI("${rdf}first"), IRI("http://example.org/foo/c")),
////                Triple(BlankNode("ANON2"), IRI("${rdf}rest"), IRI("${rdf}nil")),
////                Triple(IRI("http://example.org/foo/subject"), IRI("http://example.org/foo/predicate2"), IRI("${rdf}nil"))
////        )
////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/collections.ttl").text)
////        final def c = Comparator<Triple> { f, s -> f.toString().compareTo(s.toString()) }
////        results.sortedWith(c), expectedResults.sortedWith(c))
////    }
//////
//////    //TODO examples 19-26 and wordnetStinkpot.ttl
//////    final def wordnetTest() {
//////        final def expectedResults = [
//////                Triple(IRI(""),IRI(""),IRI(""))
//////        )
//////        final def results = turtle.parseTurtle(this.class.getResource("/turtle/wordnetStinkpot.ttl").text)
//////        results, expectedResults)
//////    }
////
////    final def malformedQuotedLiterals() {
////        try {
////            final def results = turtle.parseTurtle(this.class.getResource("/turtle/malformed/quotedLiterals.ttl").text)
////        } catch (exception: RuntimeException) {
////            return
////        }
////        throw RuntimeException("Test failed")
////    }
//}
