// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[doc = "Extended instruction operand kinds."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum ExtOperandKind {
    KernelPropertyFlags,
}
static NONSEMANTIC_CLSPVREFLECTION_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        Kernel,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne),
            (OperandKind::IdRef, ZeroOrOne),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentInfo,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne),
            (OperandKind::IdRef, ZeroOrOne),
            (OperandKind::IdRef, ZeroOrOne),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentStorageBuffer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentUniform,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentPodStorageBuffer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentPodUniform,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentPodPushConstant,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentSampledImage,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentStorageImage,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentSampler,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentWorkgroup,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        SpecConstantWorkgroupSize,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        SpecConstantGlobalOffset,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        SpecConstantWorkDim,
        [],
        [],
        [(OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PushConstantGlobalOffset,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PushConstantEnqueuedLocalSize,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PushConstantGlobalSize,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PushConstantRegionOffset,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PushConstantNumWorkgroups,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PushConstantRegionGroupOffset,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ConstantDataStorageBuffer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ConstantDataUniform,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        LiteralSampler,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PropertyRequiredWorkgroupSize,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        SpecConstantSubgroupMaxSize,
        [],
        [],
        [(OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentPointerPushConstant,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentPointerUniform,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ProgramScopeVariablesStorageBuffer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ProgramScopeVariablePointerRelocation,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ImageArgumentInfoChannelOrderPushConstant,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ImageArgumentInfoChannelDataTypePushConstant,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ImageArgumentInfoChannelOrderUniform,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ImageArgumentInfoChannelDataTypeUniform,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentStorageTexelBuffer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ArgumentUniformTexelBuffer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ConstantDataPointerPushConstant,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        ProgramScopeVariablePointerPushConstant,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PrintfInfo,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PrintfBufferStorageBuffer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        PrintfBufferPointerPushConstant,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        NormalizedSamplerMaskPushConstant,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticClspvreflection,
        NonsemanticClspvreflectionOp,
        WorkgroupVariableSize,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
];
pub static NONSEMANTIC_CLSPVREFLECTION_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        NONSEMANTIC_CLSPVREFLECTION_INSTRUCTIONS,
        std::marker::PhantomData,
    );
