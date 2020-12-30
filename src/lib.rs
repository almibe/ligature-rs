/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use async_trait::async_trait;
use futures_core::stream::Stream;

pub struct DatasetName(String);
pub struct BlankNode(u64);
pub struct IRI(String);
pub struct DefaultGraph {}
pub struct LangTag(String);
pub struct LangLiteral { pub value: String, pub lang_tag: LangTag }
pub struct UnknownLiteral { pub value: String, pub r#type: IRI }

pub enum Literal {
  LangLiteral(LangLiteral),
  StringLiteral(String),
  BooleanLiteral(bool),
  LongLiteral(i64),
  DoubleLiteral(f64),
  UnknownLiteral(UnknownLiteral)
}

pub enum Range {
  LangLiteralRange { start: LangLiteral, stop: LangLiteral },
  StringLiteralRange { start: String, stop: String },
  LongLiteralRange { start: i64, stop: i64 },
  DoubleLiteralRange { start: f64, stop: f64 }  
}

pub enum Subject {
  IRI(IRI),
  BlankNode(BlankNode),
  DefaultGraph(DefaultGraph)
}

pub enum Predicate {
  IRI(IRI)
}

pub enum Object {
  Subject(Subject),
  Literal(Literal)
}

pub enum Graph {
  IRI(IRI),
  BlankNode(BlankNode),
  DefaultGraph(DefaultGraph)
}

pub struct Statement { pub subject: Subject, pub predicate: Predicate, pub object: Object }
pub struct PersistedStatement { pub dataset: DatasetName, pub statement: Statement, pub graph: Graph }

//val a: IRI = IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").getOrElse(???)
//fn validLangTag(langTag: String) -> Boolean =
//  "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)

pub struct LigatureError(String);

#[async_trait]
pub trait Ligature {
  fn all_datasets(self) -> dyn Stream<Item = DatasetName>;
  fn match_datasets(self, prefix: String) -> dyn Stream<Item = DatasetName>;
  fn match_datasets_range(self, from: String, to: String) -> dyn Stream<Item = DatasetName>;
  async fn create_dataset(self, dataset: DatasetName) -> Result<DatasetName, LigatureError>;
  async fn delete_dataset(self, dataset: DatasetName) -> Result<DatasetName, LigatureError>;
  async fn query(self, dataset: DatasetName) -> Result<Box<dyn QueryTx>, LigatureError>;
  async fn write(self, dataset: DatasetName) -> Result<Box<dyn WriteTx>, LigatureError>;
}

pub trait QueryTx {
  fn all_statements(self) -> dyn Stream<Item = PersistedStatement>;
  fn match_statements(self,
                      subject: Option<Subject>,
                      predicate: Option<Predicate>,
                      object: Option<Object>,
                      graph: Option<Graph>) -> dyn Stream<Item = PersistedStatement>;
  fn match_statements_range(self,
                      subject: Option<Subject>,
                      predicate: Option<Predicate>,
                      graph: Option<Graph>,
                      range: Range) -> dyn Stream<Item = PersistedStatement>;
}

#[async_trait]
pub trait WriteTx {
  async fn new_blank_node(self) -> Result<BlankNode, LigatureError>;
  async fn add_statement(self, statement: Statement, graph: Graph) -> Result<PersistedStatement, LigatureError>;
  async fn remove_statement(self, statement: Statement, graph: Graph) -> Result<Statement, LigatureError>;
  async fn cancel(self) -> Result<(), LigatureError>;
}
