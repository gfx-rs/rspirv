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

/// Rust structs for deserializing the SPIR-V JSON grammar.

use serde::de;
use serde_derive::*;
use std::{convert::TryInto, fmt, result, str};

#[derive(Debug, Deserialize)]
pub struct Operand {
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub quantifier: Quantifier,
}

#[derive(Debug, Deserialize)]
pub struct Instruction {
    pub class: Option<Class>,
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
    #[serde(deserialize_with = "num_or_hex")]
    pub value: u32,
    #[serde(default)]
    pub parameters: Vec<Operand>,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct OperandKind {
    pub category: Category,
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
    #[serde(deserialize_with = "num_or_hex")]
    pub magic_number: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub revision: u32,
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<OperandKind>,
}

#[derive(Debug, Deserialize)]
pub struct ExtInstSetGrammar {
    pub copyright: Vec<String>,
    pub version: u32,
    pub revision: u32,
    pub instructions: Vec<Instruction>,
}


fn num_or_hex<'de, D: de::Deserializer<'de>>(d: D) -> result::Result<u32, D::Error> {
    struct NumOrStr;

    impl<'de> de::Visitor<'de> for NumOrStr {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "either a number or a hex string")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> result::Result<Self::Value, E> {
            Ok(u32::from_str_radix(&value[2..], 16).unwrap())
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> result::Result<Self::Value, E> {
            Ok(value.try_into().unwrap())
        }
    }

    d.deserialize_any(NumOrStr)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Quantifier {
    #[serde(rename="")]
    One,
    #[serde(rename="?")]
    ZeroOrOne,
    #[serde(rename="*")]
    ZeroOrMore,
}

impl Default for Quantifier {
    fn default() -> Self {
        Quantifier::One
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Class {
    Annotation,
    Branch,
    Constant,
    Debug,
    DebugLine,
    ExtensionDecl,
    FunctionStruct,
    ModeSetting,
    Terminator,
    Type,
    Variable, 
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Category {
    BitEnum,
    Composite,
    Id,
    Literal,
    ValueEnum,
}