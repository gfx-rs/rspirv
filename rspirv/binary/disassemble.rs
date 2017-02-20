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
use spirv;

use super::tracker;

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
                self.bound)
    }
}

include!("disas_operand.rs");

impl Disassemble for mr::Operand {
    fn disassemble(&self) -> String {
        match *self {
            mr::Operand::IdMemorySemantics(v) |
            mr::Operand::IdScope(v) |
            mr::Operand::IdRef(v) => format!("%{}", v),
            mr::Operand::ImageOperands(v) => v.disassemble(),
            mr::Operand::FPFastMathMode(v) => v.disassemble(),
            mr::Operand::SelectionControl(v) => v.disassemble(),
            mr::Operand::LoopControl(v) => v.disassemble(),
            mr::Operand::FunctionControl(v) => v.disassemble(),
            mr::Operand::MemorySemantics(v) => v.disassemble(),
            mr::Operand::MemoryAccess(v) => v.disassemble(),
            mr::Operand::KernelProfilingInfo(v) => v.disassemble(),
            _ => format!("{}", self),
        }
    }
}

/// Disassembles each instruction in `insts` and joins them together
/// with the given `delimiter`.
fn disas_join<T: Disassemble>(insts: &[T], delimiter: &str) -> String {
    insts.iter()
         .map(|i| i.disassemble())
         .collect::<Vec<String>>()
         .join(delimiter)
}

impl Disassemble for mr::Instruction {
    fn disassemble(&self) -> String {
        format!("{rid}{opcode}{rtype}{space}{operands}",
                rid = self.result_id
                          .map_or(String::new(), |w| format!("%{} = ", w)),
                opcode = format!("Op{}", self.class.opname),
                // extra space both before and after the reseult type
                rtype = self.result_type
                            .map_or(String::new(), |w| format!("  %{} ", w)),
                space = if !self.operands.is_empty() {
                    " "
                } else {
                    ""
                },
                operands = disas_join(&self.operands, " "))
    }
}

impl Disassemble for mr::BasicBlock {
    fn disassemble(&self) -> String {
        let label = self.label
                        .as_ref()
                        .map_or(String::new(), |i| i.disassemble());
        format!("{label}\n{insts}",
                label = label,
                insts = disas_join(&self.instructions, "\n"))
    }
}

impl Disassemble for mr::Function {
    /// Disassembles this module and returns the disassembly text.
    ///
    /// This method will try to link information together to be wise. E.g.,
    /// If the extended instruction set is recognized, the symbolic opcode for
    /// instructions in it will be shown.
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
        let mut ext_inst_set_tracker = tracker::ExtInstSetTracker::new();
        for i in &self.ext_inst_imports {
            ext_inst_set_tracker.track(i)
        }

        let mut text = vec![];
        if let Some(ref header) = self.header {
            push!(&mut text, header.disassemble());
        }

        let global_insts = self.global_inst_iter()
                               .map(|i| i.disassemble())
                               .collect::<Vec<String>>()
                               .join("\n");
        push!(&mut text, global_insts);

        // TODO: Code here is essentially duplicated.
        for f in &self.functions {
            push!(&mut text,
                  f.def.as_ref().map_or(String::new(), |i| i.disassemble()));
            push!(&mut text, disas_join(&f.parameters, "\n"));
            for bb in &f.basic_blocks {
                push!(&mut text,
                      bb.label
                        .as_ref()
                        .map_or(String::new(), |i| i.disassemble()));
                for inst in &bb.instructions {
                    match inst.class.opcode {
                        spirv::Op::ExtInst => {
                            push!(&mut text, disas_ext_inst(inst, &ext_inst_set_tracker))
                        }
                        _ => push!(&mut text, inst.disassemble()),
                    }
                }
            }
            push!(&mut text,
                  f.end.as_ref().map_or(String::new(), |i| i.disassemble()));
        }

        text.join("\n")
    }
}

fn disas_ext_inst(inst: &mr::Instruction,
                  ext_inst_set_tracker: &tracker::ExtInstSetTracker)
                  -> String {
    if inst.operands.len() < 2 {
        return inst.disassemble();
    }
    if let (&mr::Operand::IdRef(id), &mr::Operand::LiteralExtInstInteger(opcode)) =
           (&inst.operands[0], &inst.operands[1]) {
        if !ext_inst_set_tracker.have(id) {
            return inst.disassemble();
        }
        if let Some(grammar) = ext_inst_set_tracker.resolve(id, opcode) {
            let mut operands = vec![];
            operands.push(inst.operands[0].disassemble());
            operands.push(grammar.opname.to_string());
            for operand in &inst.operands[2..] {
                operands.push(operand.disassemble())
            }
            format!("{rid}{opcode}{rtype} {operands}",
                    rid = inst.result_id
                              .map_or(String::new(), |w| format!("%{} = ", w)),
                    opcode = format!("Op{}", inst.class.opname),
                    rtype = inst.result_type
                                .map_or(String::new(), |w| format!("  %{} ", w)),
                    operands = operands.join(" "))
        } else {
            inst.disassemble()
        }
    } else {
        inst.disassemble()
    }
}

