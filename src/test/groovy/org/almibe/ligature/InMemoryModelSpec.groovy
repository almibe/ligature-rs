package org.almibe.ligature

import kotlin.Pair
import org.almibe.ligature.loaders.NTriplesSpec
import spock.lang.Specification

class InMemoryModelSpec extends Specification {
    def model = new InMemoryModel()

    def "test adding statement and checking subject"() {
        given:
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        expect:
        model.subjects == [NTriplesSpec.spiderMan, NTriplesSpec.greenGoblin].toSet()
        model.statementsFor(NTriplesSpec.spiderMan).toList() == [new Pair(NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)]
    }

    def "test multiple statements + getter methods"() {
        given:
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, new LangLiteral("Dr. Octopus", "en"))
        model.addStatement(NTriplesSpec.greenGoblin, NTriplesSpec.enemyOf, NTriplesSpec.spiderMan)
        model.addStatement(NTriplesSpec.spiderMan, new IRI("http://xmlns.com/foaf/0.1/name"), new LangLiteral("Spiderman", "en"))
        expect:
        model.predicates == [NTriplesSpec.enemyOf, new IRI("http://xmlns.com/foaf/0.1/name")].toSet()
        model.subjects == [NTriplesSpec.spiderMan, NTriplesSpec.greenGoblin].toSet()
        model.objects == [NTriplesSpec.greenGoblin, new LangLiteral("Dr. Octopus", "en"),
                          NTriplesSpec.spiderMan, new LangLiteral("Spiderman", "en")].toSet()
        model.IRIs == [NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin, new IRI("http://xmlns.com/foaf/0.1/name")].toSet()
        model.literals == [new LangLiteral("Dr. Octopus", "en"), new LangLiteral("Spiderman", "en")].toSet()
    }

//    def "adding a single subject should be saved without any statements attached to it"() {
//
//    }
//
//    def "removing a subject should remove all predicates attached to it and all literals attached to those predicates"() {
//
//    }
//
//    def "removing statements shouldn't remove any subjects just predicates and literals"() {
//
//    }
}
