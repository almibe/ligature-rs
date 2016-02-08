/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot;

import java.util.Arrays;
import java.util.stream.Stream;

/**
 * A simple util class to make it easy to wrap a string in a stream.  Only here because Kotlin doesn't support
 * default methods yet.
 */
public class StringUtil {
    public static Stream<String> createStringStream(String text) {
        return Arrays.stream(text.split("\\r?\\n"));
    }
}
