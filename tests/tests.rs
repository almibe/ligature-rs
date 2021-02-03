// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use ligature::Dataset;

    #[test]
    fn valid_dataset_names() {
        //TODO add more tests
        let oks = vec![
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
}
