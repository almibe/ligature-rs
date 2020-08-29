/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import io.kotest.core.spec.style.StringSpec
import io.kotest.matchers.shouldBe

class LigatureSpec : StringSpec({
  "validIdentifier tests" {
    validNamedElement("") shouldBe false
    validNamedElement("http://localhost/people/7") shouldBe true
    validNamedElement("http://localhost(/people/7") shouldBe false
    validNamedElement("http://localhost /people/7") shouldBe false
    validNamedElement("hello") shouldBe true
    validNamedElement("_:") shouldBe true
    validNamedElement("_:valid") shouldBe true
    validNamedElement("_:1") shouldBe true
    validNamedElement("_:1344") shouldBe true
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
