/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import org.almibe.ligature.*
import spock.lang.Specification

class TurtleSpec extends Specification {
    def ligature = new Ligature(new InMemoryModel())
    def expectedModel = new InMemoryModel()
    final def xsd = "http://www.w3.org/2001/XMLSchema#"
    final def foafKnows = new IRI("http://xmlns.com/foaf/0.1/knows")
    final def rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#"

//    final def spidermanEnemy = new Triple(new IRI("http://example.org/#spiderman"),
//        new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
//
//    final def spidermanName = new Triple(new IRI("http://example.org/#spiderman"),
//        new IRI("http://xmlns.com/foaf/0.1/name"), new TypedLiteral("Spiderman", new IRI("http://www.w3.org/2001/XMLSchema#string")))
//
//    final def spidermanNameRu = new Triple(new IRI("http://example.org/#spiderman"),
//        new IRI("http://xmlns.com/foaf/0.1/name"), new LangLiteral("Человек-паук", "ru"))
    final def stringIRI = new IRI("http://www.w3.org/2001/XMLSchema#string")

    boolean compareModels(Model results, Model expectedResults) {
        assert results.subjects.each { subject ->
            assert results.statementsFor(subject) == expectedResults.statementsFor(subject)
        }
        assert results.subjects == expectedResults.subjects
        assert results.objects == expectedResults.objects
        assert results.predicates == expectedResults.predicates
        return true
    }

