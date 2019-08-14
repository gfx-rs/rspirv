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

        let (ty, lift_value, first_name, second_name) = match operand.kind.as_str() {
            "IdRef" => {
                let (ty, value) = match operand.name.trim_matches('\'') {
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
                        quote! { spirv::Word },
                        quote! { *value },
                    ),
                };
                (ty, value, operand.kind.as_str(), None)
            },
            "IdMemorySemantics" | "IdScope" | "IdResult" => (
                quote! { spirv::Word },
                quote! { *value },
                operand.kind.as_str(),
                None,
            ),
            "LiteralInteger" | "LiteralExtInstInteger" => (
                quote! { u32 },
                quote! { *value },
                "LiteralInt32",
                None
            ),
            "LiteralSpecConstantOpInteger" => (
                quote! { spirv::Op },
                quote! { *value },
                operand.kind.as_str(),
                None,
            ),
            "LiteralContextDependentNumber" => panic!("this kind is not expected to be handled here"),
            "LiteralString" => (
                quote! { String },
                quote! { value.clone() },
                operand.kind.as_str(),
                None,
            ),
            "PairLiteralIntegerIdRef" => (
                quote! { (u32, Token<Type>) },
                quote! { (first, Token::new(second)) },
                "LiteralInt32",
                Some("IdRef"),
            ),
            "PairIdRefLiteralInteger" => (
                quote! { (Token<Type>, u32) },
                quote! { (Token::new(first), second) },
                "IdRef",
                Some("LiteralInt32"),
            ),
            "PairIdRefIdRef" => (
                quote! { (spirv::Word, spirv::Word) },
                quote! { (first, second) },
                "IdRef",
                Some("IdRef"),
            ),
            kind => {
                let kind = Ident::new(kind, Span::call_site());
                (
                    quote! { spirv::#kind },
                    quote! { *value },
                    operand.kind.as_str(),
                    None,
                )
            }
        };

        let first_key = Ident::new(first_name, Span::call_site());
        let lift = match second_name {
            None => {
                quote! {
                    match #iter.next() {
                        Some(&dr::Operand::#first_key(ref value)) => Some(#lift_value),
                        Some(_) => Err(OperandError::Wrong)?,
                        None => None,
                    }
                }
            }
            Some(name) => {
                let second_key = Ident::new(name, Span::call_site());
                quote! {
                    match (#iter.next(), #iter.next()) {
                        (Some(&dr::Operand::#first_key(first)), Some(&dr::Operand::#second_key(second))) => Some(#lift_value),
                        (None, None) => None,
                        _ => Err(OperandError::Wrong)?,
                    }
                }
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
                    while let Some(item) = #lift {
                        vec.push(item);
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

const STANDALONE_TYPES: &[&str] = &[
    "Function",
];

pub fn gen_sr_code_from_instruction_grammar(
    grammar_instructions: &[structs::Instruction],
) -> CodeGeneratedFromInstructionGrammar {
    use structs::Class::*;

    let mut inst_structs = Vec::new();
    let mut op_variants = Vec::new();
    let mut branch_variants = Vec::new();
    let mut branch_lifts = Vec::new();
    let mut terminator_variants = Vec::new();
    let mut terminator_lifts = Vec::new();
    let mut type_structs = Vec::new();
    let mut type_variants = Vec::new();
    let mut type_checks = Vec::new();
    let mut type_constructors = Vec::new();
    let mut lifts = Vec::new();

    let iter_ident = &Ident::new(OPERAND_ITER, Span::call_site());

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
        let type_name = if inst.opname.len() > 6 {
            &inst.opname[6..]
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
            quote! {
                let mut #iter_ident = raw.operands.iter();
            }
        };

        match inst.class.as_str() {
            Some(structs::Class::Type) if STANDALONE_TYPES.contains(&type_name) => {
                type_structs.push(quote! {
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct #type_ident {
                        #( pub(in crate::sr) #field_names: #field_types ),*
                    }
                });
                let func_name = Ident::new(
                    &format!("lift_type_{}", snake_casify(type_name)),
                    Span::call_site(),
                );
                lifts.push(quote! {
                    pub fn #func_name(
                        &mut self, raw: &dr::Instruction
                    ) -> Result<types::#type_ident, LiftError> {
                        if raw.class.opcode as u32 != #opcode {
                            return Err(LiftError::OpCode)
                        }
                        #iterator_init;
                        Ok(types::#type_ident {
                            #( #field_names: #field_lifts, )*
                        })
                    }
                });
            }
            Some(structs::Class::Type) => {
                type_variants.push(if field_names.is_empty() {
                    quote!{ #type_ident }
                } else {
                    quote! { #type_ident {
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
                                TypeEnum::#type_ident #check_params => true,
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
                                ty: TypeEnum::#type_ident #init_list,
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
            Some(Branch) => {
                if field_names.is_empty() {
                    branch_variants.push(quote! { #name_ident });
                    branch_lifts.push(quote! {
                        #opcode => Ok(ops::Branch::#name_ident),
                    });
                } else {
                    branch_variants.push(quote! {
                        #name_ident {#( #field_names: #field_types ),*}
                    });
                    branch_lifts.push(quote! {
                        #opcode => Ok(ops::Branch::#name_ident {
                            #( #field_names: #field_lifts, )*
                        }),
                    });
                }
            }
            Some(Terminator) => {
                if field_names.is_empty() {
                    terminator_variants.push(quote! { #name_ident });
                    terminator_lifts.push(quote! {
                        #opcode => Ok(ops::Terminator::#name_ident),
                    });
                } else {
                    terminator_variants.push(quote! {
                        #name_ident {#( #field_names: #field_types ),*}
                    });
                    terminator_lifts.push(quote! {
                        #opcode => Ok(ops::Terminator::#name_ident {
                            #( #field_names: #field_lifts, )*
                        }),
                    });
                }
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
        #( #type_structs )*

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
        pub enum Branch {
            #( #branch_variants ),*
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Terminator {
            Branch(Branch),
            #( #terminator_variants ),*
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Op {
            #( #op_variants ),*
        }
    };

    let context_logic = quote! {
        impl Context {
            pub fn lift_branch(
                &mut self, raw: &dr::Instruction
            ) -> Result<ops::Branch, LiftError> {
                let mut #iter_ident = raw.operands.iter();
                match raw.class.opcode as u32 {
                    #( #branch_lifts )*
                    _ => Err(LiftError::OpCode),
                }
            }
            pub fn lift_terminator(
                &mut self, raw: &dr::Instruction
            ) -> Result<ops::Terminator, LiftError> {
                match raw.class.opcode as u32 {
                    #( #terminator_lifts )*
                    _ => self.lift_branch(raw)
                        .map(ops::Terminator::Branch)
                }
            }
            #( #type_constructors )*
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

