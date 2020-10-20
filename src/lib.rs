/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

enum Node {
    NamedNode(String),
    AnonymousNode(u64)
}

enum Literal {
    LangLiteral(String, String),
    StringLiteral(String),
    BooleanLiteral(bool),
    LongLiteral(i64),
    DoubleLiteral(f64)
}

enum Object {
    Node(Node),
    Literal(Literal)
}

enum Range {
    LangLiteralRange(LangLiteral, LangLiteral),
    StringLiteralRange(StringLiteral, StringLiteral),
    LongLiteralRange(LongLiteral, LongLiteral),
    DoubleLiteralRange(DoubleLiteral, DoubleLiteral)
}

struct Statement {
    subject: Node,
    predicate: NamedNode,
    object: Object
}

struct PersistedStatement {
    collection: NamedNode,
    statement: Statement,
    context: AnonymousNode
}

let a = NamedNode("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")

fn validNamedNode(node: NamedNode): Boolean = "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(node.name)

fn validLangTag(langTag: String): Boolean = "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)

trait Ligature {
  fn session: Resource[Task, LigatureSession]
}

trait LigatureSession {
  fn read: Resource[Task, ReadTx]
  fn write: Resource[Task, WriteTx]
}

trait ReadTx {
  fn collections: Observable[NamedNode]
  fn collections(prefix: NamedNode): Observable[NamedNode]
  fn collections(from: NamedNode, to: NamedNode): Observable[NamedNode]
  fn allStatements(collection: NamedNode): Observable[PersistedStatement]
  fn matchStatements(collection: NamedNode,
    subject: Option[Node] = None,
    predicate: Option[NamedNode] = None,
    `object`: Option[Object] = None): Observable[PersistedStatement]
  fn matchStatements(collection: NamedNode,
    subject: Option[Node],
    predicate: Option[NamedNode],
    range: Range): Observable[PersistedStatement]
  fn statementByContext(collection: NamedNode, context: AnonymousNode): Task[Option[PersistedStatement]]
}

trait WriteTx {
    fn createCollection(collection: NamedNode): Task[NamedNode]
    fn deleteCollection(collection: NamedNode): Task[NamedNode]
    fn newNode(collection: NamedNode): Task[AnonymousNode]
    fn addStatement(collection: NamedNode, statement: Statement): Task[PersistedStatement]
    fn removeStatement(collection: NamedNode, statement: Statement): Task[Statement]
    fn cancel(): Unit
}

#[cfg(test)]
mod tests {
test("validIdentifier tests") {
assert(!validNamedNode(NamedNode("")))
assert(validNamedNode(NamedNode("http://localhost/people/7")))
assert(!validNamedNode(NamedNode("http://localhost(/people/7")))
assert(!validNamedNode(NamedNode("http://localhost{/people/7")))
assert(!validNamedNode(NamedNode("http://localhost\\/people/7")))
assert(!validNamedNode(NamedNode("http://localhost</people/7")))
assert(!validNamedNode(NamedNode("http://localhost>/people/7")))
assert(!validNamedNode(NamedNode("http://localhost[/people/7")))
assert(!validNamedNode(NamedNode("http://localhost]/people/7")))
assert(!validNamedNode(NamedNode("http://localhost\"/people/7")))
assert(!validNamedNode(NamedNode("http://localhost'/people/7")))
assert(!validNamedNode(NamedNode("http://localhost`/people/7")))
assert(!validNamedNode(NamedNode("http://localhost\t/people/7")))
assert(!validNamedNode(NamedNode("http://localhost\n/people/7")))
assert(!validNamedNode(NamedNode("http://localhost /people/7")))
assert(validNamedNode(NamedNode("hello")))
assert(validNamedNode(NamedNode("_:")))
assert(validNamedNode(NamedNode("_:valid")))
assert(validNamedNode(NamedNode("_:1")))
assert(validNamedNode(NamedNode("_:1344")))
}

test("validLangTag tests") {
assert(!validLangTag(""))
assert(validLangTag("en"))
assert(!validLangTag("en-"))
assert(validLangTag("en-fr"))
assert(!validLangTag("en-fr-"))
assert(validLangTag("en-fr-sp"))
assert(validLangTag("ennnenefnk-dkfjkjfl-dfakjelfkjalkf-fakjeflkajlkfj"))
assert(!validLangTag("en-fr-ef "))
}
}
