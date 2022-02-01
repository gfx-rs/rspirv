use crate::dr;
use crate::spirv;

use super::tracker;

/// Trait for disassembling functionalities.
pub trait Disassemble {
    /// Disassembles the current object and returns the assembly code.
    fn disassemble(&self) -> String;
}

impl Disassemble for dr::ModuleHeader {
    fn disassemble(&self) -> String {
        let (major, minor) = self.version();
        let (vendor, _) = self.generator();
        format!(
            "; SPIR-V\n; Version: {}.{}\n; Generator: {}\n; Bound: {}",
            major, minor, vendor, self.bound
        )
    }
}

include!("autogen_disas_operand.rs");

impl Disassemble for dr::Operand {
    fn disassemble(&self) -> String {
        match *self {
            dr::Operand::IdMemorySemantics(v) | dr::Operand::IdScope(v) | dr::Operand::IdRef(v) => {
                format!("%{}", v)
            }
            dr::Operand::ImageOperands(v) => v.disassemble(),
            dr::Operand::FPFastMathMode(v) => v.disassemble(),
            dr::Operand::SelectionControl(v) => v.disassemble(),
            dr::Operand::LoopControl(v) => v.disassemble(),
            dr::Operand::FunctionControl(v) => v.disassemble(),
            dr::Operand::MemorySemantics(v) => v.disassemble(),
            dr::Operand::MemoryAccess(v) => v.disassemble(),
            dr::Operand::KernelProfilingInfo(v) => v.disassemble(),
            _ => format!("{}", self),
        }
    }
}

/// Disassembles each instruction in `insts` and joins them together
/// with the given `delimiter`.
fn disas_join(insts: &[impl Disassemble], delimiter: &str) -> String {
    insts
        .iter()
        .map(|i| i.disassemble())
        .collect::<Vec<String>>()
        .join(delimiter)
}

impl Disassemble for dr::Instruction {
    fn disassemble(&self) -> String {
        let space = if !self.operands.is_empty() { " " } else { "" };
        format!(
            "{rid}Op{opcode}{rtype}{space}{operands}",
            rid = self
                .result_id
                .map_or(String::new(), |w| format!("%{} = ", w)),
            opcode = self.class.opname,
            // extra space both before and after the reseult type
            rtype = self
                .result_type
                .map_or(String::new(), |w| format!("  %{}{}", w, space)),
            space = space,
            operands = disas_join(&self.operands, " ")
        )
    }
}

impl Disassemble for dr::Block {
    fn disassemble(&self) -> String {
        let label = self
            .label
            .as_ref()
            .map_or(String::new(), |i| i.disassemble());
        format!(
            "{label}\n{insts}",
            label = label,
            insts = disas_join(&self.instructions, "\n")
        )
    }
}

impl Disassemble for dr::Function {
    fn disassemble(&self) -> String {
        let def = self.def.as_ref().map_or(String::new(), |i| i.disassemble());
        let end = self.end.as_ref().map_or(String::new(), |i| i.disassemble());
        if self.parameters.is_empty() {
            format!(
                "{def}\n{blocks}\n{end}",
                def = def,
                blocks = disas_join(&self.blocks, "\n"),
                end = end
            )
        } else {
            format!(
                "{def}\n{params}\n{blocks}\n{end}",
                def = def,
                params = disas_join(&self.parameters, "\n"),
                blocks = disas_join(&self.blocks, "\n"),
                end = end
            )
        }
    }
}

/// Pushes the given value to the given container if the value is not empty.
macro_rules! push {
    ($container: expr, $val: expr) => {
        if !$val.is_empty() {
            $container.push($val)
        }
    };
}

