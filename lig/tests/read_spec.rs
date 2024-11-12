// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lig::read::read;
use ligature::{LigatureError, Element, Role, Value};

#[test]
fn read_empty_set_of_statements() -> Result<(), LigatureError> {
    let s = "";
    let expected: Vec<Role> = vec![];
    assert_eq!(read(s)?, expected);
    Ok(())
}

#[test]
fn read_set_of_statements() -> Result<(), LigatureError> {
    let s = "{ e a 123,\ne2 a e\n }";
    let expected = vec![
        Role {
            first: Element("e".to_string()),
            second: Element("a".to_string()),
            role: Value::Integer(123),
        },
        Role {
            first: Element("e2".to_string()),
            second: Element("a".to_string()),
            role: Value::Name(Element("e".to_string())),
        },
    ];
    assert_eq!(read(s)?, expected);
    Ok(())
}
