// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    #[doc = "Appends an OpDecorate instruction."]
    pub fn decorate(
        &mut self,
        target: spirv::Word,
        decoration: spirv::Decoration,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Decorate,
            None,
            None,
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend(additional_params);
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpMemberDecorate instruction."]
    pub fn member_decorate(
        &mut self,
        structure_type: spirv::Word,
        member: u32,
        decoration: spirv::Decoration,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemberDecorate,
            None,
            None,
            vec![
                dr::Operand::IdRef(structure_type),
                dr::Operand::LiteralBit32(member),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend(additional_params);
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpGroupDecorate instruction."]
    pub fn group_decorate(
        &mut self,
        decoration_group: spirv::Word,
        targets: impl IntoIterator<Item = spirv::Word>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupDecorate,
            None,
            None,
            vec![dr::Operand::IdRef(decoration_group)],
        );
        inst.operands
            .extend(targets.into_iter().map(dr::Operand::IdRef));
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpGroupMemberDecorate instruction."]
    pub fn group_member_decorate(
        &mut self,
        decoration_group: spirv::Word,
        targets: impl IntoIterator<Item = (spirv::Word, u32)>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupMemberDecorate,
            None,
            None,
            vec![dr::Operand::IdRef(decoration_group)],
        );
        for v in targets {
            inst.operands.push(dr::Operand::IdRef(v.0));
            inst.operands.push(dr::Operand::LiteralBit32(v.1));
        }
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpDecorateId instruction."]
    pub fn decorate_id(
        &mut self,
        target: spirv::Word,
        decoration: spirv::Decoration,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DecorateId,
            None,
            None,
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend(additional_params);
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpDecorateString instruction."]
    pub fn decorate_string(
        &mut self,
        target: spirv::Word,
        decoration: spirv::Decoration,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DecorateString,
            None,
            None,
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend(additional_params);
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpMemberDecorateString instruction."]
    pub fn member_decorate_string(
        &mut self,
        struct_type: spirv::Word,
        member: u32,
        decoration: spirv::Decoration,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemberDecorateString,
            None,
            None,
            vec![
                dr::Operand::IdRef(struct_type),
                dr::Operand::LiteralBit32(member),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend(additional_params);
        self.module.annotations.push(inst);
    }
}
