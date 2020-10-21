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

//fn validNamedNode(node: NamedNode) -> bool = "[a-zA-Z_][^\\s()\\[\\]{}'\"`<>\\\\]*".r.matches(node.name)

//fn validLangTag(langTag: String) -> bool = "[a-zA-Z]+(-[a-zA-Z0-9]+)*".r.matches(langTag)

trait Ligature {
    async fn read() -> Result<T, String>;
    async fn write() -> Result<Unit, String>;
}

trait ReadTx {
    fn collections() -> Stream<NamedNode>;
    fn collections_prefix(prefix: NamedNode) -> Stream<NamedNode>;
    fn collections_range(from: NamedNode, to: NamedNode) -> Stream<NamedNode>;
    fn all_statements(collection: NamedNode) -> Stream<PersistedStatement>;
    fn match_statements(collection: NamedNode,
        subject: Option<Node>,
        predicate: Option<NamedNode>,
        object: Option<Object>) -> Stream<PersistedStatement>;
    fn match_range(collection: NamedNode,
        subject: Option<Node>,
        predicate: Option<NamedNode>,
        range: Range) -> Stream<PersistedStatement>;
    fn statement_by_context(collection: NamedNode, context: AnonymousNode) -> Option<PersistedStatement>;
}

trait WriteTx {
    fn create_collection(collection: NamedNode) -> NamedNode;
    fn delete_collection(collection: NamedNode) -> NamedNode;
    fn new_node(collection: NamedNode) -> AnonymousNode;
    fn add_statement(collection: NamedNode, statement: Statement) -> PersistedStatement;
    fn remove_statement(collection: NamedNode, statement: Statement) -> Statement;
    fn cancel() -> Unit;
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
