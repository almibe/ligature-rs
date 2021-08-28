// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    fn read_entities() {
        let e = "<test>";
        readEntity(e).should.be.eql(new Entity("test"));
    }

    fn read_attributes() {
        let a = "@<test>";
        readAttribute(a).should.be.eql(new Attribute("test"));
    }

    fn read_string_literals() {
        let s = "\"test\"";
        readValue(s).should.be.equal("test");
    }

    fn read_integer_literals() {
        let i = "243";
        readValue(i).should.be.equal(243n);
    }

    fn read_float_literals() {
        let f = "1.2";
        readValue(f).should.be.equal(1.2);
    }

    fn read_byte_arrays_literals() {
        let b = "0x00ff";
        readValue(b).should.be.eql(new Uint8Array([0, 255]));
    }

    fn read_entity_as_value() {
        let e = "<test>";
        readValue(e).should.be.eql(new Entity("test"));
    }

    fn read_empty_set_of_statements() {
        let s = "";
        let expected: Array<Statement> = [];
        read(s).should.be.eql(expected);
    }

    fn read_set_of_statements() {
        let s = "<e> @<a> 123 <c>\n<e2> @<a> <e> <c2>\n";
        let expected = [
            new Statement(new Entity("e"), new Attribute("a"), 123n, new Entity("c")),
            new Statement(new Entity("e2"), new Attribute("a"), new Entity("e"), new Entity("c2"))
        ];
        read(s).should.be.eql(expected);
    }
}
