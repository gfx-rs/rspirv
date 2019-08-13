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

use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// The name of a local variable in the generated code that
/// represents an iterator over instruction operands.
const OPERAND_ITER: &str = "operands";

struct OperandTokens {
    /// Rust name used in structured representation
    name: Ident,
    /// Rust type used in structured representation.
    quantified_type: TokenStream,
    /// Rust expression that initializes the structured representation
    /// of an operand based on the data representation.
    lift_expression: TokenStream,
}

impl OperandTokens {
    fn new(operand: &structs::Operand) -> Self {
        let name = get_param_name(operand);
        let iter = Ident::new(OPERAND_ITER, Span::call_site());

        let (ty, lift_value) = match operand.kind.as_str() {
            "IdRef" => match operand.name.trim_matches('\'') {
                "Length" => (
                    quote! { Token<Constant> },
                    quote! { self.constants.lookup(*value).unwrap() },
                ),
                "Field Types" => (
                    quote! { StructMember },
                    quote! { super::types::StructMember::new(value.clone()) },
                ),
                "Parameter Types" => (
                    quote! { Token<Type> },
                    quote! { self.types.lookup(*value).unwrap() },
                ),
                name if name.ends_with(" Type") => (
                    quote! { Token<Type> },
                    quote! { self.types.lookup(*value).unwrap() },
                ),
                _ => (
                    quote! { spirv::Word },
                    quote! { *value },
                ),
            },
            "IdMemorySemantics" | "IdScope" | "IdResult" => (
                quote! { spirv::Word },
                quote! { *value },
            ),
            "LiteralInteger" | "LiteralExtInstInteger" => (
                quote! { u32 },
                quote! { *value },
            ),
            "LiteralSpecConstantOpInteger" => (
                quote! { spirv::Op },
                quote! { *value },
            ),
            "LiteralContextDependentNumber" => panic!("this kind is not expected to be handled here"),
            "LiteralString" => (
                quote! { String },
                quote! { value.clone() },
            ),
            "PairLiteralIntegerIdRef" => (
                quote! { (u32, spirv::Word) },
                quote! {
                   match (#iter.next(), #iter.next()) {
                       (Some(&dr::Operand::LiteralInt32(value)), Some(&dr::Operand::IdRef(id))) => Some((value, Token::new(id))),
                       (None, None) => None,
                       _ => Err(OperandError::Wrong)?,
                   }
               },
            ),
            "PairIdRefLiteralInteger" => (
                quote! { (spirv::Word, u32) },
                quote! {
                    match (#iter.next(), #iter.next()) {
                        (Some(&dr::Operand::IdRef(id)), Some(&dr::Operand::LiteralInt32(value))) => Some((Token::new(id), value)),
                        (None, None) => None,
                        _ => Err(OperandError::Wrong)?,
                    }
                },
            ),
            "PairIdRefIdRef" => (
                quote! { (spirv::Word, spirv::Word) },
                quote! {
                    match (#iter.next(), #iter.next()) {
                        (Some(&dr::Operand::IdRef(id1)), Some(&dr::Operand::IdRef(id2))) => Some((Token::new(id1), Token::new(id2))),
                        (None, None) => None,
                        _ => Err(OperandError::Wrong)?,
                    }
                },
            ),
            kind => {
                let kind = Ident::new(kind, Span::call_site());
                (
                    quote! { spirv::#kind },
                    quote! { *value },
                )
            }
        };

        let kind_ident = Ident::new(&operand.kind, Span::call_site());
        let lift = quote! {
            match #iter.next() {
                Some(&dr::Operand::#kind_ident(ref value)) => Some(#lift_value),
                Some(_) => Err(OperandError::Wrong)?,
                None => None,
            }
        };

        let (quantified_type, lift_expression) = match operand.quantifier.as_str() {
            structs::Quantifier::One => (
                ty,
                quote! {
                    (#lift).ok_or(OperandError::Missing)?
                },
            ),
            structs::Quantifier::ZeroOrOne => (
                quote! { Option<#ty> },
                lift
            ),
            structs::Quantifier::ZeroOrMore => (
                quote! { Vec<#ty> },
                quote! {{
                    let mut vec = Vec::new();
                    while let Some(value) = #lift {
                        vec.push(value);
                    }
                    vec
                }},
            ),
        };

        OperandTokens {
            name,
            quantified_type,
            lift_expression,
        }
    }
}

fn get_type_fn_name(name: &str) -> String {
    if name == "Struct" {
        "structure".to_string()
    } else {
        name.to_snake_case()
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
    pub types: String,
    pub instructions: String,
    pub ops: String,
    pub context_logic: String,
}

pub fn gen_sr_code_from_instruction_grammar(
    grammar_instructions: &[structs::Instruction],
) -> CodeGeneratedFromInstructionGrammar {
    use structs::Class::*;

    let mut inst_structs = Vec::new();
    let mut op_variants = Vec::new();
    let mut terminators = Vec::new();
    let mut type_variants = Vec::new();
    let mut type_checks = Vec::new();
    let mut type_constructors = Vec::new();
    let mut lifts = Vec::new();

    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut field_lifts = Vec::new();

    // Compose the token stream for all instructions
    for inst in grammar_instructions
        .iter() // Loop over all instructions
        .filter(|i| i.class != Some(structs::Class::Constant)) // Skip constants
    {
        // Get the token for its enumerant
        let inst_name = &inst.opname[2..];
        let name_ident = Ident::new(inst_name, Span::call_site());
        let opcode = inst.opcode;

        // Re-use the allocation between iterations of the loop
        field_names.clear();
        field_types.clear();
        field_lifts.clear();

        // Compose the token stream for all parameters
        for operand in inst.operands.iter() {
            if operand.kind.starts_with("IdResult") {
                continue
            }
            let tokens = OperandTokens::new(operand);
            field_names.push(tokens.name);
            field_types.push(tokens.quantified_type);
            field_lifts.push(tokens.lift_expression);
        }
        let field_names = field_names.as_slice();
        let field_types = field_types.as_slice();
        let field_lifts = field_lifts.as_slice();
        let iterator_init = if field_names.is_empty() {
            quote! {}
        } else {
            let iter = Ident::new(OPERAND_ITER, Span::call_site());
            quote! {
                let mut #iter = raw.operands.iter();
            }
        };

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
                    pub struct #name_ident {
                        #( pub(in crate::sr) #field_names: #field_types ),*
                    }
                });
                let func_name = Ident::new(
                    &format!("lift_{}", snake_casify(inst_name)),
                    Span::call_site(),
                );
                lifts.push(quote! {
                    pub fn #func_name(
                        &mut self, raw: &dr::Instruction
                    ) -> Result<instructions::#name_ident, LiftError> {
                        if raw.class.opcode as u32 != #opcode {
                            return Err(LiftError::OpCode)
                        }
                        #iterator_init;
                        Ok(instructions::#name_ident {
                            #( #field_names: #field_lifts, )*
                        })
                    }
                });
            }
            Some(Terminator) => {
                terminators.push(quote! {
                    #name_ident {#( #field_names: #field_types ),*}
                });
            }
            _ => {
                op_variants.push(if field_names.is_empty() {
                    quote!{ #name_ident }
                } else {
                    quote! { #name_ident {
                        #( #field_names: #field_types ),*
                    }}
                });
            }
        }
    }

    let types = quote! {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub(in crate::sr) enum TypeEnum {
            #( #type_variants ),*
        }

        impl Type {
            #( #type_checks )*
        }
    };
    let instructions = quote! {
        use crate::sr::{Token, Type};

        #( #inst_structs )*
    };
    let ops = quote! {
        use crate::sr::{Token, Type};

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Terminator {
            #( #terminators ),*
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Op {
            #( #op_variants ),*
        }
    };
    let context_logic = quote! {
        impl Context {
            #( #type_constructors )*
            // ------------------------//
            #( #lifts )*
        }
    };

    CodeGeneratedFromInstructionGrammar {
        types: types.to_string(),
        instructions: instructions.to_string(),
        ops: ops.to_string(),
        context_logic: context_logic.to_string(),
    }
}

