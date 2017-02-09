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

#![allow(dead_code)]

use mr;
use spirv;

use std::result;
use super::Error;

type BuildResult<T> = result::Result<T, Error>;

/// The memory representation builder.
///
/// Constructs a [`Module`](struct.Module.html) by aggregating results from
/// method calls for various instructions.
///
/// Methods in the builder implement little sanity check; only appending
/// instructions that violates the module structure is guarded. So methods
/// possibly returning errors are basically those related to function and basic
/// block construction. Instructions belonging to the module can be appended
/// at any time, regardless building a basic block or not. Intructions that
/// can appear both in the module and basic block will be inserted to the
/// current basic block under construction first, if any.
///
/// # Examples
///
/// ```
/// extern crate rspirv;
/// extern crate spirv_headers as spirv;
///
/// use rspirv::binary::Disassemble;
///
/// fn main() {
///     let mut b = rspirv::mr::Builder::new();
///     b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
///     let void = b.type_void();
///     let voidf = b.type_function(void, vec![void]);
///     b.begin_function(void, spirv::FUNCTION_CONTROL_NONE, voidf).unwrap();
///     b.begin_basic_block().unwrap();
///     b.ret().unwrap();
///     b.end_function().unwrap();
///
///     assert_eq!(b.module().disassemble(),
///                "OpMemoryModel Logical Simple\n\
///                 %1 = OpTypeVoid\n\
///                 %2 = OpTypeFunction %1 %1\n\
///                 %3 = OpFunction  %1  None %2\n\
///                 %4 = OpLabel\n\
///                 OpReturn\n\
///                 OpFunctionEnd");
/// }
/// ```
pub struct Builder {
    module: mr::Module,
    next_id: u32,
    function: Option<mr::Function>,
    basic_block: Option<mr::BasicBlock>,
}

impl Builder {
    /// Creates a new empty builder.
    pub fn new() -> Builder {
        Builder {
            module: mr::Module::new(),
            next_id: 1,
            function: None,
            basic_block: None,
        }
    }

    /// Returns the `Module` under construction.
    pub fn module(self) -> mr::Module {
        self.module
    }

    /// Returns the next unused id.
    #[inline(always)]
    fn id(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Begins building of a new function.
    pub fn begin_function(&mut self,
                          return_type: spirv::Word,
                          control: spirv::FunctionControl,
                          function_type: spirv::Word)
                          -> BuildResult<spirv::Word> {
        if self.function.is_some() {
            return Err(Error::NestedFunction);
        }

        let id = self.id();

        let mut f = mr::Function::new();
        f.def = Some(mr::Instruction::new(spirv::Op::Function,
                                          Some(return_type),
                                          Some(id),
                                          vec![mr::Operand::FunctionControl(control),
                                               mr::Operand::IdRef(function_type)]));
        self.function = Some(f);
        Ok(id)
    }

    /// Ends building of the current function.
    pub fn end_function(&mut self) -> BuildResult<()> {
        if self.function.is_none() {
            return Err(Error::MismatchedFunctionEnd);
        }

        let mut f = self.function.take().unwrap();
        f.end = Some(mr::Instruction::new(spirv::Op::FunctionEnd, None, None, vec![]));
        Ok(self.module.functions.push(f))
    }

    /// Declares a formal parameter for the current function.
    pub fn function_parameter(&mut self, result_type: spirv::Word) -> BuildResult<spirv::Word> {
        if self.function.is_none() {
            return Err(Error::DetachedFunctionParameter);
        }
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::FunctionParameter,
                                        Some(result_type),
                                        Some(id),
                                        vec![]);
        self.function.as_mut().unwrap().parameters.push(inst);
        Ok(id)
    }

