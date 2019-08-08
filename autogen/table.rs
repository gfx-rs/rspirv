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

use proc_macro2::{Ident, TokenStream};
use quote::quote;

fn convert_quantifier(quantifier: structs::Quantifier) -> Ident {
    use structs::Quantifier::*;
    as_ident(match quantifier {
        One => "One",
        ZeroOrOne => "ZeroOrOne",
        ZeroOrMore => "ZeroOrMore",
    })
}

/// Returns the code for the whole instruction table by walking the given
/// `grammar`.
///
/// `grammar` is expected to be an array of SPIR-V instructions.
/// `name` is the name of the generated table.
/// `is_ext` indicates whether the grammar is for an extended instruction set.
fn gen_instruction_table(grammar: &Vec<structs::Instruction>, name: &str, is_ext: bool) -> TokenStream {
    // Vector for strings for all instructions.
    let instructions = grammar.iter().map(|inst| {
        // Vector of strings for all operands.
        let operands = inst.operands.iter().map(|e| {
            let kind = as_ident(&e.kind);
            let quantifier = convert_quantifier(e.quantifier);
            quote! { (#kind, #quantifier) }
        });
        let caps = inst.capabilities.iter().map(|cap| as_ident(cap));
        if is_ext {
            let opname = as_ident(&inst.opname);
            let opcode = inst.opcode;
            quote! {
                ext_inst!(#opname, #opcode, [#(#caps),*], [#(#operands),*])
            }
        } else {
            // Omit the "Op" prefix.
            let opname = as_ident(&inst.opname[2..]);
            quote! {
                inst!(#opname, [#(#caps),*], [#(#operands),*])
            }
        }
    });
    let name = as_ident(name);
    let inst_type = as_ident(if is_ext { "ExtendedInstruction" } else { "Instruction" });
    quote! {
        static #name: &[#inst_type<'static>] = &[#(#instructions),*];
    }
}

/// Returns the generated grammar::INSTRUCTION_TABLE and grammar::OperandKind
/// by walking the given SPIR-V `grammar`.
pub fn gen_grammar_inst_table_operand_kinds(grammar: &structs::Grammar) -> TokenStream {

    // Enum for all operand kinds.
    let elements = grammar.operand_kinds.iter().map(|kind| as_ident(&kind.kind));

    // Instruction table.
    let table = gen_instruction_table(&grammar.instructions, "INSTRUCTION_TABLE", false);

    quote! {
        #[doc = "All operand kinds in the SPIR-V grammar."]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum OperandKind {
            #(#elements),*
        }

        #table
    }
}

/// Writes the generated instruction table for GLSLstd450 extended instruction
/// set from `grammar` to the file with the given `filename`.
pub fn gen_glsl_std_450_inst_table(grammar: &structs::ExtInstSetGrammar) -> TokenStream {
    gen_instruction_table(
        &grammar.instructions, "GLSL_STD_450_INSTRUCTION_TABLE", true)
}

/// Writes the generated instruction table for OpenCLstd100 extended instruction
/// set from `grammar` to the file with the given `filename`.
pub fn gen_opencl_std_100_inst_table(grammar: &structs::ExtInstSetGrammar) -> TokenStream {
    gen_instruction_table(
        &grammar.instructions, "OPENCL_STD_100_INSTRUCTION_TABLE", true)
}
