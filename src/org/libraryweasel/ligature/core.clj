; Copyright (c) 2019-2020 Alex Michael Berry
;
; This program and the accompanying materials are made
; available under the terms of the Eclipse Public License 2.0
; which is available at https://www.eclipse.org/legal/epl-2.0/
;
; SPDX-License-Identifier: EPL-2.0

(ns org.libraryweasel.ligature.core
  (:require [clojure.spec.alpha :as s]))

(defprotocol LigatureStore
  "A Store manages many named Collections."
  (collection [this collection-name]
    "Returns a collection based on the name passed.
    Calling this function will not create a new collection, it just binds a Store and Collection name.")
  (delete-collection [this collection-name]
    "Deletes the collection of the name given.")
  (all-collections [this]
    "Returns a seq of all existing collections.")
  (close [this]
    "Close connection with the Store.")
  (details [this]
    "Returns an implementation specific map of details about this Store useful for debugging."))

(defprotocol LigatureCollection
  "Manages a collection of Statements and Rules, supports ontologies, and querying."
  (collection-name [this])
  (compute [this f]
    "Accepts a closure that is passed a ReadTx, executes in a read-only transaction, and returns a value.")
  (write [this f]
    "Accepts a closure that is passed a WriteTx, executes in a read/write transaction, and returns a value.")
  (sparql-query [this query]
    "")
  (wander-query [this query]
    ""))

(defprotocol ReadTx
  (all-statements [this]
    "Accepts nothing but returns a seq of all Statements in the Collection.")
  (match-statements [this pattern]
    "")
  (all-rules [this]
    "")
  (match-rules [this pattern]
    ""))

(defprotocol WriteTx
  (new-identifier [this]
    "Returns a unique, new identifier in the form _:NUMBER")
  (add-statement [this statement]
    "Accepts a statement tuple")
  (remove-statement [this statement]
    "Accepts a statement tuple")
  (add-rule [this rule]
    "Accepts a rule tuple")
  (remove-rule [this rule]
    "Accepts a rule tuple"))

(defn identifier?
  "Accepts a String representing an identifier and returns true or false depending on if it is valid."
  [identifier]
  (and
    (string? identifier)
    (not (nil?(re-matches #"[a-zA-Z_][^\s\(\)\[\]\{\}\'\"`<>\\]*" identifier)))))

(defn lang-tag?
  "Accepts a String representing a lang tag and returns true or false depending on if it is valid."
  [lang]
  (and
    (string? lang)
    (not (nil?(re-matches #"[a-zA-Z]+(-[a-zA-Z0-9]+)*" lang)))))

(defn lang-literal?
  "Accepts a Map and returns true or false depending on if it is a valid lang literal.
  A lang literal should contain a :value key with a valid string literal and a :lang key with a valid lang code."
  [literal]
  (and
    (map? literal)
    (= (set (keys literal)) #{:lang :value})
    (lang-tag? (:lang literal))
    (string? (:value literal))))

(defn typed-literal?
  "Accepts a Map and returns true or false depending on if it is a valid typed literal.
  A typed literal should contain a :valud key with a valid string literal and a :type key with a valid identifier."
  [literal]
  (and
    (map? literal)
    (= (set (keys literal)) #{:type :value})
    (identifier? (:type literal))
    (string? (:value literal))))

(defn literal?
  "Accepts a String or Map representing a literal and returns true or false depending on if it is valid."
  [literal]
  (or (lang-literal? literal) (typed-literal? literal)))

(defn subject?
  "Accepts a String representing a subject and returns true or false depending of
  whether or not that String is a valid identifier."
  [subject]
  (identifier? subject))

(defn predicate?
  "Accepts a String representing a predicate and returns true or false depending on if it is valid."
  [predicate]
  (or
   (identifier? predicate)
   (= :a predicate)))

(defn object?
  "Accepts a String or Map representing an object and returns true or false depending on if it is valid."
  [object]
  (or (identifier? object) (literal? object)))

(defn graph?
  "Checks that a passed String value is either a valid identifier or nil"
  [graph]
  (or
    (nil? graph)
    (identifier? graph)))

(defn subject
  "Accepts a Statement tuple and returns the Subject."
  [statement]
  (get statement 0))

(defn predicate
  "Accepts a Statement tuple and returns the Predicate."
  [statement]
  (get statement 1))

(defn object
  "Accepts a Statement tuple and returns the Object."
  [statement]
  (get statement 2))

(defn graph
  "Accepts a Statement tuple and returns the Graph."
  [statement]
  (get statement 3))

(defn- expand-predicate
  [predicate]
  (if (= predicate :a) "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" predicate))

(defn statement
  "This function acts as a helper function for creating Statement maps.
  This function allow users to shortcut :a for http://www.w3.org/1999/02/22-rdf-syntax-ns#type in the predicate position."
  ([subject predicate object]
   {:subject subject :predicate (expand-predicate predicate) :object object})
  ([subject predicate object graph]
   {:subject subject :predicate (expand-predicate predicate) :object object :graph graph}))

(s/def ::literal literal?)

(s/def ::lang-literal lang-literal?)

(s/def ::typed-literal typed-literal?)

(s/def ::subject subject?)

(s/def ::predicate predicate?)

(s/def ::object object?)

(s/def ::graph graph?)

(s/def ::triple (s/tuple ::subject ::predicate ::object))

(s/def ::quad (s/tuple ::subject ::predicate ::object ::graph))

(s/def ::statement (s/or ::triple ::triple ::quad ::quad))

(s/def ::statements (s/coll-of ::statement))
