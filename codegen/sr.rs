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
    let decoration = grammar.operand_kinds.iter().find(|k| k.kind == "Decoration").unwrap();
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
        #[derive(Debug, Eq, PartialEq, From)]
        pub enum Decoration {
            #( #enumerants ),*
        }
    };
    tokens.to_string()
}
