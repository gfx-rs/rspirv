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

/// Returns true if the given operand kind can potentially have additional
/// parameters.
#[inline(always)]
pub fn has_additional_params(grammar: &structs::OperandKind) -> bool {
    grammar.enumerants.iter().any(|e| !e.parameters.is_empty())
}

/// Returns true if the given operand can potentially have additional
/// parameters.
pub fn operand_has_additional_params(operand: &structs::Operand,
                                     kinds: &[structs::OperandKind])
                                     -> bool {
    kinds.iter()
         .find(|kind| kind.kind == operand.kind)
         .map_or(false, |kind| has_additional_params(kind))

}

/// Returns the parameter list excluding result id.
fn get_param_list(params: &[structs::Operand],
                  keep_result_id: bool,
                  kinds: &[structs::OperandKind])
                  -> (Vec<TokenStream>, TokenStream) {
    let mut type_generics = TokenStream::new();
    let mut list: Vec<_> = params.iter().filter_map(|param| {
        let name = get_param_name(param);
        let kind = get_enum_underlying_type(&param.kind, true);
        if param.kind == "IdResult" {
            if keep_result_id {
                Some(quote! { result_id: Option<spirv::Word> })
            } else {
                None
            }
        } else {
            Some(if param.quantifier == "" {
                quote! { #name: #kind }
            } else if param.quantifier == "?" {
                quote! { #name: Option<#kind> }
            } else {
                type_generics = quote! { <T: AsRef<[#kind]>> };
                quote! { #name: T }
            })
        }
    }).collect();
    // The last operand may require additional parameters.
    if let Some(o) = params.last() {
        if operand_has_additional_params(o, kinds) {
            type_generics = quote! { <T: AsRef<[dr::Operand]>> };
            list.push(quote! { additional_params: T });
        }
    }
    (list, type_generics)
}

/// Returns a suitable function name for the given `opname`.
fn get_function_name(opname: &str) -> TokenStream {
    if opname == "OpReturn" {
        quote! { ret }
    } else if opname == "OpReturnValue" {
        quote! { ret_value }
    } else {
        let name = as_ident(&snake_casify(&opname[2..]));
        quote! { #name }
    }
}

/// Returns the initializer list for all the parameters required to appear
/// once and only once.
fn get_init_list(params: &[structs::Operand]) -> Vec<TokenStream> {
    params.iter().filter_map(|param| {
        if param.quantifier == "" {
            if param.kind == "IdResult" || param.kind == "IdResultType" {
                // These two operands are not stored in the operands field.
                None
            } else {
                let name = get_param_name(param);
                let kind = get_dr_operand_kind(&param.kind);
                Some(if kind == "LiteralString" {
                    quote! { dr::Operand::LiteralString(#name.into()) }
                } else {
                    quote! { dr::Operand::#kind(#name) }
                })
            }
        } else {
            None
        }
    }).collect()
}

fn get_push_extras(params: &[structs::Operand],
                   kinds: &[structs::OperandKind],
                   container: TokenStream)
                   -> Vec<TokenStream> {
    let mut list: Vec<_> = params.iter().filter_map(|param| {
        let name = get_param_name(param);
        if param.quantifier == "" {
            None
        } else if param.quantifier == "?" {
            let kind = get_dr_operand_kind(&param.kind);
            Some(quote! {
                if let Some(v) = #name {
                    #[allow(clippy::identity_conversion)]
                    #container.push(dr::Operand::#kind(v.into()));
                }
            })
        } else {
            // TODO: Ouch! Bad smell. This has special case treatment yet
            // still doesn't solve 64-bit selectors in OpSwitch.
            if param.kind == "PairLiteralIntegerIdRef" {
                Some(quote! {
                    for v in #name.as_ref() {
                        #container.push(dr::Operand::LiteralInt32(v.0));
                        #container.push(dr::Operand::IdRef(v.1));
                    }
                })
            } else if param.kind == "PairIdRefLiteralInteger" {
                Some(quote! {
                    for v in #name.as_ref() {
                        #container.push(dr::Operand::IdRef(v.0));
                        #container.push(dr::Operand::LiteralInt32(v.1));
                    }
                })
            } else if param.kind == "PairIdRefIdRef" {
                Some(quote! {
                    for v in #name.as_ref() {
                        #container.push(dr::Operand::IdRef(v.0));
                        #container.push(dr::Operand::IdRef(v.1));
                    }
                })
            } else {
                let kind = get_dr_operand_kind(&param.kind);
                Some(quote! {
                    for v in #name.as_ref() {
                        #container.push(dr::Operand::#kind(*v));
                    }
                })
            }
        }
    }).collect();
    // The last operand may require additional parameters.
    if let Some(o) = params.last() {
        if operand_has_additional_params(o, kinds) {
            list.push(quote! {
                #container.extend_from_slice(additional_params.as_ref());
            });
        }
    }
    list
}

/// Returns the generated dr::Operand and its fmt::Display implementation by
/// walking the given SPIR-V operand kinds `grammar`.
pub fn gen_dr_operand_kinds(grammar: &Vec<structs::OperandKind>) -> TokenStream {
    let kinds: Vec<_> = grammar.iter().map(|element| {
            element.kind.as_str()
        }).filter(|element| {
            // Pair kinds are not used in dr::Operand.
            // LiteralContextDependentNumber is replaced by suitable literals.
            // LiteralInteger is replaced by LiteralInt32.
            // IdResult and IdResultType are not stored as operands in `dr`.
            !(element.starts_with("Pair") ||
              *element == "LiteralContextDependentNumber" ||
              *element == "LiteralInteger" ||
              *element == "IdResult" ||
              *element == "IdResultType")
        }).map(as_ident).collect();

    let kind_enum = { // Enum for all operand kinds in data representation.
        let id_kinds = kinds.iter().filter(|element| {
            element.to_string().starts_with("Id")
        }).map(|element| {
            quote! { #element(spirv::Word) }
        });

        let num_kinds = vec![
            quote! { LiteralInt32(u32) },
            quote! { LiteralInt64(u64) },
            quote! { LiteralFloat32(f32) },
            quote! { LiteralFloat64(f64) },
            quote! { LiteralExtInstInteger(u32) },
            quote! { LiteralSpecConstantOpInteger(spirv::Op) },
            ];
        
        let str_kinds = kinds.iter().filter(|element| {
            element.to_string().ends_with("String")
        }).map(|element| {
            quote! { #element(String) }
        });

        let enum_kinds = kinds.iter().filter(|element| {
            let element = element.to_string();
            !(element.starts_with("Id") ||
              element.ends_with("String") ||
              element.ends_with("Integer") ||
              element.ends_with("Number"))
        }).map(|element| {
            quote! { #element(spirv::#element) }
        });

        quote!{
            #[doc = "Data representation of a SPIR-V operand."]
            #[derive(Clone, Debug, PartialEq, From)]
            pub enum Operand {
                #(#enum_kinds,)*
                #(#id_kinds,)*
                #(#num_kinds,)*
                #(#str_kinds,)*
            }
        }
    };

    let impl_code = { // impl fmt::Display for dr::Operand.
        let mut kinds = kinds;
        kinds.extend(["LiteralInt32", "LiteralInt64", "LiteralFloat32", "LiteralFloat64"].iter().cloned().map(as_ident));
        let cases =
            kinds.iter().map(|element| {
                if &element.to_string() == &"Dim" {
                    // Skip the "Dim" prefix, which is only used in the API to
                    // avoid having an enumerant name starting with a number
                    quote! {
                        Operand::#element(ref v) => write!(f, "{}", &format!("{:?}", v)[3..])
                    }
                } else {
                    quote! {
                        Operand::#element(ref v) => write!(f, "{:?}", v)
                    }
                }
            });
        quote! {
            impl fmt::Display for Operand {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    match *self {
                        #(#cases),*
                    }
                }
            }
        }
    };

    quote! {
        #kind_enum
        #impl_code
    }
}

/// Returns the generated build methods for SPIR-V types by walking the given
/// SPIR-V instructions `grammar`.
pub fn gen_dr_builder_types(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all types.
    let elements = grammar.instructions.iter().filter(|inst| {
        inst.class == "Type" && inst.opname != "OpTypeForwardPointer" &&
            inst.opname != "OpTypePointer" && inst.opname != "OpTypeOpaque"
    }).map(|inst| {
        // Parameter list for this build method.
        let (param_list, generic) = get_param_list(&inst.operands, false, kinds);
        // Initializer list for constructing the operands parameter
        // for Instruction.
        let init_list = get_init_list(&inst.operands[1..]);
        // Parameters that are not single values thus need special treatment.
        let extras = get_push_extras(&inst.operands[1..],
                                     kinds,
                                     quote! { self.module.types_global_values.last_mut().expect("interal error").operands });
        let opcode = as_ident(&inst.opname[2..]);
        let name = as_ident(&snake_casify(&inst.opname[2..]));
        let comment = format!("Appends an Op{} instruction and returns the result id.", opcode);
        quote! {
            #[doc = #comment]
            pub fn #name#generic(&mut self,#(#param_list),*) -> spirv::Word {
                let id = self.id();
                self.module.types_global_values.push(
                    dr::Instruction::new(spirv::Op::#opcode, None, Some(id), vec![#(#init_list),*])
                );
                #(#extras)*
                id
            }
        }
    });
    quote! { 
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_terminator(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all types.
    let elements = grammar.instructions.iter().filter(|inst| {
        inst.class == "Terminator" || inst.class == "Branch"
    }).map(|inst| {
        let (params, generic) = get_param_list(&inst.operands, false, kinds);
        let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
        let opcode = as_ident(&inst.opname[2..]);
        let comment = format!("Appends an Op{} instruction and ends the current basic block.", opcode);
        let name = get_function_name(&inst.opname);
        let init = get_init_list(&inst.operands);

        quote! {
            #[doc = #comment]
            pub fn #name#generic(&mut self,#(#params),*) -> BuildResult<()> {
                #[allow(unused_mut)]
                let mut inst = dr::Instruction::new(
                    spirv::Op::#opcode, None, None, vec![#(#init),*]);
                #(#extras)*
                self.end_basic_block(inst)
            }
        }
    });
    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_normal_insts(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all normal instructions (instructions must be
    // in some basic block).
    let elements = grammar.instructions.iter().filter(|inst| {
        inst.class == ""
    }).map(|inst| {
        let (params, generic) = get_param_list(&inst.operands, true, kinds);
        let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
        let opcode = as_ident(&inst.opname[2..]);
        let comment = format!("Appends an Op{} instruction to the current basic block.", opcode);
        let name = get_function_name(&inst.opname);
        let init = get_init_list(&inst.operands);

        if !inst.operands.is_empty() && inst.operands[0].kind == "IdResultType" {
            // For normal instructions, they either have both result type and
            // result id or have none.
            quote! {
                #[doc = #comment]
                pub fn #name#generic(&mut self,#(#params),*) -> BuildResult<spirv::Word> {
                    if self.basic_block.is_none() {
                        return Err(Error::DetachedInstruction);
                    }
                    let _id = match result_id {
                        Some(v) => v,
                        None => self.id(),
                    };
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, Some(result_type), Some(_id), vec![#(#init),*]);
                    #(#extras)*
                    self.basic_block.as_mut().unwrap().instructions.push(inst);
                    Ok(_id)
                }
            }
        } else {
            quote! {
                #[doc = #comment]
                pub fn #name#generic(&mut self,#(#params),*) -> BuildResult<()> {
                    if self.basic_block.is_none() {
                        return Err(Error::DetachedInstruction);
                    }
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, None, None, vec![#(#init),*]);
                    #(#extras)*
                    self.basic_block.as_mut().unwrap().instructions.push(inst);
                    Ok(())
                }
            }
        }
    });
    quote! {
        #[allow(clippy::identity_conversion, clippy::too_many_arguments)]
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_constants(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements = grammar.instructions.iter().filter(|inst| {
        inst.class == "Constant" && inst.opname != "OpConstant" && inst.opname != "OpSpecConstant"
    }).map(|inst| {
        let (params, generic) = get_param_list(&inst.operands, false, kinds);
        let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
        let opcode = as_ident(&inst.opname[2..]);
        let comment = format!("Appends an Op{} instruction.", opcode);
        let name = get_function_name(&inst.opname);
        let init = get_init_list(&inst.operands);
        quote! {
            #[doc = #comment]
            pub fn #name#generic(&mut self,#(#params),*) -> spirv::Word {
                let id = self.id();
                #[allow(unused_mut)]
                let mut inst = dr::Instruction::new(
                    spirv::Op::#opcode, Some(result_type), Some(id), vec![#(#init),*]);
                #(#extras)*
                self.module.types_global_values.push(inst);
                id
            }
        }
    });

    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_debug(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements = grammar.instructions.iter().filter(|inst| {
        inst.class == "Debug" && inst.opname != "OpString"
    }).map(|inst| {
        let (params, generic) = get_param_list(&inst.operands, false, kinds);
        assert!(generic.is_empty());
        let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
        let opcode = as_ident(&inst.opname[2..]);
        let comment = format!("Appends an Op{} instruction.", opcode);
        let name = get_function_name(&inst.opname);
        let init = get_init_list(&inst.operands);
        quote! {
            #[doc = #comment]
            pub fn #name<T: Into<String>>(&mut self,#(#params),*) {
                #[allow(unused_mut)]
                let mut inst = dr::Instruction::new(
                    spirv::Op::#opcode, None, None, vec![#(#init),*]);
                #(#extras)*
                self.module.debugs.push(inst);
            }
        }
    });
    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_annotation(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements = grammar.instructions.iter().filter(|inst| {
        inst.class == "Annotation" && inst.opname != "OpDecorationGroup"
    }).map(|inst| {
        let (params, generic) = get_param_list(&inst.operands, false, kinds);
        let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
        let opcode = as_ident(&inst.opname[2..]);
        let comment = format!("Appends an Op{} instruction.", opcode);
        let name = get_function_name(&inst.opname);
        let init = get_init_list(&inst.operands);
        
        quote! {
            #[doc = #comment]
            pub fn #name#generic(&mut self,#(#params),*) {
                #[allow(unused_mut)]
                let mut inst = dr::Instruction::new(
                    spirv::Op::#opcode, None, None, vec![#(#init),*]);
                #(#extras)*
                self.module.annotations.push(inst);
            }
        }
    });
    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}
