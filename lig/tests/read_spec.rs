// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lig::read::{read, read_attribute, read_entity, read_value};
use lig::*;
use ligature::{Attribute, Bytes, Entity, Statement, Value};

#[test]
fn read_entities() -> Result<(), LigError> {
    let e = "<test>";
    assert_eq!(read_entity(e)?, Entity::new("test")?);
    Ok(())
}

#[test]
fn read_attributes() -> Result<(), LigError> {
    let a = "@<test>";
    assert_eq!(read_attribute(a)?, Attribute::new("test")?);
    Ok(())
}

#[test]
fn read_string_literals() -> Result<(), LigError> {
    let s = "\"test\"";
    assert_eq!(read_value(s)?, Value::StringLiteral("test".to_string()));
    Ok(())
}

#[test]
fn read_integer_literals() -> Result<(), LigError> {
    let i = "243";
    assert_eq!(read_value(i)?, Value::IntegerLiteral(243));
    Ok(())
}

#[test]
fn read_float_literals() -> Result<(), LigError> {
    let f = "1.2";
    assert_eq!(read_value(f)?, Value::FloatLiteral(1.2));
    Ok(())
}

#[test]
fn read_byte_arrays_literals() -> Result<(), LigError> {
    let b = "0x00ff";
    assert_eq!(read_value(b)?, Value::BytesLiteral(vec![0, 255]));
    Ok(())
}

#[test]
fn read_entity_as_value() -> Result<(), LigError> {
    let e = "<test>";
    assert_eq!(read_value(e)?, Value::Entity(Entity::new("test")?));
    Ok(())
}

#[test]
fn read_empty_set_of_statements() -> Result<(), LigError> {
    let s = "";
    let expected: Vec<Statement> = vec![];
    assert_eq!(read(s)?, expected);
    Ok(())
}

#[test]
fn read_set_of_statements() -> Result<(), LigError> {
    let s = "<e> @<a> 123 <c>\n<e2> @<a> <e> <c2>\n";
    let expected = vec![
        Statement {
            entity: Entity::new("e")?,
            attribute: Attribute::new("a")?,
            value: Value::IntegerLiteral(123),
            context: Entity::new("c")?,
        },
        Statement {
            entity: Entity::new("e2")?,
            attribute: Attribute::new("a")?,
            value: Value::Entity(Entity::new("e")?),
            context: Entity::new("c2")?,
        },
    ];
    assert_eq!(read(s)?, expected);
    Ok(())
}
