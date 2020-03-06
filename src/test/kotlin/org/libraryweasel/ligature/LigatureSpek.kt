/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.ligature

import io.kotest.core.spec.style.StringSpec
import io.kotest.matchers.shouldBe

class MyTests : StringSpec({
    "validIdentifier tests" {
        validIdentifier("") shouldBe false
        validIdentifier("http://localhost/people/7") shouldBe true
        validIdentifier("http://localhost(/people/7") shouldBe false
        validIdentifier("http://localhost /people/7") shouldBe false
        validIdentifier("hello") shouldBe true
        validIdentifier("_:") shouldBe true
        validIdentifier("_:valid") shouldBe true
        validIdentifier("_:1") shouldBe true
        validIdentifier("_:1344") shouldBe true
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
