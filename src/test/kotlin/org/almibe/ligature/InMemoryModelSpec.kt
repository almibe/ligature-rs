package org.almibe.ligature

import io.kotlintest.shouldBe
import io.kotlintest.specs.StringSpec
import org.almibe.ligature.loaders.NTriplesSpec

class InMemoryModelSpec : StringSpec({
    val model = InMemoryModel()

    "test adding statement and checking subject" {
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        
        model.getSubjects() shouldBe setOf(NTriplesSpec.spiderMan, NTriplesSpec.greenGoblin)
        model.statementsFor(NTriplesSpec.spiderMan).toList() shouldBe listOf(Pair(NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin))
    }

    "test multiple statements + getter methods" {
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, LangLiteral("Dr. Octopus", "en"))
        model.addStatement(NTriplesSpec.greenGoblin, NTriplesSpec.enemyOf, NTriplesSpec.spiderMan)
        model.addStatement(NTriplesSpec.spiderMan, IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Spiderman", "en"))
        
        model.getPredicates() shouldBe setOf(NTriplesSpec.enemyOf, IRI("http://xmlns.com/foaf/0.1/name"))
        model.getSubjects() shouldBe setOf(NTriplesSpec.spiderMan, NTriplesSpec.greenGoblin)
        model.getObjects() shouldBe setOf(NTriplesSpec.greenGoblin, LangLiteral("Dr. Octopus", "en"),
                          NTriplesSpec.spiderMan, LangLiteral("Spiderman", "en"))
        model.getIRIs() shouldBe setOf(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin, IRI("http://xmlns.com/foaf/0.1/name"))
        model.getLiterals() shouldBe setOf(LangLiteral("Dr. Octopus", "en"), LangLiteral("Spiderman", "en"))
    }

    "adding a single subject should be saved without any statements attached to it" {
        model.addSubject(NTriplesSpec.spiderMan)
        
        model.getSubjects().size shouldBe 1
        model.statementsFor(NTriplesSpec.spiderMan).size shouldBe 0
        model.getSubjects().first() shouldBe NTriplesSpec.spiderMan
    }

    "removing a subject should remove all predicates attached to it and all literals attached to those predicates" {
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, LangLiteral("Dr. Octopus", "en"))
        model.addStatement(NTriplesSpec.greenGoblin, NTriplesSpec.enemyOf, NTriplesSpec.spiderMan)
        model.addStatement(NTriplesSpec.greenGoblin, IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Green Goblin", "en"))
        model.addStatement(NTriplesSpec.spiderMan, IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Spiderman", "en"))
        model.removeSubject(NTriplesSpec.greenGoblin)
        
        model.getSubjects() shouldBe setOf(NTriplesSpec.spiderMan)
        model.statementsFor(NTriplesSpec.spiderMan).size shouldBe 2
    }

    "removing statements shouldn't remove any subjects just predicates and literals" {
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        model.addStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, LangLiteral("Dr. Octopus", "en"))
        model.addStatement(NTriplesSpec.spiderMan, IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Spiderman", "en"))
        model.removeStatement(NTriplesSpec.spiderMan, NTriplesSpec.enemyOf, NTriplesSpec.greenGoblin)
        
        model.getSubjects() shouldBe setOf(NTriplesSpec.spiderMan, NTriplesSpec.greenGoblin)
        model.statementsFor(NTriplesSpec.spiderMan).size shouldBe 2
        model.statementsFor(NTriplesSpec.greenGoblin).size shouldBe 0
    }
})