impl Disassemble for dr::Module {
    /// Disassembles this module and returns the disassembly text.
    ///
    /// This method will try to link information together to be wise. E.g.,
    /// If the extended instruction set is recognized, the symbolic opcode for
    /// instructions in it will be shown.
    fn disassemble(&self) -> String {
        let mut ext_inst_set_tracker = tracker::ExtInstSetTracker::new();
        for i in &self.ext_inst_imports {
            ext_inst_set_tracker.track(i)
        }

        let mut text = vec![];
        if let Some(ref header) = self.header {
            push!(&mut text, header.disassemble());
        }

        let global_insts = self
            .global_inst_iter()
            .map(|i| i.disassemble())
            .collect::<Vec<String>>()
            .join("\n");
        push!(&mut text, global_insts);

        // TODO: Code here is essentially duplicated. Ideally we should be able
        // to call dr::Function and dr::BasicBlock's disassemble() method here
        // but because of the ExtInstSetTracker, we are not able to directly.
        for f in &self.functions {
            push!(
                &mut text,
                f.def.as_ref().map_or(String::new(), |i| i.disassemble())
            );
            push!(&mut text, disas_join(&f.parameters, "\n"));
            for bb in &f.blocks {
                push!(
                    &mut text,
                    bb.label.as_ref().map_or(String::new(), |i| i.disassemble())
                );
                for inst in &bb.instructions {
                    match inst.class.opcode {
                        spirv::Op::ExtInst => {
                            push!(&mut text, disas_ext_inst(inst, &ext_inst_set_tracker))
                        }
                        _ => push!(&mut text, inst.disassemble()),
                    }
                }
            }
            push!(
                &mut text,
                f.end.as_ref().map_or(String::new(), |i| i.disassemble())
            );
        }

        text.join("\n")
    }
}

fn disas_ext_inst(
    inst: &dr::Instruction,
    ext_inst_set_tracker: &tracker::ExtInstSetTracker,
) -> String {
    if inst.operands.len() < 2 {
        return inst.disassemble();
    }
    if let (&dr::Operand::IdRef(id), &dr::Operand::LiteralExtInstInteger(opcode)) =
        (&inst.operands[0], &inst.operands[1])
    {
        if !ext_inst_set_tracker.have(id) {
            return inst.disassemble();
        }
        if let Some(grammar) = ext_inst_set_tracker.resolve(id, opcode) {
            let mut operands = vec![inst.operands[0].disassemble(), grammar.opname.to_string()];
            for operand in &inst.operands[2..] {
                operands.push(operand.disassemble())
            }
            format!(
                "{rid}Op{opcode}{rtype} {operands}",
                rid = inst
                    .result_id
                    .map_or(String::new(), |w| format!("%{} = ", w)),
                opcode = inst.class.opname,
                rtype = inst
                    .result_type
                    .map_or(String::new(), |w| format!("  %{} ", w)),
                operands = operands.join(" ")
            )
        } else {
            inst.disassemble()
        }
    } else {
        inst.disassemble()
    }
}

#[cfg(test)]
mod tests {
    use crate::dr;
    use crate::spirv;

    use crate::binary::Disassemble;

    #[test]
    fn test_disassemble_operand_function_control() {
        let o = dr::Operand::FunctionControl(spirv::FunctionControl::NONE);
        assert_eq!("None", o.disassemble());
        let o = dr::Operand::FunctionControl(spirv::FunctionControl::INLINE);
        assert_eq!("Inline", o.disassemble());
        let o = dr::Operand::FunctionControl(
            spirv::FunctionControl::INLINE | spirv::FunctionControl::PURE,
        );
        assert_eq!("Inline|Pure", o.disassemble());
        let o = dr::Operand::FunctionControl(spirv::FunctionControl::all());
        assert_eq!("Inline|DontInline|Pure|Const|OptNoneINTEL", o.disassemble());
    }

    #[test]
    fn test_disassemble_operand_memory_semantics() {
        let o = dr::Operand::MemorySemantics(spirv::MemorySemantics::NONE);
        assert_eq!("None", o.disassemble());
        let o = dr::Operand::MemorySemantics(spirv::MemorySemantics::RELAXED);
        assert_eq!("None", o.disassemble());
        let o = dr::Operand::MemorySemantics(spirv::MemorySemantics::RELEASE);
        assert_eq!("Release", o.disassemble());
        let o = dr::Operand::MemorySemantics(
            spirv::MemorySemantics::RELEASE | spirv::MemorySemantics::WORKGROUP_MEMORY,
        );
        assert_eq!("Release|WorkgroupMemory", o.disassemble());
    }

