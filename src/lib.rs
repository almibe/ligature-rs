/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::iter::Iterator;
use std::collections::HashMap;

struct Statement(Node, Entity, Node, Entity);

struct Rule(Node, Entity, Node);

enum Node {
    Entity(Entity),
    Literal(Literal)
}

struct Entity {
    identifier: String
}

const A: &str = "_a";
const DEFAULT: &str = "_";

enum Literal {
    StringLiteral(String),
    LangLiteral(String, LangTag),
    BooleanLiteral,
    LongLiteral,
    DoubleLiteral,
    ListLiteral,
    BagLiteral,
    AltLiteral
}

enum Range {
    //LangLiteralRange(Literal::LangLiteral, Literal::LangLiteral),
    StringLiteralRange(String, String),
    LongLiteralRange(u64, u64),
    DoubleLiteralRange(f64, f64)
}

struct LangTag {
    tag: String
}

impl LangTag {
    fn is_valid(&self) -> bool {
        //return "[a-zA-Z]+(-[a-zA-Z0-9]+)*".toRegex().matches(langTag)
        unimplemented!()
    }
}

impl Entity {
    fn is_valid(&self) -> bool {
        //return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
        unimplemented!()
    }
}

trait LigatureStore {
    /**
     * Returns a collection based on the name passed.
     * Calling this function will not create a new collection, it just binds a Store and Collection name.
     */
    fn collection(&self, collection_name: Entity) -> dyn LigatureCollection;

    /**
     * Creates a new collection or does nothing if collection already exists.
     * Regardless the collection is returned.
     */
    fn create_collection(&self, collection_name: Entity) -> dyn LigatureCollection;

    /**
     * Deletes the collection of the name given and does nothing if the collection doesn't exist.
     */
    fn delete_collection(&self, collection_name: Entity);

    /**
     * Returns a Iterator of all existing collections.
     */
    fn all_collections(&self) -> dyn Iterator<Item = dyn LigatureCollection>;

    /**
     * Close connection with the Store.
     */
    fn close(&self);

    /**
     * Returns an implementation specific map of details about this Store useful for debugging.
     */
    fn details(&self) -> HashMap<String, String>;
}

/**
 * Manages a collection of Statements and Rules, supports ontologies, and querying.
 */
trait LigatureCollection {
    fn collection_name(&self) -> Entity;
    fn read_tx(&self) -> dyn ReadTx;
    fn write_tx(&self) -> dyn WriteTx;
}

trait ReadTx {
    /**
     * Accepts nothing but returns a Iterator of all Statements in the Collection.
     */
    fn all_statements(&self) -> dyn Iterator<Item = Statement>;

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    //fn matchStatements(subject: Node? = null, predicate: Entity? = null, `object`: Node? = null, graph: Entity? = null) -> Iterator<Statement>

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    //fn matchStatements(subject: Node? = null, predicate: Entity? = null, range: Range<*>, graph: Entity? = null) -> Iterator<Statement>

    /**
     * Accepts nothing but returns a seq of all Rules in the Collection.
     */
    fn all_rules(&self) -> dyn Iterator<Item = Rule>;

    /**
     * Is passed a pattern and returns a seq with all matching rules.
     */
    //fn matchRules(subject: Node? = null, predicate: Entity? = null, `object`: Node? = null) -> Iterator<Rule>

    /**
     * Cancels this transaction.
     */
    fn cancel(&self);
}

trait WriteTx: ReadTx {
    /**
     * Returns a new, unique to this collection identifier in the form _:NUMBER"
     */
    fn new_entity(&self) -> Entity;
    fn add_statement(&self, statement: Statement);
    fn remove_statement(&self, statement: Statement);
    fn add_rule(&self, rule: Rule);
    fn remove_rule(&self, rule: Rule);
    fn commit(&self);
}
