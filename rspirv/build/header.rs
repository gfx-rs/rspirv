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

use std::fs;
use std::io::Write;
use structs;

use utils::*;

/// Returns the markdown string containing a link to the spec for the given
/// operand `kind`.
fn get_spec_link(kind: &str) -> String {
    let mut symbol = snake_casify(kind);
    if symbol.starts_with("fp") {
        // Special case for FPFastMathMode and FPRoundingMode.
        symbol = symbol.replace("fp", "fp_");
    }
    format!("[{text}]({link})",
            text = kind,
            link = format!("https://www.khronos.org/registry/spir-v/\
                            specs/1.1/SPIRV.html#_a_id_{}_a_{}",
                           symbol, symbol))
}

fn gen_bit_enum_operand_kind(grammar: &structs::OperandKind) -> String {
    let elements: Vec<String> = grammar.enumerants.iter().map(|enumerant| {
        format!("        const {}_{} = {},",
                snake_casify(&grammar.kind).to_uppercase(),
                snake_casify(&enumerant.symbol).to_uppercase(),
                enumerant.value.string)
    }).collect();
    format!("bitflags!{{\n    {doc}\n    pub flags {kind} : u32 \
             {{\n{enumerants}\n    }}\n}}\n",
            doc = format!("/// SPIR-V operand kind: {}",
                          get_spec_link(&grammar.kind)),
            kind = grammar.kind,
            enumerants = elements.join("\n"))
}

fn gen_value_enum_operand_kind(grammar: &structs::OperandKind) -> String {
    let elements: Vec<String> = grammar.enumerants.iter().map(|enumerant| {
        // Special case for Dim. Its enumerants can start with a digit.
        // So prefix with the kind name here.
        if grammar.kind == "Dim" {
            format!("    Dim{} = {},", enumerant.symbol, enumerant.value.number)
        } else {
            format!("    {} = {},", enumerant.symbol, enumerant.value.number)
        }
    }).collect();
    format!("{doc}\n{attribute}\npub enum {kind} {{\n{enumerants}\n}}\n",
            doc = format!("/// SPIR-V operand kind: {}",
                          get_spec_link(&grammar.kind)),
            attribute = VAULE_ENUM_ATTRIBUTE,
            kind = grammar.kind,
            enumerants = elements.join("\n"))
}

/// Returns the code defining the enum for an operand kind by parsing
/// the given SPIR-V `grammar`.
fn gen_operand_kind(grammar: &structs::OperandKind) -> Option<String> {
    if grammar.category == "BitEnum" {
        Some(gen_bit_enum_operand_kind(grammar))
    } else if grammar.category == "ValueEnum" {
        Some(gen_value_enum_operand_kind(grammar))
    } else {
        None
    }
}

/// Writes the generated SPIR-V header to the file with the given `filename`.
pub fn write_spirv_header(grammar: &structs::Grammar, filename: &str) {
    let mut file = fs::File::create(filename).unwrap();

    { // Copyright, documentation.
        write_copyright_autogen_comment(&mut file);
        file.write_all(b"//! The SPIR-V header.\n\n").unwrap();
        file.write_all(b"#![allow(non_camel_case_types)]\n").unwrap();
        file.write_all(RUSTFMT_SKIP_BANG.as_bytes()).unwrap();
        file.write_all(b"\n\n").unwrap();
    }
    { // constants and types.
        let globals = format!("pub type Word = u32;\n\
                               pub const MAGIC_NUMBER: u32 = {};\n\
                               pub const MAJOR_VERSION: u32 = {};\n\
                               pub const MINOR_VERSION: u32 = {};\n\
                               pub const REVISION: u32 = {};\n\n",
                              grammar.magic_number,
                              grammar.major_version,
                              grammar.minor_version,
                              grammar.revision);
        file.write_all(&globals.into_bytes()).unwrap();
    }
    { // Operand kinds.
        for kind in &grammar.operand_kinds {
            let operand_kind = gen_operand_kind(kind);
            if operand_kind.is_some() {
                let kind = operand_kind.unwrap();
                file.write_all(&kind.into_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            }
        }
    }
    { // Opcodes.
        // Get the instruction table.
        let opcodes: Vec<String> = grammar.instructions.iter().map(|inst| {
            // Omit the "Op" prefix.
            format!("    {} = {},", &inst.opname[2..], inst.opcode)
        }).collect();
        let opcode_enum = format!("/// SPIR-V {link} opcodes\n\
                                   {attribute}\npub enum Op \
                                   {{\n{opcodes}\n}}\n",
                                  link = get_spec_link("instructions"),
                                  attribute = VAULE_ENUM_ATTRIBUTE,
                                  opcodes = opcodes.join("\n"));
        file.write_all(&opcode_enum.into_bytes()).unwrap();
    }
}
