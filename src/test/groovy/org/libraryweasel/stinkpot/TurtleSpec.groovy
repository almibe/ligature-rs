/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import spock.lang.Specification

class TurtleSpec  extends Specification {
    Stinkpot stinkpot = new Stinkpot()

    def spidermanEnemy = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))

    def spidermanName = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://xmlns.com/foaf/0.1/name"), new PlainLiteral("Spiderman"))

    def spidermanNameRu = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://xmlns.com/foaf/0.1/name"), new LangLiteral("Человек-паук", "ru"))

    def 'support basic IRI triple'() {
        given:
        def expectedResult = spidermanEnemy
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/basicTriple.ttl').text)
        then:
        results.size() == 1
        results.first() == expectedResult
    }

    def 'support predicate lists'() {
        given:
        def expectedResults = [spidermanEnemy, spidermanName]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/predicateList.ttl').text)
        then:
        results.size() == 2
        results == expectedResults
    }

    def 'support object lists'() {
        given:
        def expectedResults = [spidermanName, spidermanNameRu]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/objectList.ttl').text)
        then:
        results.size() == 2
        results == expectedResults
    }

    def 'support comments'() {
        given:
        def expectedResults = [spidermanEnemy, spidermanName]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/comments.ttl').text)
        then:
        results.size() == 2
        results == expectedResults
    }

    def 'support multi-line triples'() {
        given:
        def expectedResults = [spidermanEnemy]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/multilineTriple.ttl').text)
        then:
        results.size() == 1
        results == expectedResults
    }

    def base = "http://one.example/"
    def base2 = "http://one.example2/"
    def baseTwo = "http://two.example/"
    def baseTwo2 = "http://two.example2/"

    def base3 = "http://another.example/"

    def 'turtle iri parsing'() {
        given:
        def expectedResults = [
            new Triple(new IRI("http://one.example/subject1"), new IRI("http://one.example/predicate1"), new IRI("http://one.example/object1")),
            new Triple(new IRI("${base}subject2"), new IRI("${base}predicate2"), new IRI("${base}object2")),
            new Triple(new IRI("${base2}subject2"), new IRI("${base2}predicate2"), new IRI("${base2}object2")),
            new Triple(new IRI("${baseTwo}subject3"), new IRI("${baseTwo}predicate3"), new IRI("${baseTwo}object3")),
            new Triple(new IRI("${baseTwo2}subject3"), new IRI("${baseTwo2}predicate3"), new IRI("${baseTwo2}object3")),
            new Triple(new IRI("${base2}path/subject4"), new IRI("${base2}path/predicate4"), new IRI("${base2}path/object4")),
            new Triple(new IRI("${base3}subject5"), new IRI("${base3}predicate5"), new IRI("${base3}object5")),
            new Triple(new IRI("${base3}subject6"), new IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), new IRI("${base3}subject7")),
            new Triple(new IRI("http://伝言.example/?user=أكرم&amp;channel=R%26D"), new IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), new IRI("${base3}subject8"))
        ]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/comprehensivePrefixBaseExample.ttl').text)
        then:
        results.size() == 9
        results[0] == expectedResults[0]
        results[1] == expectedResults[1]
        results[2] == expectedResults[2]
        results[3] == expectedResults[3]
        results[4] == expectedResults[4]
        results[5] == expectedResults[5]
        results[6] == expectedResults[6]
        results[7] == expectedResults[7]
        results[8] == expectedResults[8]
    }

    //TODO test literals
    def 'support language literals'() {
        given:
        def expectedResults = [spidermanNameRu]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/literalWithLanguage.ttl').text)
        then:
        results.size() == 1
        results == expectedResults
    }

    def 'support quoted literals'() {
        given:
        def show = iri("http://example.org/vocab/show/218")
        def label = iri("http://www.w3.org/2000/01/rdf-schema#label")
        def localName = iri("http://example.org/vocab/show/localName")
        def blurb = iri("http://example.org/vocab/show/blurb")
        def multilineText = '''This is a multi-line
literal with many quotes (""""")
and up to two sequential apostrophes ('').'''
        def expectedResults = [
            triple(show, label, new PlainLiteral("That Seventies Show")),
            triple(show, label, new PlainLiteral("That Seventies Show")),
            triple(show, label, new PlainLiteral("That Seventies Show")),
            triple(show, localName, new LangLiteral("That Seventies Show", "en")),
            triple(show, localName, new LangLiteral("Cette Série des Années Soixante-dix", "fr")),
            triple(show, localName, new LangLiteral("Cette Série des Années Septante", "fr-be")),
            triple(show, blurb, new PlainLiteral(multilineText))
        ]
        when:
        List<Triple> results = stinkpot.parseTurtle(this.getClass().getResource('/turtle/quotedLiterals.ttl').text)
        then:
        results.size() == 7
        results == expectedResults
    }

    //TODO numbers.ttl
    //TODO booleans.ttl

    //TODO test blank nodes
    //TODO blankNodes.ttl

    //TODO Nesting Unlabeled Blank Nodes in Turtle
    //TODO nestedUnlabeledBlankNodes.ttl
    //TODO complexUnlabeledBlankNodes.ttl

    //TODO Collections
    //TODO collections.ttl

    //TODO examples 19-26 and wordnetStinkpot.ttl

    def triple = {s, o, p -> return new Triple(s, o, p)}
    def iri = {url -> return new IRI(url)}
}
