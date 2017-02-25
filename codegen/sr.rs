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

use structs;
use utils::*;

pub fn gen_sr_decoration(grammar: &structs::Grammar) -> String {
    // The decoration operand kind
    let decoration = grammar.operand_kinds.iter().find(|k| k.kind == "Decoration").unwrap();
    // Go and compose all its enumerants
    let elements: Vec<String> = decoration.enumerants.iter().map(|element| {
        // Parameters for this enumerant
        let mut params: String = element.parameters.iter().map(|p| {
            get_enum_underlying_type(&p.kind, false)
        }).collect::<Vec<_>>().join(", ");
        if !params.is_empty() {
            params = format!("({})", params);
        }
        format!("    {}{}", element.symbol, params)
    }).collect();
    format!("use spirv;\n\n\
             /// SPIR-V decorations.\n\
             #[derive(Debug, Eq, PartialEq, From)]\n\
             pub enum Decoration {{\n{}\n}}\n", elements.join(",\n"))
}
