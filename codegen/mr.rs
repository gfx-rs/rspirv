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
                  -> (String, String) {
    let mut type_generics = String::new();
    let mut list: Vec<String> = params.iter().filter_map(|param| {
        let name = get_param_name(param);
        let kind = get_enum_underlying_type(&param.kind, true);
        if param.kind == "IdResult" {
            if keep_result_id {
                Some("result_id: Option<spirv::Word>".to_string())
            } else {
                None
            }
        } else {
            Some(if param.quantifier == "" {
                format!("{}: {}", name, kind)
            } else if param.quantifier == "?" {
                format!("{}: Option<{}>", name, kind)
            } else {
                type_generics = format!("<T: AsRef<[{}]>>", kind);
                format!("{}: T", name)
            })
        }
    }).collect();
    // The last operand may require additional parameters.
    if let Some(o) = params.last() {
        if operand_has_additional_params(o, kinds) {
            type_generics = "<T: AsRef<[mr::Operand]>>".to_string();
            list.push("additional_params: T".to_string());
        }
    }
    (list.join(", "), type_generics)
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
                // These two operands are not stored in the operands field.
                None
            } else {
                let name = get_param_name(param);
                let kind = get_mr_operand_kind(&param.kind);
                Some(if kind == "LiteralString" {
                    format!("mr::Operand::LiteralString({}.into())", name)
                } else {
                    format!("mr::Operand::{}({})", kind, name)
                })
            }
        } else {
            None
        }
    }).collect()
}

fn get_push_extras(params: &[structs::Operand],
                   kinds: &[structs::OperandKind],
                   container: &str)
                   -> Vec<String> {
    let mut list: Vec<String> = params.iter().filter_map(|param| {
        let name = get_param_name(param);
        if param.quantifier == "" {
            None
        } else if param.quantifier == "?" {
            let kind = get_mr_operand_kind(&param.kind);
            Some(format!(
                    "{s:8}if let Some(v) = {name} {{\n\
                     {s:12}{container}.push(mr::Operand::{kind}(v{into}));\n\
                     {s:8}}}",
                    s = "",
                    kind = kind,
                    name = name,
                    into = if kind == "LiteralString" {
                        ".into()"
                    } else {
                        ""
                    },
                    container = container))
        } else {
            // TODO: Ouch! Bad smell. This has special case treatment yet
            // still doesn't solve 64-bit selectors in OpSwitch.
            if param.kind == "PairLiteralIntegerIdRef" {
                Some(format!(
                        "{s:8}for v in {name}.as_ref() {{\n\
                         {s:12}{container}.push(mr::Operand::LiteralInt32(v.0));\n\
                         {s:12}{container}.push(mr::Operand::IdRef(v.1));\n\
                         {s:8}}}",
                        s = "",
                        name = name,
                        container = container))
            } else if param.kind == "PairIdRefLiteralInteger" {
                Some(format!(
                        "{s:8}for v in {name}.as_ref() {{\n\
                         {s:12}{container}.push(mr::Operand::IdRef(v.0));\n\
                         {s:12}{container}.push(mr::Operand::LiteralInt32(v.1));\n\
                         {s:8}}}",
                        s = "",
                        name = name,
                        container = container))
            } else if param.kind == "PairIdRefIdRef" {
                Some(format!(
                        "{s:8}for v in {name}.as_ref() {{\n\
                         {s:12}{container}.push(mr::Operand::IdRef(v.0));\n\
                         {s:12}{container}.push(mr::Operand::IdRef(v.1));\n\
                         {s:8}}}",
                        s = "",
                        name = name,
                        container = container))
            } else {
                let kind = get_mr_operand_kind(&param.kind);
                Some(format!(
                        "{s:8}for v in {name}.as_ref() {{\n\
                         {s:12}{container}.push(mr::Operand::{kind}(*v))\n\
                         {s:8}}}",
                        s = "",
                        kind = kind,
                        name = name,
                        container = container))
            }
        }
    }).collect();
    // The last operand may require additional parameters.
    if let Some(o) =  params.last() {
        if operand_has_additional_params(o, kinds) {
            list.push(format!("{s:8}{container}.extend_from_slice(additional_params.as_ref())",
                              s = "", container = container));
        }
    }
    list
}

