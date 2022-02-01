use crate::structs;
use crate::utils::*;

use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::cmp::Ordering;
use std::collections::BTreeMap;

static GLSL_STD_450_SPEC_LINK: &str = "\
https://www.khronos.org/registry/spir-v/specs/unified1/GLSL.std.450.html";

static OPENCL_STD_SPEC_LINK: &str = "\
https://www.khronos.org/registry/spir-v/specs/unified1/OpenCL.ExtendedInstructionSet.100.html";

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
        #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
        #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    }
}

fn generate_enum(
    enum_name: &proc_macro2::Ident,
    variants: &[(u32, proc_macro2::Ident)],
    comment: String,
) -> TokenStream {
    let mut variants = variants.to_vec();
    variants.sort_by_key(|&(number, _)| number);
    let enumerants = variants
        .iter()
        .map(|(number, name)| quote! { #name = #number });

    // Each item is a tuple indicating an inclusive range as opposed to an exclusive range like is
    // common.
    let mut number_runs = vec![(variants[0].0, variants[0].0)];
    for &(number, _) in variants.iter().skip(1) {
        let last_run = number_runs.last_mut().unwrap();
        match number.cmp(&(last_run.1 + 1)) {
            Ordering::Equal => last_run.1 = number,
            Ordering::Greater => number_runs.push((number, number)),
            Ordering::Less => unreachable!("Variants not sorted by discriminant"),
        }
    }

    // We try to check if the given number is within a run of valid discriminants and if so
    // transmute the number directly to the enum type.
    let from_prim = number_runs
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

    let attribute = value_enum_attribute();
    quote! {
        #[doc = #comment]
        #attribute
        pub enum #enum_name {
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
    let mut seen_discriminator = BTreeMap::new();
    let mut variants = vec![];
    let mut aliases = vec![];
    let mut capability_clauses = BTreeMap::new();
    let mut extension_clauses = BTreeMap::new();
    let mut operand_clauses = BTreeMap::new();
    let mut from_str_impl = vec![];
    for e in &grammar.enumerants {
        if let Some(discriminator) = seen_discriminator.get(&e.value) {
            let name_str = &e.symbol;
            let symbol = as_ident(&e.symbol);
            aliases.push(quote! {
                pub const #symbol: Self = Self::#discriminator;
            });
            from_str_impl.push(quote! { #name_str => Ok(Self::#discriminator), });
        } else {
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
            seen_discriminator.insert(e.value, name.clone());
            variants.push((number, name.clone()));
            from_str_impl.push(quote! { #name_str => Ok(Self::#name), });

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
    }

    let the_enum = generate_enum(
        &kind,
        &variants,
        format!("SPIR-V operand kind: {}", get_spec_link(&grammar.kind)),
    );

    quote! {
        #the_enum

        #[allow(non_upper_case_globals)]
        impl #kind {
            #(#aliases)*
        }

        impl core::str::FromStr for #kind {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#from_str_impl)*
                    _ => Err(()),
                }
            }
        }
    }
}

/// Returns the code defining the enum for an operand kind by parsing
/// the given SPIR-V `grammar`.
fn gen_operand_kind(grammar: &structs::OperandKind) -> Option<TokenStream> {
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
    let mut seen_discriminator = BTreeMap::new();
    let mut aliases = vec![];
    let mut variants = vec![];

    // Get the instruction table.
    for inst in &grammar.instructions {
        // Omit the "Op" prefix.
        let opname = as_ident(&inst.opname[2..]);
        let opcode = inst.opcode;
        if let Some(discriminator) = seen_discriminator.get(&opcode) {
            aliases.push(quote! { pub const #opname : Op = Op::#discriminator; });
        } else {
            variants.push((opcode, opname.clone()));
            seen_discriminator.insert(opcode, opname.clone());
        }
    }

    let the_enum = generate_enum(
        &as_ident("Op"),
        &variants,
        format!("SPIR-V {} opcodes", get_spec_link("instructions")),
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

/// Returns the GLSL.std.450 extended instruction opcodes.
pub fn gen_glsl_std_450_opcodes(grammar: &structs::ExtInstSetGrammar) -> TokenStream {
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

    generate_enum(
        &as_ident("GLOp"),
        &variants,
        format!(
            "[GLSL.std.450]({}) extended instruction opcode",
            GLSL_STD_450_SPEC_LINK
        ),
    )
}

/// Returns the OpenCL.std extended instruction opcodes.
pub fn gen_opencl_std_opcodes(grammar: &structs::ExtInstSetGrammar) -> TokenStream {
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

    generate_enum(
        &as_ident("CLOp"),
        &variants,
        format!(
            "[OpenCL.std]({}) extended instruction opcode",
            OPENCL_STD_SPEC_LINK
        ),
    )
}
