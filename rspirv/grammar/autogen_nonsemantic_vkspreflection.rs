// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static NONSEMANTIC_VKSPREFLECTION_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        NonsemanticVkspreflection,
        NonsemanticVkspreflectionOp,
        Configuration,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(
        NonsemanticVkspreflection,
        NonsemanticVkspreflectionOp,
        StartCounter,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        NonsemanticVkspreflection,
        NonsemanticVkspreflectionOp,
        StopCounter,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        NonsemanticVkspreflection,
        NonsemanticVkspreflectionOp,
        PushConstants,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        NonsemanticVkspreflection,
        NonsemanticVkspreflectionOp,
        SpecializationMapEntry,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        NonsemanticVkspreflection,
        NonsemanticVkspreflectionOp,
        DescriptorSetBuffer,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(
        NonsemanticVkspreflection,
        NonsemanticVkspreflectionOp,
        DescriptorSetImage,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(
        NonsemanticVkspreflection,
        NonsemanticVkspreflectionOp,
        DescriptorSetSampler,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
];
pub static NONSEMANTIC_VKSPREFLECTION_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        NONSEMANTIC_VKSPREFLECTION_INSTRUCTIONS,
        std::marker::PhantomData,
    );
