// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lig::write::write;
use ligature::{Element, LigatureError, Role, Value};

#[test]
fn write_set_of_statements() -> Result<(), LigatureError> {
    let statements = vec![
        Role {
            first: Element("e".to_string()),
            second: Element("a".to_string()),
            role: Value::Integer(234),
        },
        Role {
            first: Element("e".to_string()),
            second: Element("a2".to_string()),
            role: Value::String("test".to_string()),
        },
    ];
    let expected = "<e> <a> 234\n<e> <a2> \"test\"\n";
    //assert_eq!(write(statements.iter()), expected.to_string());
    Ok(())
}
