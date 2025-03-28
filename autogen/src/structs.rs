//! Rust structs for deserializing the SPIR-V JSON grammar.
#![allow(dead_code)] // Parsed but unread fields

use serde::de;
use serde_derive::*;
use std::{convert::TryInto, fmt, result, str};

#[derive(Debug, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Operand {
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub quantifier: Quantifier,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Instruction {
    pub class: Option<Class>,
    pub opname: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub opcode: u32,
    #[serde(default)]
    pub operands: Vec<Operand>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub extensions: Vec<String>,
    // TODO: Skip provisional instructions?
    #[serde(default)]
    pub provisional: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Enumerant {
    #[serde(rename = "enumerant")]
    pub symbol: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(deserialize_with = "num_or_hex")]
    pub value: u32,
    #[serde(default)]
    pub parameters: Vec<Operand>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub extensions: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
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
    pub major_version: u8,
    pub minor_version: u8,
    pub revision: u8,
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<OperandKind>,
}

#[derive(Debug, Deserialize)]
pub struct ExtInstSetGrammar {
    #[serde(default)]
    pub copyright: Vec<String>,
    pub version: Option<u32>,
    pub revision: u32,
    pub instructions: Vec<Instruction>,
}

fn num_or_hex<'de, D: de::Deserializer<'de>>(d: D) -> result::Result<u32, D::Error> {
    struct NumOrStr;

    impl de::Visitor<'_> for NumOrStr {
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
pub enum Quantifier {
    #[serde(rename = "")]
    #[default]
    One,
    #[serde(rename = "?")]
    ZeroOrOne,
    #[serde(rename = "*")]
    ZeroOrMore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Class {
    Annotation,
    #[serde(rename = "Control-Flow")]
    Branch,
    #[serde(rename = "Constant-Creation")]
    Constant,
    Debug,
    DebugLine,
    #[serde(rename = "Extension")]
    ExtensionDecl,
    #[serde(rename = "Function")]
    FunctionStruct,
    #[serde(rename = "Mode-Setting")]
    ModeSetting,
    #[serde(rename = "Type-Declaration")]
    Type,
    Variable,

    //////
    Miscellaneous,
    //Extension,
    Memory,
    Composite,
    Image,
    Conversion,
    Arithmetic,
    #[serde(rename = "Relational_and_Logical")]
    RelationalAndLogical,
    Bit,
    Derivative,
    Primitive,
    Barrier,
    Atomic,
    Group,
    Pipe,
    #[serde(rename = "Device-Side_Enqueue")]
    DeviceSideEnqueue,
    #[serde(rename = "Non-Uniform")]
    NonUniform,
    Reserved,
    #[serde(rename = "@exclude")]
    Exclude,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Category {
    BitEnum,
    Composite,
    Id,
    Literal,
    ValueEnum,
}
