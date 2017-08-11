package org.almibe.ligature

import kotlin.Pair
import org.almibe.ligature.loaders.NTriplesSpec
import spock.lang.Specification

class InMemoryModelSpec extends Specification {
    def graph = new InMemoryModel()

    def "test adding statement and checking subject"() {
        given:
        graph.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        expect:
        graph.subjects == [NTriplesSpec.spiderMan].toSet()
        graph.getStatements(NTriplesSpec.spiderMan).toList() == [new Pair(NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)]
    }

    def "test multiple statements + getter methods"() {
        given:
        graph.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        graph.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, new LangLiteral("Dr. Octopus", "en"))
        graph.addStatement(NTriplesSpec.greenGoblin, NTriplesSpec.enemyOf, NTriplesSpec.spiderMan)
        graph.addStatement(NTriplesSpec.spiderMan, new IRI("http://xmlns.com/foaf/0.1/name"), new LangLiteral("Spiderman", "en"))
        expect:
        graph.predicates == [NTriplesSpec.enemyOf, new IRI("http://xmlns.com/foaf/0.1/name")].toSet()
        graph.subjects == [NTriplesSpec.spiderMan, NTriplesSpec.greenGoblin].toSet()
        graph.objects == [NTriplesSpec.greenGoblin, new LangLiteral("Dr. Octopus", "en"),
                          NTriplesSpec.spiderMan, new LangLiteral("Spiderman", "en")].toSet()
        graph.IRIs == [NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin, new IRI("http://xmlns.com/foaf/0.1/name")].toSet()
        graph.literals == [new LangLiteral("Dr. Octopus", "en"), new LangLiteral("Spiderman", "en")].toSet()
    }
}
