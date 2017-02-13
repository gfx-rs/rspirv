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
    /// Appends an OpDecorate instruction.
    pub fn decorate(&mut self, target: spirv::Word, decoration: spirv::Decoration, mut additional_params: Vec<mr::Operand>) {
        let mut inst = mr::Instruction::new(spirv::Op::Decorate, None, None, vec![mr::Operand::IdRef(target), mr::Operand::Decoration(decoration)]);
        inst.operands.append(&mut additional_params);
        self.module.annotations.push(inst);
    }

    /// Appends an OpMemberDecorate instruction.
    pub fn member_decorate(&mut self, structure_type: spirv::Word, member: u32, decoration: spirv::Decoration, mut additional_params: Vec<mr::Operand>) {
        let mut inst = mr::Instruction::new(spirv::Op::MemberDecorate, None, None, vec![mr::Operand::IdRef(structure_type), mr::Operand::LiteralInt32(member), mr::Operand::Decoration(decoration)]);
        inst.operands.append(&mut additional_params);
        self.module.annotations.push(inst);
    }

    /// Appends an OpGroupDecorate instruction.
    pub fn group_decorate(&mut self, decoration_group: spirv::Word, targets: Vec<spirv::Word>) {
        let mut inst = mr::Instruction::new(spirv::Op::GroupDecorate, None, None, vec![mr::Operand::IdRef(decoration_group)]);
        for v in targets {
            inst.operands.push(mr::Operand::IdRef(v))
        };
        self.module.annotations.push(inst);
    }

    /// Appends an OpGroupMemberDecorate instruction.
    pub fn group_member_decorate(&mut self, decoration_group: spirv::Word, targets: Vec<(spirv::Word, u32)>) {
        let mut inst = mr::Instruction::new(spirv::Op::GroupMemberDecorate, None, None, vec![mr::Operand::IdRef(decoration_group)]);
        for v in targets {
            inst.operands.push(mr::Operand::IdRef(v.0));
            inst.operands.push(mr::Operand::LiteralInt32(v.1));
        };
        self.module.annotations.push(inst);
    }
}