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

use regex::Regex;
use utils::*;

/// Returns a suitable name for the given parameter.
fn get_param_name(param: &structs::Operand) -> String {
    if param.name.len() == 0 {
        if param.kind == "IdResultType" {
            "result_type".to_string()
        } else {
            snake_casify(&param.kind)
        }
    } else {
        let re = Regex::new(r"\W").unwrap();
        snake_casify(&re.replace_all(&param.name.replace(" ", "_"), ""))
    }
}

/// Returns the parameter list excluding result id.
fn get_param_list(params: &[structs::Operand]) -> Vec<String> {
    params.iter().filter_map(|param| {
        let name = get_param_name(param);
        let kind = get_enum_underlying_type(&param.kind);
        if param.kind == "IdResult" {
            None
        } else {
            Some(if param.quantifier == "" {
                format!("{}: {}", name, kind)
                } else if param.quantifier == "?" {
                    format!("{}: Option<{}>", name, kind)
                } else {
                    format!("{}: Vec<{}>", name, kind)
                })
        }
    }).collect()
}

/// Returns a suitable function name for the given `opname`.
fn get_function_name(opname: &str) -> String {
    if opname == "OpReturn" {
        "ret".to_string()
    } else if opname == "OpReturnValue" {
        "ret_value".to_string()
    } else {
        snake_casify(&opname[2..])
    }
}

/// Returns the initializer list for all the parameters required to appear
/// once and only once.
fn get_init_list(params: &[structs::Operand]) -> Vec<String> {
    params.iter().filter_map(|param| {
        if param.quantifier == "" {
            if param.kind == "IdResult" || param.kind == "IdResultType" {
                // These two operands are not stored in the operand field.
                None
            } else {
                let name = get_param_name(param);
                let kind = get_mr_operand_kind(&param.kind);
                Some(format!("mr::Operand::{}({})", kind, name))
            }
        } else {
            None
        }
    }).collect()
}

fn get_push_extras(params: &[structs::Operand], container: &str)
                   -> Vec<String> {
    params.iter().filter_map(|param| {
        let name = get_param_name(param);
        if param.quantifier == "" {
            None
        } else if param.quantifier == "?" {
            let kind = get_mr_operand_kind(&param.kind);
            Some(format!(
                    "{s:8}if {name}.is_some() {{\n\
                     {s:12}{container}.push(mr::Operand::{kind}({name}.unwrap()))\n\
                     {s:8}}}",
                    s = "",
                    kind = kind,
                    name = name,
                    container = container))
        } else {
            // TODO: Ouch! Bad smell. This has special case treatment yet
            // still doesn't solve 64-bit selectors in OpSwitch.
            if param.kind == "PairLiteralIntegerIdRef" {
                Some(format!(
                        "{s:8}for v in {name} {{\n\
                         {s:12}{container}.push(mr::Operand::LiteralInt32(v.0));\n\
                         {s:12}{container}.push(mr::Operand::IdRef(v.1));\n\
                         {s:8}}}",
                        s = "",
                        name = name,
                        container = container))
            } else if param.kind == "PairIdRefIdRef" {
                Some(format!(
                        "{s:8}for v in {name} {{\n\
                         {s:12}{container}.push(mr::Operand::IdRef(v.0));\n\
                         {s:12}{container}.push(mr::Operand::IdRef(v.1));\n\
                         {s:8}}}",
                        s = "",
                        name = name,
                        container = container))
            } else {
                let kind = get_mr_operand_kind(&param.kind);
                Some(format!(
                        "{s:8}for v in {name} {{\n\
                         {s:12}{container}.push(mr::Operand::{kind}(v))\n\
                         {s:8}}}",
                        s = "",
                        kind = kind,
                        name = name,
                        container = container))
            }
        }
    }).collect()
}

