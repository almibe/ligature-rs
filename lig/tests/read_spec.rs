// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lig::read::read;
use lig::*;
use ligature::{Identifier, Statement, Value};

// #[test]
// fn read_entities() {
//     let e = "<test>";
//     assert_eq!(
//         read_identifier(e),
//         Identifier::new("test").map_err(|_| LigError("Could not create Identifier.".into()))
//     );
// }

// #[test]
// fn read_string_literals() -> Result<(), LigError> {
//     let s = "\"test\"";
//     assert_eq!(read_value(s)?, Value::StringLiteral("test".to_string()));
//     Ok(())
// }

// #[test]
// fn read_integer_literals() -> Result<(), LigError> {
//     let i = "243";
//     assert_eq!(read_value(i)?, Value::IntegerLiteral(243));
//     Ok(())
// }

// // #[test]
// // fn read_float_literals() -> Result<(), LigError> {
// //     let f = "1.2";
// //     assert_eq!(read_value(f)?, Value::FloatLiteral(1.2));
// //     Ok(())
// // }

// #[test]
// fn read_byte_arrays_literals() -> Result<(), LigError> {
//     let b = "0x00ff";
//     assert_eq!(read_value(b)?, Value::BytesLiteral(vec![0, 255]));
//     Ok(())
// }

// #[test]
// fn read_identifier_as_value() -> Result<(), LigError> {
//     let e = "<test>";
//     assert_eq!(read_value(e)?, Value::Identifier(Identifier::new("test")?));
//     Ok(())
// }

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
            value: Value::IntegerLiteral(123),
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
