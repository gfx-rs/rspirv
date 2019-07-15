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

// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    /// Appends an OpSourceContinued instruction.
    pub fn source_continued<T: Into<String>>(&mut self, continued_source: T) {
        let inst = mr::Instruction::new(spirv::Op::SourceContinued, None, None, vec![mr::Operand::LiteralString(continued_source.into())]);
        self.module.debugs.push(inst);
    }

    /// Appends an OpSource instruction.
    pub fn source<T: Into<String>>(&mut self, source_language: spirv::SourceLanguage, version: u32, file: Option<spirv::Word>, source: Option<T>) {
        let mut inst = mr::Instruction::new(spirv::Op::Source, None, None, vec![mr::Operand::SourceLanguage(source_language), mr::Operand::LiteralInt32(version)]);
        if let Some(v) = file {
            inst.operands.push(mr::Operand::IdRef(v));
        };
        if let Some(v) = source {
            inst.operands.push(mr::Operand::LiteralString(v.into()));
        };
        self.module.debugs.push(inst);
    }

    /// Appends an OpSourceExtension instruction.
    pub fn source_extension<T: Into<String>>(&mut self, extension: T) {
        let inst = mr::Instruction::new(spirv::Op::SourceExtension, None, None, vec![mr::Operand::LiteralString(extension.into())]);
        self.module.debugs.push(inst);
    }

    /// Appends an OpName instruction.
    pub fn name<T: Into<String>>(&mut self, target: spirv::Word, name: T) {
        let inst = mr::Instruction::new(spirv::Op::Name, None, None, vec![mr::Operand::IdRef(target), mr::Operand::LiteralString(name.into())]);
        self.module.debugs.push(inst);
    }

    /// Appends an OpMemberName instruction.
    pub fn member_name<T: Into<String>>(&mut self, target_type: spirv::Word, member: u32, name: T) {
        let inst = mr::Instruction::new(spirv::Op::MemberName, None, None, vec![mr::Operand::IdRef(target_type), mr::Operand::LiteralInt32(member), mr::Operand::LiteralString(name.into())]);
        self.module.debugs.push(inst);
    }

    /// Appends an OpModuleProcessed instruction.
    pub fn module_processed<T: Into<String>>(&mut self, process: T) {
        let inst = mr::Instruction::new(spirv::Op::ModuleProcessed, None, None, vec![mr::Operand::LiteralString(process.into())]);
        self.module.debugs.push(inst);
    }
}