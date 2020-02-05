/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

 export interface LigatureStore {
  collection(collectionName: Identifier): Promise<LigatureCollection>
  deleteCollection(collectionName: Identifier): Promise<null>
  allCollections(): Promise<IterableIterator<Identifier>>
  close(): Promise<null>
  details(): Promise<Map<string, string>>
}

export interface LigatureCollection {
  addStatements(statements: Statements): Promise<null>
  removeStatements(statements: Statements): Promise<null>
  allStatements(): Promise<IterableIterator<Statement>>
  newIdentifier(): Promise<Identifier>
  matchStatements(pattern: Pattern): Promise<IterableIterator<Statement>>
  collectionName(): Promise<Identifier>
  addRules(rules: Rules): Promise<null>
  removeRules(rules: Rules): Promise<null>
  allRules(): Promise<IterableIterator<Rule>>
  matchRules(pattern: Pattern): Promise<IterableIterator<Rule>>
  sparqlQuery(query: Query): Promise<any>
  wanderQuery(query: Query): Promise<any>
}

export type Identifier = string
export type Literal = PlainLiteral | TypedLiteral
export type PlainLiteral = {
  readonly value: string
  readonly langTag?: string
}
export type TypedLiteral = {
  readonly value: string
  readonly type: string
}
export type Subject = Identifier
export type Predicate = Identifier
export type Object = Identifier | Literal
export type Graph = Identifier
export type Statement = Readonly<[Subject, Predicate, Object, Graph?]>
export type Statements = ReadonlyArray<Statement>
export type Rule = Readonly<[Subject, Predicate, Object]>
export type Rules = ReadonlyArray<Rule>
export type Pattern = Readonly<[(Subject | typeof _)?, (Predicate | typeof _)?,
  (Object | typeof _)?, (Graph | typeof _)?]>
export type Query = any //TODO replace with real type

export const a = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
export const _ = "*"
const identifierPattern = /^[a-zA-Z_][^\s\(\)\[\]\{\}\'\"`<>\\]*$/
export const validIdentifier = (i: Identifier): boolean => identifierPattern.test(i)
//export const validLangTag = (l: string): boolean => false //TODO copy logic from Clojure impl
//export const validLangLiteral = (l: Literal): boolean => (l as PlainLiteral).langTag != null && 
//  (l as PlainLiteral).value != null && 
//  (l as TypedLiteral).type == null && 
//  validLangTag((l as PlainLiteral).langTag)
//export const validTypedLiteral = (l: Literal): boolean => (l as TypedLiteral).type != null &&
//  (l as TypedLiteral).value != null &&
//  (l as PlainLiteral).langTag == null &&
//  validIdentifier((l as TypedLiteral).type)
//export const validLiteral = (l: Literal): boolean => validLangLiteral(l) || validTypedLiteral(l)

/*

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

(defn predicate? [predicate]
  "Accepts a String representing a predicate and returns true or false depending on if it is valid."
  (or
    (identifier? predicate)
    (= :a predicate)))

(defn object? [object]
  "Accepts a String or Map representing an object and returns true or false depending on if it is valid."
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
*/
