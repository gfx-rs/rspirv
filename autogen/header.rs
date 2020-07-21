use crate::structs;
use crate::utils::*;

use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

static GLSL_STD_450_SPEC_LINK: &'static str = "\
https://www.khronos.org/registry/spir-v/specs/unified1/GLSL.std.450.html";

static OPENCL_STD_SPEC_LINK: &'static str = "\
https://www.khronos.org/registry/spir-v/specs/unified1/OpenCL.ExtendedInstructionSet.100.html";

/// Returns the markdown string containing a link to the spec for the given
/// operand `kind`.
fn get_spec_link(kind: &str) -> String {
    let symbol = kind.to_snake_case();
    format!(
        "[{text}]({link})",
        text = kind,
        link = format!(
            "https://www.khronos.org/registry/spir-v/\
                            specs/unified1/SPIRV.html#_a_id_{}_a_{}",
            symbol, symbol
        )
    )
}

fn value_enum_attribute() -> TokenStream {
    quote! {
        #[repr(u32)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
        #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    }
}

fn bit_enum_attribute() -> TokenStream {
    quote! {
        #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
        #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    }
}

fn from_primitive_impl(from_prim: &[TokenStream], kind: &proc_macro2::Ident) -> TokenStream {
    quote! {
        impl num_traits::FromPrimitive for #kind {
            #[allow(trivial_numeric_casts)]
            fn from_i64(n: i64) -> Option<Self> {
                Some(match n as u32 {
                    #(#from_prim,)*
                    _ => return None
                })
            }

            fn from_u64(n: u64) -> Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    }
}

fn gen_bit_enum_operand_kind(grammar: &structs::OperandKind) -> TokenStream {
    let elements = grammar.enumerants.iter().map(|enumerant| {
        // Special treatment for "NaN"
        let symbol = as_ident(
            &enumerant
                .symbol
                .to_shouty_snake_case()
                .replace("NA_N", "NAN"),
        );
        let value = enumerant.value;
        quote! {
            const #symbol = #value;
        }
    });
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
    let mut enumerants = vec![];
    let mut from_prim_list = vec![];
    let mut aliases = vec![];
    let mut capability_clauses = BTreeMap::new();
    for e in &grammar.enumerants {
        if let Some(discriminator) = seen_discriminator.get(&e.value) {
            let symbol = as_ident(&e.symbol);
            aliases.push(quote! {
                pub const #symbol: #kind = #kind::#discriminator;
            });
        } else {
            // Special case for Dim. Its enumerants can start with a digit.
            // So prefix with the kind name here.
            let name = if grammar.kind == "Dim" {
                let mut name = "Dim".to_string();
                name.push_str(&e.symbol);
                name
            } else {
                e.symbol.to_string()
            };
            let name = as_ident(&name);
            let number = e.value;
            seen_discriminator.insert(e.value, name.clone());
            enumerants.push(quote! { #name = #number });
            from_prim_list.push(quote! { #number => #kind::#name });

            capability_clauses
                .entry(&e.capabilities)
                .or_insert_with(Vec::new)
                .push(name);
        }
    }

    let capabilities = capability_clauses.into_iter().map(|(k, v)| {
        let kinds = std::iter::repeat(&kind);
        let capabilities = k.into_iter().map(|cap| as_ident(cap));
        quote! {
            #( #kinds::#v )|* => &[#( Capability::#capabilities ),*]
        }
    });

    let comment = format!("/// SPIR-V operand kind: {}", get_spec_link(&grammar.kind));
    let attribute = value_enum_attribute();

    let from_prim_impl = from_primitive_impl(&from_prim_list, &kind);

    quote! {
        #[doc = #comment]
        #attribute
        pub enum #kind {
            #(#enumerants),*
        }

        #[allow(non_upper_case_globals)]
        impl #kind {
            #(#aliases)*

            pub fn required_capabilities(self) -> &'static [Capability] {
                match self {
                    #(#capabilities),*
                }
            }
        }

        #from_prim_impl
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
    let mut opcodes = vec![];
    let mut aliases = vec![];
    let mut from_prim_list = vec![];

    // Get the instruction table.
    for inst in &grammar.instructions {
        // Omit the "Op" prefix.
        let opname = as_ident(&inst.opname[2..]);
        let opcode = inst.opcode;
        if let Some(discriminator) = seen_discriminator.get(&opcode) {
            aliases.push(quote! { pub const #opname : Op = Op::#discriminator; });
        } else {
            opcodes.push(quote! { #opname = #opcode });
            from_prim_list.push(quote! { #opcode => Op::#opname });
            seen_discriminator.insert(opcode, opname.clone());
        }
    }

    let comment = format!("SPIR-V {} opcodes", get_spec_link("instructions"));
    let attribute = value_enum_attribute();
    let from_prim_impl = from_primitive_impl(&from_prim_list, &as_ident("Op"));

    quote! {
        pub type Word = u32;
        pub const MAGIC_NUMBER: u32 = #magic_number;
        pub const MAJOR_VERSION: u8 = #major_version;
        pub const MINOR_VERSION: u8 = #minor_version;
        pub const REVISION: u8 = #revision;

        #(#kinds)*

        #[doc = #comment]
        #attribute
        pub enum Op {
            #(#opcodes),*
        }

        #[allow(non_upper_case_globals)]
        impl Op {
            #(#aliases)*
        }

        #from_prim_impl
    }
}

/// Returns the GLSL.std.450 extended instruction opcodes.
pub fn gen_glsl_std_450_opcodes(grammar: &structs::ExtInstSetGrammar) -> TokenStream {
    // Get the instruction table.
    let opcodes = grammar.instructions.iter().map(|inst| {
        // Omit the "Op" prefix.
        let opname = as_ident(&inst.opname);
        let opcode = inst.opcode;
        quote! { #opname = #opcode }
    });

    let from_prim_list = grammar
        .instructions
        .iter()
        .map(|inst| {
            let opname = as_ident(&inst.opname);
            let opcode = inst.opcode;
            quote! { #opcode => GLOp::#opname }
        })
        .collect::<Vec<_>>();

    let comment = format!(
        "[GLSL.std.450]({}) extended instruction opcode",
        GLSL_STD_450_SPEC_LINK
    );
    let attribute = value_enum_attribute();
    let from_prim_impl = from_primitive_impl(&from_prim_list, &as_ident("GLOp"));

    quote! {
        #[doc = #comment]
        #attribute
        pub enum GLOp {
            #(#opcodes),*
        }

        #from_prim_impl
    }
}

/// Returns the OpenCL.std extended instruction opcodes.
pub fn gen_opencl_std_opcodes(grammar: &structs::ExtInstSetGrammar) -> TokenStream {
    // Get the instruction table.
    let opcodes = grammar.instructions.iter().map(|inst| {
        // Omit the "Op" prefix.
        let opname = as_ident(&inst.opname);
        let opcode = inst.opcode;
        quote! { #opname = #opcode }
    });

    let from_prim_list = grammar
        .instructions
        .iter()
        .map(|inst| {
            let opname = as_ident(&inst.opname);
            let opcode = inst.opcode;
            quote! { #opcode => CLOp::#opname }
        })
        .collect::<Vec<_>>();

    let comment = format!(
        "[OpenCL.std]({}) extended instruction opcode",
        OPENCL_STD_SPEC_LINK
    );
    let attribute = value_enum_attribute();
    let from_prim_impl = from_primitive_impl(&from_prim_list, &as_ident("CLOp"));

    quote! {
        #[doc = #comment]
        #attribute
        pub enum CLOp {
            #(#opcodes),*
        }

        #from_prim_impl
    }
}
