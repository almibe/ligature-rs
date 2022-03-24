// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lig::write::{write, write_identifier, write_value};
use ligature::{Identifier, LigatureError, Statement, Value};

#[test]
fn write_entities() -> Result<(), LigatureError> {
    let e = Identifier::new("test")?;
    assert_eq!(write_identifier(&e), "<test>".to_string());
    Ok(())
}

#[test]
fn write_string_literals() {
    assert_eq!(
        write_value(&Value::StringLiteral("test".to_string())),
        "\"test\""
    );
}

#[test]
fn write_integer_literals() {
    assert_eq!(write_value(&Value::IntegerLiteral(5)), "5");
}

// #[test]
// fn write_float_literals() {
//     assert_eq!(write_value(&Value::FloatLiteral(5.5)), "5.5");
//     assert_eq!(write_value(&Value::FloatLiteral(5f64)), "5.0");
// }

#[test]
fn write_bytes_literals() {
    assert_eq!(write_value(&Value::BytesLiteral(vec![0, 255])), "0x00ff");
}

#[test]
fn write_set_of_statements() -> Result<(), LigatureError> {
    let statements = vec![
        Statement {
            entity: Identifier::new("e")?,
            attribute: Identifier::new("a")?,
            value: Value::IntegerLiteral(234),
        },
        Statement {
            entity: Identifier::new("e")?,
            attribute: Identifier::new("a2")?,
            value: Value::StringLiteral("test".to_string()),
        },
    ];
    let expected = "<e> <a> 234\n<e> <a2> \"test\"\n";
    assert_eq!(write(statements.iter()), expected);
    Ok(())
}
