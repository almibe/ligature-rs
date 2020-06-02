/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

 import org.junit.Test
import org.junit.Assert._

class Test {
  @Test def test(): Unit = {
    assertEquals("I was compiled by dotty :)", Main.msg)
  }
}
