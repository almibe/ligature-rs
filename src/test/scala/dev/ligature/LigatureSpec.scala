/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

class LigatureSpec extends AnyFlatSpec with Matchers {
}

import io.kotlintest.shouldBe
import io.kotlintest.specs.StringSpec

class LigatureSpec : StringSpec({
"validIdentifier tests" {
validNamedEntity("") shouldBe false
validNamedEntity("http://localhost/people/7") shouldBe true
validNamedEntity("http://localhost(/people/7") shouldBe false
validNamedEntity("http://localhost /people/7") shouldBe false
validNamedEntity("hello") shouldBe true
validNamedEntity("_:") shouldBe true
validNamedEntity("_:valid") shouldBe true
validNamedEntity("_:1") shouldBe true
validNamedEntity("_:1344") shouldBe true
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
