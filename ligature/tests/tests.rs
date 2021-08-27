// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use ligature::{new_entity, Attribute, Dataset, Entity, LigatureError};
    use regex::Regex;

    #[test]
    fn valid_dataset_names() {
        //TODO add more tests
        let oks = vec![
            "t",
            "T",
            "test",
            "test/test/test",
            "test/test",
            "this/is/a/test",
            "_",
            "_/_",
            "_/_/_",
            "_test",
            "__test__",
            "test/_test",
            "test3/test",
        ];
        let errs = vec![
            "",
            "/",
            "test/",
            "/test",
            "_/_/",
            "/_/_",
            "test//test",
            "test test",
            "test/ /test",
            " test",
        ];

        for ok in oks {
            assert!(Dataset::new(ok).is_ok(), "{} should be a valid Dataset", ok);
        }

        for err in errs {
            assert!(
                Dataset::new(err).is_err(),
                "{} should be an invalid Dataset",
                err
            );
        }
    }

    #[test]
    fn valid_entity_names() {
        //TODO add more tests
        let oks = vec![
            "t",
            "T",
            "test",
            "test/test/test",
            "test/test",
            "this/is/a/test",
            "_",
            "_/_",
            "_/_/_",
            "_test",
            "__test__",
            "test/_test",
            "test3/test",
            "_/_/",
            "test/",
            "test//test",
        ];
        let errs = vec![
            "",
            "/",
            "/test",
            "/_/_",
            "test test",
            "test/ /test",
            " test",
        ];

        for ok in oks {
            assert!(Entity::new(ok).is_ok(), "{} should be a valid Entity", ok);
        }

        for err in errs {
            assert!(
                Entity::new(err).is_err(),
                "{} should be an invalid Entity",
                err
            );
        }
    }

    #[test]
    fn test_generate_entities() -> Result<(), LigatureError> {
        let entity_uuid_pattern: Regex =
            Regex::new(r".*[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
        let entity = new_entity(None)?;
        let entity2 = new_entity(Some(String::from("prefix")))?;

        entity_uuid_pattern.is_match(entity.id());
        entity_uuid_pattern.is_match(entity2.id());
        Ok(())
    }

    #[test]
    fn valid_attribute_names() {
        //TODO add more tests
        let oks = vec![
            "test",
            "test_test_test",
            "test_test",
            "this1_is2_a_test",
            "_",
            "_test",
            "__test__",
            "testTest",
            "G",
            "HELLO",
            "test!",
            "test//test",
        ];
        let errs = vec![
            "",
            "2",
            "5test",
            "this is a test",
            "/_/_",
            "test test",
            "test/ /test",
            " test",
        ];

        for ok in oks {
            assert!(
                Attribute::new(ok).is_ok(),
                "{} should be a valid Attribute",
                ok
            );
        }

        for err in errs {
            assert!(
                Attribute::new(err).is_err(),
                "{} should be an invalid Attribute",
                err
            );
        }
    }
}
