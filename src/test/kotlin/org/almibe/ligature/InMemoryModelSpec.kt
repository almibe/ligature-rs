package org.almibe.ligature

import io.kotlintest.shouldBe
import io.kotlintest.specs.StringSpec
import org.almibe.ligature.loaders.enemyOf
import org.almibe.ligature.loaders.greenGoblin
import org.almibe.ligature.loaders.spiderMan

class InMemoryModelSpec : StringSpec() {
    override fun isInstancePerTest() = true

    init {
        val model = InMemoryModel()

        "test adding statement and checking subject" {
            model.addStatement(spiderMan, enemyOf, greenGoblin)

            model.getSubjects() shouldBe setOf(spiderMan, greenGoblin)
            model.statementsFor(spiderMan).toList() shouldBe listOf(Pair(enemyOf, greenGoblin))
        }

        "test multiple statements + getter methods" {
            model.addStatement(spiderMan, enemyOf, greenGoblin)
            model.addStatement(spiderMan, enemyOf, LangLiteral("Dr. Octopus", "en"))
            model.addStatement(greenGoblin, enemyOf, spiderMan)
            model.addStatement(spiderMan, IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Spiderman", "en"))

            model.getPredicates() shouldBe setOf(enemyOf, IRI("http://xmlns.com/foaf/0.1/name"))
            model.getSubjects() shouldBe setOf(spiderMan, greenGoblin)
            model.getObjects() shouldBe setOf(greenGoblin, LangLiteral("Dr. Octopus", "en"),
                    spiderMan, LangLiteral("Spiderman", "en"))
            model.getIRIs() shouldBe setOf(spiderMan, enemyOf, greenGoblin, IRI("http://xmlns.com/foaf/0.1/name"))
            model.getLiterals() shouldBe setOf(LangLiteral("Dr. Octopus", "en"), LangLiteral("Spiderman", "en"))
        }

        "adding a single subject should be saved without any statements attached to it" {
            model.addSubject(spiderMan)

            model.getSubjects().size shouldBe 1
            model.statementsFor(spiderMan).size shouldBe 0
            model.getSubjects().first() shouldBe spiderMan
        }

        "removing a subject should remove all predicates attached to it and all literals attached to those predicates" {
            model.addStatement(spiderMan, enemyOf, greenGoblin)
            model.addStatement(spiderMan, enemyOf, LangLiteral("Dr. Octopus", "en"))
            model.addStatement(greenGoblin, enemyOf, spiderMan)
            model.addStatement(greenGoblin, IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Green Goblin", "en"))
            model.addStatement(spiderMan, IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Spiderman", "en"))
            model.removeSubject(greenGoblin)

            model.getSubjects() shouldBe setOf(spiderMan)
            model.statementsFor(spiderMan).size shouldBe 2
        }

        "removing statements shouldn't remove any subjects just predicates and literals" {
            model.addStatement(spiderMan, enemyOf, greenGoblin)
            model.addStatement(spiderMan, enemyOf, LangLiteral("Dr. Octopus", "en"))
            model.addStatement(spiderMan, IRI("http://xmlns.com/foaf/0.1/name"), LangLiteral("Spiderman", "en"))
            model.removeStatement(spiderMan, enemyOf, greenGoblin)

            model.getSubjects() shouldBe setOf(spiderMan, greenGoblin)
            model.statementsFor(spiderMan).size shouldBe 2
            model.statementsFor(greenGoblin).size shouldBe 0
        }
    }
}
