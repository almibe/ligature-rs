// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use ligature::{Entity, Attribute, LigatureError, Value, Statement};
    use lig::{write_entity, write_attribute, write_value, write};

    #[test]
    fn write_entities() -> Result<(), LigatureError> {
        let e = Entity::new("test")?;
        assert_eq!(write_entity(&e), "<test>".to_string());
        Ok(())
    }

    #[test]
    fn write_attributes() -> Result<(), LigatureError> {
        let a = Attribute::new("test")?;
        assert_eq!(write_attribute(&a), "@<test>");
        Ok(())
    }

    #[test]
    fn write_string_literals() {
        assert_eq!(write_value(&Value::StringLiteral("test".to_string())), "\"test\"");
    }

    #[test]
    fn write_integer_literals() {
        assert_eq!(write_value(&Value::IntegerLiteral(5)), "5");
    }

    #[test]
    fn write_float_literals() {
        assert_eq!(write_value(&Value::FloatLiteral(5.5)), "5.5");
        assert_eq!(write_value(&Value::FloatLiteral(5f64)), "5.0");
    }

    #[test]
    fn write_bytes_literals() {
        todo!()
//        assert_eq!(write_value(&Value::BytesLiteral([0,255])), "0x00ff");
    }

    #[test]
    fn write_set_of_statements() -> Result<(), LigatureError> {
        let statements = vec![
            Statement { entity: Entity::new("e")?, attribute: Attribute::new("a")?, value: Value::IntegerLiteral(234), context: Entity::new("c")? },
            Statement { entity: Entity::new("e")?, attribute: Attribute::new("a2")?, value: Value::StringLiteral("test".to_string()), context: Entity::new("c2")? }
        ];
        let expected = "<e> @<a> 234 <c>\n<e> @<a2> \"test\" <c2>\n";
        assert_eq!(write(statements.iter()), expected);
        Ok(())
    }
}
