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
    #[doc = "Appends an OpSourceContinued instruction."]
    pub fn source_continued<T: Into<String>>(&mut self, continued_source: T) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SourceContinued,
            None,
            None,
            vec![dr::Operand::LiteralString(continued_source.into())],
        );
        self.module.debugs.push(inst);
    }
    #[doc = "Appends an OpSource instruction."]
    pub fn source<T: Into<String>>(
        &mut self,
        source_language: spirv::SourceLanguage,
        version: u32,
        file: Option<spirv::Word>,
        source: Option<T>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Source,
            None,
            None,
            vec![
                dr::Operand::SourceLanguage(source_language),
                dr::Operand::LiteralInt32(version),
            ],
        );
        if let Some(v) = file {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        if let Some(v) = source {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::LiteralString(v.into()));
        }
        self.module.debugs.push(inst);
    }
    #[doc = "Appends an OpSourceExtension instruction."]
    pub fn source_extension<T: Into<String>>(&mut self, extension: T) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SourceExtension,
            None,
            None,
            vec![dr::Operand::LiteralString(extension.into())],
        );
        self.module.debugs.push(inst);
    }
    #[doc = "Appends an OpName instruction."]
    pub fn name<T: Into<String>>(&mut self, target: spirv::Word, name: T) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Name,
            None,
            None,
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::LiteralString(name.into()),
            ],
        );
        self.module.debugs.push(inst);
    }
    #[doc = "Appends an OpMemberName instruction."]
    pub fn member_name<T: Into<String>>(&mut self, target_type: spirv::Word, member: u32, name: T) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemberName,
            None,
            None,
            vec![
                dr::Operand::IdRef(target_type),
                dr::Operand::LiteralInt32(member),
                dr::Operand::LiteralString(name.into()),
            ],
        );
        self.module.debugs.push(inst);
    }
    #[doc = "Appends an OpModuleProcessed instruction."]
    pub fn module_processed<T: Into<String>>(&mut self, process: T) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ModuleProcessed,
            None,
            None,
            vec![dr::Operand::LiteralString(process.into())],
        );
        self.module.debugs.push(inst);
    }
}
