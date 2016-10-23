extern crate regex;
extern crate serde_json;

use regex::Regex;
use serde_json::Value;
use std::{fs, path};
use std::io::{Read, Write};

#[cfg_attr(rustfmt, rustfmt_skip)]
static COPYRIGHT : &'static str = "\
// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the \"License\");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an \"AS IS\" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.";

#[cfg_attr(rustfmt, rustfmt_skip)]
static AUTOGEN_COMMENT : &'static str = "\
// This rust module is automatically generated from SPIR-V C++ header file:
//   https://raw.githubusercontent.com/KhronosGroup/
//           SPIRV-Headers/master/include/spirv/1.1/spirv.hpp";

#[cfg_attr(rustfmt, rustfmt_skip)]
static VAULE_ENUM_ATTRIBUTE: &'static str = "\
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, NumFromPrimitive)]";


/// Converts the given name to comply with the constant naming style in Rust.
///
/// For example, aCamelCaseName is changed to A_CAMEL_CASE_NAME.
fn constantify_name(name: &str) -> String {
    // Two named capture groups: the lowercase letter and the uppercase letter.
    let re = Regex::new(r"(?P<l>[:lower:])(?P<u>[:upper:])").unwrap();
    re.replace_all(name, "$l-$u").replace("-", "_").to_uppercase()
}

fn gen_bit_enum_operand_kind(value: &Value) -> String {
    let object = value.as_object().unwrap();
    let kind = object.get("kind").unwrap().as_str().unwrap();
    let enumerants = object.get("enumerants").unwrap().as_array().unwrap();
    let elements: Vec<String> = enumerants.iter().map(|ref element| {
        let enumerant = element.as_object().unwrap();
        let symbol = enumerant.get("enumerant").unwrap().as_str().unwrap();
        let value = enumerant.get("value").unwrap().as_str().unwrap();
        format!("        const {}_{} = {},",
                constantify_name(kind),
                constantify_name(symbol),
                value)
    }).collect();
    format!("bitflags!{{\n    pub flags {kind} : u32 \
             {{\n{enumerants}\n    }}\n}}\n",
            kind = kind,
            enumerants = elements.join("\n"))
}

fn gen_value_enum_operand_kind(value: &Value) -> String {
    let object = value.as_object().unwrap();
    let kind = object.get("kind").unwrap().as_str().unwrap();
    let enumerants = object.get("enumerants").unwrap().as_array().unwrap();
    let elements: Vec<String> = enumerants.iter().map(|ref element| {
        let enumerant = element.as_object().unwrap();
        let symbol = enumerant.get("enumerant").unwrap().as_str().unwrap();
        let value = enumerant.get("value").unwrap().as_u64().unwrap();
        // Special case for Dim. Its enumerants can start with a digit.
        // So prefix with the kind name here.
        if kind == "Dim" {
            format!("    {}{} = {},", kind, symbol, value)
        } else {
            format!("    {} = {},", symbol, value)
        }
    }).collect();
    format!("{attribute}\npub enum {kind} {{\n{enumerants}\n}}\n",
            attribute = VAULE_ENUM_ATTRIBUTE,
            kind = kind,
            enumerants = elements.join("\n"))
}

/// Returns the code defining the enum for an operand kind by parsing
/// the given JSON object.
///
/// The JSON object is expected to be an element in the "operand_kind"
/// array of the SPIR-V grammar.
fn gen_operand_kind(value: &Value) -> String {
    let object = value.as_object().unwrap();
    let category = object.get("category").unwrap().as_str().unwrap();
    if category == "BitEnum" {
        gen_bit_enum_operand_kind(value)
    } else if category == "ValueEnum" {
        gen_value_enum_operand_kind(value)
    } else {
        String::new()
    }
}

fn main() {
    // Path to the grammar file.
    let mut path = path::PathBuf::from(file!());
    path.pop();
    path.push("external");
    path.push("spirv.core.grammar.json");

    let mut contents = String::new();
    {
        let filename = path.to_str().unwrap();
        let mut file = fs::File::open(filename).unwrap();
        file.read_to_string(&mut contents).unwrap();
    }
    let grammar: Value = serde_json::from_str(&contents).unwrap();
    let root = grammar.as_object().unwrap();

    {
        // Path to the generated SPIR-V header file.
        path.pop();
        path.pop();
        path.push("spirv.rs");
        let filename = path.to_str().unwrap();
        let mut file = fs::File::create(filename).unwrap();

        { // Copyright, documentation.
            file.write_all(COPYRIGHT.as_bytes()).unwrap();
            file.write_all(b"\n\n").unwrap();
            file.write_all(AUTOGEN_COMMENT.as_bytes()).unwrap();
            file.write_all(b"\n\n").unwrap();
            file.write_all(b"//! The SPIR-V header.").unwrap();
            file.write_all(b"\n\n").unwrap();
            file.write_all(b"#![allow(non_camel_case_types)]").unwrap();
            file.write_all(b"\n\n").unwrap();
        }
        { // constants.
            file.write_all(b"pub type Word = u32;\n").unwrap();
            let magic_number =
                root.get("magic_number").unwrap().as_str().unwrap();
            let major_version =
                root.get("major_version").unwrap().as_u64().unwrap();
            let minor_version =
                root.get("minor_version").unwrap().as_u64().unwrap();
            let revision = root.get("revision").unwrap().as_u64().unwrap();
            let constants = format!("pub const MAGIC_NUMBER: u32 = {};\n\
                                     pub const MAJOR_VERSION: u32 = {};\n\
                                     pub const MINOR_VERSION: u32 = {};\n\
                                     pub const REVISION: u32 = {};\n",
                                    magic_number,
                                    major_version,
                                    minor_version,
                                    revision);
            file.write_all(&constants.into_bytes())
                .unwrap();
            file.write_all(b"\n").unwrap();

        }
        { // Operand kinds.
            let operand_kinds =
                root.get("operand_kinds").unwrap().as_array().unwrap();
            for kind in operand_kinds.iter() {
                let operand_kind = gen_operand_kind(kind);
                if !operand_kind.is_empty() {
                file.write_all(&operand_kind.into_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
                }
            }
        }
        { // Opcodes.
            // Get the instruction table.
            let insts = root.get("instructions").unwrap().as_array().unwrap();
            let opcodes: Vec<String> = insts.iter()
                .map(|ref inst| {
                let instruction = inst.as_object().unwrap();
                let opname =
                    instruction.get("opname").unwrap().as_str().unwrap();
                let opcode = instruction.get("opcode").unwrap();
                // Omit the "Op" prefix.
                format!("    {} = {},", &opname[2..], opcode)
            }).collect();
            let opcode_enum = format!("{attribute}\npub enum Op \
                                       {{\n{opcodes}\n}}\n",
                                      attribute = VAULE_ENUM_ATTRIBUTE,
                                      opcodes = opcodes.join("\n"));
            file.write_all(&opcode_enum.into_bytes()).unwrap();
        }
    }
}
