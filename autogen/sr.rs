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

struct OperandTokens {
    /// Rust name used in structured representation
    name: Ident,
    /// Rust type used in structured representation.
    quantified_type: TokenStream,
}

impl OperandTokens {
    fn new(operand: &structs::Operand) -> Self {
        let name = get_param_name(operand);

        let ty = match operand.kind.as_str() {
            "IdRef" => match operand.name.trim_matches('\'') {
                "Length" => quote! { Token<Constant> },
                "Field Types" => quote! { StructMember },
                "Parameter Types" => quote! { Token<Type> },
                name if name.ends_with(" Type") => quote! { Token<Type> },
                _ => quote! { spirv::Word },
            },
            "IdMemorySemantics" | "IdScope" | "IdResult" => quote! { spirv::Word },
            "LiteralInteger" | "LiteralExtInstInteger" => quote! { u32 },
            "LiteralSpecConstantOpInteger" => quote! { spirv::Op },
            "LiteralContextDependentNumber" => panic!("this kind is not expected to be handled here"),
            "LiteralString" => quote! { String },
            "PairLiteralIntegerIdRef" => quote! { (u32, spirv::Word) },
            "PairIdRefLiteralInteger" => quote! { (spirv::Word, u32) },
            "PairIdRefIdRef" => quote! { (spirv::Word, spirv::Word) },
            kind => {
                let kind = Ident::new(kind, Span::call_site());
                quote! { spirv::#kind }
            }
        };

        OperandTokens {
            name,
            quantified_type: match operand.quantifier {
                structs::Quantifier::One => ty,
                structs::Quantifier::ZeroOrOne => quote! { Option<#ty> },
                structs::Quantifier::ZeroOrMore => quote! { Vec<#ty> },
            },
        }
    }
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
                .map(|p| OperandTokens::new(p).quantified_type)
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
    use structs::Class::*;

    let mut inst_structs = Vec::new();
    let mut inst_variants = Vec::new();
    let mut terminators = Vec::new();
    let mut type_variants = Vec::new();
    let mut type_checks = Vec::new();
    let mut type_constructors = Vec::new();

    let mut field_names = Vec::new();
    let mut field_types = Vec::new();

    // Compose the token stream for all instructions
    for inst in grammar_instructions
        .iter() // Loop over all instructions
        .filter(|i| i.class != Some(structs::Class::Constant)) // Skip constants
    {
        // Get the token for its enumerant
        let name = Ident::new(&inst.opname[2..], Span::call_site());

        // Re-use the allocation between iterations of the loop
        field_names.clear();
        field_types.clear();

        // Compose the token stream for all parameters
        for operand in inst.operands.iter() {
            if operand.kind.starts_with("IdResult") {
                continue
            }
            let tokens = OperandTokens::new(operand);
            field_names.push(tokens.name);
            field_types.push(tokens.quantified_type);
        }
        let field_names = field_names.as_slice();
        let field_types = field_types.as_slice();

        match inst.class {
            Some(structs::Class::Type) => {
                let type_name = &inst.opname[6..];
                let symbol = Ident::new(type_name, Span::call_site());

                type_variants.push(if field_names.is_empty() {
                    quote!{ #symbol }
                } else {
                    quote! { #symbol {
                        #( #field_names: #field_types ),*
                    }}
                });

                let generate_type_check = true;
                if generate_type_check {
                    let func_name_ident = Ident::new(
                        &format!("is_{}_type", get_type_fn_name(type_name)),
                        Span::call_site(),
                    );
                    // If the type requires parameters, attach `{ .. }` to the match arm.
                    let check_params = if field_names.is_empty() {
                        quote!{}
                    } else {
                        quote! { {..} }
                    };
                    type_checks.push(quote! {
                        pub fn #func_name_ident(&self) -> bool {
                            match self.ty {
                                TypeEnum::#symbol #check_params => true,
                                _ => false
                            }
                        }
                    });
                }

                if inst.opname != "OpTypeStruct" {
                    let func_name_ident = Ident::new(
                        &format!("type_{}", get_type_fn_name(type_name)),
                        Span::call_site(),
                    );
                    let init_list = if field_names.is_empty() {
                        quote!{}
                    } else {
                        quote! { {#( #field_names ),*} }
                    };
                    type_constructors.push(quote! {
                        pub fn #func_name_ident(
                            &mut self, #( #field_names: #field_types ),*
                        ) -> Token<Type> {
                            self.types.fetch_or_append(Type {
                                ty: TypeEnum::#symbol #init_list,
                                decorations: Vec::new(),
                            })
                        }
                    });
                }
            }
            Some(ModeSetting) |
            Some(ExtensionDecl) |
            Some(FunctionStruct) => {
                // Create a standalone struct
                inst_structs.push(quote! {
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct #name {
                        #( #field_names: #field_types ),*
                    }
                })
            }
            Some(Terminator) => {
                terminators.push(quote! {
                    #name {#( #field_names: #field_types ),*}
                });
            }
            _ => {
                inst_variants.push(if field_names.is_empty() {
                    quote!{ #name }
                } else {
                    quote! { #name {
                        #( #field_names: #field_types ),*
                    }}
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

