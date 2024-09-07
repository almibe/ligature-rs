// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lig::write::write;
use ligature::{Name, LigatureError, Statement, Value};

#[test]
fn write_set_of_statements() -> Result<(), LigatureError> {
    let statements = vec![
        Statement {
            entity: Name("e".to_string()),
            attribute: Name("a".to_string()),
            value: Value::Integer(234),
        },
        Statement {
            entity: Name("e".to_string()),
            attribute: Name("a2".to_string()),
            value: Value::String("test".to_string()),
        },
    ];
    let expected = "<e> <a> 234\n<e> <a2> \"test\"\n";
    //assert_eq!(write(statements.iter()), expected.to_string());
    Ok(())
}
