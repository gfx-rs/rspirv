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

use crate::structs;
use crate::utils::*;

static VAULE_ENUM_ATTRIBUTE: &'static str = "\
#[repr(u32)]\n#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]";

static GLSL_STD_450_SPEC_LINK: &'static str = "\
https://www.khronos.org/registry/spir-v/specs/unified1/GLSL.std.450.html";

static OPENCL_STD_SPEC_LINK: &'static str = "\
https://www.khronos.org/registry/spir-v/specs/unified1/OpenCL.ExtendedInstructionSet.100.html";

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
                            specs/unified1/SPIRV.html#_a_id_{}_a_{}",
                           symbol, symbol))
}

fn gen_bit_enum_operand_kind(grammar: &structs::OperandKind) -> String {
    let elements: Vec<String> = grammar.enumerants.iter().map(|enumerant| {
        // Special treatment for "NaN"
        let mut symbol = snake_casify(&enumerant.symbol);
        if &symbol == "not_na_n" {
            symbol = "not_nan".to_string()
        }
        format!("        const {} = {};",
                symbol.to_uppercase(),
                enumerant.value.string)
    }).collect();
    format!("bitflags!{{\n    {doc}\n    pub struct {kind} : u32 \
             {{\n{enumerants}\n    }}\n}}\n",
            doc = format!("/// SPIR-V operand kind: {}",
                          get_spec_link(&grammar.kind)),
            kind = grammar.kind,
            enumerants = elements.join("\n"))
}

fn gen_value_enum_operand_kind(grammar: &structs::OperandKind) -> String {
    use std::collections::BTreeMap;

    // We can have more than one enumerants mapping to the same discriminator.
    // Use associated constants for these aliases.
    let mut seen_discriminator = BTreeMap::new();
    let mut enumerants = vec![];
    let mut aliases = vec![];
    for e in &grammar.enumerants {
        if seen_discriminator.contains_key(&e.value.number) {
            aliases.push(format!("    pub const {}: {} = {}::{};",
                                 e.symbol, grammar.kind, grammar.kind,
                                 seen_discriminator.get(&e.value.number).unwrap()));
        } else {
            seen_discriminator.insert(e.value.number, &e.symbol);
            enumerants.push(
                if grammar.kind == "Dim" {
                    // Special case for Dim. Its enumerants can start with a digit.
                    // So prefix with the kind name here.
                    format!("    Dim{} = {},", e.symbol, e.value.number)
                } else {
                    format!("    {} = {},", e.symbol, e.value.number)
                });
        }
    }

    let mut associated_consts = String::new();
    if !aliases.is_empty() {
        associated_consts = format!("\n#[allow(non_upper_case_globals)]\nimpl {} {{\n{}\n}}",
                                    grammar.kind, aliases.join("\n"));
    }

    format!("{doc}\n{attribute}\npub enum {kind} {{\n{enumerants}\n}}\n{aliases}",
            doc = format!("/// SPIR-V operand kind: {}",
                          get_spec_link(&grammar.kind)),
            attribute = VAULE_ENUM_ATTRIBUTE,
            kind = grammar.kind,
            aliases = associated_consts,
            enumerants = enumerants.join("\n"))
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

/// Returns the generated SPIR-V header.
pub fn gen_spirv_header(grammar: &structs::Grammar) -> String {
    let mut ret = String::new();

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
        ret.push_str(&globals);
    }
    { // Operand kinds.
        for kind in &grammar.operand_kinds {
            let operand_kind = gen_operand_kind(kind);
            if operand_kind.is_some() {
                let kind = operand_kind.unwrap();
                ret.push_str(&kind);
                ret.push('\n');
            }
        }
    }
    { // Opcodes.
        // Get the instruction table.
        let opcodes: Vec<String> = grammar.instructions.iter().map(|inst| {
            // Omit the "Op" prefix.
            format!("    {} = {},", &inst.opname[2..], inst.opcode)
        }).collect();
        ret.push_str(&format!("/// SPIR-V {link} opcodes\n\
                               {attribute}\n\
                               pub enum Op {{\n{opcodes}\n}}\n",
                              link = get_spec_link("instructions"),
                              attribute = VAULE_ENUM_ATTRIBUTE,
                              opcodes = opcodes.join("\n")));
    }

    ret
}

/// Returns the GLSL.std.450 extended instruction opcodes.
pub fn gen_glsl_std_450_opcodes(grammar: &structs::ExtInstSetGrammar) -> String {
    let mut ret = String::new();

    { // Opcodes.
        // Get the instruction table.
        let opcodes: Vec<String> = grammar.instructions.iter().map(|inst| {
            // Omit the "Op" prefix.
            format!("    {} = {},", inst.opname, inst.opcode)
        }).collect();
        ret.push_str(&format!("/// [GLSL.std.450]({link}) extended instruction opcode\n\
                               {attribute}\n\
                               pub enum GLOp {{\n{opcodes}\n}}\n",
                              link = GLSL_STD_450_SPEC_LINK,
                              attribute = VAULE_ENUM_ATTRIBUTE,
                              opcodes = opcodes.join("\n")));
    }

    ret
}

/// Returns the OpenCL.std extended instruction opcodes.
pub fn gen_opencl_std_opcodes(grammar: &structs::ExtInstSetGrammar) -> String {
    let mut ret = String::new();

    { // Opcodes.
        // Get the instruction table.
        let opcodes: Vec<String> = grammar.instructions.iter().map(|inst| {
            // Omit the "Op" prefix.
            format!("    {} = {},", inst.opname, inst.opcode)
        }).collect();
        ret.push_str(&format!("/// [OpenCL.std]({link}) extended instruction opcode\n\
                               {attribute}\n\
                               pub enum CLOp {{\n{opcodes}\n}}\n",
                              link = OPENCL_STD_SPEC_LINK,
                              attribute = VAULE_ENUM_ATTRIBUTE,
                              opcodes = opcodes.join("\n")));
    }

    ret
}
