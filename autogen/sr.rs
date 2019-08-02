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
pub fn get_operand_name_ident(param: &structs::Operand) -> Ident {
    let name = get_param_name(param);
    Ident::new(&name, Span::call_site())
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
        match operand.name.trim_matches('\'') {
            "Length" => quote! { Token<Constant> },
            _ => quote! { Token<Type> },
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

pub struct CodeGeneratedFromOperandKindGrammar {
    pub decoration: String,
}

pub fn gen_sr_code_from_operand_kind_grammar(
    grammar_operand_kinds: &[structs::OperandKind],
) -> CodeGeneratedFromOperandKindGrammar {
    // The decoration operand kind
    let decoration = grammar_operand_kinds
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

    CodeGeneratedFromOperandKindGrammar {
        decoration: tokens.to_string(),
    }
}

pub struct CodeGeneratedFromInstructionGrammar {
    pub type_enums: String,
    pub type_creation: String,
    pub instruction_structs: String,
    pub instruction_enums: String,
}

pub fn gen_sr_code_from_instruction_grammar(
    grammar_instructions: &[structs::Instruction],
) -> CodeGeneratedFromInstructionGrammar {
    let mut inst_structs = Vec::new();
    let mut inst_variants = Vec::new();
    let mut terminators = Vec::new();
    let mut type_variants = Vec::new();
    let mut type_checks = Vec::new();
    let mut type_constructors = Vec::new();

    // Compose the token stream for all instructions
    for inst in grammar_instructions
        .iter() // Loop over all instructions
        .filter(|i| i.class != "Constant") // Skip constants
    {
        // Get the token for its enumerant
        let name = Ident::new(&inst.opname[2..], Span::call_site());

        // Compose the token stream for all parameters
        let params: Vec<_> = inst.operands
            .iter() // Loop over all parameters
            .filter_map(|operand| {
                if operand.kind.starts_with("IdResult") {
                    None
                } else {
                    let field_name = get_operand_name_ident(operand);
                    let field_type = get_operand_type_sr_tokens(&operand.kind);
                    let quantified = get_quantified_type_tokens(field_type, &operand.quantifier);
                    Some(quote! { #field_name : #quantified })
                }
            }).collect();
        let type_operands = inst.operands
            .iter()
            .skip(1)
            .map(|op| {
                let name = Ident::new(&get_param_name(op), Span::call_site());
                let ty = get_operand_type_ident(op);
                (name, ty)
            });

        match inst.class.as_str() {
            "Type" => {
                let type_name = &inst.opname[6..];
                let func_name = Ident::new(
                    &format!("type_{}", get_type_fn_name(type_name)),
                    Span::call_site(),
                );
                let symbol = Ident::new(type_name, Span::call_site());

                let param_list: Vec<_> = type_operands
                    .clone()
                    .map(|(ref name, ref ty)| {
                        // structures support per-member decorations
                        if symbol == "Struct" {
                            quote! { #name : Vec<StructMember> }
                        } else {
                            quote! { #name : #ty }
                        }
                    })
                    .collect();
                let param_list = param_list.as_slice();
                type_variants.push(if param_list.is_empty() {
                    quote!{ #symbol }
                } else {
                    quote! { #symbol { #( #param_list ),* } }
                });

                let generate_type_check = true;
                if generate_type_check {
                    let func_name = Ident::new(
                        &format!("is_{}_type", get_type_fn_name(type_name)),
                        Span::call_site(),
                    );
                    // If the type requires parameters, attach `{ .. }` to the match arm.
                    let check_params = if param_list.is_empty() {
                        quote!{}
                    } else {
                        quote! { {..} }
                    };
                    type_checks.push(quote! {
                        pub fn #func_name(&self) -> bool {
                            match self.ty {
                                TypeEnum::#symbol #check_params => true,
                                _ => false
                            }
                        }
                    });
                }

                if inst.opname != "OpTypeStruct" {
                    let init_list: Vec<_> = type_operands
                        .map(|(ref name, _)| {
                            quote! { #name }
                        })
                        .collect();
                    let init_list = if init_list.is_empty() {
                        quote!{}
                    } else {
                        quote! { {#( #init_list ),*} }
                    };
                    type_constructors.push(quote! {
                        pub fn #func_name(&mut self, #( #param_list ),*) -> Token<Type> {
                            self.types.fetch_or_append(Type {
                                ty: TypeEnum::#symbol #init_list,
                                decorations: Vec::new(),
                            })
                        }
                    });
                }
            }
            "ModeSetting" |
            "ExtensionDecl" |
            "FunctionStruct" => {
                // Create a standalone struct
                inst_structs.push(quote! {
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct #name {
                        #( #params ),*
                    }
                })
            }
            "Terminator" => {
                terminators.push(quote! {
                    #name {#( #params ),*}
                });
            }
            _ => {
                inst_variants.push(if params.is_empty() {
                    quote!{ #name }
                } else {
                    quote! { #name {#( #params ),*} }
                });
            }
        }
    }

    let inst_structs = quote! {
        #( #inst_structs )*
    };
    let inst_enums = quote! {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Terminator {
            #( #terminators ),*
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Instruction {
            #( #inst_variants ),*
        }
    };
    let type_enums = quote! {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub(in crate::sr) enum TypeEnum {
            #( #type_variants ),*
        }

        impl Type {
            #( #type_checks )*
        }
    };
    let type_creation = quote! {
        impl Context {
            #( #type_constructors )*
        }
    };

    CodeGeneratedFromInstructionGrammar {
        type_enums: type_enums.to_string(),
        type_creation: type_creation.to_string(),
        instruction_structs: inst_structs.to_string(),
        instruction_enums: inst_enums.to_string(),   
    }
}

