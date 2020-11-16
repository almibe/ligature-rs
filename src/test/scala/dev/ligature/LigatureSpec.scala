/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature

import dev.ligature.Ligature.{validLangTag, validAttribute, validDataset}
import munit.FunSuite

class LigatureSpec extends FunSuite {
  test("validDataset tests") {
    assert(!validDataset(Dataset("")))
    assert(!validDataset(Dataset("http://localhost/people/7")))
    assert(!validDataset(Dataset("http://localhost(/people/7")))
    assert(!validDataset(Dataset("http://localhost{/people/7")))
    assert(!validDataset(Dataset("http://localhost\\/people/7")))
    assert(!validDataset(Dataset("http://localhost</people/7")))
    assert(!validDataset(Dataset("http://localhost>/people/7")))
    assert(!validDataset(Dataset("http://localhost[/people/7")))
    assert(!validDataset(Dataset("http://localhost]/people/7")))
    assert(!validDataset(Dataset("http://localhost\"/people/7")))
    assert(!validDataset(Dataset("http://localhost'/people/7")))
    assert(!validDataset(Dataset("http://localhost`/people/7")))
    assert(!validDataset(Dataset("http://localhost\t/people/7")))
    assert(!validDataset(Dataset("http://localhost\n/people/7")))
    assert(!validDataset(Dataset("http://localhost /people/7")))
    assert(validDataset(Dataset("hello")))
    assert(!validDataset(Dataset("_:")))
    assert(!validDataset(Dataset("_:valid")))
    assert(!validDataset(Dataset("_:1")))
    assert(!validDataset(Dataset("_:1344")))
    assert(validDataset(Dataset("test/test")))
    assert(!validDataset(Dataset("/test/test")))
    assert(!validDataset(Dataset("test/test/")))
    assert(!validDataset(Dataset("tEst/test")))
    assert(!validDataset(Dataset("test//test")))
    assert(validDataset(Dataset("test/test_/_/_")))
  }

  test("validAttribute tests") {
    assert(!validAttribute(Predicate("")))
    assert(validAttribute(Predicate("http://localhost/people/7")))
    assert(!validAttribute(Predicate("http://localhost(/people/7")))
    assert(!validAttribute(Predicate("http://localhost{/people/7")))
    assert(!validAttribute(Predicate("http://localhost\\/people/7")))
    assert(!validAttribute(Predicate("http://localhost</people/7")))
    assert(!validAttribute(Predicate("http://localhost>/people/7")))
    assert(!validAttribute(Predicate("http://localhost[/people/7")))
    assert(!validAttribute(Predicate("http://localhost]/people/7")))
    assert(!validAttribute(Predicate("http://localhost\"/people/7")))
    assert(!validAttribute(Predicate("http://localhost'/people/7")))
    assert(!validAttribute(Predicate("http://localhost`/people/7")))
    assert(!validAttribute(Predicate("http://localhost\t/people/7")))
    assert(!validAttribute(Predicate("http://localhost\n/people/7")))
    assert(!validAttribute(Predicate("http://localhost /people/7")))
    assert(validAttribute(Predicate("hello")))
    assert(validAttribute(Predicate("_:")))
    assert(validAttribute(Predicate("_:valid")))
    assert(validAttribute(Predicate("_:1")))
    assert(validAttribute(Predicate("_:1344")))
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
