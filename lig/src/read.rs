// // This Source Code Form is subject to the terms of the Mozilla Public
// // License, v. 2.0. If a copy of the MPL was not distributed with this
// // file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::LigError;
use ligature::{validate_identifier_characters, Attribute, Bytes, Entity, Statement, Value};
use gaze::Gaze;
use gaze::steps::{NoMatch, take_string, take_while_str};

#[derive(Debug, Clone, Copy)]
pub enum LigToken {
    OpenAngle,
    CloseAngle,
    AtSign,
    Whitespace,
    NewLine,
    DoubleQuote,
    Dot,
    Identifier,
    Integer,
}

/// Reads an Entity from the given &str.
/// Will return an error if there is anything other than an Entity + whitespace in the input.
pub fn read_entity(input: &str) -> Result<Entity, LigError> {
    let mut gaze = Gaze::<&str>::from_str(input.trim());
    gaze.attempt(&entity_step).map_err(|_| LigError("Could not read Entity.".into()))
}

pub fn read_attribute(input: &str) -> Result<Attribute, LigError> {
    let mut gaze = Gaze::<&str>::from_str(input.trim());
    gaze.attempt(&attribute_step).map_err(|_| LigError("Could not read Attribute.".into()))
}

/// Reads a value from a passed str.
/// Ignores white space but will return an Err if there is any input besides an encoded value.
pub fn read_value(input: &str) -> Result<Value, LigError> {
    let mut gaze = Gaze::<&str>::from_str(input.trim());
    gaze.attempt(&value_step).map_err(|_| LigError("Could not read Value.".into()))
}

pub fn read(input: &str) -> Result<Vec<Statement>, LigError> {
    todo!()
}

fn entity_step(gaze: &mut Gaze<&str>) -> Result<Entity, NoMatch> {
    gaze.attempt(&take_string("<"))?;//.map_err(|_| LigError("Could not read Entity.".into()))?;
    let res = gaze.attempt(&take_while_str(&|c: &str| validate_identifier_characters(c)))?;
    gaze.attempt(&take_string(">"))?;//.map_err(|_| LigError("Could not read Entity.".into()))?;
    Ok(Entity::new(&res).map_err(|_| NoMatch)?)
}

fn attribute_step(gaze: &mut Gaze<&str>) -> Result<Attribute, NoMatch> {
    gaze.attempt(&take_string("@<"))?;//.map_err(|_| LigError("Could not read Attribute.".into()))?;
    let res = gaze.attempt(&take_while_str(&|c: &str| validate_identifier_characters(c)))?;
    gaze.attempt(&take_string(">"))?;//.map_err(|_| LigError("Could not read Attribute.".into()))?;
    Ok(Attribute::new(&res).map_err(|_| NoMatch)?)
}

fn value_step(gaze: &mut Gaze<&str>) -> Result<Value, NoMatch> {
    // let next_char = gaze.peek();
    // return match next_char {
    //     None => Err(LigError("Could not match Value".into())),
    //     Some(c) => {
    //         if c == "<" {
    //             Ok(Value::Entity(gaze.run(&EntityTokenizer())?))
    //         } else if is_digit(&c) {
    //             gaze.run(&NumberTokenizer())
    //         } else if c == "\"" {
    //             gaze.run(&StringTokenizer())
    //         } else {
    //             Err(LigError("Could not match Value".into()))
    //         }
    //     }
    // };
    todo!()
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

// pub struct NumberTokenizer();
// impl Tokenizer<Value, LigError> for NumberTokenizer {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
//         let is_hex = gaze.run(&TakeString::new("0x"));
//         match is_hex {
//             Ok(_) => {
//                 return gaze.run(&HexTokenizer());
//             },
//             Err(_) => {
//                 let integer = gaze.run(&IntegerTokenizer())?;
//                 let is_float = gaze.run(&TakeString::new("."));
//                 match is_float {
//                     Ok(_) => todo!(),
//                     Err(_) => todo!(),
//                 }
//                 //if not read integer
//                 //then check for decimal point
//                 //handle float if decimal point exists
//             },
//         }
//         todo!()
//     }
// }

// pub struct HexTokenizer();
// impl Tokenizer<Value, LigError> for HexTokenizer {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
//         todo!()
//     }
// }

// pub struct IntegerTokenizer();
// impl Tokenizer<Value, LigError> for IntegerTokenizer {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
//         todo!()
//     }
// }

// pub struct BytesTokenizer();
// impl Tokenizer<Value, LigError> for BytesTokenizer {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
//         todo!()
//     }
// }

// pub struct StringTokenizer();
// impl Tokenizer<Value, LigError> for StringTokenizer {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<Value, LigError> {
//         todo!()
//     }
// }

// pub struct StatementTokenizer();
// impl Tokenizer<Statement, LigError> for StatementTokenizer {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<Statement, LigError> {
//         todo!()
//     }
// }
