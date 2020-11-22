/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import io.kotest.core.spec.style.StringSpec
import io.kotest.matchers.shouldBe

class LigatureSpec : StringSpec ({
  "validDataset tests" {
    validDataset(Dataset("")) shouldBe false
    validDataset(Dataset("http://localhost/people/7")) shouldBe false
    validDataset(Dataset("http://localhost(/people/7")) shouldBe false
    validDataset(Dataset("http://localhost{/people/7")) shouldBe false
    validDataset(Dataset("http://localhost\\/people/7")) shouldBe false
    validDataset(Dataset("http://localhost</people/7")) shouldBe false
    validDataset(Dataset("http://localhost>/people/7")) shouldBe false
    validDataset(Dataset("http://localhost[/people/7")) shouldBe false
    validDataset(Dataset("http://localhost]/people/7")) shouldBe false
    validDataset(Dataset("http://localhost\"/people/7")) shouldBe false
    validDataset(Dataset("http://localhost'/people/7")) shouldBe false
    validDataset(Dataset("http://localhost`/people/7")) shouldBe false
    validDataset(Dataset("http://localhost\t/people/7")) shouldBe false
    validDataset(Dataset("http://localhost\n/people/7")) shouldBe false
    validDataset(Dataset("http://localhost /people/7")) shouldBe false
    validDataset(Dataset("hello")) shouldBe true
    validDataset(Dataset("_:")) shouldBe false
    validDataset(Dataset("_:valid")) shouldBe false
    validDataset(Dataset("_:1")) shouldBe false
    validDataset(Dataset("_:1344")) shouldBe false
    validDataset(Dataset("test/test")) shouldBe true
    validDataset(Dataset("/test/test")) shouldBe false
    validDataset(Dataset("test/test/")) shouldBe false
    validDataset(Dataset("tEst/test")) shouldBe false
    validDataset(Dataset("test//test")) shouldBe false
    validDataset(Dataset("test/test_/_/_")) shouldBe true
  }

  "validIdentifier tests" {
    validNamedNode(NamedNode("")) shouldBe false
    validNamedNode(NamedNode("http://localhost/people/7")) shouldBe true
    validNamedNode(NamedNode("http://localhost(/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost{/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost\\/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost</people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost>/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost[/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost]/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost\"/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost'/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost`/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost\t/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost\n/people/7")) shouldBe false
    validNamedNode(NamedNode("http://localhost /people/7")) shouldBe false
    validNamedNode(NamedNode("hello")) shouldBe true
    validNamedNode(NamedNode("_:")) shouldBe true
    validNamedNode(NamedNode("_:valid")) shouldBe true
    validNamedNode(NamedNode("_:1")) shouldBe true
    validNamedNode(NamedNode("_:1344")) shouldBe true
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
