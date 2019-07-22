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

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Returns the corresponding Rust type used in structured representation
/// for the given operand kind in the SPIR-V JSON grammar.
pub fn get_operand_type_sr_tokens(kind: &str) -> TokenStream {
    match kind {
        "IdMemorySemantics" | "IdScope" | "IdRef" | "IdResult" => quote! { spirv::Word },
        "LiteralInteger" | "LiteralExtInstInteger" => quote! { u32 },
        "LiteralSpecConstantOpInteger" => quote! { spirv::Op },
        "LiteralContextDependentNumber" => panic!("this kind is not expected to be handled here"),
        "LiteralString" => quote! { String },
        "PairLiteralIntegerIdRef" => quote! { (u32, spirv::Word) },
        "PairIdRefLiteralInteger" => quote! { (spirv::Word, u32) },
        "PairIdRefIdRef" => quote! { (spirv::Word, spirv::Word) },
        _ => {
            let kind = Ident::new(kind, Span::call_site());
            quote! { spirv::#kind }
        }
    }
}

/// Returns the corresponding Rust name used in structured representation
/// for the given operand kind in the SPIR-V JSON grammar.
pub fn get_operand_name_sr_tokens(param: &structs::Operand) -> TokenStream {
    if param.name.is_empty() {
        if param.kind == "IdResultType" {
            quote! { result_type }
        } else {
            let name = snake_casify(&param.kind);
            let token = Ident::new(&name, Span::call_site());
            quote! { #token }
        }
    } else {
        let re = regex::Regex::new(r"\W").unwrap();
        let name = snake_casify(&re.replace_all(&param.name.replace(" ", "_"), ""));
        let token = Ident::new(&name, Span::call_site());
        quote! { #token }
    }
}

pub fn gen_sr_decoration(grammar: &structs::Grammar) -> String {
    // The decoration operand kind
    let decoration = grammar
        .operand_kinds
        .iter()
        .find(|k| k.kind == "Decoration")
        .unwrap();
    // Go and compose all its enumerants
    let enumerants: Vec<_> = decoration
        .enumerants
        .iter()
        .map(|enumerant| {
            // Parameters for this enumerant
            let types: Vec<_> = enumerant
                .parameters
                .iter()
                .map(|p| get_operand_type_sr_tokens(&p.kind))
                .collect();
            let params = if types.is_empty() {
                quote!{}
            } else {
                quote! { (#( #types ),*) }
            };
            let symbol = Ident::new(enumerant.symbol.as_str(), Span::call_site());
            quote! { #symbol #params }
        })
        .collect();
    let tokens = quote! {
        use derive_more::From;
        use spirv;

        /// SPIR-V decorations.
        #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, From)]
        pub enum Decoration {
            #( #enumerants ),*
        }
    };
    tokens.to_string()
}

pub fn get_quantified_type_tokens(ty: TokenStream, quantifier: &str) -> TokenStream {
    match quantifier {
        "" => quote! { #ty },
        "?" => quote! { Option<#ty> },
        "*" => quote! { Vec<#ty> },
        other => panic!("wrong quantifier: {}", other),
    }
}

pub fn get_operand_type_ident(operand: &structs::Operand) -> TokenStream {
    let ty = if operand.kind == "IdRef" {
        if operand.name == "'Length'" {
            quote! { Token<Constant> }
        } else {
            quote! { Token<Type> }
        }
    } else {
        get_operand_type_sr_tokens(&operand.kind)
    };

    get_quantified_type_tokens(ty, &operand.quantifier)
}

fn get_type_fn_name(name: &str) -> String {
    if name == "Struct" {
        "structure".to_string()
    } else {
        snake_casify(name)
    }
}

pub fn gen_sr_type_check(grammar: &structs::Grammar) -> String {
    // Collect all types and their parameters in the following format:
    //   (type-name: &str, Vec<(param-name: quote::Ident, param-type: quote::Ident)>)
    let cases: Vec<_> = grammar
        .instructions
        .iter()
        .filter(|k| k.class == "Type")
        .map(|kind| {
            let operands: Vec<_> = kind
                .operands
                .iter()
                .skip(1)
                .map(|op| {
                    let name = Ident::new(&get_param_name(op), Span::call_site());
                    let ty = get_operand_type_ident(op);
                    (name, ty)
                })
                .collect();
            let symbol = &kind.opname[6..];
            (symbol, operands)
        })
        .collect();
    let types: Vec<_> = cases
        .iter()
        .map(|&(symbol, ref params)| {
            let symbol = Ident::new(symbol, Span::call_site());
            let param_list: Vec<_> = params
                .iter()
                .map(|&(ref name, ref ty)| {
                    // structures support per-member decorations
                    if symbol == "Struct" {
                        quote! { #name : Vec<StructMember> }
                    } else {
                        quote! { #name : #ty }
                    }
                })
                .collect();
            let param_list = if param_list.is_empty() {
                quote!{}
            } else {
                quote! { { #( #param_list ),* } }
            };
            quote! { #symbol #param_list }
        })
        .collect();
    let checks: Vec<_> = cases
        .iter()
        .map(|&(symbol, ref params)| {
            let func_name = Ident::new(
                &format!("is_{}_type", get_type_fn_name(symbol)),
                Span::call_site(),
            );
            let symbol = Ident::new(symbol, Span::call_site());
            // If the type requires parameters, attach `{ .. }` to the match arm.
            let params = if params.is_empty() {
                quote!{}
            } else {
                quote! { {..} }
            };
            quote! {
                pub fn #func_name(&self) -> bool {
                    match self.ty {
                        TypeEnum::#symbol #params => true,
                        _ => false
                    }
                }
            }
        })
        .collect();
    let tokens = quote! {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub (in crate::sr) enum TypeEnum {
            #( #types ),*
        }

        impl Type {
            #( #checks )*
        }
    };
    tokens.to_string()
}

pub fn gen_sr_type_creation(grammar: &structs::Grammar) -> String {
    // Collect all types and their parameters in the following format:
    //   (type-name: &str, Vec<(param-name: quote::Ident, param-type: quote::Ident)>)
    let cases: Vec<_> = grammar
        .instructions
        .iter()
        .filter(|k| k.class == "Type")
        .filter(|k| k.opname != "OpTypeStruct")
        .map(|kind| {
            let operands: Vec<_> = kind
                .operands
                .iter()
                .skip(1)
                .map(|op| {
                    let name = Ident::new(&get_param_name(op), Span::call_site());
                    let ty = get_operand_type_ident(op);
                    (name, ty)
                })
                .collect();
            let symbol = &kind.opname[6..];
            (symbol, operands)
        })
        .collect();
    let constructors: Vec<_> = cases
        .iter()
        .map(|&(symbol, ref params)| {
            let func_name = Ident::new(
                &format!("type_{}", get_type_fn_name(symbol)),
                Span::call_site(),
            );
            let symbol = Ident::new(symbol, Span::call_site());
            let param_list: Vec<_> = params
                .iter()
                .map(|&(ref name, ref ty)| {
                    quote! { #name : #ty }
                })
                .collect();
            let param_list = quote! { (&mut self, #( #param_list ),*) };
            let init_list: Vec<_> = params
                .iter()
                .map(|&(ref name, _)| {
                    quote! { #name : #name }
                })
                .collect();
            let init_list = if init_list.is_empty() {
                quote!{}
            } else {
                quote! { {#( #init_list ),*} }
            };
            quote! {
                pub fn #func_name #param_list -> Token<Type> {
                    self.types.fetch_or_append(&mut self.max_index, Type {
                        ty: TypeEnum::#symbol #init_list,
                        decorations: Vec::new(),
                    })
                }
            }
        })
        .collect();
    let tokens = quote! {
        impl Context {
            #( #constructors )*
        }
    };
    tokens.to_string()
}

pub fn gen_sr_instruction(grammar: &structs::Grammar) -> String {
    // Compose the token stream for all instructions
    let insts: Vec<_> = grammar
        .instructions
        .iter() // Loop over all instructions
        .filter(|i| i.class != "Type") // Skip types
        .filter(|i| i.class != "Constant") // Skip constants
        .map(|inst| {
            // Get the token for its enumerant
            let name = Ident::new(&inst.opname[2..], Span::call_site());

            // Compose the token stream for all parameters
            let params: Vec<_> = inst.operands
                .iter() // Loop over all parameters
                .filter_map(|operand| {
                    if operand.kind.starts_with("IdResult") {
                        None
                    } else {
                        let field_name = get_operand_name_sr_tokens(operand);
                        let field_type = get_operand_type_sr_tokens(&operand.kind);
                        let quantified = get_quantified_type_tokens(field_type, &operand.quantifier);
                        Some(quote! { #field_name : #quantified })
                    }
                }).collect();
            let params = if params.is_empty() {
                quote!{}
            } else {
                // Create a list parameter tokens separated by comma
                quote! { {#( #params ),*} }
            };

            // Get token stream for this enumerant
            quote! { #name #params }
        }).collect();

    // Wrap it up with enum definition boilerplate
    let insts = quote! {
        enum Instruction {
            #( #insts ),*
        }
    };
    insts.to_string()
}
