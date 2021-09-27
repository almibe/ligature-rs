// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use ligature::{Attribute, Dataset};

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
            assert!(Dataset::new(ok).is_ok());
        }

        for err in errs {
            assert!(Dataset::new(err).is_err());
        }
    }

    #[test]
    fn valid_entity_names() {
        todo!()
    }

    #[test]
    fn test_generate_entities() {
        todo!()
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
        ];
        let errs = vec![
            "",
            "2",
            "5test",
            "test!",
            "this is a test",
            "/_/_",
            "test//test",
            "test test",
            "test/ /test",
            " test",
        ];

        for ok in oks {
            assert!(Attribute::new(ok).is_ok());
        }

        for err in errs {
            assert!(Attribute::new(err).is_err());
        }
    }
}
