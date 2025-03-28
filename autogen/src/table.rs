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
pub(crate) fn gen_instruction_table(
    grammar: &[structs::Instruction],
    name: &str,
    is_ext: bool,
) -> TokenStream {
    // Vector for strings for all instructions.
    let instructions = grammar.iter().map(|inst| {
        // Vector of strings for all operands.
        let operands = inst.operands.iter().map(|e| {
            let kind = as_ident(&e.kind);
            let quantifier = convert_quantifier(e.quantifier);
            quote! { (#kind, #quantifier) }
        });
        let caps = inst.capabilities.iter().map(|cap| as_ident(cap));
        let exts = &inst.extensions;
        if is_ext {
            let opname = as_ident(&inst.opname);
            let opcode = inst.opcode;
            quote! {
                ext_inst!(#opname, #opcode, [#(#caps),*], [#(#exts),*], [#(#operands),*])
            }
        } else {
            let opname = as_ident(inst.opname.strip_prefix("Op").unwrap());
            quote! {
                inst!(#opname, [#(#caps),*], [#(#exts),*], [#(#operands),*])
            }
        }
    });
    let name = as_ident(name);
    let inst_type = as_ident(if is_ext {
        "ExtendedInstruction"
    } else {
        "Instruction"
    });
    quote! {
        static #name: &[#inst_type<'static>] = &[#(#instructions),*];
    }
}

/// Returns the generated grammar::INSTRUCTION_TABLE and grammar::OperandKind
/// by walking the given SPIR-V `grammar`.
pub fn gen_grammar_inst_table_operand_kinds(grammar: &structs::Grammar) -> TokenStream {
    // Enum for all operand kinds.
    let elements = grammar
        .operand_kinds
        .iter()
        .map(|kind| as_ident(&kind.kind));

    // Instruction table.
    let table = gen_instruction_table(&grammar.instructions, "INSTRUCTION_TABLE", false);

    quote! {
        #[doc = "All operand kinds in the SPIR-V grammar."]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum OperandKind {
            #(#elements),*
        }

        #table
    }
}
