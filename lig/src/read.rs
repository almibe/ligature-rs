// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::LigError;
use gaze::steps::{TakeString, TakeWhile};
use gaze::{Gaze, Step};
use ligature::{validate_identifier_characters, Attribute, Bytes, Entity, Statement, Value};

/// Reads an Entity from the given &str.
/// Will return an error if there is anything other than an Entity + whitespace in the input.
pub fn read_entity(input: &str) -> Result<Entity, LigError> {
    let mut gaze = Gaze::new(input.trim());
    let step = EntityStep();
    gaze.run(&step)
}

pub fn read_attribute(input: &str) -> Result<Attribute, LigError> {
    let mut gaze = Gaze::new(input.trim());
    let step = AttributeStep();
    gaze.run(&step)
}

/// Reads a value from a passed str.
/// Ignores white space but will return an Err if there is any input besides an encoded value.
pub fn read_value(input: &str) -> Result<Value, LigError> {
    let mut gaze = Gaze::new(input.trim());
    let step = ValueStep();
    gaze.run(&step)
}

pub fn read(input: &str) -> Result<Vec<Statement>, LigError> {
    todo!()
}

pub struct EntityStep();
impl Step<Entity, LigError> for EntityStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Entity, LigError> {
        gaze.run(&TakeString::new("<"))
            .map_err(|_| LigError("Could not read Entity.".into()))?;
        let res = gaze.run(&TakeWhile(&|c: &str| validate_identifier_characters(c)));
        gaze.run(&TakeString::new(">"))
            .map_err(|_| LigError("Could not read Entity.".into()))?;
        match res {
            Ok(st) => Ok(Entity::new(&st)?),
            Err(_) => Err(LigError("Couldn't parse Entity.".into())),
        }
    }
}

pub struct AttributeStep();
impl Step<Attribute, LigError> for AttributeStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Attribute, LigError> {
        gaze.run(&TakeString::new("@<"))
            .map_err(|_| LigError("Could not read Attribute.".into()))?;
        let res = gaze.run(&TakeWhile(&|c: &str| validate_identifier_characters(c)));
        gaze.run(&TakeString::new(">"))
            .map_err(|_| LigError("Could not read Attribute.".into()))?;
        match res {
            Ok(st) => Ok(Attribute::new(&st)?),
            Err(_) => Err(LigError("Couldn't parse Attribute.".into())),
        }
    }
}

pub struct ValueStep();
impl Step<Value, LigError> for ValueStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
        let next_char = gaze.peek();
        return match next_char {
            None => Err(LigError("Could not match Value".into())),
            Some(c) => {
                if c == "<" {
                    Ok(Value::Entity(gaze.run(&EntityStep())?))
                } else if is_digit(&c) {
                    gaze.run(&NumberStep())
                } else if c == "\"" {
                    gaze.run(&StringStep())
                } else {
                    Err(LigError("Could not match Value".into()))
                }
            }
        };
    }
}

fn is_digit(s: &str) -> bool {
    s == "0"
        || s == "1"
        || s == "2"
        || s == "3"
        || s == "4"
        || s == "5"
        || s == "6"
        || s == "7"
        || s == "8"
        || s == "9"
}

pub struct NumberStep();
impl Step<Value, LigError> for NumberStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
        let is_hex = gaze.run(&TakeString::new("0x"));
        match is_hex {
            Ok(_) => {
                return gaze.run(&HexStep());
            },
            Err(_) => {
                let integer = gaze.run(&IntegerStep())?;
                let is_float = gaze.run(&TakeString::new("."));
                match is_float {
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                }
                //if not read integer
                //then check for decimal point
                //handle float if decimal point exists
            },
        }
        todo!()
    }
}

pub struct HexStep();
impl Step<Value, LigError> for HexStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
        todo!()
    }
}

pub struct IntegerStep();
impl Step<Value, LigError> for IntegerStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
        todo!()
    }
}

pub struct BytesStep();
impl Step<Value, LigError> for BytesStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
        todo!()
    }
}

pub struct StringStep();
impl Step<Value, LigError> for StringStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
        todo!()
    }
}

pub struct StatementStep();
impl Step<Statement, LigError> for StatementStep {
    fn attempt(&self, gaze: &mut Gaze) -> Result<Statement, LigError> {
        todo!()
    }
}
