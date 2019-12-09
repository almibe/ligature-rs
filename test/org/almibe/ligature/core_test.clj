(ns org.almibe.ligature.core-test
  (:require [clojure.test :refer :all]
            [org.almibe.ligature.core :refer :all]))

(deftest iri?-test
  (testing "Common examples"
    (is ((iri? "") false)))
    (is ((iri? "http://localhost/people/7") true))) ;TODO more test cases

(deftest blank-node?-test
  (testing "Common examples"
    (is ((blank-node? "_:a") true)))
    (is ((blank-node? "") false))
    (is ((blank-node? "_:sometext") true))
    (is ((blank-node? "_:sometext moretext") false))) ;TODO more test cases
