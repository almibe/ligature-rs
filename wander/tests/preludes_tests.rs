// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature_graph::Graph;
use wander::{preludes::common, run, ScriptValue};

#[test]
fn empty_graph() {
    let input = "graph()";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Graph(Graph::default()));
    assert_eq!(res, expected);
}
