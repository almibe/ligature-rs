/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import dev.ligature.Ligature.{validLangTag, validNamedNode}
import munit.FunSuite

class LigatureSpec extends FunSuite {
  test("validIdentifier tests") {
    assert(validNamedNode(LocalNode("")) == false)
    assert(validNamedNode(LocalNode("http://localhost/people/7")) == true)
    assert(validNamedNode(LocalNode("http://localhost(/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost{/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost\\/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost</people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost>/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost[/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost]/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost\"/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost'/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost`/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost\t/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost\n/people/7")) == false)
    assert(validNamedNode(LocalNode("http://localhost /people/7")) == false)
    assert(validNamedNode(LocalNode("hello")) == true)
    assert(validNamedNode(LocalNode("_:")) == true)
    assert(validNamedNode(LocalNode("_:valid")) == true)
    assert(validNamedNode(LocalNode("_:1")) == true)
    assert(validNamedNode(LocalNode("_:1344")) == true)

    //TODO add set of tests for IRINode
  }

  test("validLangTag tests") {
    assert(validLangTag("") == false)
    assert(validLangTag("en") == true)
    assert(validLangTag("en-") == false)
    assert(validLangTag("en-fr") == true)
    assert(validLangTag("en-fr-") == false)
    assert(validLangTag("en-fr-sp") == true)
    assert(validLangTag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj") == true)
    assert(validLangTag("en-fr-ef ") == false)
  }
}
