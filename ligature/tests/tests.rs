// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Element, LigatureError};
use regex::Regex;

// #[test]
// fn valid_entity_names() {
//     //TODO add more tests
//     let oks = vec![
//         "t",
//         "T",
//         "test",
//         "test/test/test",
//         "test/test",
//         "this/is/a/test",
//         "_",
//         "_/_",
//         "_/_/_",
//         "_test",
//         "__test__",
//         "test/_test",
//         "test3/test",
//         "_/_/",
//         "test/",
//         "test//test",
//     ];
//     let errs = vec!["", "test test", "test/ /test", " test"];

//     for ok in oks {
//         assert!(
//             Name(ok).is_ok(),
//             "{} should be a valid Entity",
//             ok
//         );
//     }

//     for err in errs {
//         assert!(
//             Name::new(err).is_err(),
//             "{} should be an invalid Entity",
//             err
//         );
//     }
// }

// #[test]
// fn test_generate_entities() -> Result<(), LigatureError> {
//     let entity_uuid_pattern: Regex =
//         Regex::new(r".*[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
//     let entity = new_identifier(None)?;
//     let entity2 = new_identifier(Some(String::from("prefix")))?;

//     entity_uuid_pattern.is_match(entity.id());
//     entity_uuid_pattern.is_match(entity2.id());
//     Ok(())
// }

// #[test]
// fn valid_attribute_names() {
//     //TODO add more tests
//     let oks = vec![
//         "test",
//         "test_test_test",
//         "test_test",
//         "this1_is2_a_test",
//         "_",
//         "_test",
//         "__test__",
//         "testTest",
//         "G",
//         "HELLO",
//         "2",
//         "5test",
//         "test!",
//         "/_/_",
//         "test//test",
//     ];
//     let errs = vec!["", "this is a test", "test test", "test/ /test", " test"];

//     for ok in oks {
//         assert!(
//             Identifier::new(ok).is_ok(),
//             "{} should be a valid Attribute",
//             ok
//         );
//     }

//     for err in errs {
//         assert!(
//             Identifier::new(err).is_err(),
//             "{} should be an invalid Attribute",
//             err
//         );
//     }
// }
