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

use structs;

use utils::*;

fn convert_quantifier(quantifier: &str) -> &str {
    if quantifier == "" {
        "One"
    } else if quantifier == "?" {
        "ZeroOrOne"
    } else {
        "ZeroOrMore"
    }
}

/// Returns the code for the whole instruction table by walking the given
/// `grammar`.
///
/// `grammar` is expected to be an array of SPIR-V instructions.
/// `name` is the name of the generated table.
/// `is_ext` indicates whether the grammar is for an extended instruction set.
fn gen_instruction_table(grammar: &Vec<structs::Instruction>,
                         name: &str, is_ext: bool)
                         -> String {
    // Vector for strings for all instructions.
    let elements: Vec<String> = grammar.iter().map(|inst| {
        // Vector of strings for all operands.
        let operands: Vec<String> = inst.operands.iter().map(|e| {
            format!("({}, {})", e.kind, convert_quantifier(&e.quantifier))
        }).collect();
        if is_ext {
            format!("    ext_inst!({name}, {code}, [{caps}], [{operands}]),",
                    // Omit the "Op" prefix.
                    name = &inst.opname,
                    code = inst.opcode,
                    caps = inst.capabilities.join(", "),
                    operands = operands.join(", "))
        } else {
            format!("    inst!({opname}, [{caps}], [{operands}]),",
                    // Omit the "Op" prefix.
                    opname = &inst.opname[2..],
                    caps = inst.capabilities.join(", "),
                    operands = operands.join(", "))
        }
    }).collect();
    format!("{skip}\nstatic {name}: \
             &'static [{ext}Instruction<'static>] = &[\n{insts}\n];\n",
            skip = RUSTFMT_SKIP,
            name = name,
            ext = if is_ext { "Extended" } else { "" },
            insts = elements.join("\n"))
}

/// Returns the generated grammar::INSTRUCTION_TABLE and grammar::OperandKind
/// by walking the given SPIR-V `grammar`.
pub fn gen_grammar_inst_table_operand_kinds(grammar: &structs::Grammar)
                                            -> String {
    let mut ret = String::new();

    { // Enum for all operand kinds.
        let elements: Vec<String> =
            grammar.operand_kinds.iter().map(|kind| {
                format!("    {},", kind.kind)
            }).collect();
        let kind_enum = format!(
            "/// All operand kinds in the SPIR-V grammar.\n\
             #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n\
             pub enum OperandKind {{\n{}\n}}\n\n",
            elements.join("\n"));
        ret.push_str(&kind_enum);
    }

    { // Instruction table.
        let table = gen_instruction_table(
            &grammar.instructions, "INSTRUCTION_TABLE", false);
        ret.push_str(&table);
    }

    ret
}

/// Writes the generated instruction table for GLSLstd450 extended instruction
/// set from `grammar` to the file with the given `filename`.
pub fn gen_glsl_std_450_inst_table(grammar: &structs::ExtInstSetGrammar) -> String {
    gen_instruction_table(
        &grammar.instructions, "GLSL_STD_450_INSTRUCTION_TABLE", true)
}

/// Writes the generated instruction table for OpenCLstd100 extended instruction
/// set from `grammar` to the file with the given `filename`.
pub fn gen_opencl_std_100_inst_table(grammar: &structs::ExtInstSetGrammar) -> String {
    gen_instruction_table(
        &grammar.instructions, "OPENCL_STD_100_INSTRUCTION_TABLE", true)
}