/// Returns the underlying type used in operand kind enums for the operand
/// kind `kind` in the grammar.
fn get_enum_underlying_type(kind: &str) -> String {
    if kind.starts_with("Id") {
        "spirv::Word".to_string()
    } else if kind == "LiteralInteger" || kind == "LiteralExtInstInteger" {
        "u32".to_string()
    } else if kind == "LiteralSpecConstantOpInteger" {
        "spirv::Op".to_string()
    } else if kind == "LiteralContextDependentNumber" {
        panic!("this kind is not expected to be handled here")
    } else if kind == "LiteralString" {
        "String".to_string()
    } else if kind == "PairLiteralIntegerIdRef" {
        "(u32, spirv::Word)".to_string()
    } else if kind == "PairIdRefLiteralInteger" {
        "(spirv::Word, u32)".to_string()
    } else if kind == "PairIdRefIdRef" {
        "(spirv::Word, spirv::Word)".to_string()
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
             enum_kinds = enum_kinds.join("\n"),
             id_kinds = id_kinds.join("\n"),
             num_kinds = num_kinds.join("\n"),
             str_kinds = str_kinds.join("\n"));
        ret.push_str(&kind_enum);
    }

    { // impl fmt::Display for mr::Operand.
        let mut kinds = kinds;
        kinds.append(&mut vec!["LiteralInt32", "LiteralInt64",
                               "LiteralFloat32", "LiteralFloat64"]);
        let cases: Vec<String> =
            kinds.iter().map(|element| {
                format!("{s:12}Operand::{kind}(ref v) => \
                         write!(f, \"{{:?}}\", v),",
                        s = "",
                        kind = element)
            }).collect();
        let impl_code = format!(
            "impl fmt::Display for Operand {{\n\
             {s:4}fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\n\
             {s:8}match *self {{\n{cases}\n{s:8}}}\n{s:4}}}\n}}\n",
             s = "",
             cases = cases.join("\n"));
        ret.push_str(&impl_code);
    }

    ret
}

/// Returns the generated build methods for SPIR-V types by walking the given
/// SPIR-V instructions `grammar`.
pub fn gen_mr_builder_types(grammar: &Vec<structs::Instruction>) -> String {
    // Generate build methods for all types.
    let elements: Vec<String> = grammar.iter().filter(|inst| {
        inst.class == "Type"
    }).map(|inst| {
        // Parameter list for this build method.
        let param_list = get_param_list(&inst.operands[1..]).join(", ");
        // Initializer list for constructing the operands parameter
        // for Instruction.
        let init_list = get_init_list(&inst.operands[1..]).join(", ");
        // Parameters that are not single values thus need special treatment.
        let extras = get_push_extras(&inst.operands[1..],
                                     "self.module.types_global_values.last_mut()\
                                     .expect(\"interal error\").operands").join(";\n");
        format!("{s:4}/// Appends an Op{opcode} instruction and returns the result id.\n\
                 {s:4}pub fn {name}(&mut self{sep}{param}) -> spirv::Word {{\n\
                 {s:8}let id = self.id();\n\
                 {s:8}self.module.types_global_values.push(\
                     mr::Instruction::new(spirv::Op::{opcode}, \
                     None, Some(id), vec![{init}]));\n\
                 {extras}{x}\
                 {s:8}id\n\
                 {s:4}}}",
                s = "",
                sep = if param_list.len() != 0 { ", " } else { "" },
                opcode = &inst.opname[2..],
                name = snake_casify(&inst.opname[2..]),
                param = param_list,
                init = init_list,
                extras = extras,
                x = if extras.len() != 0 { ";\n" } else { "" })
    }).collect();
    format!("impl Builder {{\n{}\n}}", elements.join("\n\n"))
}

pub fn gen_mr_builder_terminator(grammar: &Vec<structs::Instruction>) -> String {
    // Generate build methods for all types.
    let elements: Vec<String> = grammar.iter().filter(|inst| {
        inst.class == "Terminator"
    }).map(|inst| {
        let params = get_param_list(&inst.operands).join(", ");
        let extras = get_push_extras(&inst.operands, "inst.operands").join(";\n");
        format!("{s:4}/// Appends an Op{opcode} instruction and ends the current basic block.\n\
                 {s:4}pub fn {name}(&mut self{x}{params}) -> BuildResult<()> {{\n\
                 {s:8}let {m}inst = mr::Instruction::new(\
                     spirv::Op::{opcode}, None, None, vec![{init}]);\n\
                 {extras}{y}\
                 {s:8}self.end_basic_block(inst)\n\
                 {s:4}}}",
                s = "",
                name = get_function_name(&inst.opname),
                params = params,
                extras = extras,
                m = if extras.len() == 0 { "" } else { "mut " },
                x = if params.len() == 0 { "" } else { ", " },
                y = if extras.len() != 0 { ";\n" } else { "" },
                init = get_init_list(&inst.operands).join(", "),
                opcode = &inst.opname[2..])
    }).collect();
    format!("impl Builder {{\n{}\n}}", elements.join("\n\n"))
}

