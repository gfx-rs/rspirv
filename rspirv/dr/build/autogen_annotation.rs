// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    #[doc = "Appends an OpDecorate instruction."]
    pub fn decorate<T: AsRef<[dr::Operand]>>(
        &mut self,
        target: spirv::Word,
        decoration: spirv::Decoration,
        additional_params: T,
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpMemberDecorate instruction."]
    pub fn member_decorate<T: AsRef<[dr::Operand]>>(
        &mut self,
        structure_type: spirv::Word,
        member: u32,
        decoration: spirv::Decoration,
        additional_params: T,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemberDecorate,
            None,
            None,
            vec![
                dr::Operand::IdRef(structure_type),
                dr::Operand::LiteralInt32(member),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpGroupDecorate instruction."]
    pub fn group_decorate<T: AsRef<[spirv::Word]>>(
        &mut self,
        decoration_group: spirv::Word,
        targets: T,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupDecorate,
            None,
            None,
            vec![dr::Operand::IdRef(decoration_group)],
        );
        for v in targets.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpGroupMemberDecorate instruction."]
    pub fn group_member_decorate<T: AsRef<[(spirv::Word, u32)]>>(
        &mut self,
        decoration_group: spirv::Word,
        targets: T,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupMemberDecorate,
            None,
            None,
            vec![dr::Operand::IdRef(decoration_group)],
        );
        for v in targets.as_ref() {
            inst.operands.push(dr::Operand::IdRef(v.0));
            inst.operands.push(dr::Operand::LiteralInt32(v.1));
        }
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpDecorateId instruction."]
    pub fn decorate_id<T: AsRef<[dr::Operand]>>(
        &mut self,
        target: spirv::Word,
        decoration: spirv::Decoration,
        additional_params: T,
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpDecorateString instruction."]
    pub fn decorate_string<T: AsRef<[dr::Operand]>>(
        &mut self,
        target: spirv::Word,
        decoration: spirv::Decoration,
        additional_params: T,
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpDecorateStringGOOGLE instruction."]
    pub fn decorate_string_google<T: AsRef<[dr::Operand]>>(
        &mut self,
        target: spirv::Word,
        decoration: spirv::Decoration,
        additional_params: T,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DecorateStringGOOGLE,
            None,
            None,
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpMemberDecorateString instruction."]
    pub fn member_decorate_string<T: AsRef<[dr::Operand]>>(
        &mut self,
        struct_type: spirv::Word,
        member: u32,
        decoration: spirv::Decoration,
        additional_params: T,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemberDecorateString,
            None,
            None,
            vec![
                dr::Operand::IdRef(struct_type),
                dr::Operand::LiteralInt32(member),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.module.annotations.push(inst);
    }
    #[doc = "Appends an OpMemberDecorateStringGOOGLE instruction."]
    pub fn member_decorate_string_google<T: AsRef<[dr::Operand]>>(
        &mut self,
        struct_type: spirv::Word,
        member: u32,
        decoration: spirv::Decoration,
        additional_params: T,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemberDecorateStringGOOGLE,
            None,
            None,
            vec![
                dr::Operand::IdRef(struct_type),
                dr::Operand::LiteralInt32(member),
                dr::Operand::Decoration(decoration),
            ],
        );
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.module.annotations.push(inst);
    }
}
