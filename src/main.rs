/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

 use async_trait::async_trait;

struct BlankNode(u64);
struct IRI(String);
struct DefaultGraph {}
struct LangLiteral { value: String, langTag: String }

enum Literal {
  LangLiteral { value: LangLiteral },
  StringLiteral { value: String },
  BooleanLiteral { value: bool },
  LongLiteral { value: i64 },
  DoubleLiteral { value: f64 },
  UnknownLiteral { value: String, r#type: IRI }  
}

enum Range {
  LangLiteralRange { start: LangLiteral, stop: LangLiteral },
  StringLiteralRange { start: String, stop: String },
  LongLiteralRange { start: i64, stop: i64 },
  DoubleLiteralRange { start: f64, stop: f64 }  
}

enum Subject {
  IRI,
  BlankNode,
  DefaultGraph
}

enum Predicate {
  IRI, 
}

enum Graph { 
  IRI,
  BlankNode,
  DefaultGraph
}

enum Object {
  Subject,
  Literal
}

struct Statement { subject: Subject, predicate: Predicate, object: Object }
struct PersistedStatement { dataset: LocalName, statement: Statement, graph: Graph }

//val a: IRI = IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").getOrElse(???)
//fn validLangTag(langTag: String) -> Boolean =
//  "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)

#[async_trait]
trait Ligature {
  async fn all_datasets() -> Observable<LocalName>;
  async fn match_datasets(prefix: String) -> Observable<LocalName>;
  async fn match_datasets_range(from: String, to: String) -> Observable<LocalName>;
  async fn createDataset(dataset: LocalName) -> LocalName;
  async fn deleteDataset(dataset: LocalName) -> LocalName;
  async fn query() -> QueryTx;
  async fn addStatements(dataset: LocalName, statements: Iterator<Statement>);
  async fn removeStatements(dataset: LocalName, statements: Iterator<Statement>);
}

#[async_trait]
trait QueryTx {
  async fn all_statements(dataset: LocalName) -> Observable<PersistedStatement>;
  async fn match_statements(dataset: LocalName,
                      subject: Option<Subject>,
                      predicate: Option<Predicate>,
                      object: Option<Object>,
                      graph: Option<Graph>) -> Observable<PersistedStatement>;
  async fn match_statements_range(dataset: LocalName,
                      subject: Option<Subject>,
                      predicate: Option<Predicate>,
                      graph: Option<Graph>,
                      range: Range) -> Observable<PersistedStatement>;
}

#[async_trait]
trait WriteTx {
  async fn newNode(dataset: LocalName) -> Task<BlankNode>;
  async fn addStatement(dataset: LocalName, statement: Statement, graph: Graph) -> Task<PersistedStatement>;
  async fn removeStatement(dataset: LocalName, statement: Statement, graph: Graph) -> Task<Statement>;
  async fn cancel() -> Unit;
}
