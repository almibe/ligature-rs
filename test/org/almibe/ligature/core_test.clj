(ns org.almibe.ligature.core-test
  (:require [clojure.test :refer :all]
            [org.almibe.ligature.core :refer :all]))

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
    (is (statement? (statement "hello" "world" "triple")))
    (is (statement? (statement "hello" "world" "triple" "graph")))
    (is (not (statement? (statement 5 3 66 554))))
    (is (not (statement? (statement "test" "test" :a)))))) ; TODO more test cases

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
