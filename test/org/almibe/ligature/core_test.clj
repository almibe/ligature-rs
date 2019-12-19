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
    (is (statement? (statement "hello" "world" "triple")))))
