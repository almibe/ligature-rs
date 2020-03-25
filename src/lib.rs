/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use futures::Stream;

struct Statement(Node, Entity, Node, Entity);

struct Rule(Node, Entity, Node);

enum Node {
    Entity(Entity),
    Literal(Literal)
}

struct Entity {
    identifier: String
}

impl Entity {
    fn is_valid(identifier: &str) -> bool {
        //return "[a-zA-Z_][^\\s\\(\\)\\[\\]\\{\\}'\"`<>\\\\]*".toRegex().matches(identifier)
        unimplemented!()
    }
    fn from(identifier: &str) -> Result<Entity, String> {
        if Entity::is_valid(identifier) {
            Result::Ok(Entity { identifier: String::from(identifier) })
        } else {
            unimplemented!()
        }
    }
    fn a() -> Entity {
        Entity { identifier: String::from("_a") }
    }
    fn default() -> Entity {
        Entity { identifier: String::from("_") }
    }
    fn identifier(&self) -> &String {
        &self.identifier
    }
}

enum Literal {
    StringLiteral(String),
    LangLiteral(LangLiteral),
    BooleanLiteral(bool),
    LongLiteral(i64),
    DoubleLiteral(f64)//,
//    ListLiteral,
//    BagLiteral,
//    AltLiteral
}

enum Range {
    LangLiteralRange(LangLiteral, LangLiteral),
    StringLiteralRange(String, String),
    LongLiteralRange(u64, u64),
    DoubleLiteralRange(f64, f64)
}

struct LangLiteral {
    value: String,
    tag: String
}

impl LangLiteral {
    fn valid_tag(tag: &str) -> bool {
        //return "[a-zA-Z]+(-[a-zA-Z0-9]+)*".toRegex().matches(langTag)
        unimplemented!()
    }
    fn from(value: &str, tag: &str) -> Result<LangLiteral, String> {
        if LangLiteral::valid_tag(tag) {
            Result::Ok(LangLiteral { value: String::from(value), tag: String::from(tag) })
        } else {
            unimplemented!()
        }
    }
    fn value(&self) -> &String {
        &self.value
    }
    fn tag(&self) -> &String {
        &self.tag
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
    fn all_collections(&self) -> dyn Stream<Item = dyn LigatureCollection>;

    /**
     * Close connection with the Store.
     */
    fn close(&self);
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
     * Accepts nothing but returns a Stream of all Statements in the Collection.
     */
    fn all_statements(&self) -> dyn Stream<Item = Statement>;

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn match_statements(&self, subject: Option<Node>, predicate: Option<Entity>, object: Option<Node>, graph: Option<Entity>) -> dyn Stream<Item = Statement>;

    /**
     * Is passed a pattern and returns a seq with all matching Statements.
     */
    fn match_range(&self, subject: Option<Node>, predicate: Option<Entity>, range: Range, graph: Option<Entity>) -> dyn Stream<Item = Statement>;

    /**
     * Accepts nothing but returns a seq of all Rules in the Collection.
     */
    fn all_rules(&self) -> dyn Stream<Item = Rule>;

    /**
     * Is passed a pattern and returns a seq with all matching rules.
     */
    fn match_rules(&self, subject: Option<Node>, predicate: Option<Entity>, object: Option<Node>) -> dyn Stream<Item = Rule>;

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
