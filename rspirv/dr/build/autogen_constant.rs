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
}
