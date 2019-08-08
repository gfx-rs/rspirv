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
    #[doc = "Appends an OpBranch instruction and ends the current basic block."]
    pub fn branch(&mut self, target_label: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Branch,
            None,
            None,
            vec![dr::Operand::IdRef(target_label)],
        );
        self.end_basic_block(inst)
    }
    #[doc = "Appends an OpBranchConditional instruction and ends the current basic block."]
    pub fn branch_conditional<T: AsRef<[u32]>>(
        &mut self,
        condition: spirv::Word,
        true_label: spirv::Word,
        false_label: spirv::Word,
        branch_weights: T,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BranchConditional,
            None,
            None,
            vec![
                dr::Operand::IdRef(condition),
                dr::Operand::IdRef(true_label),
                dr::Operand::IdRef(false_label),
            ],
        );
        for v in branch_weights.as_ref() {
            inst.operands.push(dr::Operand::LiteralInt32(*v));
        }
        self.end_basic_block(inst)
    }
    #[doc = "Appends an OpSwitch instruction and ends the current basic block."]
    pub fn switch<T: AsRef<[(u32, spirv::Word)]>>(
        &mut self,
        selector: spirv::Word,
        default: spirv::Word,
        target: T,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Switch,
            None,
            None,
            vec![dr::Operand::IdRef(selector), dr::Operand::IdRef(default)],
        );
        for v in target.as_ref() {
            inst.operands.push(dr::Operand::LiteralInt32(v.0));
            inst.operands.push(dr::Operand::IdRef(v.1));
        }
        self.end_basic_block(inst)
    }
    #[doc = "Appends an OpKill instruction and ends the current basic block."]
    pub fn kill(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Kill, None, None, vec![]);
        self.end_basic_block(inst)
    }
    #[doc = "Appends an OpReturn instruction and ends the current basic block."]
    pub fn ret(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Return, None, None, vec![]);
        self.end_basic_block(inst)
    }
    #[doc = "Appends an OpReturnValue instruction and ends the current basic block."]
    pub fn ret_value(&mut self, value: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReturnValue,
            None,
            None,
            vec![dr::Operand::IdRef(value)],
        );
        self.end_basic_block(inst)
    }
    #[doc = "Appends an OpUnreachable instruction and ends the current basic block."]
    pub fn unreachable(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Unreachable, None, None, vec![]);
        self.end_basic_block(inst)
    }
}
