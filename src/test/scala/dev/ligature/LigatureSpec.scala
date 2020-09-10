/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import dev.ligature.Ligature.{validLangTag, validIdNode}
import munit.FunSuite

class LigatureSpec extends FunSuite {
  test("validIdentifier tests") {
    assert(!validIdNode(IdNode("")))
    assert(validIdNode(IdNode("http://localhost/people/7")))
    assert(!validIdNode(IdNode("http://localhost(/people/7")))
    assert(!validIdNode(IdNode("http://localhost{/people/7")))
    assert(!validIdNode(IdNode("http://localhost\\/people/7")))
    assert(!validIdNode(IdNode("http://localhost</people/7")))
    assert(!validIdNode(IdNode("http://localhost>/people/7")))
    assert(!validIdNode(IdNode("http://localhost[/people/7")))
    assert(!validIdNode(IdNode("http://localhost]/people/7")))
    assert(!validIdNode(IdNode("http://localhost\"/people/7")))
    assert(!validIdNode(IdNode("http://localhost'/people/7")))
    assert(!validIdNode(IdNode("http://localhost`/people/7")))
    assert(!validIdNode(IdNode("http://localhost\t/people/7")))
    assert(!validIdNode(IdNode("http://localhost\n/people/7")))
    assert(!validIdNode(IdNode("http://localhost /people/7")))
    assert(validIdNode(IdNode("hello")))
    assert(validIdNode(IdNode("_:")))
    assert(validIdNode(IdNode("_:valid")))
    assert(validIdNode(IdNode("_:1")))
    assert(validIdNode(IdNode("_:1344")))
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
