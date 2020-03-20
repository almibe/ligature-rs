/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

struct Statement(Node, Entity, Node, Entity);

struct Rule(Node, Entity, Node);

enum Node {
    Entity(Entity),
    Literal(Literal)
}

struct Entity {
    identifier: String
}

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

struct LangTag {
    tag: String
}

impl LangTag {
    fn is_valid(&self) -> bool {
        unimplemented!()
    }
}

impl Entity {
    fn is_valid(&self) -> bool {
        unimplemented!()
    }
}
