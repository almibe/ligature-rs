/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

test('adds 1 + 2 to equal 3', () => {
  expect((1 + 2)).toBe(3);
});

/*
(deftest identifier?-test
  (testing "Common examples"
    (is (not (identifier? "")))
    (is (identifier? "http://localhost/people/7"))
    (is (not (identifier? "http://localhost(/people/7")))
    (is (not (identifier? "http://localhost /people/7")))
    (is (identifier? "hello"))
    (is (identifier? "_:"))
    (is (identifier? "_:valid"))
    (is (identifier? "_:1"))
    (is (identifier? "_:1344")))) ; TODO more test cases

(deftest lang-literal?-test
  (testing "Common examples"
    (is (not (lang-literal? "not a lang lit")))
    (is (not (lang-literal? {:value "" :lang ""})))
    (is (lang-literal? {:value "Hello" :lang "en"}))
    (is (not (lang-literal? {:value "Bonjour" :lang "fr" :type "fr"}))))) ; TODO more test cases

(deftest typed-literal?-test
  (testing "Common examples"
    (is (not (typed-literal? "not a typed literal")))
    (is (not (typed-literal? {})))
    (is (typed-literal? {:value "Hello" :type "identifier"}))
    (is (not (typed-literal? {:value "56" :type "number" :lang "en"}))))) ; TODO more test cases

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

(deftest statements?-test
  (testing "Common examples"
    (is (s/valid? ::l/statements [["hello" "world" "triple"]]))
    (is (s/valid? ::l/statements #{["hello" "world" "triple"]}))
    (is (s/valid? ::l/statements '(["hello" "world" "triple"]))))) ; TODO more test cases

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