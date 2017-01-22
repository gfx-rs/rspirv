// Copyright 2017 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::de;
use std::{result, str};

#[derive(Debug, Deserialize)]
pub struct Operand {
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub quantifier: String,
}

#[derive(Debug, Deserialize)]
pub struct Instruction {
    pub opname: String,
    pub opcode: u32,
    #[serde(default)]
    pub operands: Vec<Operand>,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Enumerant {
    #[serde(rename = "enumerant")]
    pub symbol: String,
    #[serde(deserialize_with = "num_or_str")]
    pub value: EnumValue,
    #[serde(default)]
    pub parameters: Vec<Operand>,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct OperandKind {
    pub category: String,
    pub kind: String,
    #[serde(default)]
    pub doc: String,
    #[serde(default)]
    pub enumerants: Vec<Enumerant>,
    #[serde(default)]
    pub bases: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Grammar {
    pub copyright: Vec<String>,
    pub magic_number: String,
    pub major_version: u32,
    pub minor_version: u32,
    pub revision: u32,
    pub instructions:  Vec<Instruction>,
    pub operand_kinds: Vec<OperandKind>,
}

#[derive(Debug, Deserialize)]
pub struct GlslGrammar {
    pub copyright: Vec<String>,
    pub version: u32,
    pub revision: u32,
    pub instructions:  Vec<Instruction>,
}

/// The struct that represents either a number or a string.
///
/// It is defined as a struct instead of enum to ease usage, although
/// essentially it is an enum.
#[derive(Debug, Deserialize)]
pub struct EnumValue {
    pub number: u32,
    pub string: String,
}

/// Deserializes a field that can either be a number or a string into a EnumValue.
fn num_or_str<D: de::Deserializer>(d: &mut D) -> result::Result<EnumValue, D::Error> {
    struct NumOrStr;

    impl de::Visitor for NumOrStr {
        type Value = EnumValue;

        fn visit_str<E: de::Error>(&mut self, value: &str) -> result::Result<EnumValue, E> {
            Ok(EnumValue {
                number: 0,
                string: value.to_string(),
            })
        }

        fn visit_u64<E: de::Error>(&mut self, value: u64) -> result::Result<EnumValue, E> {
            Ok(EnumValue {
                number: value as u32,
                string: String::new(),
            })
        }
    }

    d.deserialize(NumOrStr)
}
