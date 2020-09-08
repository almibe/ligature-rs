/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import dev.ligature.Ligature.{validLangTag, validNamedNode}
import munit.FunSuite

class LigatureSpec extends FunSuite {
  test("validIdentifier tests") {
    assert(!validNamedNode(LocalNode("")))
    assert(validNamedNode(LocalNode("http://localhost/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost(/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost{/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost\\/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost</people/7")))
    assert(!validNamedNode(LocalNode("http://localhost>/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost[/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost]/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost\"/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost'/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost`/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost\t/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost\n/people/7")))
    assert(!validNamedNode(LocalNode("http://localhost /people/7")))
    assert(validNamedNode(LocalNode("hello")))
    assert(validNamedNode(LocalNode("_:")))
    assert(validNamedNode(LocalNode("_:valid")))
    assert(validNamedNode(LocalNode("_:1")))
    assert(validNamedNode(LocalNode("_:1344")))

    //TODO add set of tests for IRINode
  }

  test("validLangTag tests") {
    assert(!validLangTag(""))
    assert(validLangTag("en"))
    assert(!validLangTag("en-"))
    assert(validLangTag("en-fr"))
    assert(!validLangTag("en-fr-"))
    assert(validLangTag("en-fr-sp"))
    assert(validLangTag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj"))
    assert(!validLangTag("en-fr-ef "))
  }
}
