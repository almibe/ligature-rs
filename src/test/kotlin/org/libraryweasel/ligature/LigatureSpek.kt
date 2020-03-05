/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.ligature

(deftest identifier?-test
(testing "Common examples"
(is (not (l/identifier? "")))
(is (l/identifier? "http://localhost/people/7"))
(is (not (l/identifier? "http://localhost(/people/7")))
(is (not (l/identifier? "http://localhost /people/7")))
(is (l/identifier? "hello"))
(is (l/identifier? "_:"))
(is (l/identifier? "_:valid"))
(is (l/identifier? "_:1"))
(is (l/identifier? "_:1344")))) ; TODO more test cases

(deftest plain-literal?-test
(testing "Common examples"
(is (not (l/plain-literal? "not a lang lit")))
(is (not (l/plain-literal? {:value "" :lang ""})))
(is (l/plain-literal? {:value "Hello" :lang "en"}))
(is (not (l/plain-literal? {:value "Bonjour" :lang "fr" :type "fr"}))))) ; TODO more test cases

(deftest typed-literal?-test
(testing "Common examples"
(is (not (l/typed-literal? "not a typed literal")))
(is (not (l/typed-literal? {})))
(is (l/typed-literal? {:value "Hello" :type "identifier"}))
(is (not (l/typed-literal? {:value "56" :type "number" :lang "en"}))))) ; TODO more test cases

(deftest statement?-test
(testing "Common examples"
(is (not (s/valid? ::l/statement ["hello" "world" "triple"])))
(is (s/valid? ::l/statement ["hello" "world" "triple" "graph"]))
(is (s/valid? ::l/statement ["hello" "world" "triple" l/_]))
(is (not (s/valid? ::l/statement [])))
(is (not (s/valid? ::l/statement ["g"])))
(is (not (s/valid? ::l/statement ["test" "test"])))
(is (not (s/valid? ::l/statement ["test" "test" "g" "h" "e"])))
(is (not (s/valid? ::l/statement [5 3 66 554])))
(is (s/valid? ::l/statement ["test" l/a "test" "test"])))) ; TODO more test cases

(deftest statements?-test
(testing "Common examples"
(is (s/valid? ::l/statements [["hello" "world" "triple" l/_]]))
(is (s/valid? ::l/statements #{["hello" "world" "triple" l/_]})))) ; TODO more test cases

(deftest lang-tag?-test
(testing "Common examples"
(is (not (l/lang-tag? "")))
(is (l/lang-tag? "en"))
(is (not (l/lang-tag? "en-")))
(is (l/lang-tag? "en-fr"))
(is (not (l/lang-tag? "en-fr-")))
(is (l/lang-tag? "en-fr-sp"))
(is (l/lang-tag? "ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj"))
(is (not (l/lang-tag? "en-fr-ef ")))))
