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
/// method calls for various instructions. Most of the methods return the
/// SPIR-V id assigned for that SPIR-V instruction.
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
        bb.label = Some(mr::Instruction::new(spirv::Op::Label, None, None, vec![]));

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
include!("build_terminator.rs");

impl Builder {
    pub fn string(&mut self, s: String) -> spirv::Word {
        let id = self.id();
        self.module.debugs.push(mr::Instruction::new(spirv::Op::String,
                                                     None,
                                                     Some(id),
                                                     vec![mr::Operand::LiteralString(s)]));
        id
    }

    pub fn source(&mut self,
                  language: spirv::SourceLanguage,
                  version: u32,
                  file: Option<spirv::Word>,
                  source: Option<String>) {
        let mut operands = vec![mr::Operand::SourceLanguage(language),
                                mr::Operand::LiteralInt32(version)];
        match file {
            Some(f) => operands.push(mr::Operand::IdRef(f)),
            None => (),
        }
        match source {
            Some(s) => operands.push(mr::Operand::LiteralString(s)),
            None => (),
        }
        self.module.debugs.push(mr::Instruction::new(spirv::Op::Source, None, None, operands));
    }

    pub fn source_extension(&mut self, extension: String) {
        self.module.debugs.push(mr::Instruction::new(spirv::Op::SourceExtension,
                                                     None,
                                                     None,
                                                     vec![mr::Operand::LiteralString(extension)]));
    }

    pub fn source_continued(&mut self, source: String) {
        self.module.debugs.push(mr::Instruction::new(spirv::Op::SourceContinued,
                                                     None,
                                                     None,
                                                     vec![mr::Operand::LiteralString(source)]));
    }

    pub fn name(&mut self, target: spirv::Word, name: String) {
        self.module.debugs.push(mr::Instruction::new(spirv::Op::Name,
                                                     None,
                                                     None,
                                                     vec![mr::Operand::IdRef(target),
                                                          mr::Operand::LiteralString(name)]));
    }

    pub fn member_name(&mut self, target_type: spirv::Word, member: u32, name: String) {
        self.module.debugs.push(mr::Instruction::new(spirv::Op::MemberName,
                                                     None,
                                                     None,
                                                     vec![mr::Operand::IdRef(target_type),
                                                          mr::Operand::LiteralInt32(member),
                                                          mr::Operand::LiteralString(name)]));
    }
}

impl Builder {
    /// Appends an OpDecorate instruction and returns the result id.
    pub fn decorate(&mut self,
                    target: spirv::Word,
                    decoration: spirv::Decoration,
                    mut params: Vec<mr::Operand>)
                    -> spirv::Word {
        let id = self.id();
        let mut operands = vec![mr::Operand::IdRef(target), mr::Operand::Decoration(decoration)];
        operands.append(&mut params);
        self.module
            .annotations
            .push(mr::Instruction::new(spirv::Op::Decorate, None, Some(id), operands));
        id
    }

    /// Appends an OpMemberDecorate instruction and returns the result id.
    pub fn member_decorate(&mut self,
                           structure: spirv::Word,
                           member: spirv::Word,
                           decoration: spirv::Decoration,
                           mut params: Vec<mr::Operand>)
                           -> spirv::Word {
        let id = self.id();
        let mut operands = vec![mr::Operand::IdRef(structure),
                                mr::Operand::IdRef(member),
                                mr::Operand::Decoration(decoration)];
        operands.append(&mut params);
        self.module
            .annotations
            .push(mr::Instruction::new(spirv::Op::MemberDecorate, None, Some(id), operands));
        id
    }

    /// Appends an OpDecorationGroup instruction and returns the result id.
    pub fn decoration_group(&mut self) -> spirv::Word {
        let id = self.id();
        self.module
            .annotations
            .push(mr::Instruction::new(spirv::Op::DecorationGroup, None, Some(id), vec![]));
        id
    }

    /// Appends an OpGroupDecorate instruction and returns the result id.
    pub fn group_decorate(&mut self, group: spirv::Word, targets: Vec<spirv::Word>) -> spirv::Word {
        let id = self.id();
        let mut operands = vec![mr::Operand::IdRef(group)];
        for v in targets {
            operands.push(mr::Operand::IdRef(v));
        }
        self.module
            .annotations
            .push(mr::Instruction::new(spirv::Op::GroupDecorate, None, Some(id), operands));
        id
    }

    /// Appends an OpGroupMemberDecorate instruction and returns the result id.
    pub fn group_member_decorate(&mut self,
                                 group: spirv::Word,
                                 targets: Vec<(spirv::Word, u32)>)
                                 -> spirv::Word {
        let id = self.id();
        let mut operands = vec![mr::Operand::IdRef(group)];
        for (target, member) in targets {
            operands.push(mr::Operand::IdRef(target));
            operands.push(mr::Operand::LiteralInt32(member));
        }
        self.module
            .annotations
            .push(mr::Instruction::new(spirv::Op::GroupMemberDecorate, None, Some(id), operands));
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