    /// Begins building of a new basic block.
    pub fn begin_basic_block(&mut self) -> BuildResult<spirv::Word> {
        if self.function.is_none() {
            return Err(Error::DetachedBasicBlock);
        }
        if self.basic_block.is_some() {
            return Err(Error::NestedBasicBlock);
        }

        let id = self.id();

        let mut bb = mr::BasicBlock::new();
        bb.label = Some(mr::Instruction::new(spirv::Op::Label, None, Some(id), vec![]));

        self.basic_block = Some(bb);
        Ok(id)
    }

    fn end_basic_block(&mut self, inst: mr::Instruction) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::MismatchedTerminator);
        }

        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(self.function.as_mut().unwrap().basic_blocks.push(self.basic_block.take().unwrap()))
    }

    /// Appends an OpCapability instruction.
    pub fn capability(&mut self, capability: spirv::Capability) {
        let inst = mr::Instruction::new(spirv::Op::Capability,
                                        None,
                                        None,
                                        vec![mr::Operand::Capability(capability)]);
        self.module.capabilities.push(inst);
    }

    /// Appends an OpExtension instruction.
    pub fn extension(&mut self, extension: String) {
        let inst = mr::Instruction::new(spirv::Op::Extension,
                                        None,
                                        None,
                                        vec![mr::Operand::LiteralString(extension)]);
        self.module.extensions.push(inst);
    }

    /// Appends an OpExtInstImport instruction and returns the result id.
    pub fn ext_inst_import(&mut self, extended_inst_set: String) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::ExtInstImport,
                                        None,
                                        Some(id),
                                        vec![mr::Operand::LiteralString(extended_inst_set)]);
        self.module.ext_inst_imports.push(inst);
        id
    }

    /// Appends an OpMemoryModel instruction.
    pub fn memory_model(&mut self,
                        addressing_model: spirv::AddressingModel,
                        memory_model: spirv::MemoryModel) {
        let inst = mr::Instruction::new(spirv::Op::MemoryModel,
                                        None,
                                        None,
                                        vec![mr::Operand::AddressingModel(addressing_model),
                                             mr::Operand::MemoryModel(memory_model)]);
        self.module.memory_model = Some(inst);
    }

    /// Appends an OpEntryPoint instruction.
    pub fn entry_point(&mut self,
                       execution_model: spirv::ExecutionModel,
                       entry_point: spirv::Word,
                       name: String,
                       interface: Vec<spirv::Word>) {
        let mut operands = vec![mr::Operand::ExecutionModel(execution_model),
                                mr::Operand::IdRef(entry_point),
                                mr::Operand::LiteralString(name)];
        for v in interface {
            operands.push(mr::Operand::IdRef(v));
        }

        let inst = mr::Instruction::new(spirv::Op::EntryPoint, None, None, operands);
        self.module.entry_points.push(inst);
    }

    /// Appends an OpExecutionMode instruction.
    pub fn execution_mode(&mut self,
                          entry_point: spirv::Word,
                          execution_mode: spirv::ExecutionMode,
                          params: Vec<u32>) {
        let mut operands = vec![mr::Operand::IdRef(entry_point),
                                mr::Operand::ExecutionMode(execution_mode)];
        for v in params {
            operands.push(mr::Operand::LiteralInt32(v));
        }

        let inst = mr::Instruction::new(spirv::Op::ExecutionMode, None, None, operands);
        self.module.execution_modes.push(inst);
    }
}

include!("build_type.rs");
include!("build_constant.rs");
include!("build_annotation.rs");
include!("build_terminator.rs");
include!("build_debug.rs");

impl Builder {
    /// Appends an OpDecorationGroup instruction and returns the result id.
    pub fn decoration_group(&mut self) -> spirv::Word {
        let id = self.id();
        self.module
            .annotations
            .push(mr::Instruction::new(spirv::Op::DecorationGroup, None, Some(id), vec![]));
        id
    }

    pub fn string(&mut self, s: String) -> spirv::Word {
        let id = self.id();
        self.module.debugs.push(mr::Instruction::new(spirv::Op::String,
                                                     None,
                                                     Some(id),
                                                     vec![mr::Operand::LiteralString(s)]));
        id
    }

