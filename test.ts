import { validIdentifier, validPlainLiteral } from "./index";

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

describe('Plain literal tests', () => {
  expect(validPlainLiteral({value : "plain lit"})).toBe(true)
  expect(validPlainLiteral({value :"", lang: ""})).toBe(false)
  expect(validPlainLiteral({value: "Hello", lang: "en"})).toBe(true)
  expect(validPlainLiteral({value: "Bonjour", lang: "fr", type: "fr"})).toBe(false) //TODO more test cases
})

describe('Typed literal tests', () => {
/*
(deftest typed-literal?-test
  (testing "Common examples"
    (is (not (typed-literal? "not a typed literal")))
    (is (not (typed-literal? {})))
    (is (typed-literal? {:value "Hello" :type "identifier"}))
    (is (not (typed-literal? {:value "56" :type "number" :lang "en"}))))) ; TODO more test cases
*/
})

describe('Valid statement tests', () => {
/*
(deftest statement?-test
  (testing "Common examples"
    (is (s/valid? ::l/statement ["hello" "world" "triple"]))
    (is (s/valid? ::l/statement ["hello" "world" "triple" "graph"]))
    (is (not (s/valid? ::l/statement [])))
    (is (not (s/valid? ::l/statement ["g"])))
    (is (not (s/valid? ::l/statement ["test" "test"])))
    (is (not (s/valid? ::l/statement ["test" "test" "g" "h" "e"])))
    (is (not (s/valid? ::l/statement [5 3 66 554])))
    (is (not (s/valid? ::l/statement ["test" "test" :a])))
    (is (s/valid? ::l/statement ["test" :a "test" "test"])))) ; TODO more test cases
*/
})

describe('Valid statements tests', () => {
/*
(deftest statements?-test
  (testing "Common examples"
    (is (s/valid? ::l/statements [["hello" "world" "triple"]]))
    (is (s/valid? ::l/statements #{["hello" "world" "triple"]}))
    (is (s/valid? ::l/statements '(["hello" "world" "triple"]))))) ; TODO more test cases
*/
})

describe('Valid lang tag tests', () => {
/*
(deftest lang-tag?-test
  (testing "Common examples"
    (is (not (lang-tag? "")))
    (is (lang-tag? "en"))
    (is (not (lang-tag? "en-")))
    (is (lang-tag? "en-fr"))
    (is (not (lang-tag? "en-fr-")))
    (is (lang-tag? "en-fr-sp"))
    (is (lang-tag? "ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj"))
    (is (not (lang-tag? "en-fr-ef ")))))
*/
})
