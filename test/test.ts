import { validIdentifier, validPlainLiteral, validTypedLiteral, validLangTag } from "./index";

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

test('Valid identifier tests', () => {
  expect(validIdentifier("")).toBe(false);
  expect(validIdentifier("http://localhost/people/7")).toBe(true)
  expect(validIdentifier("http://localhost(/people/7")).toBe(false)
  expect(validIdentifier("http://localhost /people/7")).toBe(false)
  expect(validIdentifier("hello")).toBe(true)
  expect(validIdentifier("_:")).toBe(true)
  expect(validIdentifier("_:valid")).toBe(true)
  expect(validIdentifier("_:1")).toBe(true)
  expect(validIdentifier("_:1344")).toBe(true) //TODO more test cases
})

test('Plain literal tests', () => {
  expect(validPlainLiteral({value : "plain lit"})).toBe(true)
  expect(validPlainLiteral({value :"", lang: ""})).toBe(false)
  expect(validPlainLiteral({value: "Hello", lang: "en"})).toBe(true)
  expect(validPlainLiteral({value: "Bonjour", lang: "fr", type: "fr"})).toBe(false) //TODO more test cases
})

test('Typed literal tests', () => {
  expect(validTypedLiteral({value: "Hello", type: "identifier"})).toBe(true)
  expect(validTypedLiteral({value: "56", type: "number", lang: "en"})).toBe(false) //TODO more test cases
})

test('Valid lang tag tests', () => {
  expect(validLangTag("")).toBe(false)
  expect(validLangTag("en")).toBe(true)
  expect(validLangTag("en-")).toBe(false)
  expect(validLangTag("en-fr")).toBe(true)
  expect(validLangTag("en-fr-")).toBe(false)
  expect(validLangTag("en-fr-sp")).toBe(true)
  expect(validLangTag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj")).toBe(true)
  expect(validLangTag("en-fr-ef ")).toBe(false)
})