    final def "support basic IRI triple"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/01-basicTriple.ttl").text)
        expectedModel.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "support predicate lists"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/02-predicateList.ttl").text)
        expectedModel.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        expectedModel.addStatement(NTriplesSpec.spiderMan, new IRI("http://xmlns.com/foaf/0.1/name"),
                new TypedLiteral("Spiderman", new IRI("http://www.w3.org/2001/XMLSchema#string")))
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "support object lists"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/03-objectList.ttl").text)
        expectedModel.addStatement(NTriplesSpec.spiderMan, new IRI("http://xmlns.com/foaf/0.1/name"),
                new TypedLiteral("Spiderman", new IRI("http://www.w3.org/2001/XMLSchema#string")))
        expectedModel.addStatement(NTriplesSpec.spiderMan, new IRI("http://xmlns.com/foaf/0.1/name"),
                new LangLiteral("Человек-паук", "ru"))
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "support comments"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/04-comments.ttl").text)
        expectedModel.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        expectedModel.addStatement(NTriplesSpec.spiderMan, new IRI("http://xmlns.com/foaf/0.1/name"),
                new TypedLiteral("Spiderman", new IRI("http://www.w3.org/2001/XMLSchema#string")))
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "support multiline triples"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/05-multilineTriple.ttl").text)
        expectedModel.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        expect:
        compareModels(ligature, expectedModel)
    }

    final def base = "http://one.example/"
    final def base2 = "http://one.example2/"
    final def baseTwo = "http://two.example/"
    final def baseTwo2 = "http://two.example2/"
    final def base3 = "http://another.example/"

    final def "turtle IRI parsing with base"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/06-baseTriples.ttl").text)
        expectedModel.addStatement(new IRI("${base}subject2"), new IRI("${base}predicate2"), new IRI("${base}object2"))
        expectedModel.addStatement(new IRI("${base2}subject2"), new IRI("${base2}predicate2"), new IRI("${base2}object2"))
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "turtle IRI parsing with prefixes"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/07-prefixTriples.ttl").text)
        expectedModel.addStatement(new IRI("${baseTwo}subject3"), new IRI("${baseTwo}predicate3"), new IRI("${baseTwo}object3"))
        expectedModel.addStatement(new IRI("${baseTwo2}subject3"), new IRI("${baseTwo2}predicate3"), new IRI("${baseTwo2}object3"))
        expectedModel.addStatement(new IRI("${base2}path/subject4"), new IRI("${base2}path/predicate4"), new IRI("${base2}path/object4"))
        expectedModel.addStatement(new IRI("${base3}subject5"), new IRI("${base3}predicate5"), new IRI("${base3}object5"))
        expectedModel.addStatement(new IRI("${base3}subject6"), new IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), new IRI("${base3}subject7"))
        expectedModel.addStatement(new IRI("http://伝言.example/?user=أكرم&amp;channel=R%26D"), new IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), new IRI("${base3}subject8"))
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "support language literals"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/08-literalWithLanguage.ttl").text)
        expectedModel.addStatement(new IRI("http://example.org/#spiderman"),
                new IRI("http://xmlns.com/foaf/0.1/name"), new LangLiteral("Человек-паук", "ru"))
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "support quoted literals"() {
        given:
        final def base = "http://www.libraryweasel.org/fake/madeup#"
        final def show = new IRI("http://example.org/vocab/show/218")
        final def show219 = new IRI("http://example.org/vocab/show/219")
        final def label = new IRI("http://www.w3.org/2000/01/rdf-schema#label")
        final def localName = new IRI("http://example.org/vocab/show/localName")
        final def blurb = new IRI("http://example.org/vocab/show/blurb")
        final def multilineText = "This is a multi-line\n" +
            "literal with many quotes (\"\"\"\"\")\n" +
            "and up to two sequential apostrophes ('')."
        final def multilineText2 = "Another\n" +
            "multiline string with' 'a' \"custom datatype\"\\\"."
        ligature.loadTurtle(this.class.getResource("/turtle/09-quotedLiterals.ttl").text)
        expectedModel.addStatement(show, label, new TypedLiteral("That Seventies Show", stringIRI))
        expectedModel.addStatement(show, new IRI("${base}pred"), new TypedLiteral("That Seventies Show", new IRI("${base}string")))
        expectedModel.addStatement(show, localName, new LangLiteral("That Seventies Show", "en"))
        expectedModel.addStatement(show, localName, new LangLiteral("Cette Série des Années Soixante-dix", "fr"))
        expectedModel.addStatement(show, localName, new LangLiteral("Cette Série des Années Septante", "fr-be"))
        expectedModel.addStatement(show, blurb, new TypedLiteral(multilineText, stringIRI))
        expectedModel.addStatement(show219, blurb, new TypedLiteral(multilineText2, new IRI("${base}long-string")))
        expectedModel.addStatement(show219, blurb, new TypedLiteral("", stringIRI))
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "support number types"() {
        given:
        final def helium = "http://en.wikipedia.org/wiki/Helium"
        final def prefix = "http://example.org/elements"
        ligature.loadTurtle(this.class.getResource("/turtle/10-numbers.ttl").text)
        expectedModel.addStatement(new IRI(helium), new IRI("${prefix}atomicNumber"), new TypedLiteral("2", new IRI("${xsd}integer")))
        expectedModel.addStatement(new IRI(helium), new IRI("${prefix}atomicMass"), new TypedLiteral("4.002602", new IRI("${xsd}float")))
        expectedModel.addStatement(new IRI(helium), new IRI("${prefix}specificGravity"), new TypedLiteral("1.663E-4", new IRI("${xsd}double")))
        expect:
        compareModels(ligature, expectedModel)
    }

    final def "support booleans"() {
        given:
        ligature.loadTurtle(this.class.getResource("/turtle/11-booleans.ttl").text)
        expectedModel.addStatement(new IRI("http://somecountry.example/census2007"), new IRI("http://example.org/stats/isLandlocked"),
                new TypedLiteral("false", new IRI("${xsd}boolean")))
        expect:
        compareModels(ligature, expectedModel)
    }

//    final def "support blank nodes"() {
//        given:
//        final def expectedResults = [
//                new Triple(new LabeledBlankNode("alice"), new IRI("http://xmlns.com/foaf/0.1/knows"), new LabeledBlankNode("bob")),
//                new Triple(new LabeledBlankNode("bob"), new IRI("http://xmlns.com/foaf/0.1/knows"), new LabeledBlankNode("alice"))
//        ]
//        ligature.loadTurtle(this.class.getResource("/turtle/12-blankNodes.ttl").text)
//        expect:
//        compareModels(ligature, expectedModel)
//    }
//
//    final def "unlabeled blank nodes"() {
//        given:
//        final def expectedResults = [
//                new Triple(new IRI("http://example.com/person/bob"), foafKnows, new IRI("http://example.com/person/george")),
//                new Triple(new LabeledBlankNode("ANON0"), foafKnows, new IRI("http://example.com/person/george")),
//                new Triple(new IRI("http://example.com/person/bob"), foafKnows, new LabeledBlankNode("ANON1")),
//                new Triple(new LabeledBlankNode("ANON2"), new IRI("http://xmlns.com/foaf/0.1/knows"), new LabeledBlankNode("ANON3"))
//        ]
//        ligature.loadTurtle(this.class.getResource("/turtle/13-unlabeledBlankNodes.ttl").text)
//        expect:
//        compareModels(ligature, expectedModel)
//    }
//
//    final def "nested unlabeled blank nodes"() {
//        given:
//        final def expectedResults = [
//                new Triple(new LabeledBlankNode("ANON1"), new IRI("http://xmlns.com/foaf/0.1/name"), new TypedLiteral("Bob", stringIRI)),
//                new Triple(new LabeledBlankNode("ANON0"), new IRI("http://xmlns.com/foaf/0.1/knows"), new LabeledBlankNode("ANON1"))
//        ]
//        ligature.loadTurtle(this.class.getResource("/turtle/14-nestedUnlabeledBlankNodes.ttl").text)
//        expect:
//        compareModels(ligature, expectedModel)
//    }
//////
//////    final def complexUnlabeledBlankNodes() {
//////        final def expectedResults = [
//////            new Triple(new BlankNode("ANON0"), new IRI("http://xmlns.com/foaf/0.1/name"), new TypedLiteral("Alice", stringIRI)),
//////            new Triple(new BlankNode("ANON1"), new IRI("http://xmlns.com/foaf/0.1/name"), new TypedLiteral("Bob", stringIRI)),
//////            new Triple(new BlankNode("ANON0"), new IRI("http://xmlns.com/foaf/0.1/knows"), new BlankNode("ANON1")),
//////            new Triple(new BlankNode("ANON2"), new IRI("http://xmlns.com/foaf/0.1/name"), new TypedLiteral("Eve", stringIRI)),
//////            new Triple(new BlankNode("ANON1"), new IRI("http://xmlns.com/foaf/0.1/knows"), new BlankNode("ANON2")),
//////            new Triple(new BlankNode("ANON1"), new IRI("http://xmlns.com/foaf/0.1/mbox"), new IRI("http://bob@example.com"))
//////        )
//////        ligature.loadTurtle(this.class.getResource("/turtle/15-complexUnlabeledBlankNodes.ttl").text)
//////        final def c = Comparator<Triple> { f, s -> f.toString().compareTo(s.toString()) }
//////        results.sortedWith(c), expectedResults.sortedWith(c))
//////    }
//////
//////    final def supportCollections() {
//////        final def expectedResults = [
//////                new Triple(new IRI("http://example.org/foo/subject"), new IRI("http://example.org/foo/predicate"), new BlankNode("ANON0")),
//////                new Triple(new BlankNode("ANON0"), new IRI("${rdf}first"), new IRI("http://example.org/foo/a")),
//////                new Triple(new BlankNode("ANON0"), new IRI("${rdf}rest"), new BlankNode("ANON1")),
//////                new Triple(new BlankNode("ANON1"), new IRI("${rdf}first"), new IRI("http://example.org/foo/b")),
//////                new Triple(new BlankNode("ANON1"), new IRI("${rdf}rest"), new BlankNode("ANON2")),
//////                new Triple(new BlankNode("ANON2"), new IRI("${rdf}first"), new IRI("http://example.org/foo/c")),
//////                new Triple(new BlankNode("ANON2"), new IRI("${rdf}rest"), new IRI("${rdf}nil")),
//////                new Triple(new IRI("http://example.org/foo/subject"), new IRI("http://example.org/foo/predicate2"), new IRI("${rdf}nil"))
//////        )
//////        ligature.loadTurtle(this.class.getResource("/turtle/16-collections.ttl").text)
//////        final def c = Comparator<Triple> { f, s -> f.toString().compareTo(s.toString()) }
//////        results.sortedWith(c), expectedResults.sortedWith(c))
//////    }
////////
////////    //TODO examples 19-26 and wordnetStinkpot.ttl
////////    final def wordnetTest() {
////////        final def expectedResults = [
////////                new Triple(new IRI(""),IRI(""),IRI(""))
////////        )
////////        ligature.loadTurtle(this.class.getResource("/turtle/wordnetStinkpot.ttl").text)
////////        compareModels(ligature, expectedModel)
////////    }
//////
//////    final def malformedQuotedLiterals() {
//////        try {
//////            ligature.loadTurtle(this.class.getResource("/turtle/malformed/09-quotedLiterals.ttl").text)
//////        } catch (exception: RuntimeException) {
//////            return
//////        }
//////        throw RuntimeException("Test failed")
//////    }
}
