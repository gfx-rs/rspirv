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

use std::fs;
use std::io::Write;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[cfg_attr(rustfmt, rustfmt_skip)]
static COPYRIGHT : &'static str = "\
// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the \"License\");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an \"AS IS\" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.";

#[cfg_attr(rustfmt, rustfmt_skip)]
static AUTOGEN_COMMENT : &'static str = "\
// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!";

pub fn write_copyright_autogen_comment(file: &mut fs::File) {
    file.write_all(COPYRIGHT.as_bytes()).unwrap();
    file.write_all(b"\n\n").unwrap();
    file.write_all(AUTOGEN_COMMENT.as_bytes()).unwrap();
    file.write_all(b"\n\n").unwrap();
}

/// Converts the given string into an `Ident`, with call-site span.
pub fn as_ident(ident: &str) -> Ident {
    Ident::new(ident, Span::call_site())
}

/// Converts the given `symbol` to use snake case style.
pub fn snake_casify(symbol: &str) -> String {
    let re = regex::Regex::new(r"(?P<l>[a-z])(?P<u>[A-Z])").unwrap();
    re.replace_all(symbol, "$l-$u").replace("-", "_").to_lowercase()
}

/// Returns the corresponding operand kind in data representation for the
/// given operand `kind` in the grammar.
pub fn get_dr_operand_kind(kind: &str) -> Ident {
    as_ident(if kind == "LiteralInteger" {
        "LiteralInt32"
    } else if kind == "LiteralContextDependentNumber" {
        // TODO: should use the correct type to decode
        "LiteralInt32"
    } else {
        kind
    })
}

/// Returns the underlying type used in operand kind enums for the operand
/// kind `kind` in the grammar.
pub fn get_enum_underlying_type(kind: &str, generic_string: bool) -> TokenStream {
    if kind.starts_with("Id") {
        quote! { spirv::Word }
    } else if kind == "LiteralInteger" || kind == "LiteralExtInstInteger" {
        quote! { u32 }
    } else if kind == "LiteralSpecConstantOpInteger" {
        quote! { spirv::Op }
    } else if kind == "LiteralContextDependentNumber" {
        panic!("this kind is not expected to be handled here")
    } else if kind == "LiteralString" {
        if generic_string { quote! { T } } else { quote! { String } }
    } else if kind == "PairLiteralIntegerIdRef" {
        quote! { (u32, spirv::Word) }
    } else if kind == "PairIdRefLiteralInteger" {
        quote! { (spirv::Word, u32) }
    } else if kind == "PairIdRefIdRef" {
        quote! { (spirv::Word, spirv::Word) }
    } else {
        let kind = as_ident(kind);
        quote! { spirv::#kind }
    }
}

/// Returns a suitable name for the given parameter.
pub fn get_param_name(param: &structs::Operand) -> Ident {
    let name = if param.name.len() == 0 {
        if param.kind == "IdResultType" {
            "result_type".to_string()
        } else {
            snake_casify(&param.kind)
        }
    } else {
        let re = regex::Regex::new(r"\W").unwrap();
        snake_casify(&re.replace_all(&param.name.replace(" ", "_"), ""))
    };
    as_ident(&name)
}
