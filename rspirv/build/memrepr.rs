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

/// Returns the underlying type used in operand kind enums for the operand
/// kind `kind` in the grammar.
fn get_enum_underlying_type(kind: &str) -> String {
    if kind.starts_with("Id") || kind == "LiteralInteger" {
        "spirv::Word".to_string()
    } else if kind == "LiteralString" {
        "String".to_string()
    } else {
        format!("spirv::{}", kind)
    }
}

/// Returns the generated mr::Operand and its fmt::Display implementation by
/// walking the given SPIR-V operand kinds `grammar`.
pub fn gen_mr_operand_kinds(grammar: &Vec<structs::OperandKind>) -> String {
    let mut ret = String::new();

    { // Attributes, uses.
        ret.push_str(RUSTFMT_SKIP_BANG);
        ret.push_str("\n\nuse spirv;\nuse std::fmt;\n\n");
    }

    let kinds: Vec<&str> = grammar.iter().map(|element| {
            element.kind.as_str()
        }).filter(|element| {
            // Pair kinds are not used in mr::Operand.
            // LiteralContextDependentNumber is replaced by suitable literals.
            // LiteralInteger is replaced by LiteralInt32.
            // IdResult and IdResultType are not stored as operands in mr.
            !(element.starts_with("Pair") ||
              *element == "LiteralContextDependentNumber" ||
              *element == "LiteralInteger" ||
              *element == "IdResult" ||
              *element == "IdResultType")
        }).collect();

    { // Enum for all operand kinds in memory representation.
        let id_kinds: Vec<String> = kinds.iter().filter(|element| {
            element.starts_with("Id")
        }).map(|element| {
            format!("    {}(spirv::Word),", element)
        }).collect();
        let num_kinds: Vec<&str> = vec![
            "    LiteralInt32(u32),",
            "    LiteralInt64(u64),",
            "    LiteralFloat32(f32),",
            "    LiteralFloat64(f64),",
            "    LiteralExtInstInteger(u32),",
            "    LiteralSpecConstantOpInteger(spirv::Op),"];
        let str_kinds: Vec<String> = kinds.iter().filter(|element| {
            element.ends_with("String")
        }).map(|element| {
            format!("    {}(String),", element)
        }).collect();
        let enum_kinds: Vec<String> = kinds.iter().filter(|element| {
            !(element.starts_with("Id") ||
              element.ends_with("String") ||
              element.ends_with("Integer") ||
              element.ends_with("Number"))
        }).map(|element| {
            format!("    {k}(spirv::{k}),", k=element)
        }).collect();

        let kind_enum = format!(
            "/// Memory representation of a SPIR-V operand.\n\
             #[derive(Debug, PartialEq)]\n\
             pub enum Operand {{\n\
             {enum_kinds}\n{id_kinds}\n{num_kinds}\n{str_kinds}\n\
             }}\n\n",
             enum_kinds=enum_kinds.join("\n"),
             id_kinds=id_kinds.join("\n"),
             num_kinds=num_kinds.join("\n"),
             str_kinds=str_kinds.join("\n"));
        ret.push_str(&kind_enum);
    }

    { // impl fmt::Display for mr::Operand.
        let mut kinds = kinds;
        kinds.append(&mut vec!["LiteralInt32", "LiteralInt64",
                               "LiteralFloat32", "LiteralFloat64"]);
        let cases: Vec<String> =
            kinds.iter().map(|element| {
                format!("{space:12}Operand::{kind}(ref v) => \
                         write!(f, \"{{:?}}\", v),",
                        space="",
                        kind=element)
            }).collect();
        let impl_code = format!(
            "impl fmt::Display for Operand {{\n\
             {s:4}fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\n\
             {s:8}match *self {{\n{cases}\n{s:8}}}\n{s:4}}}\n}}\n",
             s="",
             cases=cases.join("\n"));
        ret.push_str(&impl_code);
    }

    ret
}

/// Returns the generated build methods for SPIR-V types by walking the given
/// SPIR-V instructions `grammar`.
pub fn gen_mr_builder_types(grammar: &Vec<structs::Instruction>) -> String {
    // Generate build methods for all types.
    let elements: Vec<String> = grammar.iter().filter(|inst| {
        inst.opname.starts_with("OpType")
    }).map(|inst| {
        // Get the kind, name, and quantifier for all operands.
        let operands: Vec<(&str, String, &str)> =
            inst.operands.iter().skip(1).map(|e| {
                let mut name = e.name.replace("'", "").replace(" ", "_");
                if name.len() == 0 {
                    name = snake_casify(&e.kind)
                }
                (e.kind.as_str(), snake_casify(&name), e.quantifier.as_str())
        }).collect();
        // Parameter list for this build method.
        let param_list = operands.iter().map(|&(kind, ref name, quant)| {
            let kind = get_enum_underlying_type(kind);
            if quant == "" {
                format!("{}: {}", name, kind)
            } else if quant == "?" {
                format!("{}: Option<{}>", name, kind)
            } else {
                format!("{}: Vec<{}>", name, kind)
            }
        }).collect::<Vec<_>>().join(", ");
        // Initializer list for constructing the operands parameter
        // for Instruction.
        let init_list = operands.iter().filter_map(|&(kind, ref name, quant)| {
            if quant == "" {
                let kind = get_mr_operand_kind(kind);
                Some(format!("mr::Operand::{}({})", kind, name))
            } else {
                None
            }
        }).collect::<Vec<_>>().join(", ");
        // Parameters that are not single values thus need special treatment.
        let extras = operands.iter().filter_map(|&(kind, ref name, quant)| {
            let kind = get_mr_operand_kind(&kind);
            if quant == "" {
                None
            } else if quant == "?" {
                Some(format!(
                        "{s:8}if {name}.is_some() {{\n\
                           {s:12}self.module.types_global_values.last_mut()\
                             .expect(\"internal error\").operands\
                             .push(mr::Operand::{kind}({name}.unwrap()))\n\
                         {s:8}}}",
                        s="", kind=kind, name=name))
            } else {
                Some(format!(
                        "{s:8}for v in {name} {{\n\
                         {s:12}self.module.types_global_values.last_mut()\
                           .expect(\"internal error\").operands\
                           .push(mr::Operand::{kind}(v))\n\
                         {s:8}}}",
                        s="", kind=kind, name=name))
            }
        }).collect::<Vec<_>>().join(";\n");
        format!("{s:4}/// Creates {opcode} and returns the result id.\n\
                 {s:4}pub fn {name}(&mut self{sep}{param}) -> spirv::Word {{\n\
                   {s:8}let id = self.next_id;\n\
                   {s:8}self.next_id += 1;\n\
                   {s:8}self.module.types_global_values.push(\
                      mr::Instruction::new(spirv::Op::{opcode}, \
                      None, Some(id), vec![{init}]));\n\
                   {extras}{x}\
                   {s:8}id\n\
                 {s:4}}}",
                s="",
                sep=if param_list.len() != 0 { ", " } else { "" },
                opcode=&inst.opname[2..],
                name=snake_casify(&inst.opname[2..]),
                param=param_list,
                init=init_list,
                extras=extras,
                x=if extras.len() != 0 { ";\n" } else { "" })
    }).collect();
    format!("impl Builder {{\n{}\n}}", elements.join("\n\n"))
}
