/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import dev.ligature.Ligature.{validLangTag, validNamedNode}
import munit.FunSuite

class LigatureSpec extends FunSuite {
  test("validIdentifier tests") {
    assert(!validNamedNode(NamedNode("")))
    assert(validNamedNode(NamedNode("http://localhost/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost(/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost{/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost\\/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost</people/7")))
    assert(!validNamedNode(NamedNode("http://localhost>/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost[/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost]/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost\"/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost'/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost`/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost\t/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost\n/people/7")))
    assert(!validNamedNode(NamedNode("http://localhost /people/7")))
    assert(validNamedNode(NamedNode("hello")))
    assert(validNamedNode(NamedNode("_:")))
    assert(validNamedNode(NamedNode("_:valid")))
    assert(validNamedNode(NamedNode("_:1")))
    assert(validNamedNode(NamedNode("_:1344")))
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
