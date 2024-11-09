// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use trips::Trips;
use trips::mem::TripsMem;

#[test]
fn store_should_start_empty() {
    let store: TripsMem<String, String> = trips::mem::TripsMem::new();
    let collections: Vec<String> = store.collections().unwrap();
    let result: Vec<String> = vec![];
    assert_eq!(collections, result);
}
