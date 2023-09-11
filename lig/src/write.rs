// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::Statement;
use wander::write_statement;

pub fn write(statements: std::slice::Iter<Statement>) -> String {
    let mut result = String::new();
    for statement in statements {
        result += &*write_statement(statement);
    }
    result
}
