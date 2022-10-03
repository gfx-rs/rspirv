// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    #[doc = "Appends an OpSourceContinued instruction."]
    pub fn source_continued(&mut self, continued_source: impl Into<String>) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SourceContinued,
            None,
            None,
            vec![dr::Operand::LiteralString(continued_source.into())],
        );
        self.module.debug_string_source.push(inst);
    }
    #[doc = "Appends an OpSource instruction."]
    pub fn source(
        &mut self,
        source_language: spirv::SourceLanguage,
        version: u32,
        file: Option<spirv::Word>,
        source: Option<impl Into<String>>,
    ) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Source,
            None,
            None,
            vec![
                dr::Operand::SourceLanguage(source_language),
                dr::Operand::LiteralBit32(version),
            ],
        );
        if let Some(v) = file {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = source {
            inst.operands.push(dr::Operand::LiteralString(v.into()));
        }
        self.module.debug_string_source.push(inst);
    }
    #[doc = "Appends an OpSourceExtension instruction."]
    pub fn source_extension(&mut self, extension: impl Into<String>) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SourceExtension,
            None,
            None,
            vec![dr::Operand::LiteralString(extension.into())],
        );
        self.module.debug_string_source.push(inst);
    }
    #[doc = "Appends an OpName instruction."]
    pub fn name(&mut self, target: spirv::Word, name: impl Into<String>) {
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
        self.module.debug_names.push(inst);
    }
    #[doc = "Appends an OpMemberName instruction."]
    pub fn member_name(&mut self, ty: spirv::Word, member: u32, name: impl Into<String>) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemberName,
            None,
            None,
            vec![
                dr::Operand::IdRef(ty),
                dr::Operand::LiteralBit32(member),
                dr::Operand::LiteralString(name.into()),
            ],
        );
        self.module.debug_names.push(inst);
    }
    #[doc = "Appends an OpModuleProcessed instruction."]
    pub fn module_processed(&mut self, process: impl Into<String>) {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ModuleProcessed,
            None,
            None,
            vec![dr::Operand::LiteralString(process.into())],
        );
        self.module.debug_module_processed.push(inst);
    }
}
