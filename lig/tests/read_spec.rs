// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lig::read::read;
use lig::*;
use ligature::{Identifier, Statement, Value};

#[test]
fn read_empty_set_of_statements() -> Result<(), LigError> {
    let s = "";
    let expected: Vec<Statement> = vec![];
    assert_eq!(read(s)?, expected);
    Ok(())
}

#[test]
fn read_set_of_statements() -> Result<(), LigError> {
    let s = "<e> <a> 123\n<e2> <a> <e>\n";
    let expected = vec![
        Statement {
            entity: Identifier::new("e")?,
            attribute: Identifier::new("a")?,
            value: Value::Integer(123),
        },
        Statement {
            entity: Identifier::new("e2")?,
            attribute: Identifier::new("a")?,
            value: Value::Identifier(Identifier::new("e")?),
        },
    ];
    assert_eq!(read(s)?, expected);
    Ok(())
}

#[test]
fn read_value_list() -> Result<(), LigError> {
    let s = "<a> <b> [123 <e>]";
    let expected = vec![
        Statement {
            entity: Identifier::new("a")?,
            attribute: Identifier::new("b")?,
            value: Value::Integer(123),
        },
        Statement {
            entity: Identifier::new("a")?,
            attribute: Identifier::new("b")?,
            value: Value::Identifier(Identifier::new("e")?),
        },
    ];
    assert_eq!(read(s)?, expected);
    Ok(())
}

#[test]
fn read_entity_expansion() -> Result<(), LigError> {
    let s = "<a> { <b> 1 <c> 2}";
    let expected = vec![
        Statement {
            entity: Identifier::new("a")?,
            attribute: Identifier::new("b")?,
            value: Value::Integer(1),
        },
        Statement {
            entity: Identifier::new("a")?,
            attribute: Identifier::new("c")?,
            value: Value::Integer(2),
        },
    ];
    assert_eq!(read(s)?, expected);
    Ok(())
}