/// Returns the generated mr::Operand and its fmt::Display implementation by
/// walking the given SPIR-V operand kinds `grammar`.
pub fn gen_mr_operand_kinds(grammar: &Vec<structs::OperandKind>) -> String {
    let mut ret = String::new();

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

    { // Enum for all operand kinds in data representation.
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
            "/// Data representation of a SPIR-V operand.\n\
             #[derive(Clone, Debug, PartialEq, From)]\n\
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
pub fn gen_mr_builder_types(grammar: &structs::Grammar) -> String {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all types.
    let elements: Vec<String> = grammar.instructions.iter().filter(|inst| {
        inst.class == "Type" && inst.opname != "OpTypeForwardPointer" &&
            inst.opname != "OpTypePointer" && inst.opname != "OpTypeOpaque"
    }).map(|inst| {
        // Parameter list for this build method.
        let (param_list, type_generics) = get_param_list(&inst.operands, false, kinds);
        // Initializer list for constructing the operands parameter
        // for Instruction.
        let init_list = get_init_list(&inst.operands[1..]).join(", ");
        // Parameters that are not single values thus need special treatment.
        let extras = get_push_extras(&inst.operands[1..],
                                     kinds,
                                     "self.module.types_global_values.last_mut()\
                                     .expect(\"interal error\").operands").join(";\n");
        format!("{s:4}/// Appends an Op{opcode} instruction and returns the result id.\n\
                 {s:4}pub fn {name}{generic}(&mut self{sep}{param}) -> spirv::Word {{\n\
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
                generic = type_generics,
                param = param_list,
                init = init_list,
                extras = extras,
                x = if extras.len() != 0 { ";\n" } else { "" })
    }).collect();
    format!("impl Builder {{\n{}\n}}", elements.join("\n\n"))
}

pub fn gen_mr_builder_terminator(grammar: &structs::Grammar) -> String {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all types.
    let elements: Vec<String> = grammar.instructions.iter().filter(|inst| {
        inst.class == "Terminator"
    }).map(|inst| {
        let (params, type_generics) = get_param_list(&inst.operands, false, kinds);
        let extras = get_push_extras(&inst.operands, kinds, "inst.operands").join(";\n");
        format!("{s:4}/// Appends an Op{opcode} instruction and ends the current basic block.\n\
                 {s:4}pub fn {name}{generic}(&mut self{x}{params}) -> BuildResult<()> {{\n\
                 {s:8}let {m}inst = mr::Instruction::new(\
                     spirv::Op::{opcode}, None, None, vec![{init}]);\n\
                 {extras}{y}\
                 {s:8}self.end_basic_block(inst)\n\
                 {s:4}}}",
                s = "",
                name = get_function_name(&inst.opname),
                generic = type_generics,
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

pub fn gen_mr_builder_normal_insts(grammar: &structs::Grammar) -> String {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all normal instructions (instructions must be
    // in some basic block).
    let elements: Vec<String> = grammar.instructions.iter().filter(|inst| {
        inst.class == ""
    }).map(|inst| {
        let (params, type_generics) = get_param_list(&inst.operands, true, kinds);
        let extras = get_push_extras(&inst.operands, kinds, "inst.operands").join(";\n");
        if !inst.operands.is_empty() && inst.operands[0].kind == "IdResultType" {
            // For normal instructions, they either have both result type and
            // result id or have none.
            format!("{s:4}/// Appends an Op{opcode} instruction to the current basic block.\n\
                     {s:4}pub fn {name}{generic}(&mut self{x}{params}) -> BuildResult<spirv::Word> {{\n\
                     {s:8}if self.basic_block.is_none() {{\n\
                     {s:12}return Err(Error::DetachedInstruction);\n\
                     {s:8}}}\n\
                     {s:8}let id = match result_id {{\n\
                     {s:12}Some(v) => v,\n\
                     {s:12}None => self.id(),\n\
                     {s:8}}};\n\
                     {s:8}let {m}inst = mr::Instruction::new(\
                         spirv::Op::{opcode}, Some(result_type), Some(id), vec![{init}]);\n\
                     {extras}{y}\
                     {s:8}self.basic_block.as_mut().unwrap().instructions.push(inst);\n\
                     {s:8}Ok(id)\n\
                     {s:4}}}",
                    s = "",
                    name = get_function_name(&inst.opname),
                    generic = type_generics,
                    extras = extras,
                    params = params,
                    x = if params.len() == 0 { "" } else { ", " },
                    m = if extras.len() == 0 { "" } else { "mut " },
                    y = if extras.len() != 0 { ";\n" } else { "" },
                    init = get_init_list(&inst.operands).join(", "),
                    opcode = &inst.opname[2..])
        } else {
            format!("{s:4}/// Appends an Op{opcode} instruction to the current basic block.\n\
                     {s:4}pub fn {name}{generic}(&mut self{x}{params}) -> BuildResult<()> {{\n\
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
                    generic = type_generics,
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

pub fn gen_mr_builder_constants(grammar: &structs::Grammar) -> String {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements: Vec<String> = grammar.instructions.iter().filter(|inst| {
        inst.class == "Constant" && inst.opname != "OpConstant" && inst.opname != "OpSpecConstant"
    }).map(|inst| {
        let (params, type_generics) = get_param_list(&inst.operands, false, kinds);
        let extras = get_push_extras(&inst.operands, kinds, "inst.operands").join(";\n");
        format!("{s:4}/// Appends an Op{opcode} instruction.\n\
                 {s:4}pub fn {name}{generic}(&mut self{x}{params}) -> spirv::Word {{\n\
                 {s:8}let id = self.id();\n\
                 {s:8}let {m}inst = mr::Instruction::new(\
                     spirv::Op::{opcode}, Some(result_type), Some(id), vec![{init}]);\n\
                 {extras}{y}\
                 {s:8}self.module.types_global_values.push(inst);\n\
                 {s:8}id\n\
                 {s:4}}}",
                s = "",
                name = get_function_name(&inst.opname),
                generic = type_generics,
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

pub fn gen_mr_builder_debug(grammar: &structs::Grammar) -> String {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements: Vec<String> = grammar.instructions.iter().filter(|inst| {
        inst.class == "Debug" && inst.opname != "OpString"
    }).map(|inst| {
        let (params, type_generics) = get_param_list(&inst.operands, false, kinds);
        assert!(type_generics.is_empty());
        let extras = get_push_extras(&inst.operands, kinds, "inst.operands").join(";\n");
        format!("{s:4}/// Appends an Op{opcode} instruction.\n\
                 {s:4}pub fn {name}<T: Into<String>>(&mut self{x}{params}) {{\n\
                 {s:8}let {m}inst = mr::Instruction::new(\
                     spirv::Op::{opcode}, None, None, vec![{init}]);\n\
                 {extras}{y}\
                 {s:8}self.module.debugs.push(inst);\n\
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

pub fn gen_mr_builder_annotation(grammar: &structs::Grammar) -> String {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements: Vec<String> = grammar.instructions.iter().filter(|inst| {
        inst.class == "Annotation" && inst.opname != "OpDecorationGroup"
    }).map(|inst| {
        let (params, type_generics) = get_param_list(&inst.operands, false, kinds);
        let extras = get_push_extras(&inst.operands, kinds, "inst.operands").join(";\n");
        format!("{s:4}/// Appends an Op{opcode} instruction.\n\
                 {s:4}pub fn {name}{generic}(&mut self{x}{params}) {{\n\
                 {s:8}let {m}inst = mr::Instruction::new(\
                     spirv::Op::{opcode}, None, None, vec![{init}]);\n\
                 {extras}{y}\
                 {s:8}self.module.annotations.push(inst);\n\
                 {s:4}}}",
                s = "",
                name = get_function_name(&inst.opname),
                generic = type_generics,
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
