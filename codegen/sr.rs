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

use quote;
use structs;
use utils::*;

pub fn gen_sr_decoration(grammar: &structs::Grammar) -> String {
    // The decoration operand kind
    let decoration = grammar.operand_kinds
        .iter()
        .find(|k| k.kind == "Decoration")
        .unwrap();
    // Go and compose all its enumerants
    let enumerants: Vec<_> = decoration.enumerants
        .iter()
        .map(|enumerant| {
            // Parameters for this enumerant
            let types: Vec<_> = enumerant.parameters
                .iter()
                .map(|p| quote::Ident::from(get_enum_underlying_type(&p.kind, false)))
                .collect();
            let params = if types.is_empty() {
                quote!{}
            } else {
                quote! { (#( #types ),*) }
            };
            let symbol = quote::Ident::from(enumerant.symbol.as_str());
            quote! { #symbol #params }
        })
        .collect();
    let tokens = quote! {
        use spirv;

        /// SPIR-V decorations.
        #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, From)]
        pub enum Decoration {
            #( #enumerants ),*
        }
    };
    tokens.to_string()
}

pub fn get_operand_type_ident(grammar: &structs::Operand) -> quote::Tokens {
    let ty = if grammar.kind == "IdRef" {
        if grammar.name == "'Length'" {
            "ConstantToken".to_string()
        } else {
            "TypeToken".to_string()
        }
    } else {
        get_enum_underlying_type(&grammar.kind, false)
    };
    let ty = quote::Ident::from(ty);
    if grammar.quantifier.is_empty() {
        quote! { #ty }
    } else if grammar.quantifier == "?" {
        quote! { Option<#ty> }
    } else {
        quote! { Vec<#ty> }
    }
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
    let cases: Vec<_> = grammar.instructions
        .iter()
        .filter(|k| k.class == "Type")
        .map(|kind| {
            let operands: Vec<_> = kind.operands
                .iter()
                .skip(1)
                .map(|op| {
                         let name = quote::Ident::from(get_param_name(op));
                         let ty = get_operand_type_ident(op);
                         (name, ty)
                     })
                .collect();
            let symbol = &kind.opname[6..];
            (symbol, operands)
        })
        .collect();
    let types: Vec<_> = cases.iter()
        .map(|&(symbol, ref params)| {
            let symbol = quote::Ident::from(symbol);
            let param_list: Vec<_> = params.iter()
                .map(|&(ref name, ref ty)| {
                         quote! { #name : #ty }
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
    let checks: Vec<_> = cases.iter()
        .map(|&(symbol, ref params)| {
            let func_name = quote::Ident::from(format!("is_{}_type", get_type_fn_name(symbol)));
            let symbol = quote::Ident::from(symbol);
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
        pub (in sr) enum TypeEnum {
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
    let cases: Vec<_> = grammar.instructions
        .iter()
        .filter(|k| k.class == "Type")
        .filter(|k| k.opname != "OpTypeStruct")
        .map(|kind| {
            let operands: Vec<_> = kind.operands
                .iter()
                .skip(1)
                .map(|op| {
                         let name = quote::Ident::from(get_param_name(op));
                         let ty = get_operand_type_ident(op);
                         (name, ty)
                     })
                .collect();
            let symbol = &kind.opname[6..];
            (symbol, operands)
        })
        .collect();
    let constructors: Vec<_> = cases.iter()
        .map(|&(symbol, ref params)| {
            let func_name = quote::Ident::from(format!("type_{}", get_type_fn_name(symbol)));
            let symbol = quote::Ident::from(symbol);
            let param_list: Vec<_> = params.iter()
                .map(|&(ref name, ref ty)| {
                         quote! { #name : #ty }
                     })
                .collect();
            let param_list = quote! { (&mut self, #( #param_list ),*) };
            let init_list: Vec<_> = params.iter()
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
                pub fn #func_name #param_list -> TypeToken {
                    let t = Type { ty: TypeEnum::#symbol #init_list, decorations: BTreeSet::new() };
                    if let Some(index) = self.types.iter().position(|x| *x == t) {
                        TypeToken::new(index)
                    } else {
                        self.types.push(t);
                        TypeToken::new(self.types.len() - 1)
                    }
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
