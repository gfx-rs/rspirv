use std::collections::BTreeSet;

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

enum OperandTy<'a> {
    Single(&'a str),
    Pair { first: &'a str, second: &'a str },
    // Image operands consist of a bitmask followed by a list of id refs
    // These refs are not included in the json grammar.
    ImageOperands,
}

impl OperandTokens {
    fn new(
        operands: &[structs::Operand],
        operand_index: usize,
        inst: Option<&structs::Instruction>,
    ) -> Self {
        let operand = &operands[operand_index];
        let name = get_param_name(operands, operand_index);
        let iter = Ident::new(OPERAND_ITER, Span::call_site());

        let (ty, lift_value, op_ty) = match operand.kind.as_str() {
            "IdRef" => {
                let (ty, value) = match operand.name.trim_matches('\'') {
                    "Length" => (
                        quote! { Token<Constant> },
                        quote! { self.constants.lookup_token(*value) },
                    ),
                    "Field Types" => (
                        quote! { StructMember },
                        quote! { StructMember::new(self.types.lookup_token(*value)) },
                    ),
                    "Parameter Types" | "Type" => (
                        quote! { Token<Type> },
                        quote! { self.types.lookup_token(*value) },
                    ),
                    // Function type is manually linked by the code.
                    "Function Type" => (quote! { spirv::Word }, quote! { *value }),
                    name if name.ends_with(" Type") => (
                        quote! { Token<Type> },
                        quote! { self.types.lookup_token(*value) },
                    ),
                    //TODO:
                    //"Condition" => Token<Instruction>,
                    //"Default" => Token<Block>,
                    //"Value" => Token<Instruction>,
                    name if name.ends_with(" Label") => (
                        quote! { spirv::Word },
                        quote! { *value },
                        //TODO:
                        //quote! { Token<Type> },
                        //quote! { self.types[value] },
                    ),
                    _ => {
                        let special_case = if let Some(inst) = inst {
                            match inst.opname.as_str() {
                                "OpTypeStruct" => Some((
                                    quote! { StructMember },
                                    quote! { StructMember::new(self.types.lookup_token(*value)) },
                                )),
                                "OpTypeFunction" => Some((
                                    quote! { Token<Type> },
                                    quote! { self.types.lookup_token(*value) },
                                )),
                                _ => None,
                            }
                        } else {
                            None
                        };

                        if let Some(special_case) = special_case {
                            special_case
                        } else {
                            (
                                //TODO: proper `Token<>`
                                quote! { spirv::Word },
                                quote! { *value },
                            )
                        }
                    }
                };
                (ty, value, OperandTy::Single(operand.kind.as_str()))
            }
            "IdMemorySemantics" | "IdScope" | "IdResult" => (
                //TODO: proper `Token<>`
                quote! { spirv::Word },
                quote! { *value },
                OperandTy::Single(operand.kind.as_str()),
            ),
            "LiteralInteger" => (
                quote! { u32 },
                quote! { *value },
                OperandTy::Single("LiteralBit32"),
            ),
            "LiteralExtInstInteger" => (
                quote! { u32 },
                quote! { *value },
                OperandTy::Single(operand.kind.as_str()),
            ),
            "LiteralSpecConstantOpInteger" => (
                quote! { spirv::Op },
                quote! { *value },
                OperandTy::Single(operand.kind.as_str()),
            ),
            "LiteralContextDependentNumber" => (
                quote! { u32 },
                quote! { *value },
                OperandTy::Single(operand.kind.as_str()),
            ),
            "LiteralString" => (
                quote! { String },
                quote! { value.clone() },
                OperandTy::Single(operand.kind.as_str()),
            ),
            "PairLiteralIntegerIdRef" => (
                quote! { (u32, Jump) },
                quote! { (first, self.lookup_jump(second)) },
                OperandTy::Pair {
                    first: "LiteralBit32",
                    second: "IdRef",
                },
            ),
            "PairIdRefLiteralInteger" => (
                quote! { (Jump, u32) },
                quote! { (self.lookup_jump(first), second) },
                OperandTy::Pair {
                    first: "IdRef",
                    second: "LiteralBit32",
                },
            ),
            "PairIdRefIdRef" => (
                //TODO: proper `Token<>`
                quote! { (spirv::Word, spirv::Word) },
                quote! { (first, second) },
                OperandTy::Pair {
                    first: "IdRef",
                    second: "IdRef",
                },
            ),
            "ImageOperands" => (
                quote! { (spirv::ImageOperands, Vec<spirv::Word>) },
                quote! { (first, second) },
                OperandTy::ImageOperands,
            ),
            kind => {
                let kind = Ident::new(kind, Span::call_site());
                (
                    quote! { spirv::#kind },
                    quote! { *value },
                    OperandTy::Single(operand.kind.as_str()),
                )
            }
        };

        let lift = match op_ty {
            OperandTy::Single(name) => {
                let key = Ident::new(name, Span::call_site());
                quote! {
                    match #iter.next() {
                        Some(dr::Operand::#key(value)) => Some(#lift_value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    }
                }
            }
            OperandTy::Pair { first, second } => {
                let first_key = Ident::new(first, Span::call_site());
                let second_key = Ident::new(second, Span::call_site());

                quote! {
                    match (#iter.next(), #iter.next()) {
                        (Some(&dr::Operand::#first_key(first)), Some(&dr::Operand::#second_key(second))) => Some(#lift_value),
                        (None, None) => None,
                        _ => return Err(OperandError::WrongType.into()),
                    }
                }
            }
            OperandTy::ImageOperands => {
                // Generated code will consume trailing id refs.
                let first_key = Ident::new("ImageOperands", Span::call_site());
                let second_key = Ident::new("IdRef", Span::call_site());
                quote! {
                    match #iter.next() {
                        Some(dr::Operand::#first_key(value)) => {
                            let operands = #iter.map(|op| {
                                match *op {
                                    dr::Operand::#second_key(second) => Ok(second),
                                    _ => Err(OperandError::WrongType),
                                }
                            }).collect::<Result<Vec<_>, _>>()?;
                            Some((*value, operands))
                        },
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
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
            structs::Quantifier::ZeroOrOne => (quote! { Option<#ty> }, lift),
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
            let types: Vec<_> = (0..enumerant.parameters.len())
                .map(|p| OperandTokens::new(&enumerant.parameters, p, None).quantified_type)
                .collect();
            let params = if types.is_empty() {
                quote! {}
            } else {
                quote! { (#( #types ),*) }
            };
            let symbol = Ident::new(enumerant.symbol.as_str(), Span::call_site());
            quote! { #symbol #params }
        })
        .collect();
    let tokens = quote! {
        #![allow(clippy::upper_case_acronyms)]

        /// SPIR-V decorations.
        #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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
    pub lift_context: String,
}

const TYPE_PREFIX_LENGTH: usize = 6;

pub fn gen_sr_code_from_instruction_grammar(
    grammar_instructions: &[structs::Instruction],
) -> CodeGeneratedFromInstructionGrammar {
    use structs::Class::*;

    let mut inst_structs = Vec::new();
    let mut op_variants = Vec::new();
    let mut op_lifts = Vec::new();
    let mut branch_variants = Vec::new();
    let mut branch_lifts = Vec::new();
    let mut terminator_variants = Vec::new();
    let mut terminator_lifts = Vec::new();
    let mut type_variants = Vec::new();
    let mut type_lifts = Vec::new();
    let mut lifts = Vec::new();

    let iter_ident = &Ident::new(OPERAND_ITER, Span::call_site());

    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut field_lifts = Vec::new();

    let mut seen_discriminator = BTreeSet::new();

    // Compose the token stream for all instructions
    for inst in grammar_instructions
        .iter()
        // Skip constants
        .filter(|i| i.class != Some(structs::Class::Constant))
    {
        if !seen_discriminator.insert(inst.opcode) {
            continue;
        }

        // Get the token for its enumerant
        let inst_name = &inst.opname[2..];

        let name_ident = Ident::new(inst_name, Span::call_site());
        let type_name = if inst.opname.len() > TYPE_PREFIX_LENGTH {
            &inst.opname[TYPE_PREFIX_LENGTH..]
        } else {
            "_"
        };

        let type_name = if type_name.chars().next().unwrap().is_ascii_digit() {
            format!("p_{}", type_name)
        } else {
            type_name.to_string()
        };

        let type_ident = Ident::new(&type_name, Span::call_site());
        let opcode = inst.opcode;

        // Re-use the allocation between iterations of the loop
        field_names.clear();
        field_types.clear();
        field_lifts.clear();

        // Compose the token stream for all parameters
        for (index, operand) in inst.operands.iter().enumerate() {
            if operand.kind.starts_with("IdResult") {
                continue;
            }
            let tokens = OperandTokens::new(&inst.operands, index, Some(inst));
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
                if field_names.is_empty() {
                    type_variants.push(quote! { #type_ident });
                    type_lifts.push(quote! {
                        #opcode => Ok(Type::#type_ident),
                    });
                } else {
                    type_variants.push(quote! {
                        #type_ident {
                            #( #field_names: #field_types ),*
                        }
                    });
                    type_lifts.push(quote! {
                        #opcode => Ok(Type::#type_ident {
                            #( #field_names: #field_lifts, )*
                        }),
                    });
                }
            }
            Some(ModeSetting) | Some(ExtensionDecl) | Some(FunctionStruct) => {
                let derive = if let Some(FunctionStruct) = inst.class {
                    quote! { #[derive(Clone, Debug)] }
                } else {
                    quote! { #[derive(Clone, Debug, Eq, PartialEq)] }
                };
                // Create a standalone struct
                inst_structs.push(quote! {
                    #derive
                    pub struct #name_ident {
                        #( pub #field_names: #field_types ),*
                    }
                });
                let func_name = Ident::new(
                    &format!("lift_{}", inst_name.to_snake_case()),
                    Span::call_site(),
                );
                lifts.push(quote! {
                    #[allow(unused)] //TODO
                    pub fn #func_name(
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
            Some(Reserved)
                if matches!(
                    inst_name,
                    "TerminateRayKHR" | "IgnoreIntersectionKHR" | "EmitMeshTasksEXT"
                ) =>
            {
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
            // Skip OpPhi as explicitly processed
            _ if inst_name == "Phi" => {}
            _ => {
                if field_names.is_empty() {
                    op_variants.push(quote! { #name_ident });
                    op_lifts.push(quote! {
                        #opcode => Ok(ops::Op::#name_ident),
                    });
                } else {
                    op_variants.push(quote! {
                        #name_ident {
                            #( #field_names: #field_types ),*
                        }
                    });
                    op_lifts.push(quote! {
                        #opcode => Ok(ops::Op::#name_ident {
                            #( #field_names: #field_lifts, )*
                        }),
                    });
                }
            }
        }
    }

    let types = quote! {
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Clone, Debug)]
        pub enum Type {
            #( #type_variants ),*
        }
    };

    let instructions = quote! {
        #( #inst_structs )*
    };

    let ops = quote! {
        use crate::sr::{module::Jump, storage::Token, Type};

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Branch {
            #( #branch_variants ),*
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum Terminator {
            Branch(Branch),
            #( #terminator_variants ),*
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum Op {
            #( #op_variants ),*
        }
    };

    let lift_context = quote! {
        impl LiftContext {
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
                let mut #iter_ident = raw.operands.iter();
                match raw.class.opcode as u32 {
                    #( #terminator_lifts )*
                    _ => self.lift_branch(raw)
                        .map(ops::Terminator::Branch)
                }
            }
            // TODO: filter duplicated symbols mapping to the same discriminator to avoid
            // unreachable patterns.
            #[allow(unreachable_patterns)]
            pub fn lift_op(
                &mut self, raw: &dr::Instruction
            ) -> Result<ops::Op, InstructionError> {
                let mut #iter_ident = raw.operands.iter();
                match raw.class.opcode as u32 {
                    #( #op_lifts )*
                    _ => Err(InstructionError::WrongOpcode),
                }
            }
            pub fn lift_type(
                &mut self, raw: &dr::Instruction
            ) -> Result<Type, InstructionError> {
                let mut #iter_ident = raw.operands.iter();
                match raw.class.opcode as u32 {
                    #( #type_lifts )*
                    _ => Err(InstructionError::WrongOpcode),
                }
            }

            #( #lifts )*
        }
    };

    CodeGeneratedFromInstructionGrammar {
        types: types.to_string(),
        instructions: instructions.to_string(),
        ops: ops.to_string(),
        lift_context: lift_context.to_string(),
    }
}
