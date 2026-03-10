// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    #[doc = "Appends an OpConstantTrue instruction."]
    pub fn constant_true(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst =
            dr::Instruction::new(spirv::Op::ConstantTrue, Some(result_type), Some(id), vec![]);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpConstantFalse instruction."]
    pub fn constant_false(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantFalse,
            Some(result_type),
            Some(id),
            vec![],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpConstantComposite instruction."]
    pub fn constant_composite(
        &mut self,
        result_type: spirv::Word,
        constituents: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantComposite,
            Some(result_type),
            Some(id),
            vec![],
        );
        inst.operands
            .extend(constituents.into_iter().map(dr::Operand::IdRef));
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpConstantSampler instruction."]
    pub fn constant_sampler(
        &mut self,
        result_type: spirv::Word,
        sampler_addressing_mode: spirv::SamplerAddressingMode,
        param: u32,
        sampler_filter_mode: spirv::SamplerFilterMode,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantSampler,
            Some(result_type),
            Some(id),
            vec![
                dr::Operand::SamplerAddressingMode(sampler_addressing_mode),
                dr::Operand::LiteralBit32(param),
                dr::Operand::SamplerFilterMode(sampler_filter_mode),
            ],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpConstantNull instruction."]
    pub fn constant_null(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst =
            dr::Instruction::new(spirv::Op::ConstantNull, Some(result_type), Some(id), vec![]);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpSpecConstantTrue instruction."]
    pub fn spec_constant_true(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantTrue,
            Some(result_type),
            Some(id),
            vec![],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpSpecConstantFalse instruction."]
    pub fn spec_constant_false(&mut self, result_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantFalse,
            Some(result_type),
            Some(id),
            vec![],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpSpecConstantComposite instruction."]
    pub fn spec_constant_composite(
        &mut self,
        result_type: spirv::Word,
        constituents: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantComposite,
            Some(result_type),
            Some(id),
            vec![],
        );
        inst.operands
            .extend(constituents.into_iter().map(dr::Operand::IdRef));
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpSpecConstantOp instruction."]
    pub fn spec_constant_op(&mut self, result_type: spirv::Word, opcode: spirv::Op) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantOp,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralSpecConstantOpInteger(opcode)],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpConstantCompositeReplicateEXT instruction."]
    pub fn constant_composite_replicate_ext(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantCompositeReplicateEXT,
            Some(result_type),
            Some(id),
            vec![dr::Operand::IdRef(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpSpecConstantCompositeReplicateEXT instruction."]
    pub fn spec_constant_composite_replicate_ext(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantCompositeReplicateEXT,
            Some(result_type),
            Some(id),
            vec![dr::Operand::IdRef(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpConstantSizeOfEXT instruction."]
    pub fn constant_size_of_ext(
        &mut self,
        result_type: spirv::Word,
        ty: spirv::Word,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantSizeOfEXT,
            Some(result_type),
            Some(id),
            vec![dr::Operand::IdRef(ty)],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpSpecConstantTargetINTEL instruction."]
    pub fn spec_constant_target_intel(
        &mut self,
        result_type: spirv::Word,
        target: u32,
        features: impl IntoIterator<Item = u32>,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantTargetINTEL,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralBit32(target)],
        );
        inst.operands
            .extend(features.into_iter().map(dr::Operand::LiteralBit32));
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpSpecConstantArchitectureINTEL instruction."]
    pub fn spec_constant_architecture_intel(
        &mut self,
        result_type: spirv::Word,
        category: u32,
        family: u32,
        opcode: u32,
        architecture: u32,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantArchitectureINTEL,
            Some(result_type),
            Some(id),
            vec![
                dr::Operand::LiteralBit32(category),
                dr::Operand::LiteralBit32(family),
                dr::Operand::LiteralBit32(opcode),
                dr::Operand::LiteralBit32(architecture),
            ],
        );
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpSpecConstantCapabilitiesINTEL instruction."]
    pub fn spec_constant_capabilities_intel(
        &mut self,
        result_type: spirv::Word,
        capabilities: impl IntoIterator<Item = spirv::Capability>,
    ) -> spirv::Word {
        let id = self.id();
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantCapabilitiesINTEL,
            Some(result_type),
            Some(id),
            vec![],
        );
        inst.operands
            .extend(capabilities.into_iter().map(dr::Operand::Capability));
        self.module.types_global_values.push(inst);
        id
    }
}
