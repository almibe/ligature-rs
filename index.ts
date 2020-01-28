
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
  matchStatements(pattern): Promise<IterableIterator<Statement>>
  collectionName(): Promise<Identifier>
  addRules(rules: Rules): Promise<null>
  removeRules(rules: Rules): Promise<null>
  allRules(): Promise<IterableIterator<Rule>>
  matchRules(pattern): Promise<IterableIterator<Rule>>
  sparqlQuery(query): Promise<any>
  wanderQuery(query): Promise<any>
}

export type Identifier = string
export type Literal = LangLiteral | TypedLiteral
export class LangLiteral {
  readonly value: string
  readonly langTag: string
}
export class TypedLiteral {
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

export const a = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
export const validIdentifier = (i: Identifier): boolean => false //TODO copy logic from Clojure impl
export const validLangTag = (l: string): boolean => false //TODO copy logic from Clojure impl
export const validLangLiteral = (l: LangLiteral): boolean => validLangTag(l.langTag)
export const validTypedLiteral = (l: TypedLiteral): boolean => validIdentifier(l.type)
export const validLiteral = (l: Literal): boolean => {
  if (l instanceof LangLiteral) {
    return validLangLiteral(l)
  } else if (l instanceof TypedLiteral) {
    return validTypedLiteral(l)
  } else {
    return false
  }
}
