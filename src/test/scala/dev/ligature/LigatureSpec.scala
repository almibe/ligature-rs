/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import dev.ligature.Ligature.validLabel
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

class LigatureSpec extends AnyFlatSpec with Matchers {
  it should "validIdentifier tests" in {
    validLabel("") shouldBe false
    validLabel("http://localhost/people/7") shouldBe true
    validLabel("http://localhost(/people/7") shouldBe false
    validLabel("http://localhost{/people/7") shouldBe false
    validLabel("http://localhost\\/people/7") shouldBe false
    validLabel("http://localhost</people/7") shouldBe false
    validLabel("http://localhost>/people/7") shouldBe false
    validLabel("http://localhost[/people/7") shouldBe false
    validLabel("http://localhost]/people/7") shouldBe false
    validLabel("http://localhost\"/people/7") shouldBe false
    validLabel("http://localhost'/people/7") shouldBe false
    validLabel("http://localhost`/people/7") shouldBe false
    validLabel("http://localhost\t/people/7") shouldBe false
    validLabel("http://localhost\n/people/7") shouldBe false
    validLabel("http://localhost /people/7") shouldBe false
    validLabel("hello") shouldBe true
    validLabel("_:") shouldBe true
    validLabel("_:valid") shouldBe true
    validLabel("_:1") shouldBe true
    validLabel("_:1344") shouldBe true
    validLabel("@@test") shouldBe false
  }
}
