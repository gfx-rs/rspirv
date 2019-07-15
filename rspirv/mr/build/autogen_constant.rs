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
    /// Appends an OpConstantTrue instruction.
    pub fn constant_true(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::ConstantTrue, Some(result_type), Some(id), vec![]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstantFalse instruction.
    pub fn constant_false(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::ConstantFalse, Some(result_type), Some(id), vec![]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstantComposite instruction.
    pub fn constant_composite<T: AsRef<[spirv::Word]>>(&mut self, result_type: spirv::Word, constituents: T) -> spirv::Word {
        let id = self.id();
        let mut inst = mr::Instruction::new(spirv::Op::ConstantComposite, Some(result_type), Some(id), vec![]);
        for v in constituents.as_ref() {
            inst.operands.push(mr::Operand::IdRef(*v))
        };
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstantSampler instruction.
    pub fn constant_sampler(&mut self, result_type: spirv::Word, sampler_addressing_mode: spirv::SamplerAddressingMode, param: u32, sampler_filter_mode: spirv::SamplerFilterMode) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::ConstantSampler, Some(result_type), Some(id), vec![mr::Operand::SamplerAddressingMode(sampler_addressing_mode), mr::Operand::LiteralInt32(param), mr::Operand::SamplerFilterMode(sampler_filter_mode)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstantNull instruction.
    pub fn constant_null(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::ConstantNull, Some(result_type), Some(id), vec![]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstantTrue instruction.
    pub fn spec_constant_true(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::SpecConstantTrue, Some(result_type), Some(id), vec![]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstantFalse instruction.
    pub fn spec_constant_false(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::SpecConstantFalse, Some(result_type), Some(id), vec![]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstantComposite instruction.
    pub fn spec_constant_composite<T: AsRef<[spirv::Word]>>(&mut self, result_type: spirv::Word, constituents: T) -> spirv::Word {
        let id = self.id();
        let mut inst = mr::Instruction::new(spirv::Op::SpecConstantComposite, Some(result_type), Some(id), vec![]);
        for v in constituents.as_ref() {
            inst.operands.push(mr::Operand::IdRef(*v))
        };
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstantOp instruction.
    pub fn spec_constant_op(&mut self, result_type: spirv::Word, opcode: spirv::Op) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::SpecConstantOp, Some(result_type), Some(id), vec![mr::Operand::LiteralSpecConstantOpInteger(opcode)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstantPipeStorage instruction.
    pub fn constant_pipe_storage(&mut self, result_type: spirv::Word, packet_size: u32, packet_alignment: u32, capacity: u32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::ConstantPipeStorage, Some(result_type), Some(id), vec![mr::Operand::LiteralInt32(packet_size), mr::Operand::LiteralInt32(packet_alignment), mr::Operand::LiteralInt32(capacity)]);
        self.module.types_global_values.push(inst);
        id
    }
}