    #[allow(unused_variables)]
    pub fn line(&mut self, file: spirv::Word, line: spirv::Word, column: spirv::Word) {
        unimplemented!()
    }
    pub fn no_line(&mut self) {
        unimplemented!()
    }
}

impl Builder {
    /// Appends an OpConstant instruction with the given 32-bit float `value`.
    /// or the module if no basic block is under construction.
    pub fn constant_f32(&mut self, result_type: spirv::Word, value: f32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::Constant,
                                        Some(result_type),
                                        Some(id),
                                        vec![mr::Operand::LiteralFloat32(value)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstant instruction with the given 32-bit integer `value`.
    /// or the module if no basic block is under construction.
    pub fn constant_u32(&mut self, result_type: spirv::Word, value: u32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::Constant,
                                        Some(result_type),
                                        Some(id),
                                        vec![mr::Operand::LiteralInt32(value)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstant instruction with the given 32-bit float `value`.
    /// or the module if no basic block is under construction.
    pub fn spec_constant_f32(&mut self, result_type: spirv::Word, value: f32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::SpecConstant,
                                        Some(result_type),
                                        Some(id),
                                        vec![mr::Operand::LiteralFloat32(value)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstant instruction with the given 32-bit integer `value`.
    /// or the module if no basic block is under construction.
    pub fn spec_constant_u32(&mut self, result_type: spirv::Word, value: u32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::SpecConstant,
                                        Some(result_type),
                                        Some(id),
                                        vec![mr::Operand::LiteralInt32(value)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpCapability instruction to either the current basic block
    /// or the module if no basic block is under construction.
    pub fn variable(&mut self,
                    result_type: spirv::Word,
                    storage_class: spirv::StorageClass,
                    initializer: Option<spirv::Word>)
                    -> spirv::Word {
        let id = self.id();
        let mut operands = vec![mr::Operand::StorageClass(storage_class)];
        match initializer {
            Some(val) => operands.push(mr::Operand::IdRef(val)),
            None => (),
        }
        let inst = mr::Instruction::new(spirv::Op::Variable, Some(result_type), Some(id), operands);

        match self.basic_block {
            Some(ref mut bb) => bb.instructions.push(inst),
            None => self.module.types_global_values.push(inst),
        }
        id
    }

    /// Appends an OpUndef instruction to either the current basic block
    /// or the module if no basic block is under construction.
    pub fn undef(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::Undef, Some(result_type), Some(id), vec![]);

        match self.basic_block {
            Some(ref mut bb) => bb.instructions.push(inst),
            None => self.module.types_global_values.push(inst),
        }
        id
    }
}

include!("build_norm_insts.rs");

#[cfg(test)]
mod tests {
    use mr;
    use spirv;
    use super::Builder;

    fn has_only_one_pre_inst(module: &mr::Module) -> bool {
        if !module.functions.is_empty() {
            return false;
        }
        (module.capabilities.len() + module.extensions.len() + module.ext_inst_imports.len() +
         module.entry_points.len() + module.types_global_values.len() +
         module.execution_modes.len() +
         module.debugs.len() + module.annotations.len()) +
        (if module.memory_model.is_some() {
            1
        } else {
            0
        }) == 1
    }

    #[test]
    fn test_memory_model() {
        let mut b = Builder::new();
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        let m = b.module();
        assert!(m.memory_model.is_some());
        let inst = m.memory_model.as_ref().unwrap();
        assert!(has_only_one_pre_inst(&m));
        assert_eq!("MemoryModel", inst.class.opname);
        assert_eq!(2, inst.operands.len());
        assert_eq!(mr::Operand::AddressingModel(spirv::AddressingModel::Logical),
                   inst.operands[0]);
        assert_eq!(mr::Operand::MemoryModel(spirv::MemoryModel::Simple),
                   inst.operands[1]);
    }
}
