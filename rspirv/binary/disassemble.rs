// Copyright 2016 Google Inc.
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

use mr;

/// Trait for disassembling functionalities.
pub trait Disassemble {
    /// Disassembles the current object and returns the assembly code.
    fn disassemble(&self) -> String;
}

impl Disassemble for mr::ModuleHeader {
    fn disassemble(&self) -> String {
        let (major, minor) = self.version();
        let (vendor, _) = self.generator();
        format!("; SPIR-V\n; Version: {}.{}\n; Generator: {}\n; Bound: {}",
                major,
                minor,
                vendor,
                self.bound())
    }
}

impl Disassemble for mr::Operand {
    fn disassemble(&self) -> String {
        match *self {
            mr::Operand::IdMemorySemantics(v) |
            mr::Operand::IdScope(v) |
            mr::Operand::IdRef(v) => format!("%{}", v),
            _ => format!("{}", self),
        }
    }
}

/// Disassembles each instruction in `insts` and joins them together
/// with the given `delimiter`.
fn disas_join<T: Disassemble>(insts: &Vec<T>, delimiter: &str) -> String {
    insts.iter()
        .map(|ref i| i.disassemble())
        .collect::<Vec<String>>()
        .join(delimiter)
}

impl Disassemble for mr::Instruction {
    fn disassemble(&self) -> String {
        format!("{rid}{opcode}{rtype} {operands}",
                rid = self.result_id
                    .map_or(String::new(), |w| format!("%{} = ", w)),
                opcode = format!("Op{}", self.class.opname),
                // extra space both before and after the reseult type
                rtype = self.result_type
                    .map_or(String::new(), |w| format!("  %{} ", w)),
                operands = disas_join(&self.operands, " "))
    }
}

impl Disassemble for mr::BasicBlock {
    fn disassemble(&self) -> String {
        let label =
            self.label.as_ref().map_or(String::new(), |i| i.disassemble());
        format!("{label}\n{insts}",
                label = label,
                insts = disas_join(&self.instructions, "\n"))
    }
}

impl Disassemble for mr::Function {
    fn disassemble(&self) -> String {
        let def = self.def.as_ref().map_or(String::new(), |i| i.disassemble());
        let end = self.end.as_ref().map_or(String::new(), |i| i.disassemble());
        if self.parameters.is_empty() {
            format!("{def}\n{blocks}\n{end}",
                    def = def,
                    blocks = disas_join(&self.basic_blocks, "\n"),
                    end = end)
        } else {
            format!("{def}\n{params}\n{blocks}\n{end}",
                    def = def,
                    params = disas_join(&self.parameters, "\n"),
                    blocks = disas_join(&self.basic_blocks, "\n"),
                    end = end)
        }
    }
}

/// Pushes the given value to the given container if the value is not empty.
macro_rules! push {
    ($container: expr, $val: expr) => (if !$val.is_empty() {
        $container.push($val)
    });
}

impl Disassemble for mr::Module {
    fn disassemble(&self) -> String {
        let mut text = vec![];
        if self.header.is_some() {
            push!(&mut text, self.header.as_ref().unwrap().disassemble());
        }
        push!(&mut text,
              self.capabilities
                  .iter()
                  .map(|c| format!("OpCapability {:?}", c))
                  .collect::<Vec<String>>()
                  .join("\n"));
        push!(&mut text,
              self.extensions
                  .iter()
                  .map(|e| format!("OpExtension {:?}", e))
                  .collect::<Vec<String>>()
                  .join("\n"));
        push!(&mut text,
              self.ext_inst_imports
                  .iter()
                  .map(|e| format!("OpExtInstImport {:?}", e))
                  .collect::<Vec<String>>()
                  .join("\n"));
        // Well, addressing model and memory model are both encoded
        // in OpMemoryModel. But or mr::Module allow only one them exists.
        if self.addressing_model.is_some() && self.memory_model.is_some() {
            push!(&mut text,
                  format!("OpMemoryModel {:?} {:?}",
                          self.addressing_model.unwrap(),
                          self.memory_model.unwrap()));
        }
        push!(&mut text, disas_join(&self.entry_points, "\n"));
        push!(&mut text, disas_join(&self.execution_modes, "\n"));
        push!(&mut text, disas_join(&self.debugs, "\n"));
        push!(&mut text,
              self.names
                  .iter()
                  .map(|(k, v)| format!("OpName %{} {:?}", k, v))
                  .collect::<Vec<String>>()
                  .join("\n"));
        push!(&mut text, disas_join(&self.annotations, "\n"));
        push!(&mut text, disas_join(&self.types_global_values, "\n"));
        push!(&mut text, disas_join(&self.functions, "\n"));
        text.join("\n")
    }
}
