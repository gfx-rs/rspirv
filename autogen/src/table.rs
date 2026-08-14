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

/// Returns the code for the whole instruction table.
///
/// `instructions` is expected to be an array of SPIR-V instructions.
/// `name` is the name of the generated table.
///
/// For extended instruction sets, `ext_op_name` is the spirv Op enum name,
/// `ext_variant_name` is the `ExtInstOp` wrapper variant name,
/// and `ext_operand_info` maps extension-specific operand kind names to their
/// `OperandKind` wrapper variant name.
pub(crate) fn gen_instruction_table(
    instructions: &[structs::Instruction],
    ext_op_name: Option<&str>,
    ext_variant_name: Option<&str>,
    name: &str,
    ext_operand_info: Option<(&str, &[String])>,
) -> TokenStream {
    // Vector for strings for all instructions.
    let instructions = instructions.iter().map(|inst| {
        // Vector of strings for all operands.
        let operands = inst.operands.iter().map(|e| {
            let kind = as_ident(&e.kind);
            let quantifier = convert_quantifier(e.quantifier);
            if let Some((variant_name, ext_kinds)) = &ext_operand_info {
                // Extension with own operand kinds: emit full expressions
                // (both core and extension kinds) so the expr arm matches
                let kind_expr = if ext_kinds.iter().any(|k| k == &e.kind) {
                    let variant = as_ident(variant_name);
                    quote! { OperandKind::#variant(ExtOperandKind::#kind) }
                } else {
                    quote! { OperandKind::#kind }
                };
                quote! { (#kind_expr, #quantifier) }
            } else {
                // Core instructions and extensions without own operand kinds:
                // emit plain idents, the macro prepends OperandKind::
                quote! { (#kind, #quantifier) }
            }
        });
        let caps = inst.capabilities.iter().map(|cap| as_ident(cap));
        let exts = &inst.extensions;
        if let Some(spirv_op) = &ext_op_name {
            let opname = as_ident(&inst.opname);
            let spirv_op = as_ident(spirv_op);
            let variant = as_ident(ext_variant_name.unwrap());
            if ext_operand_info.is_some() {
                // Has extension operand kinds: all operands pre-wrapped,
                // trailing comma triggers the expr arm
                quote! {
                    ext_inst!(#variant, #spirv_op, #opname, [#(#caps),*], [#(#exts),*], [#(#operands),*],)
                }
            } else {
                // All-ident operands: the ident arm wraps them
                quote! {
                    ext_inst!(#variant, #spirv_op, #opname, [#(#caps),*], [#(#exts),*], [#(#operands),*])
                }
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

/// Generates the core `OperandKind` enum (with wrapper variants for extensions),
/// the `ExtInstOp` wrapper enum for extended instruction opcodes,
/// and the core instruction table.
pub fn gen_grammar_inst_table_operand_kinds(
    operand_kinds: &[structs::OperandKind],
    instructions: &[structs::Instruction],
    ext_wrapper_variants: &[(&str, &str)], // [(variant_name, module_name)] - only extensions with operand kinds
    ext_inst_variants: &[(&str, &str)],    // [(variant_name, op_name)] - ALL extensions
) -> TokenStream {
    // Enum for all operand kinds.
    let elements = operand_kinds.iter().map(|kind| as_ident(&kind.kind));

    // Instruction table.
    let table = gen_instruction_table(instructions, None, None, "INSTRUCTION", None);

    // Wrapper variants for extensions with their own operand kinds
    let wrapper_variants = ext_wrapper_variants.iter().map(|(variant, module)| {
        let variant = as_ident(variant);
        let module = as_ident(module);
        quote! { #variant(#module::ExtOperandKind) }
    });

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
            #(#elements,)*
            #(#wrapper_variants),*
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

/// Generates an `ExtOperandKind` enum and instruction table for an extended
/// instruction set.
pub fn gen_ext_instruction_file(
    operand_kinds: &[structs::OperandKind],
    instructions: &[structs::Instruction],
    ext_op_name: &str,
    ext_variant_name: &str,
    name: &str,
    wrapper_variant_name: &str,
) -> TokenStream {
    let ext_kind_names: Vec<String> = operand_kinds.iter().map(|k| k.kind.clone()).collect();

    let ext_operand_kind = if operand_kinds.is_empty() {
        None
    } else {
        let elements = operand_kinds.iter().map(|kind| as_ident(&kind.kind));
        Some(quote! {
            #[doc = "Extended instruction operand kinds."]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            #[allow(clippy::upper_case_acronyms)]
            pub enum ExtOperandKind {
                #(#elements),*
            }
        })
    };

    let ext_info = if ext_kind_names.is_empty() {
        None
    } else {
        Some((wrapper_variant_name, ext_kind_names.as_slice()))
    };

    let table = gen_instruction_table(
        instructions,
        Some(ext_op_name),
        Some(ext_variant_name),
        name,
        ext_info,
    );

    quote! {
        #ext_operand_kind
        #table
    }
}