#[cfg(test)]
mod tests {
    use mr;
    use spirv;

    use binary::Disassemble;

    #[test]
    fn test_disassemble_operand_function_control() {
        let o = mr::Operand::FunctionControl(spirv::FUNCTION_CONTROL_NONE);
        assert_eq!("None", o.disassemble());
        let o = mr::Operand::FunctionControl(spirv::FUNCTION_CONTROL_INLINE);
        assert_eq!("Inline", o.disassemble());
        let o = mr::Operand::FunctionControl(spirv::FUNCTION_CONTROL_INLINE |
                                             spirv::FUNCTION_CONTROL_PURE);
        assert_eq!("Inline|Pure", o.disassemble());
        let o = mr::Operand::FunctionControl(spirv::FunctionControl::all());
        assert_eq!("Inline|DontInline|Pure|Const", o.disassemble());
    }

    #[test]
    fn test_disassemble_operand_memory_semantics() {
        let o = mr::Operand::MemorySemantics(spirv::MEMORY_SEMANTICS_NONE);
        assert_eq!("None", o.disassemble());
        let o = mr::Operand::MemorySemantics(spirv::MEMORY_SEMANTICS_RELAXED);
        assert_eq!("None", o.disassemble());
        let o = mr::Operand::MemorySemantics(spirv::MEMORY_SEMANTICS_RELEASE);
        assert_eq!("Release", o.disassemble());
        let o = mr::Operand::MemorySemantics(spirv::MEMORY_SEMANTICS_RELEASE |
                                             spirv::MEMORY_SEMANTICS_WORKGROUP_MEMORY);
        assert_eq!("Release|WorkgroupMemory", o.disassemble());
    }

    #[test]
    fn test_disassemble_module_one_inst_in_each_section() {
        let mut b = mr::Builder::new();

        b.capability(spirv::Capability::Shader);
        b.extension("awesome-extension".to_string());
        b.ext_inst_import("GLSL.std.450".to_string());
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        b.source(spirv::SourceLanguage::GLSL, 450, None, None);

        let void = b.type_void(None);
        let float32 = b.type_float(None, 32);
        let voidfvoid = b.type_function(None, void, vec![void]);

        let f = b.begin_function(void,
                                 None,
                                 (spirv::FUNCTION_CONTROL_DONT_INLINE |
                                  spirv::FUNCTION_CONTROL_CONST),
                                 voidfvoid)
                 .unwrap();
        b.begin_basic_block(None).unwrap();
        let var = b.variable(float32, None, spirv::StorageClass::Function, None);
        b.ret().unwrap();
        b.end_function().unwrap();

        b.entry_point(spirv::ExecutionModel::Fragment,
                      f,
                      "main".to_string(),
                      vec![]);
        b.execution_mode(f, spirv::ExecutionMode::OriginUpperLeft, vec![]);
        b.name(f, "main".to_string());
        b.decorate(var, spirv::Decoration::RelaxedPrecision, vec![]);

        assert_eq!(b.module().disassemble(),
                   "; SPIR-V\n\
                    ; Version: 1.1\n\
                    ; Generator: Unknown\n\
                    ; Bound: 8\n\
                    OpCapability Shader\n\
                    OpExtension \"awesome-extension\"\n\
                    %1 = OpExtInstImport \"GLSL.std.450\"\n\
                    OpMemoryModel Logical Simple\n\
                    OpEntryPoint Fragment %5 \"main\"\n\
                    OpExecutionMode %5 OriginUpperLeft\n\
                    OpSource GLSL 450\n\
                    OpName %5 \"main\"\n\
                    OpDecorate %7 RelaxedPrecision\n\
                    %2 = OpTypeVoid\n\
                    %3 = OpTypeFloat 32\n\
                    %4 = OpTypeFunction %2 %2\n\
                    %5 = OpFunction  %2  DontInline|Const %4\n\
                    %6 = OpLabel\n\
                    %7 = OpVariable  %3  Function\n\
                    OpReturn\n\
                    OpFunctionEnd");
    }
}
