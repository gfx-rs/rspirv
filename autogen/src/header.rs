use crate::structs;
use crate::utils::*;

use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::cmp::Ordering;
use std::collections::BTreeMap;

/// Returns the markdown string containing a link to the spec for the given
/// operand `kind`.
fn get_spec_link(kind: &str) -> String {
    let symbol = kind.to_snake_case();
    format!(
        "[{kind}](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_{symbol}_a_{symbol})",
    )
}

fn value_enum_attribute() -> TokenStream {
    quote! {
        #[repr(u32)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
        #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
        #[allow(clippy::upper_case_acronyms)]
    }
}

fn bit_enum_attribute() -> TokenStream {
    quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
        #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    }
}

fn generate_enum(
    enum_name: &proc_macro2::Ident,
    variants: &[(u32, proc_macro2::Ident)],
    comment: &str,
) -> TokenStream {
    let mut variants = variants.to_vec();
    variants.sort_by_key(|&(number, _)| number);
    let enumerants = variants
        .iter()
        .map(|(number, name)| quote! { #name = #number });

    // Each item is a tuple indicating an inclusive range as opposed to an exclusive range like
    // is common.
    let mut number_runs = Vec::<(u32, u32)>::new();
    for &(number, _) in variants.iter() {
        if let Some(last_run) = number_runs.last_mut() {
            match number.cmp(&(last_run.1 + 1)) {
                Ordering::Equal => last_run.1 = number,
                Ordering::Greater => number_runs.push((number, number)),
                Ordering::Less => unreachable!("Variants not sorted by discriminant"),
            }
        } else {
            number_runs.push((number, number));
        }
    }
    // We try to check if the given number is within a run of valid discriminants and if so
    // transmute the number directly to the enum type.
    let mut from_prim = number_runs
        .iter()
        .map(|&(start, end)| {
            if end == start {
                // Fast path if a run only contains a single discriminant
                quote! { #start => unsafe { core::mem::transmute::<u32, #enum_name>(#start) } }
            } else {
                quote! { #start..=#end => unsafe { core::mem::transmute::<u32, #enum_name>(n) } }
            }
        })
        .collect::<Vec<_>>();

    // At least one variant is required for repr(u32).  Generate a Max member like Spirv-Headers
    // tooling does.
    let empty_enum = variants.is_empty().then_some(quote!(Max = 0x7fffffff));

    // TODO: Only FPEncoding doesn't list any valid values besides a "Max". This type shouldn't be
    // an `enum` but a `pub` newtype wrapper?
    if variants.is_empty() {
        from_prim.push(quote! { 0x7fffffff => Self::Max });
    }

    let attribute = value_enum_attribute();
    quote! {
        #[doc = #comment]
        #attribute
        pub enum #enum_name {
            #empty_enum
            #(#enumerants),*
        }

        impl #enum_name {
            pub fn from_u32(n: u32) -> Option<Self> {
                Some(match n {
                    #(#from_prim,)*
                    _ => return None
                })
            }
        }
    }
}

fn gen_bit_enum_operand_kind(grammar: &structs::OperandKind) -> TokenStream {
    let mut elements = vec![];
    let mut operands = vec![];
    let mut additional_operands_list = vec![];

    for enumerant in grammar.enumerants.iter() {
        // Special treatment for "NaN"
        let symbol = as_ident(
            &enumerant
                .symbol
                .to_shouty_snake_case()
                .replace("NA_N", "NAN"),
        );
        let value = enumerant.value;

        elements.push(quote! {
            const #symbol = #value;
        });

        let parameters = enumerant.parameters.iter().map(|op| {
            let kind = as_ident(&op.kind);

            let quant = match op.quantifier {
                structs::Quantifier::One => quote! { OperandQuantifier::One },
                structs::Quantifier::ZeroOrOne => quote! { OperandQuantifier::ZeroOrOne },
                structs::Quantifier::ZeroOrMore => quote! { OperandQuantifier::ZeroOrMore },
            };

            quote! {
                LogicalOperand {
                    kind: OperandKind::#kind,
                    quantifier: #quant
                }
            }
        });

        if !enumerant.parameters.is_empty() {
            additional_operands_list.push(quote! { Self::#symbol });

            operands.push(quote! {
                Self::#symbol if self.contains(*v) => {
                    [#( #parameters ),*].iter()
                }
            });
        }
    }

    let comment = format!("SPIR-V operand kind: {}", get_spec_link(&grammar.kind));
    let kind = as_ident(&grammar.kind);
    let attribute = bit_enum_attribute();

    quote! {
        bitflags! {
            #[doc = #comment]
            #attribute
            pub struct #kind: u32 {
                #(#elements)*
            }
        }
    }
}

fn gen_value_enum_operand_kind(grammar: &structs::OperandKind) -> TokenStream {
    let kind = as_ident(&grammar.kind);

    // We can have more than one enumerants mapping to the same discriminator.
    // Use associated constants for these aliases.
    let mut variants = vec![];
    let mut aliases = vec![];
    let mut capability_clauses = BTreeMap::new();
    let mut extension_clauses = BTreeMap::new();
    let mut operand_clauses = BTreeMap::new();
    let mut from_str_impl = vec![];
    for e in &grammar.enumerants {
        // Special case for Dim. Its enumerants can start with a digit.
        // So prefix with the kind name here.
        let name_str = if grammar.kind == "Dim" {
            let mut name = "Dim".to_string();
            name.push_str(&e.symbol);
            name
        } else {
            e.symbol.to_string()
        };
        let name = as_ident(&name_str);
        let number = e.value;
        variants.push((number, name.clone()));
        from_str_impl.push(quote! { #name_str => Self::#name });

        for alias in &e.aliases {
            let alias_ident = as_ident(alias);
            aliases.push(quote!(pub const #alias_ident: Self = Self::#name;));
            from_str_impl.push(quote!(#alias => Self::#name));
        }

        capability_clauses
            .entry(&e.capabilities)
            .or_insert_with(Vec::new)
            .push(name.clone());

        extension_clauses
            .entry(&e.extensions)
            .or_insert_with(Vec::new)
            .push(name.clone());

        operand_clauses
            .entry(name.clone())
            .or_insert_with(Vec::new)
            .extend(e.parameters.iter().map(|op| {
                let kind = as_ident(&op.kind);

                let quant = match op.quantifier {
                    structs::Quantifier::One => quote! { OperandQuantifier::One },
                    structs::Quantifier::ZeroOrOne => quote! { OperandQuantifier::ZeroOrOne },
                    structs::Quantifier::ZeroOrMore => quote! { OperandQuantifier::ZeroOrMore },
                };

                quote! {
                    LogicalOperand {
                        kind: OperandKind::#kind,
                        quantifier: #quant
                    }
                }
            }));
    }

    let the_enum = generate_enum(
        &kind,
        &variants,
        &format!("SPIR-V operand kind: {}", get_spec_link(&grammar.kind)),
    );

    // TODO: Only FPEncoding doesn't list any valid values besides a "Max". This type shouldn't be
    // an `enum` but a `pub` newtype wrapper?
    if variants.is_empty() {
        from_str_impl.push(quote! { "Max" => Self::Max });
    }

    quote! {
        #the_enum

        #[allow(non_upper_case_globals)]
        impl #kind {
            #(#aliases)*
        }

        impl core::str::FromStr for #kind {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    #(#from_str_impl,)*
                    _ => return Err(())
                })
            }
        }
    }
}

/// Returns the code defining the enum for an operand kind by parsing
/// the given SPIR-V `grammar`.
pub fn gen_operand_kind(grammar: &structs::OperandKind) -> Option<TokenStream> {
    use structs::Category::*;
    match grammar.category {
        BitEnum => Some(gen_bit_enum_operand_kind(grammar)),
        ValueEnum => Some(gen_value_enum_operand_kind(grammar)),
        _ => None,
    }
}

/// Returns the generated SPIR-V header.
pub fn gen_spirv_header(grammar: &structs::Grammar) -> TokenStream {
    // constants and types.
    let magic_number = format!("{:#010X}", grammar.magic_number)
        .parse::<TokenStream>()
        .unwrap();
    let major_version = grammar.major_version;
    let minor_version = grammar.minor_version;
    let revision = grammar.revision;

    // Operand kinds.
    let kinds = grammar.operand_kinds.iter().filter_map(gen_operand_kind);

    // Opcodes.

    // We can have more than one op symbol mapping to the same opcode.
    // Use associated constants for these aliases.
    let mut aliases = vec![];
    let mut variants = vec![];

    // Get the instruction table.
    for inst in &grammar.instructions {
        let opname = as_ident(inst.opname.strip_prefix("Op").unwrap());
        let opcode = inst.opcode;
        for alias in &inst.aliases {
            let alias = as_ident(alias.strip_prefix("Op").unwrap());
            aliases.push(quote! { pub const #alias: Op = Op::#opname; });
        }
        variants.push((opcode, opname.clone()));
    }

    let the_enum = generate_enum(
        &as_ident("Op"),
        &variants,
        &format!("SPIR-V {} opcodes", get_spec_link("instructions")),
    );

    quote! {
        //pub use crate::grammar::{OperandKind, OperandQuantifier, LogicalOperand};
        pub type Word = u32;
        pub const MAGIC_NUMBER: u32 = #magic_number;
        pub const MAJOR_VERSION: u8 = #major_version;
        pub const MINOR_VERSION: u8 = #minor_version;
        pub const REVISION: u8 = #revision;

        #(#kinds)*

        #the_enum

        #[allow(clippy::upper_case_acronyms)]
        #[allow(non_upper_case_globals)]
        impl Op {
            #(#aliases)*
        }
    }
}

/// Returns extended instruction opcodes
pub fn gen_opcodes(op: &str, grammar: &structs::ExtInstSetGrammar, comment: &str) -> TokenStream {
    let op = as_ident(op);
    // Get the instruction table.
    let variants = grammar
        .instructions
        .iter()
        .map(|inst| {
            let opname = as_ident(&inst.opname);
            let opcode = inst.opcode;
            (opcode, opname)
        })
        .collect::<Vec<_>>();

    generate_enum(&op, &variants, comment)
}
