/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import io.kotlintest.shouldBe
import io.kotlintest.specs.StringSpec

class LigatureSpec : StringSpec({
    "validIdentifier tests" {
        validPredicate("") shouldBe false
        validPredicate("http://localhost/people/7") shouldBe true
        validPredicate("http://localhost(/people/7") shouldBe false
        validPredicate("http://localhost /people/7") shouldBe false
        validPredicate("hello") shouldBe true
        validPredicate("_:") shouldBe true
        validPredicate("_:valid") shouldBe true
        validPredicate("_:1") shouldBe true
        validPredicate("_:1344") shouldBe true
    }

    "validLangTag tests" {
        validLangTag("") shouldBe false
        validLangTag("en") shouldBe true
        validLangTag("en-") shouldBe false
        validLangTag("en-fr") shouldBe true
        validLangTag("en-fr-") shouldBe false
        validLangTag("en-fr-sp") shouldBe true
        validLangTag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj") shouldBe true
        validLangTag("en-fr-ef ") shouldBe false
    }
})
