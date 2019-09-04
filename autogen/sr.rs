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
                    //TODO:
                    //"Condition" => Token<Instruction>,
                    //"Default" => Token<BasicBlock>,
                    //"Value" => Token<Instruction>,
                    name if name.ends_with(" Label") => (
                        quote! { spirv::Word },
                        quote! { *value },
                        //TODO:
                        //quote! { Token<Type> },
                        //quote! { self.types.lookup(*value).unwrap() },
                    ),
                    _ => (
                    	//TODO: proper `Token<>`
                        quote! { spirv::Word },
                        quote! { *value },
                    ),
                };
                (ty, value, operand.kind.as_str(), None)
            },
            "IdMemorySemantics" | "IdScope" | "IdResult" => (
                //TODO: proper `Token<>`
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
                //TODO: Token<BasicBlock>
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
                //TODO: proper `Token<>`
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
                        Some(_) => Err(OperandError::WrongType)?,
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
                        _ => Err(OperandError::WrongType)?,
                    }
                }
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
    pub module_logic: String,
}

const TYPE_PREFIX_LENGTH: usize = 6;

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
    let mut type_variants = Vec::new();
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
            quote! {
                let mut #iter_ident = raw.operands.iter();
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
        #[derive(Clone, Debug)]
        pub enum Type {
            #( #type_variants ),*
        }
    };

    let instructions = quote! {
        use crate::sr::{storage::Token, Type};

        #( #inst_structs )*
    };

    let ops = quote! {
        use crate::sr::{storage::Token, Type};

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

    let module_logic = quote! {
        impl Module {
            pub fn lift_branch(
                &mut self, raw: &dr::Instruction
            ) -> Result<ops::Branch, InstructionError> {
                let mut #iter_ident = raw.operands.iter();
                match raw.class.opcode as u32 {
                    #( #branch_lifts )*
                    _ => Err(InstructionError::WrongOpcode),
                }
            }
            pub fn lift_terminator(
                &mut self, raw: &dr::Instruction
            ) -> Result<ops::Terminator, InstructionError> {
                match raw.class.opcode as u32 {
                    #( #terminator_lifts )*
                    _ => self.lift_branch(raw)
                        .map(ops::Terminator::Branch)
                }
            }
            //TODO: these are DR-specific and may need a new home
            #( #lifts )*
        }
    };

    CodeGeneratedFromInstructionGrammar {
        types: types.to_string(),
        instructions: instructions.to_string(),
        ops: ops.to_string(),
        module_logic: module_logic.to_string(),
    }
}

