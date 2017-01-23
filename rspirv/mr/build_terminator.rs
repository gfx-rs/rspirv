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
    pub fn branch(&mut self, target_label: spirv::Word) -> BuildResult<()> {
        let inst = mr::Instruction::new(spirv::Op::Branch, None, None, vec![mr::Operand::IdRef(target_label)]);
        self.end_basic_block(inst)
    }

    pub fn branch_conditional(&mut self, condition: spirv::Word, true_label: spirv::Word, false_label: spirv::Word, branch_weights: Vec<spirv::Word>) -> BuildResult<()> {
        let mut inst = mr::Instruction::new(spirv::Op::BranchConditional, None, None, vec![mr::Operand::IdRef(condition), mr::Operand::IdRef(true_label), mr::Operand::IdRef(false_label)]);
        for v in branch_weights {
            inst.operands.push(mr::Operand::LiteralInt32(v))
        };
        self.end_basic_block(inst)
    }

    pub fn switch(&mut self, selector: spirv::Word, default: spirv::Word, target: Vec<(spirv::Word, spirv::Word)>) -> BuildResult<()> {
        let mut inst = mr::Instruction::new(spirv::Op::Switch, None, None, vec![mr::Operand::IdRef(selector), mr::Operand::IdRef(default)]);
        for v in target {
            inst.operands.push(mr::Operand::LiteralInt32(v.0));
            inst.operands.push(mr::Operand::IdRef(v.1));
        };
        self.end_basic_block(inst)
    }

    pub fn kill(&mut self) -> BuildResult<()> {
        let inst = mr::Instruction::new(spirv::Op::Kill, None, None, vec![]);
        self.end_basic_block(inst)
    }

    pub fn ret(&mut self) -> BuildResult<()> {
        let inst = mr::Instruction::new(spirv::Op::Return, None, None, vec![]);
        self.end_basic_block(inst)
    }

    pub fn ret_value(&mut self, value: spirv::Word) -> BuildResult<()> {
        let inst = mr::Instruction::new(spirv::Op::ReturnValue, None, None, vec![mr::Operand::IdRef(value)]);
        self.end_basic_block(inst)
    }

    pub fn unreachable(&mut self) -> BuildResult<()> {
        let inst = mr::Instruction::new(spirv::Op::Unreachable, None, None, vec![]);
        self.end_basic_block(inst)
    }
}