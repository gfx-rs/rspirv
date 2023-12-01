use crate::structs;

use std::fs;
use std::io::Write;

use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[rustfmt::skip]
static AUTOGEN_COMMENT : &str = "\
// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!";

pub fn write_autogen_comment(file: &mut fs::File) {
    file.write_all(AUTOGEN_COMMENT.as_bytes()).unwrap();
    file.write_all(b"\n\n").unwrap();
}

/// Converts the given string into an `Ident`, with call-site span.
pub fn as_ident(ident: &str) -> Ident {
    Ident::new(ident, Span::call_site())
}

/// Returns the corresponding operand kind in data representation for the
/// given operand `kind` in the grammar.
pub fn get_dr_operand_kind(kind: &str) -> Ident {
    as_ident(
        if matches!(
            kind,
            "LiteralInteger" | "LiteralContextDependentNumber" | "LiteralFloat"
        ) {
            // TODO: LiteralContextDependentNumber should use the correct type to decode
            "LiteralBit32"
        } else {
            kind
        },
    )
}

/// Returns the underlying type used in operand kind enums for the operand
/// kind `kind` in the grammar.
pub fn get_enum_underlying_type(kind: &str, generic_string: bool) -> TokenStream {
    if kind.starts_with("Id") {
        quote! { spirv::Word }
    } else if kind == "LiteralInteger" || kind == "LiteralExtInstInteger" || kind == "LiteralFloat"
    {
        quote! { u32 }
    } else if kind == "LiteralSpecConstantOpInteger" {
        quote! { spirv::Op }
    } else if kind == "LiteralContextDependentNumber" {
        panic!("this kind is not expected to be handled here")
    } else if kind == "LiteralString" {
        if generic_string {
            quote! { impl Into<String> }
        } else {
            quote! { String }
        }
    } else if kind == "PairLiteralIntegerIdRef" {
        quote! { (dr::Operand, spirv::Word) }
    } else if kind == "PairIdRefLiteralInteger" {
        quote! { (spirv::Word, u32) }
    } else if kind == "PairIdRefIdRef" {
        quote! { (spirv::Word, spirv::Word) }
    } else {
        let kind = as_ident(kind);
        quote! { spirv::#kind }
    }
}

/// Returns a suitable name for the given parameter.
pub fn get_param_name(params: &[structs::Operand], param_index: usize) -> Ident {
    fn get_original_name(param: &structs::Operand) -> &str {
        if param.name.is_empty() {
            if param.kind == "IdResultType" {
                "result_type"
            } else {
                &param.kind
            }
        } else {
            &param.name
        }
    }

    let param = &params[param_index];

    let raw_name = get_original_name(param);

    // OpCopyMemory and OpCopyMemorySized have duplicate field names
    let duplicate_count = params
        .iter()
        .take(param_index)
        .filter(|p| get_original_name(p) == raw_name)
        .count();

    let mut name = raw_name.to_snake_case();

    // Rename/remap rust reserved keywords
    if name == "type" {
        name = "ty".to_owned();
    }
    if name == "use" {
        name = "usage".to_owned();
    }

    if duplicate_count > 0 {
        name = format!("{}_{}", name, duplicate_count + 1);
    }

    as_ident(&name)
}
