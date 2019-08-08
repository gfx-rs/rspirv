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

use proc_macro2::TokenStream;
use quote::quote;

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

fn value_enum_attribute() -> TokenStream {
    quote! {
        #[repr(u32)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
    }
}

fn gen_bit_enum_operand_kind(grammar: &structs::OperandKind) -> TokenStream {
    let elements = grammar.enumerants.iter().map(|enumerant| {
        // Special treatment for "NaN"
        let symbol = as_ident(&snake_casify(&enumerant.symbol).replace("na_n", "nan").to_uppercase());
        // value.string is a hex string of the form 0x12345678
        let value = u32::from_str_radix(&enumerant.value.string[2..], 16).unwrap();
        quote! {
            const #symbol = #value;
        }
    });
    let comment = format!("SPIR-V operand kind: {}", get_spec_link(&grammar.kind));
    let kind = as_ident(&grammar.kind);
    quote! {
        bitflags! {
            #[doc = #comment]
            pub struct #kind: u32 {
                #(#elements)*
            }
        }
    }
}

fn gen_value_enum_operand_kind(grammar: &structs::OperandKind) -> TokenStream {
    use std::collections::BTreeMap;

    let kind = as_ident(&grammar.kind);

    // We can have more than one enumerants mapping to the same discriminator.
    // Use associated constants for these aliases.
    let mut seen_discriminator = BTreeMap::new();
    let mut enumerants = vec![];
    let mut aliases = vec![];
    let mut capability_clauses = BTreeMap::new();
    for e in &grammar.enumerants {
        if let Some(discriminator) = seen_discriminator.get(&e.value.number) {
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
            let number = e.value.number;
            seen_discriminator.insert(e.value.number, name.clone());
            enumerants.push(quote! { #name = #number });
            capability_clauses.entry(&e.capabilities).or_insert_with(Vec::new).push(name);
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

    quote! {
        #[doc = #comment]
        #attribute
        pub enum #kind {
            #(#enumerants),*
        }

        #[allow(non_upper_case_globals)]
        impl #kind {
            #(#aliases)*

            pub fn required_capabilities(&self) -> &'static [Capability] {
                match self {
                    #(#capabilities),*
                }
            }
        }
    }
}

/// Returns the code defining the enum for an operand kind by parsing
/// the given SPIR-V `grammar`.
fn gen_operand_kind(grammar: &structs::OperandKind) -> Option<TokenStream> {
    if grammar.category == "BitEnum" {
        Some(gen_bit_enum_operand_kind(grammar))
    } else if grammar.category == "ValueEnum" {
        Some(gen_value_enum_operand_kind(grammar))
    } else {
        None
    }
}

/// Returns the generated SPIR-V header.
pub fn gen_spirv_header(grammar: &structs::Grammar) -> TokenStream {
    // constants and types.
    let magic_number = u32::from_str_radix(&grammar.magic_number[2..], 16).expect("Magic number not a u32");
    let major_version = grammar.major_version;
    let minor_version = grammar.minor_version;
    let revision = grammar.revision;

    // Operand kinds.
    let kinds = grammar.operand_kinds.iter().filter_map(gen_operand_kind);

    // Opcodes.
    // Get the instruction table.
    let opcodes = grammar.instructions.iter().map(|inst| {
        // Omit the "Op" prefix.
        let opname = as_ident(&inst.opname[2..]);
        let opcode = inst.opcode;
        quote! { #opname = #opcode }
    });
    let comment = format!("SPIR-V {} opcodes", get_spec_link("instructions"));
    let attribute = value_enum_attribute();

    quote! {
        pub type Word = u32;
        pub const MAGIC_NUMBER: u32 = #magic_number;
        pub const MAJOR_VERSION: u32 = #major_version;
        pub const MINOR_VERSION: u32 = #minor_version;
        pub const REVISION: u32 = #revision;

        #(#kinds)*
        
        #[doc = #comment]
        #attribute
        pub enum Op {
            #(#opcodes),*
        }
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
    let comment = format!("[GLSL.std.450]({}) extended instruction opcode", GLSL_STD_450_SPEC_LINK);
    let attribute = value_enum_attribute();

    quote! {
        #[doc = #comment]
        #attribute
        pub enum GLOp {
            #(#opcodes),*
        }
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
    let comment = format!("[OpenCL.std]({}) extended instruction opcode", OPENCL_STD_SPEC_LINK);
    let attribute = value_enum_attribute();

    quote! {
        #[doc = #comment]
        #attribute
        pub enum CLOp {
            #(#opcodes),*
        }
    }
}
