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
                    quote! { types::StructMember::new(value.clone()) },
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
                    //TODO: proper `Token<>`
                    quote! { spirv::Word },
                    quote! { *value },
                ),
            },
            "IdMemorySemantics" | "IdScope" | "IdResult" => (
                //TODO: proper `Token<>`
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
                //TODO: proper `Token<>`
                quote! { (u32, spirv::Word) },
                quote! {
                   match (#iter.next(), #iter.next()) {
                       (Some(&dr::Operand::LiteralInt32(value)), Some(&dr::Operand::IdRef(id))) => Some((value, Token::new(id))),
                       (None, None) => None,
                       _ => Err(OperandError::WrongType)?,
                   }
               },
            ),
            "PairIdRefLiteralInteger" => (
                //TODO: proper `Token<>`
                quote! { (spirv::Word, u32) },
                quote! {
                    match (#iter.next(), #iter.next()) {
                        (Some(&dr::Operand::IdRef(id)), Some(&dr::Operand::LiteralInt32(value))) => Some((Token::new(id), value)),
                        (None, None) => None,
                        _ => Err(OperandError::WrongType)?,
                    }
                },
            ),
            "PairIdRefIdRef" => (
                //TODO: proper `Token<>`
                quote! { (spirv::Word, spirv::Word) },
                quote! {
                    match (#iter.next(), #iter.next()) {
                        (Some(&dr::Operand::IdRef(id1)), Some(&dr::Operand::IdRef(id2))) => Some((Token::new(id1), Token::new(id2))),
                        (None, None) => None,
                        _ => Err(OperandError::WrongType)?,
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
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            }
        };

        let (quantified_type, lift_expression) = match operand.quantifier {
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

const TYPE_PREFIX_LENGTH: usize = 6;

pub fn gen_sr_code_from_instruction_grammar(
    grammar_instructions: &[structs::Instruction],
) -> CodeGeneratedFromInstructionGrammar {
    use structs::Class::*;

    let mut inst_structs = Vec::new();
    let mut op_variants = Vec::new();
    let mut terminators = Vec::new();
    let mut type_variants = Vec::new();
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
        let type_name = if inst.opname.len() > TYPE_PREFIX_LENGTH {
            &inst.opname[TYPE_PREFIX_LENGTH ..]
        } else {
            "_"
        };
        let type_ident = Ident::new(type_name, Span::call_site());
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
                type_variants.push(if field_names.is_empty() {
                    quote!{ #type_ident }
                } else {
                    quote! { #type_ident {
                        #( #field_names: #field_types ),*
                    }}
                });
            }
            Some(ModeSetting) |
            Some(ExtensionDecl) |
            Some(FunctionStruct) => {
                let derive = if let Some(FunctionStruct) = inst.class {
                    quote! { #[derive(Clone, Debug)] }
                } else {
                    quote! { #[derive(Clone, Debug, Eq, PartialEq)] }
                };
                // Create a standalone struct
                inst_structs.push(quote! {
                    #derive
                    pub struct #name_ident {
                        #( pub(in crate::sr) #field_names: #field_types ),*
                    }
                });
                let func_name = Ident::new(
                    &format!("lift_{}", inst_name.to_snake_case()),
                    Span::call_site(),
                );
                lifts.push(quote! {
                    #[allow(unused)] //TODO
                    pub(in crate::sr) fn #func_name(
                        &mut self, raw: &dr::Instruction
                    ) -> Result<instructions::#name_ident, InstructionError> {
                        if raw.class.opcode as u32 != #opcode {
                            return Err(InstructionError::WrongOpcode)
                        }
                        #iterator_init
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
        #[derive(Clone, Debug)]
        pub enum Type {
            #( #type_variants ),*
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
            //TODO: these are DR-specific and may need a new home
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

