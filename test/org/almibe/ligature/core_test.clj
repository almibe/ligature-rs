(ns org.almibe.ligature.core-test
  (:require [clojure.test :refer :all]
            [org.almibe.ligature.core :refer :all]))

(deftest identifier?-test
  (testing "Common examples"
    (is ((identifier? "") false)))
    (is ((identifier? "http://localhost/people/7") true))
    (is ((identifier? "http://localhost(/people/7") false))
    (is ((identifier? "http://localhost /people/7") false))) ;TODO more test cases

(deftest string-literal?-test
  (testing "Common examples"
    (is (string-literal? "This is a string") true)
    (is (string-literal? "") true)
    (is (string-literal? {}) false))) ;TODO more test cases

(deftest lang-literal?-test
  (testing "Common examples"
    (is (lang-literal? "not a lang lit") false)
    (is (lang-literal? {:value "" :lang ""}) false)
    (is (lang-literal? {:value "Hello" :lang "en"}) true)
    (is (lang-literal? {:value "Bonjour" :lang "fr" :type "fr"}) false))) ; TODO more test cases

(deftest typed-literal?-test
  (testing "Common examples"
    (is (typed-literal? "not a typed literal") false)
    (is (typed-literal? {}) false)
    (is (typed-literal? {:value "Hello" :type "identifier"}) true)
    (is (typed-literal? {:value "56" :type "number" :lang "en"}) false))) ; TODO more test cases

(deftest literal?-test
  (testing "Common examples"
    ))