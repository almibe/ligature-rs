// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    fn write_entities() {
        let e = new Entity("test");
        writeEntity(e).should.be.equal("<test>");
    }

    fn write_attributes() {
        let a = new Attribute("test");
        writeAttribute(a).should.be.equal("@<test>");
    }

    fn write_string_literals() {
        writeValue("test").should.be.equal("\"test\"");
    }

    fn write_integer_literals() {
        writeValue(5n).should.be.equal("5");
    }

    fn write_float_literals() {
        writeValue(5.5).should.be.equal("5.5");
        writeValue(5).should.be.equal("5.0");
    }

    fn write_bytes_literals() {
        writeValue(new Uint8Array([0,255])).should.be.equal("0x00ff");
    }

    fn write_set_of_statements() {
        let statements = [
            new Statement(new Entity("e"), new Attribute("a"), 234n, new Entity("c")),
            new Statement(new Entity("e"), new Attribute("a2"), "test", new Entity("c2"))
        ]
        let expected = "<e> @<a> 234 <c>\n<e> @<a2> \"test\" <c2>\n";
        write(statements).should.be.equal(expected)
    }
}
