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
///
/// For extended instruction sets, `ext_op_name` is the spirv Op enum name
/// and `ext_variant_name` is the `ExtInstOp` wrapper variant name.
pub(crate) fn gen_instruction_table(
    grammar: &[structs::Instruction],
    ext_op_name: Option<&str>,
    ext_variant_name: Option<&str>,
    name: &str,
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
        if let Some(spirv_op) = &ext_op_name {
            let opname = as_ident(&inst.opname);
            let spirv_op = as_ident(spirv_op);
            let variant = as_ident(ext_variant_name.unwrap());
            quote! {
                ext_inst!(#variant, #spirv_op, #opname, [#(#caps),*], [#(#exts),*], [#(#operands),*])
            }
        } else {
            let opname = as_ident(inst.opname.strip_prefix("Op").unwrap());
            quote! {
                inst!(#opname, [#(#caps),*], [#(#exts),*], [#(#operands),*])
            }
        }
    });
    let list_name = as_ident(&format!("{name}S"));
    let table_name = as_ident(&format!("{name}_TABLE"));

    let inst_type = as_ident(if ext_op_name.is_some() {
        "ExtendedInstruction"
    } else {
        "Instruction"
    });
    if ext_op_name.is_some() {
        quote! {
            static #list_name: &[#inst_type<'static>] = &[#(#instructions),*];
            pub static #table_name: InstructionTable<ExtInstOp> = InstructionTable(#list_name, std::marker::PhantomData);
        }
    } else {
        quote! {
            static #list_name: &[#inst_type<'static>] = &[#(#instructions),*];
            pub static #table_name: InstructionTable<spirv::Op> = InstructionTable(#list_name, std::marker::PhantomData);
        }
    }
}

/// Returns the generated grammar::INSTRUCTION_TABLE, grammar::OperandKind,
/// and grammar::ExtInstOp by walking the given SPIR-V `grammar`.
///
/// `ext_inst_variants` contains `(variant_name, op_name)` for all extended
/// instruction sets, used to generate the `ExtInstOp` wrapper enum.
pub fn gen_grammar_inst_table_operand_kinds(
    grammar: &structs::Grammar,
    ext_inst_variants: &[(&str, &str)],
) -> TokenStream {
    // Enum for all operand kinds.
    let elements = grammar
        .operand_kinds
        .iter()
        .map(|kind| as_ident(&kind.kind));

    // Instruction table.
    let table = gen_instruction_table(&grammar.instructions, None, None, "INSTRUCTION");

    // ExtInstOp enum: wrapper for all extended instruction set opcodes
    let ext_inst_variants_def = ext_inst_variants.iter().map(|(variant, op_name)| {
        let variant = as_ident(variant);
        let op = as_ident(op_name);
        quote! { #variant(spirv::#op) }
    });
    let ext_inst_from_arms = ext_inst_variants.iter().map(|(variant, _)| {
        let variant = as_ident(variant);
        quote! { ExtInstOp::#variant(v) => v as spirv::Word }
    });

    quote! {
        #[doc = "All operand kinds in the SPIR-V grammar."]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum OperandKind {
            #(#elements),*
        }

        #[doc = "Wrapper enum for all extended instruction set opcodes."]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum ExtInstOp {
            #(#ext_inst_variants_def),*
        }

        impl From<ExtInstOp> for spirv::Word {
            fn from(op: ExtInstOp) -> spirv::Word {
                match op {
                    #(#ext_inst_from_arms),*
                }
            }
        }

        #table
    }
}
