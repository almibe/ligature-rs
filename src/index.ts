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
  readonly lang?: string
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
const langTagPattern = /^[a-zA-Z]+(-[a-zA-Z0-9]+)*$/
export const validLangTag = (l: string | undefined): boolean => l == undefined ? true : langTagPattern.test(l)
export const validPlainLiteral = (l: Literal): boolean => (l as PlainLiteral).value != null && 
  (l as TypedLiteral).type == null && 
  validLangTag((l as PlainLiteral).lang)
export const validTypedLiteral = (l: Literal): boolean => (l as TypedLiteral).type != null &&
  (l as TypedLiteral).value != null &&
  (l as PlainLiteral).lang == null &&
  validIdentifier((l as TypedLiteral).type)
export const validLiteral = (l: Literal): boolean => validPlainLiteral(l) || validTypedLiteral(l)
