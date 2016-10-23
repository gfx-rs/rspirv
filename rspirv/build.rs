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
// This rust module is automatically generated from the SPIR-V JSON grammar:
//   https://github.com/KhronosGroup/SPIRV-Headers/
//           blob/master/include/spirv/1.1/spirv.core.grammar.json";

#[cfg_attr(rustfmt, rustfmt_skip)]
static VAULE_ENUM_ATTRIBUTE: &'static str = "\
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, NumFromPrimitive)]";

static RUSTFMT_SKIP: &'static str = "#[cfg_attr(rustfmt, rustfmt_skip)]";
static RUSTFMT_SKIP_BANG: &'static str = "#![cfg_attr(rustfmt, rustfmt_skip)]";

/// Converts the given `symbol` to use snake case style.
fn split_words_with_underscore(symbol: &str) -> String {
    // Two named capture groups: the lowercase letter and the uppercase letter.
    let re = Regex::new(r"(?P<l>[:lower:])(?P<u>[:upper:])").unwrap();
    re.replace_all(symbol, "$l-$u").replace("-", "_")
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
                split_words_with_underscore(kind).to_uppercase(),
                split_words_with_underscore(symbol).to_uppercase(),
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
/// the given JSON object `value`.
///
/// `value` is expected to be an element in the "operand_kind" array
/// of the SPIR-V grammar.
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

fn write_copyright_autogen_comment(file: &mut fs::File) {
    file.write_all(COPYRIGHT.as_bytes()).unwrap();
    file.write_all(b"\n\n").unwrap();
    file.write_all(AUTOGEN_COMMENT.as_bytes()).unwrap();
    file.write_all(b"\n\n").unwrap();
}

/// Writes the generated SPIR-V header from parsing the given JSON object
/// `grammar` to the file with the given `filename`.
///
/// `grammar` is expected to be the root object of the SPIR-V grammar.
fn write_spirv_header(grammar: &Value, filename: &str) {
    let root = grammar.as_object().unwrap();
    let mut file = fs::File::create(filename).unwrap();

    { // Copyright, documentation.
        write_copyright_autogen_comment(&mut file);
        file.write_all(b"//! The SPIR-V header.\n\n").unwrap();
        file.write_all(b"#![allow(non_camel_case_types)]\n\n").unwrap();
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

fn convert_quantifier(quantifier: &str) -> &str {
    if quantifier == "" {
        "One"
    } else if quantifier == "?" {
        "ZeroOrOne"
    } else {
        "ZeroOrMore"
    }
}

/// Returns the code for the whole instruction table by parsing the given
/// JSON object `value`.
///
/// `value` is expected to be the "instructions" array of the SPIR-V grammar.
fn gen_instruction_table(value: &Value) -> String {
    let object = value.as_array().unwrap();
    let empty_array = Value::Array(vec![]);
    let empty_string = Value::String(String::new());
    // Vector for strings for all instructions.
    let elements: Vec<String> = object.iter().map(|ref element| {
        let inst = element.as_object().unwrap();
        let opname = inst.get("opname").unwrap().as_str().unwrap();
        let caps =
            inst.get("capabilities").unwrap_or(&empty_array)
                .as_array().unwrap();
        let caps: Vec<String> = caps.iter().map(|ref cap| {
            format!("{}", cap.as_str().unwrap())
        }).collect();
        let operands =
            inst.get("operands").unwrap_or(&empty_array)
                .as_array().unwrap();
        // Vector of strings for all operands.
        let operands: Vec<String> = operands.iter().map(|ref e| {
            let operand = e.as_object().unwrap();
            let kind = operand.get("kind").unwrap().as_str().unwrap();
            let quantifier =
                operand.get("quantifier").unwrap_or(&empty_string)
                    .as_str().unwrap();
            format!("({}, {})", kind, convert_quantifier(quantifier))
        }).collect();
        format!("    inst!({opname}, [{caps}], [{operands}]),",
                // Omit the "Op" prefix.
                opname=&opname[2..],
                caps=caps.join(", "),
                operands=operands.join(", "))
    }).collect();
    format!("{skip}\nstatic INSTRUCTION_TABLE: \
             &'static [Instruction<'static>] = &[\n{insts}\n];\n",
            skip=RUSTFMT_SKIP,
            insts=elements.join("\n"))
}

/// Writes the generated grammar::INSTRUCTION_TABLE and grammar::OperandKind
/// from parsing the given JSON object `grammar` to the file with the given
/// `filename`.
///
/// `grammar` is expected to be the root object of the SPIR-V grammar.
fn write_grammar_inst_table_operand_kinds(grammar: &Value, filename: &str) {
    let root = grammar.as_object().unwrap();
    let mut file = fs::File::create(filename).unwrap();

    write_copyright_autogen_comment(&mut file);

    { // Enum for all operand kinds.
        let kinds = root.get("operand_kinds").unwrap().as_array().unwrap();
        let elements: Vec<String> = kinds.iter().map(|ref element| {
            let kind = element.as_object().unwrap();
            let kind = kind.get("kind").unwrap().as_str().unwrap();
            format!("    {},", kind)
        }).collect();
        let kind_enum = format!(
            "/// All operand kinds in the SPIR-V grammar.\n\
             #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
             pub enum OperandKind {{\n{}\n}}\n",
            elements.join("\n"));
        file.write_all(&kind_enum.into_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    { // Instruction table.
        let table = gen_instruction_table(root.get("instructions").unwrap());
        file.write_all(&table.into_bytes()).unwrap();
    }
}

/// Writes the generated mr::Operand and its fmt::Display implementation from
/// parsing the given JSON object `value` to the file with the given `filename`.
///
/// `value` is expected to be the "operand_kinds" array of the SPIR-V grammar.
fn write_mr_operand_kinds(value: &Value, filename: &str) {
    let mut file = fs::File::create(filename).unwrap();

    write_copyright_autogen_comment(&mut file);

    { // Attributes, uses.
        file.write_all(RUSTFMT_SKIP_BANG.as_bytes()).unwrap();
        file.write_all(b"\n\nuse spirv;\nuse std::fmt;\n\n").unwrap();
    }

    let object = value.as_array().unwrap();
    let kinds: Vec<&str> =
        object.iter().filter(|ref element| {
            let kind = element.as_object().unwrap();
            // Pair kinds are not used in mr::Operands.
            !kind.get("kind").unwrap().as_str().unwrap().starts_with("Pair")
        }).map(|ref element| {
            let kind = element.as_object().unwrap();
            kind.get("kind").unwrap().as_str().unwrap()
        }).collect();

    { // Enum for all operand kinds in memory representation.
        let id_kinds: Vec<String> =
            kinds.iter().filter(|ref element| {
                element.starts_with("Id")
            }).map(|ref element| {
                format!("    {}(spirv::Word),", element)
            }).collect();
        let num_kinds: Vec<String> =
            kinds.iter().filter(|ref element| {
                element.ends_with("Integer") || element.ends_with("Number")
            }).map(|ref element| {
                format!("    {}(u32),", element)
            }).collect();
        let str_kinds: Vec<String> =
            kinds.iter().filter(|ref element| {
                element.ends_with("String")
            }).map(|ref element| {
                format!("    {}(String),", element)
            }).collect();
        let enum_kinds: Vec<String> =
            kinds.iter().filter(|ref element| {
                !(element.starts_with("Id") ||
                  element.ends_with("String") ||
                  element.ends_with("Integer") ||
                  element.ends_with("Number"))
            }).map(|ref element| {
                format!("    {k}(spirv::{k}),", k=element)
            }).collect();

        let kind_enum = format!(
            "/// Memory representation of a SPIR-V operand.\n\
             #[derive(Debug, PartialEq)]\n\
             pub enum Operand {{\n\
             {enum_kinds}\n{id_kinds}\n{num_kinds}\n{str_kinds}\n\
             }}\n",
             enum_kinds=enum_kinds.join("\n"),
             id_kinds=id_kinds.join("\n"),
             num_kinds=num_kinds.join("\n"),
             str_kinds=str_kinds.join("\n"));
        file.write_all(&kind_enum.into_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    { // impl fmt::Display for mr::Operand.
        let cases: Vec<String> =
            kinds.iter().map(|ref element| {
                format!("{space:12}Operand::{kind}(ref v) => \
                         write!(f, \"{{:?}}\", v),",
                        space="",
                        kind=element)
            }).collect();
        let impl_code = format!(
            "impl fmt::Display for Operand {{\n\
             {s:4}fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\n\
             {s:8}match *self {{\n{cases}\n{s:8}}}\n{s:4}}}\n}}\n",
             s="",
             cases=cases.join("\n"));
        file.write_all(&impl_code.into_bytes()).unwrap();
    }
}

/// Writes the generated operand decoding errors for binary::Decoder by
/// parsing the given JSON object `value` to the file with the given `filename`.
///
/// `value` is expected to be the "operand_kinds" array of the SPIR-V grammar.
fn write_operand_decode_errors(value: &Value, filename: &str) {
    let mut file = fs::File::create(filename).unwrap();

    { // Comments, attributes, uses.
        write_copyright_autogen_comment(&mut file);
        file.write_all(RUSTFMT_SKIP_BANG.as_bytes()).unwrap();
        file.write_all(b"\n\nuse spirv;\nuse std::{error, fmt};\n\n").unwrap();
    }

    let kinds = value.as_array().unwrap();
    let kinds: Vec<&str> =
        kinds.iter().filter(|ref element| {
            let kind = element.as_object().unwrap();
            let kind = kind.get("kind").unwrap().as_str().unwrap();
            !(kind.starts_with("Pair") ||
              kind.starts_with("Id") ||
              kind.starts_with("Literal"))
        }).map(|ref element| {
            let kind = element.as_object().unwrap();
            kind.get("kind").unwrap().as_str().unwrap()
        }).collect();

    // The Error enum.
    let errors: Vec<String> =
        kinds.iter().map(|ref element| {
            format!("    {}Unknown(usize, spirv::Word),", element)
        }).collect();
    let error_enum = format!(
        "/// Decoder Error.\n\
         #[derive(Debug, PartialEq)]\n\
         pub enum Error {{\n\
         {s:4}StreamExpected(usize),\n\
         {s:4}LimitReached(usize),\n\
         {errors}\n\
         {s:4}/// Failed to decode a string.\n\
         {s:4}///\n\
         {s:4}/// For structured error handling, the second element could be\n\
         {s:4}/// `string::FromUtf8Error`, but the will prohibit the compiler\n\
         {s:4}/// from generating `PartialEq` for this enum.\n\
         {s:4}DecodeStringFailed(usize, String),\n\
         }}\n",
        s="",
        errors=errors.join("\n"));
    file.write_all(&error_enum.into_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    // impl fmt::Display for the Error enum.
    let errors: Vec<String> =
        kinds.iter().map(|ref element| {
            format!("{s:12}Error::{kind}Unknown(index, word) => write!(\
                     f, \"unknown value {{}} for operand kind {kind} \
                     at index {{}}\", word, index),",
                    s="",
                    kind=element)
        }).collect();
    let display_impl = format!(
        "impl fmt::Display for Error {{\n\
         {s:4}fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\n\
         {s:8}match *self {{\n\
         {s:12}Error::StreamExpected(index) => write!(f, \"expected more \
             bytes in the stream at index {{}}\", index),\n\
         {s:12}Error::LimitReached(index) => write!(f, \"reached word limit \
             at index {{}}\", index),\n\
         {errors}\n\
         {s:12}Error::DecodeStringFailed(index, ref e) => write!(f, \
             \"cannot decode string at index {{}}: {{}}\", index, e),\n\
         {s:8}}}\n{s:4}}}\n}}\n",
        s="",
        errors=errors.join("\n"));
    file.write_all(&display_impl.into_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    // impl error::Error for the Error enum.
    let error_impl = format!(
        "impl error::Error for Error {{\n\
         {s:4}fn description(&self) -> &str {{\n\
         {s:8}match *self {{\n\
         {s:12}Error::StreamExpected(_) => \"expected more bytes \
             in the stream\",\n\
         {s:12}_ => \"unknown operand value for the given kind\",\n\
         {s:8}}}\n{s:4}}}\n}}\n",
        s="");
    file.write_all(&error_impl.into_bytes()).unwrap();
}

/// Writes the generated operand decoding methods for binary::Decoder by
/// parsing the given JSON object `value` to the file with the given `filename`.
///
/// `value` is expected to be the "operand_kinds" array of the SPIR-V grammar.
fn write_operand_decode_methods(value: &Value, filename: &str) {
    let mut file = fs::File::create(filename).unwrap();

    write_copyright_autogen_comment(&mut file);

    file.write_all(b"use num::FromPrimitive;\n\n").unwrap();

    let kinds = value.as_array().unwrap();
    let methods: Vec<String> =
        kinds.iter().filter(|ref element| {
            let kind = element.as_object().unwrap();
            let kind = kind.get("kind").unwrap().as_str().unwrap();
            // For kinds whose values may occupy more than one word, we need to
            // implement manually.
            !(kind.starts_with("Pair") ||
              kind.starts_with("Id") ||
              kind.starts_with("Literal"))
        }).map(|ref element| {
            let kind = element.as_object().unwrap();
            let category = kind.get("category").unwrap().as_str().unwrap();
            let kind = kind.get("kind").unwrap().as_str().unwrap();
            // Method definition for decoding values of a particular operand
            // kind. If the operand kind belongs to BitEnum, we use from_bits(),
            // otherwise, from_u32().
            format!(
                "{s:4}/// Decodes and returns the next SPIR-V word as\n\
                 {s:4}/// a SPIR-V {kind} value.\n\
                 {s:4}pub fn {fname}(&mut self) -> Result<spirv::{kind}> {{\n\
                 {s:8}if let Ok(word) = self.word() {{\n\
                     {s:12}spirv::{kind}::from_{ty}(word).ok_or(Error::\
                     {kind}Unknown(self.offset - WORD_NUM_BYTES, word))\n\
                 {s:8}}} else {{\n\
                     {s:12}Err(Error::StreamExpected(self.offset))\n\
                 {s:8}}}\n{s:4}}}\n",
                 s="",
                 fname=split_words_with_underscore(kind).to_lowercase(),
                 kind=kind,
                 ty=if category == "BitEnum" { "bits" } else { "u32" })
        }).collect();

    let impl_code = format!("impl Decoder {{\n{}}}\n", methods.join("\n"));
    file.write_all(&impl_code.into_bytes()).unwrap();
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

    {
        // Path to the generated SPIR-V header file.
        path.pop();
        path.pop();
        path.push("spirv.rs");
        let filename = path.to_str().unwrap();
        write_spirv_header(&grammar, filename);
    }

    {
        // Path to the generated instruction table.
        path.pop();
        path.push("grammar");
        path.push("table.rs");
        let filename = path.to_str().unwrap();
        write_grammar_inst_table_operand_kinds(&grammar, filename);
    }

    let root = grammar.as_object().unwrap();
    let operand_kinds = root.get("operand_kinds").unwrap();

    {
        // Path to the generated operands kind in memory representation.
        path.pop();
        path.pop();
        path.push("mr");
        path.push("operand.rs");
        let filename = path.to_str().unwrap();
        write_mr_operand_kinds(operand_kinds, filename);
    }

    {
        // Path to the generated decoding errors.
        path.pop();
        path.pop();
        path.push("binary");
        path.push("error.rs");
        let filename = path.to_str().unwrap();
        write_operand_decode_errors(operand_kinds, filename);
    }

    {
        // Path to the generated operand decoding methods.
        path.pop();
        path.push("decode_operand.rs");
        let filename = path.to_str().unwrap();
        write_operand_decode_methods(operand_kinds, filename);
    }
}