pub fn gen_mr_builder_normal_insts(grammar: &Vec<structs::Instruction>) -> String {
    // Generate build methods for all normal instructions (instructions must be
    // in some basic block).
    let elements: Vec<String> = grammar.iter().filter(|inst| {
        inst.class == ""
    }).map(|inst| {
        let params = get_param_list(&inst.operands).join(", ");
        let extras = get_push_extras(&inst.operands, "inst.operands").join(";\n");
        if !inst.operands.is_empty() && inst.operands[0].kind == "IdResultType" {
            // For normal instructions, they either have both result type and
            // result id or have none.
            format!("{s:4}/// Appends an Op{opcode} instruction to the current basic block.\n\
                     {s:4}pub fn {name}(&mut self{x}{params}) -> BuildResult<spirv::Word> {{\n\
                     {s:8}if self.basic_block.is_none() {{\n\
                     {s:12}return Err(Error::DetachedInstruction);\n\
                     {s:8}}}\n\
                     {s:8}let id = self.id();\n\
                     {s:8}let {m}inst = mr::Instruction::new(\
                         spirv::Op::{opcode}, Some(result_type), Some(id), vec![{init}]);\n\
                     {extras}{y}\
                     {s:8}self.basic_block.as_mut().unwrap().instructions.push(inst);\n\
                     {s:8}Ok(id)\n\
                     {s:4}}}",
                    s = "",
                    name = get_function_name(&inst.opname),
                    extras = extras,
                    params = params,
                    x = if params.len() == 0 { "" } else { ", " },
                    m = if extras.len() == 0 { "" } else { "mut " },
                    y = if extras.len() != 0 { ";\n" } else { "" },
                    init = get_init_list(&inst.operands).join(", "),
                    opcode = &inst.opname[2..])
        } else {
            format!("{s:4}/// Appends an Op{opcode} instruction to the current basic block.\n\
                     {s:4}pub fn {name}(&mut self{x}{params}) -> BuildResult<()> {{\n\
                     {s:8}if self.basic_block.is_none() {{\n\
                     {s:12}return Err(Error::DetachedInstruction);\n\
                     {s:8}}}\n\
                     {s:8}let {m}inst = mr::Instruction::new(\
                         spirv::Op::{opcode}, None, None, vec![{init}]);\n\
                     {extras}{y}\
                     {s:8}Ok(self.basic_block.as_mut().unwrap().instructions.push(inst))\n\
                     {s:4}}}",
                    s = "",
                    name = get_function_name(&inst.opname),
                    extras = extras,
                    params = params,
                    x = if params.len() == 0 { "" } else { ", " },
                    m = if extras.len() == 0 { "" } else { "mut " },
                    y = if extras.len() != 0 { ";\n" } else { "" },
                    init = get_init_list(&inst.operands).join(", "),
                    opcode = &inst.opname[2..])
        }
    }).collect();
    format!("impl Builder {{\n{}\n}}", elements.join("\n\n"))
}

pub fn gen_mr_builder_constants(grammar: &Vec<structs::Instruction>) -> String {
    // Generate build methods for all constants.
    let elements: Vec<String> = grammar.iter().filter(|inst| {
        inst.class == "Constant" && inst.opname != "OpConstant" && inst.opname != "OpSpecConstant"
    }).map(|inst| {
        let params = get_param_list(&inst.operands).join(", ");
        let extras = get_push_extras(&inst.operands, "inst.operands").join(";\n");
        format!("{s:4}/// Appends an Op{opcode} instruction.\n\
                 {s:4}pub fn {name}(&mut self{x}{params}) -> spirv::Word {{\n\
                 {s:8}let id = self.id();\n\
                 {s:8}let {m}inst = mr::Instruction::new(\
                     spirv::Op::{opcode}, Some(result_type), Some(id), vec![{init}]);\n\
                 {extras}{y}\
                 {s:8}self.module.types_global_values.push(inst);\n\
                 {s:8}id\n\
                 {s:4}}}",
                s = "",
                name = get_function_name(&inst.opname),
                extras = extras,
                params = params,
                x = if params.len() == 0 { "" } else { ", " },
                m = if extras.len() == 0 { "" } else { "mut " },
                y = if extras.len() != 0 { ";\n" } else { "" },
                init = get_init_list(&inst.operands).join(", "),
                opcode = &inst.opname[2..])
    }).collect();
    format!("impl Builder {{\n{}\n}}", elements.join("\n\n"))
}