    #[test]
    fn test_disassemble_module_one_inst_in_each_section() {
        let mut b = dr::Builder::new();

        b.capability(spirv::Capability::Shader);
        b.extension("awesome-extension");
        b.ext_inst_import("GLSL.std.450");
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        b.source(spirv::SourceLanguage::GLSL, 450, None, None::<String>);

        let void = b.type_void();
        let float32 = b.type_float(32);
        let voidfvoid = b.type_function(void, vec![void]);

        let f = b
            .begin_function(
                void,
                None,
                spirv::FunctionControl::DONT_INLINE | spirv::FunctionControl::CONST,
                voidfvoid,
            )
            .unwrap();
        b.begin_block(None).unwrap();
        let var = b.variable(float32, None, spirv::StorageClass::Function, None);
        b.ret().unwrap();
        b.end_function().unwrap();

        b.entry_point(spirv::ExecutionModel::Fragment, f, "main", vec![]);
        b.execution_mode(f, spirv::ExecutionMode::OriginUpperLeft, vec![]);
        b.name(f, "main");
        b.decorate(var, spirv::Decoration::RelaxedPrecision, vec![]);

        assert_eq!(
            b.module().disassemble(),
            "; SPIR-V\n\
                    ; Version: 1.5\n\
                    ; Generator: rspirv\n\
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
                    OpFunctionEnd"
        );
    }

    #[test]
    fn test_disassemble_ext_inst_glsl() {
        let mut b = dr::Builder::new();

        b.capability(spirv::Capability::Shader);
        let glsl = b.ext_inst_import("GLSL.std.450");
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);

        let void = b.type_void();
        let float32 = b.type_float(32);
        let voidfvoid = b.type_function(void, vec![void]);

        assert!(b
            .begin_function(void, None, spirv::FunctionControl::NONE, voidfvoid)
            .is_ok());
        b.begin_block(None).unwrap();
        let var = b.variable(float32, None, spirv::StorageClass::Function, None);
        let args = std::iter::once(dr::Operand::IdRef(var));
        b.ext_inst(float32, None, glsl, 6, args).unwrap();
        b.ret().unwrap();
        b.end_function().unwrap();

        assert_eq!(
            b.module().disassemble(),
            "; SPIR-V\n\
                    ; Version: 1.5\n\
                    ; Generator: rspirv\n\
                    ; Bound: 9\n\
                    OpCapability Shader\n\
                    %1 = OpExtInstImport \"GLSL.std.450\"\n\
                    OpMemoryModel Logical Simple\n\
                    %2 = OpTypeVoid\n\
                    %3 = OpTypeFloat 32\n\
                    %4 = OpTypeFunction %2 %2\n\
                    %5 = OpFunction  %2  None %4\n\
                    %6 = OpLabel\n\
                    %7 = OpVariable  %3  Function\n\
                    %8 = OpExtInst  %3  %1 FSign %7\n\
                    OpReturn\n\
                    OpFunctionEnd"
        );
    }

    #[test]
    fn test_disassemble_ext_inst_opencl() {
        let mut b = dr::Builder::new();

        let opencl = b.ext_inst_import("OpenCL.std");
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::OpenCL);

        let void = b.type_void();
        let float32 = b.type_float(32);
        let voidfvoid = b.type_function(void, vec![void]);

        assert!(b
            .begin_function(void, None, spirv::FunctionControl::NONE, voidfvoid)
            .is_ok());
        b.begin_block(None).unwrap();
        let var = b.variable(float32, None, spirv::StorageClass::Function, None);

        let args = std::iter::once(dr::Operand::IdRef(var));
        b.ext_inst(float32, None, opencl, 15, args).unwrap();
        b.ret().unwrap();

        b.end_function().unwrap();

        assert_eq!(
            b.module().disassemble(),
            "; SPIR-V\n\
                    ; Version: 1.5\n\
                    ; Generator: rspirv\n\
                    ; Bound: 9\n\
                    %1 = OpExtInstImport \"OpenCL.std\"\n\
                    OpMemoryModel Logical OpenCL\n\
                    %2 = OpTypeVoid\n\
                    %3 = OpTypeFloat 32\n\
                    %4 = OpTypeFunction %2 %2\n\
                    %5 = OpFunction  %2  None %4\n\
                    %6 = OpLabel\n\
                    %7 = OpVariable  %3  Function\n\
                    %8 = OpExtInst  %3  %1 cosh %7\n\
                    OpReturn\n\
                    OpFunctionEnd"
        );
    }
}
