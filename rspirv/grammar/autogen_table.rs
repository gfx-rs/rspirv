// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[doc = "All operand kinds in the SPIR-V grammar."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum OperandKind {
    ImageOperands,
    FPFastMathMode,
    SelectionControl,
    LoopControl,
    FunctionControl,
    MemorySemantics,
    MemoryAccess,
    KernelProfilingInfo,
    RayFlags,
    FragmentShadingRate,
    SourceLanguage,
    ExecutionModel,
    AddressingModel,
    MemoryModel,
    ExecutionMode,
    StorageClass,
    Dim,
    SamplerAddressingMode,
    SamplerFilterMode,
    ImageFormat,
    ImageChannelOrder,
    ImageChannelDataType,
    FPRoundingMode,
    FPDenormMode,
    QuantizationModes,
    FPOperationMode,
    OverflowModes,
    LinkageType,
    AccessQualifier,
    HostAccessQualifier,
    FunctionParameterAttribute,
    Decoration,
    BuiltIn,
    Scope,
    GroupOperation,
    KernelEnqueueFlags,
    Capability,
    RayQueryIntersection,
    RayQueryCommittedIntersectionType,
    RayQueryCandidateIntersectionType,
    PackedVectorFormat,
    CooperativeMatrixOperands,
    CooperativeMatrixLayout,
    CooperativeMatrixUse,
    InitializationModeQualifier,
    LoadCacheControl,
    StoreCacheControl,
    IdResultType,
    IdResult,
    IdMemorySemantics,
    IdScope,
    IdRef,
    LiteralInteger,
    LiteralString,
    LiteralFloat,
    LiteralContextDependentNumber,
    LiteralExtInstInteger,
    LiteralSpecConstantOpInteger,
    PairLiteralIntegerIdRef,
    PairIdRefLiteralInteger,
    PairIdRefIdRef,
}
static INSTRUCTION_TABLE: &[Instruction<'static>] = &[
    inst!(Nop, [], [], []),
    inst!(Undef, [], [], [(IdResultType, One), (IdResult, One)]),
    inst!(SourceContinued, [], [], [(LiteralString, One)]),
    inst!(
        Source,
        [],
        [],
        [
            (SourceLanguage, One),
            (LiteralInteger, One),
            (IdRef, ZeroOrOne),
            (LiteralString, ZeroOrOne)
        ]
    ),
    inst!(SourceExtension, [], [], [(LiteralString, One)]),
    inst!(Name, [], [], [(IdRef, One), (LiteralString, One)]),
    inst!(
        MemberName,
        [],
        [],
        [(IdRef, One), (LiteralInteger, One), (LiteralString, One)]
    ),
    inst!(String, [], [], [(IdResult, One), (LiteralString, One)]),
    inst!(
        Line,
        [],
        [],
        [(IdRef, One), (LiteralInteger, One), (LiteralInteger, One)]
    ),
    inst!(Extension, [], [], [(LiteralString, One)]),
    inst!(
        ExtInstImport,
        [],
        [],
        [(IdResult, One), (LiteralString, One)]
    ),
    inst!(
        ExtInst,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralExtInstInteger, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(
        MemoryModel,
        [],
        [],
        [(AddressingModel, One), (MemoryModel, One)]
    ),
    inst!(
        EntryPoint,
        [],
        [],
        [
            (ExecutionModel, One),
            (IdRef, One),
            (LiteralString, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(ExecutionMode, [], [], [(IdRef, One), (ExecutionMode, One)]),
    inst!(Capability, [], [], [(Capability, One)]),
    inst!(TypeVoid, [], [], [(IdResult, One)]),
    inst!(TypeBool, [], [], [(IdResult, One)]),
    inst!(
        TypeInt,
        [],
        [],
        [
            (IdResult, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(TypeFloat, [], [], [(IdResult, One), (LiteralInteger, One)]),
    inst!(
        TypeVector,
        [],
        [],
        [(IdResult, One), (IdRef, One), (LiteralInteger, One)]
    ),
    inst!(
        TypeMatrix,
        [Matrix],
        [],
        [(IdResult, One), (IdRef, One), (LiteralInteger, One)]
    ),
    inst!(
        TypeImage,
        [],
        [],
        [
            (IdResult, One),
            (IdRef, One),
            (Dim, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (ImageFormat, One),
            (AccessQualifier, ZeroOrOne)
        ]
    ),
    inst!(TypeSampler, [], [], [(IdResult, One)]),
    inst!(TypeSampledImage, [], [], [(IdResult, One), (IdRef, One)]),
    inst!(
        TypeArray,
        [],
        [],
        [(IdResult, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        TypeRuntimeArray,
        [Shader],
        [],
        [(IdResult, One), (IdRef, One)]
    ),
    inst!(TypeStruct, [], [], [(IdResult, One), (IdRef, ZeroOrMore)]),
    inst!(
        TypeOpaque,
        [Kernel],
        [],
        [(IdResult, One), (LiteralString, One)]
    ),
    inst!(
        TypePointer,
        [],
        [],
        [(IdResult, One), (StorageClass, One), (IdRef, One)]
    ),
    inst!(
        TypeFunction,
        [],
        [],
        [(IdResult, One), (IdRef, One), (IdRef, ZeroOrMore)]
    ),
    inst!(TypeEvent, [Kernel], [], [(IdResult, One)]),
    inst!(TypeDeviceEvent, [DeviceEnqueue], [], [(IdResult, One)]),
    inst!(TypeReserveId, [Pipes], [], [(IdResult, One)]),
    inst!(TypeQueue, [DeviceEnqueue], [], [(IdResult, One)]),
    inst!(
        TypePipe,
        [Pipes],
        [],
        [(IdResult, One), (AccessQualifier, One)]
    ),
    inst!(
        TypeForwardPointer,
        [Addresses, PhysicalStorageBufferAddresses],
        [],
        [(IdRef, One), (StorageClass, One)]
    ),
    inst!(ConstantTrue, [], [], [(IdResultType, One), (IdResult, One)]),
    inst!(
        ConstantFalse,
        [],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        Constant,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (LiteralContextDependentNumber, One)
        ]
    ),
    inst!(
        ConstantComposite,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        ConstantSampler,
        [LiteralSampler],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (SamplerAddressingMode, One),
            (LiteralInteger, One),
            (SamplerFilterMode, One)
        ]
    ),
    inst!(ConstantNull, [], [], [(IdResultType, One), (IdResult, One)]),
    inst!(
        SpecConstantTrue,
        [],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        SpecConstantFalse,
        [],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        SpecConstant,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (LiteralContextDependentNumber, One)
        ]
    ),
    inst!(
        SpecConstantComposite,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        SpecConstantOp,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (LiteralSpecConstantOpInteger, One)
        ]
    ),
    inst!(
        Function,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (FunctionControl, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FunctionParameter,
        [],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(FunctionEnd, [], [], []),
    inst!(
        FunctionCall,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(
        Variable,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (StorageClass, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        ImageTexelPointer,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        Load,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (MemoryAccess, ZeroOrOne)
        ]
    ),
    inst!(
        Store,
        [],
        [],
        [(IdRef, One), (IdRef, One), (MemoryAccess, ZeroOrOne)]
    ),
    inst!(
        CopyMemory,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (MemoryAccess, ZeroOrOne),
            (MemoryAccess, ZeroOrOne)
        ]
    ),
    inst!(
        CopyMemorySized,
        [Addresses],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (MemoryAccess, ZeroOrOne),
            (MemoryAccess, ZeroOrOne)
        ]
    ),
    inst!(
        AccessChain,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(
        InBoundsAccessChain,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(
        PtrAccessChain,
        [
            Addresses,
            VariablePointers,
            VariablePointersStorageBuffer,
            PhysicalStorageBufferAddresses
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(
        ArrayLength,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        GenericPtrMemSemantics,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        InBoundsPtrAccessChain,
        [Addresses],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(Decorate, [], [], [(IdRef, One), (Decoration, One)]),
    inst!(
        MemberDecorate,
        [],
        [],
        [(IdRef, One), (LiteralInteger, One), (Decoration, One)]
    ),
    inst!(DecorationGroup, [], [], [(IdResult, One)]),
    inst!(GroupDecorate, [], [], [(IdRef, One), (IdRef, ZeroOrMore)]),
    inst!(
        GroupMemberDecorate,
        [],
        [],
        [(IdRef, One), (PairIdRefLiteralInteger, ZeroOrMore)]
    ),
    inst!(
        VectorExtractDynamic,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        VectorInsertDynamic,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        VectorShuffle,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, ZeroOrMore)
        ]
    ),
    inst!(
        CompositeConstruct,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        CompositeExtract,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, ZeroOrMore)
        ]
    ),
    inst!(
        CompositeInsert,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, ZeroOrMore)
        ]
    ),
    inst!(
        CopyObject,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        Transpose,
        [Matrix],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SampledImage,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ImageSampleImplicitLod,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSampleExplicitLod,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, One)
        ]
    ),
    inst!(
        ImageSampleDrefImplicitLod,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSampleDrefExplicitLod,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, One)
        ]
    ),
    inst!(
        ImageSampleProjImplicitLod,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSampleProjExplicitLod,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, One)
        ]
    ),
    inst!(
        ImageSampleProjDrefImplicitLod,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSampleProjDrefExplicitLod,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, One)
        ]
    ),
    inst!(
        ImageFetch,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageGather,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageDrefGather,
        [Shader],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageRead,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageWrite,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        Image,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQueryFormat,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQueryOrder,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQuerySizeLod,
        [Kernel, ImageQuery],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ImageQuerySize,
        [Kernel, ImageQuery],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQueryLod,
        [ImageQuery],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ImageQueryLevels,
        [Kernel, ImageQuery],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQuerySamples,
        [Kernel, ImageQuery],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertFToU,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertFToS,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertSToF,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertUToF,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        UConvert,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SConvert,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FConvert,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        QuantizeToF16,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertPtrToU,
        [Addresses, PhysicalStorageBufferAddresses],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SatConvertSToU,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SatConvertUToS,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertUToPtr,
        [Addresses, PhysicalStorageBufferAddresses],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        PtrCastToGeneric,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GenericCastToPtr,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GenericCastToPtrExplicit,
        [Kernel],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (StorageClass, One)
        ]
    ),
    inst!(
        Bitcast,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SNegate,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FNegate,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IAdd,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FAdd,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ISub,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FSub,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        IMul,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FMul,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UDiv,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SDiv,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FDiv,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UMod,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SRem,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SMod,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FRem,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FMod,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        VectorTimesScalar,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        MatrixTimesScalar,
        [Matrix],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        VectorTimesMatrix,
        [Matrix],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        MatrixTimesVector,
        [Matrix],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        MatrixTimesMatrix,
        [Matrix],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        OuterProduct,
        [Matrix],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        Dot,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        IAddCarry,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ISubBorrow,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UMulExtended,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SMulExtended,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        Any,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        All,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IsNan,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IsInf,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IsFinite,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IsNormal,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SignBitSet,
        [Kernel],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        LessOrGreater,
        [Kernel],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        Ordered,
        [Kernel],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        Unordered,
        [Kernel],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        LogicalEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        LogicalNotEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        LogicalOr,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        LogicalAnd,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        LogicalNot,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        Select,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        IEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        INotEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UGreaterThan,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SGreaterThan,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UGreaterThanEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SGreaterThanEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ULessThan,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SLessThan,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ULessThanEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SLessThanEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FOrdEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FUnordEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FOrdNotEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FUnordNotEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FOrdLessThan,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FUnordLessThan,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FOrdGreaterThan,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FUnordGreaterThan,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FOrdLessThanEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FUnordLessThanEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FOrdGreaterThanEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FUnordGreaterThanEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ShiftRightLogical,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ShiftRightArithmetic,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ShiftLeftLogical,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        BitwiseOr,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        BitwiseXor,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        BitwiseAnd,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        Not,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        BitFieldInsert,
        [Shader, BitInstructions],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        BitFieldSExtract,
        [Shader, BitInstructions],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        BitFieldUExtract,
        [Shader, BitInstructions],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        BitReverse,
        [Shader, BitInstructions],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        BitCount,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdx,
        [Shader],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdy,
        [Shader],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        Fwidth,
        [Shader],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdxFine,
        [DerivativeControl],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdyFine,
        [DerivativeControl],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FwidthFine,
        [DerivativeControl],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdxCoarse,
        [DerivativeControl],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdyCoarse,
        [DerivativeControl],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FwidthCoarse,
        [DerivativeControl],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(EmitVertex, [Geometry], [], []),
    inst!(EndPrimitive, [Geometry], [], []),
    inst!(EmitStreamVertex, [GeometryStreams], [], [(IdRef, One)]),
    inst!(EndStreamPrimitive, [GeometryStreams], [], [(IdRef, One)]),
    inst!(
        ControlBarrier,
        [],
        [],
        [(IdScope, One), (IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(
        MemoryBarrier,
        [],
        [],
        [(IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(
        AtomicLoad,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One)
        ]
    ),
    inst!(
        AtomicStore,
        [],
        [],
        [
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicExchange,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicCompareExchange,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdMemorySemantics, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicCompareExchangeWeak,
        [Kernel],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdMemorySemantics, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicIIncrement,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One)
        ]
    ),
    inst!(
        AtomicIDecrement,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One)
        ]
    ),
    inst!(
        AtomicIAdd,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicISub,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicSMin,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicUMin,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicSMax,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicUMax,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicAnd,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicOr,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicXor,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        Phi,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (PairIdRefIdRef, ZeroOrMore)
        ]
    ),
    inst!(
        LoopMerge,
        [],
        [],
        [(IdRef, One), (IdRef, One), (LoopControl, One)]
    ),
    inst!(
        SelectionMerge,
        [],
        [],
        [(IdRef, One), (SelectionControl, One)]
    ),
    inst!(Label, [], [], [(IdResult, One)]),
    inst!(Branch, [], [], [(IdRef, One)]),
    inst!(
        BranchConditional,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, ZeroOrMore)
        ]
    ),
    inst!(
        Switch,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (PairLiteralIntegerIdRef, ZeroOrMore)
        ]
    ),
    inst!(Kill, [Shader], [], []),
    inst!(Return, [], [], []),
    inst!(ReturnValue, [], [], [(IdRef, One)]),
    inst!(Unreachable, [], [], []),
    inst!(
        LifetimeStart,
        [Kernel],
        [],
        [(IdRef, One), (LiteralInteger, One)]
    ),
    inst!(
        LifetimeStop,
        [Kernel],
        [],
        [(IdRef, One), (LiteralInteger, One)]
    ),
    inst!(
        GroupAsyncCopy,
        [Kernel],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupWaitEvents,
        [Kernel],
        [],
        [(IdScope, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        GroupAll,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupAny,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupBroadcast,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupIAdd,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupFAdd,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupFMin,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupUMin,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupSMin,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupFMax,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupUMax,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupSMax,
        [Groups],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ReadPipe,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        WritePipe,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ReservedReadPipe,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ReservedWritePipe,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ReserveReadPipePackets,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ReserveWritePipePackets,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        CommitReadPipe,
        [Pipes],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        CommitWritePipe,
        [Pipes],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        IsValidReserveId,
        [Pipes],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GetNumPipePackets,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GetMaxPipePackets,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupReserveReadPipePackets,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupReserveWritePipePackets,
        [Pipes],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupCommitReadPipe,
        [Pipes],
        [],
        [
            (IdScope, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupCommitWritePipe,
        [Pipes],
        [],
        [
            (IdScope, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        EnqueueMarker,
        [DeviceEnqueue],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        EnqueueKernel,
        [DeviceEnqueue],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
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
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(
        GetKernelNDrangeSubGroupCount,
        [DeviceEnqueue],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GetKernelNDrangeMaxSubGroupSize,
        [DeviceEnqueue],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GetKernelWorkGroupSize,
        [DeviceEnqueue],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GetKernelPreferredWorkGroupSizeMultiple,
        [DeviceEnqueue],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(RetainEvent, [DeviceEnqueue], [], [(IdRef, One)]),
    inst!(ReleaseEvent, [DeviceEnqueue], [], [(IdRef, One)]),
    inst!(
        CreateUserEvent,
        [DeviceEnqueue],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        IsValidEvent,
        [DeviceEnqueue],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SetUserEventStatus,
        [DeviceEnqueue],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        CaptureEventProfilingInfo,
        [DeviceEnqueue],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        GetDefaultQueue,
        [DeviceEnqueue],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        BuildNDRange,
        [DeviceEnqueue],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ImageSparseSampleImplicitLod,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSparseSampleExplicitLod,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, One)
        ]
    ),
    inst!(
        ImageSparseSampleDrefImplicitLod,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSparseSampleDrefExplicitLod,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, One)
        ]
    ),
    inst!(
        ImageSparseSampleProjImplicitLod,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSparseSampleProjExplicitLod,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, One)
        ]
    ),
    inst!(
        ImageSparseSampleProjDrefImplicitLod,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSparseSampleProjDrefExplicitLod,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, One)
        ]
    ),
    inst!(
        ImageSparseFetch,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSparseGather,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSparseDrefGather,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        ImageSparseTexelsResident,
        [SparseResidency],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(NoLine, [], [], []),
    inst!(
        AtomicFlagTestAndSet,
        [Kernel],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One)
        ]
    ),
    inst!(
        AtomicFlagClear,
        [Kernel],
        [],
        [(IdRef, One), (IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(
        ImageSparseRead,
        [SparseResidency],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        SizeOf,
        [Addresses],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(TypePipeStorage, [PipeStorage], [], [(IdResult, One)]),
    inst!(
        ConstantPipeStorage,
        [PipeStorage],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        CreatePipeFromPipeStorage,
        [PipeStorage],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GetKernelLocalSizeForSubgroupCount,
        [SubgroupDispatch],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GetKernelMaxNumSubgroups,
        [SubgroupDispatch],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(TypeNamedBarrier, [NamedBarrier], [], [(IdResult, One)]),
    inst!(
        NamedBarrierInitialize,
        [NamedBarrier],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        MemoryNamedBarrier,
        [NamedBarrier],
        [],
        [(IdRef, One), (IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(ModuleProcessed, [], [], [(LiteralString, One)]),
    inst!(
        ExecutionModeId,
        [],
        [],
        [(IdRef, One), (ExecutionMode, One)]
    ),
    inst!(
        DecorateId,
        [],
        ["SPV_GOOGLE_hlsl_functionality1"],
        [(IdRef, One), (Decoration, One)]
    ),
    inst!(
        GroupNonUniformElect,
        [GroupNonUniform],
        [],
        [(IdResultType, One), (IdResult, One), (IdScope, One)]
    ),
    inst!(
        GroupNonUniformAll,
        [GroupNonUniformVote],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformAny,
        [GroupNonUniformVote],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformAllEqual,
        [GroupNonUniformVote],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformBroadcast,
        [GroupNonUniformBallot],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformBroadcastFirst,
        [GroupNonUniformBallot],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformBallot,
        [GroupNonUniformBallot],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformInverseBallot,
        [GroupNonUniformBallot],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformBallotBitExtract,
        [GroupNonUniformBallot],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformBallotBitCount,
        [GroupNonUniformBallot],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformBallotFindLSB,
        [GroupNonUniformBallot],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformBallotFindMSB,
        [GroupNonUniformBallot],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformShuffle,
        [GroupNonUniformShuffle],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformShuffleXor,
        [GroupNonUniformShuffle],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformShuffleUp,
        [GroupNonUniformShuffleRelative],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformShuffleDown,
        [GroupNonUniformShuffleRelative],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformIAdd,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformFAdd,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformIMul,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformFMul,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformSMin,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformUMin,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformFMin,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformSMax,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformUMax,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformFMax,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformBitwiseAnd,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformBitwiseOr,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformBitwiseXor,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformLogicalAnd,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformLogicalOr,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformLogicalXor,
        [
            GroupNonUniformArithmetic,
            GroupNonUniformClustered,
            GroupNonUniformPartitionedNV
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        GroupNonUniformQuadBroadcast,
        [GroupNonUniformQuad],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupNonUniformQuadSwap,
        [GroupNonUniformQuad],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        CopyLogical,
        [],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        PtrEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        PtrNotEqual,
        [],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        PtrDiff,
        [Addresses, VariablePointers, VariablePointersStorageBuffer],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ColorAttachmentReadEXT,
        [TileImageColorReadAccessEXT],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        DepthAttachmentReadEXT,
        [TileImageDepthReadAccessEXT],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrOne)]
    ),
    inst!(
        StencilAttachmentReadEXT,
        [TileImageStencilReadAccessEXT],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrOne)]
    ),
    inst!(
        TerminateInvocation,
        [Shader],
        ["SPV_KHR_terminate_invocation"],
        []
    ),
    inst!(
        SubgroupBallotKHR,
        [SubgroupBallotKHR],
        ["SPV_KHR_shader_ballot"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupFirstInvocationKHR,
        [SubgroupBallotKHR],
        ["SPV_KHR_shader_ballot"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAllKHR,
        [SubgroupVoteKHR],
        ["SPV_KHR_subgroup_vote"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAnyKHR,
        [SubgroupVoteKHR],
        ["SPV_KHR_subgroup_vote"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAllEqualKHR,
        [SubgroupVoteKHR],
        ["SPV_KHR_subgroup_vote"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GroupNonUniformRotateKHR,
        [GroupNonUniformRotateKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    inst!(
        SubgroupReadInvocationKHR,
        [SubgroupBallotKHR],
        ["SPV_KHR_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        TraceRayKHR,
        [RayTracingKHR],
        ["SPV_KHR_ray_tracing"],
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
            (IdRef, One)
        ]
    ),
    inst!(
        ExecuteCallableKHR,
        [RayTracingKHR],
        ["SPV_KHR_ray_tracing"],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        ConvertUToAccelerationStructureKHR,
        [RayTracingKHR, RayQueryKHR],
        ["SPV_KHR_ray_tracing", "SPV_KHR_ray_query"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IgnoreIntersectionKHR,
        [RayTracingKHR],
        ["SPV_KHR_ray_tracing"],
        []
    ),
    inst!(
        TerminateRayKHR,
        [RayTracingKHR],
        ["SPV_KHR_ray_tracing"],
        []
    ),
    inst!(
        SDot,
        [DotProduct],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        SDotKHR,
        [DotProductKHR],
        ["SPV_KHR_integer_dot_product"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        UDot,
        [DotProduct],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        UDotKHR,
        [DotProductKHR],
        ["SPV_KHR_integer_dot_product"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        SUDot,
        [DotProduct],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        SUDotKHR,
        [DotProductKHR],
        ["SPV_KHR_integer_dot_product"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        SDotAccSat,
        [DotProduct],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        SDotAccSatKHR,
        [DotProductKHR],
        ["SPV_KHR_integer_dot_product"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        UDotAccSat,
        [DotProduct],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        UDotAccSatKHR,
        [DotProductKHR],
        ["SPV_KHR_integer_dot_product"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        SUDotAccSat,
        [DotProduct],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        SUDotAccSatKHR,
        [DotProductKHR],
        ["SPV_KHR_integer_dot_product"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (PackedVectorFormat, ZeroOrOne)
        ]
    ),
    inst!(
        TypeCooperativeMatrixKHR,
        [CooperativeMatrixKHR],
        [],
        [
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        CooperativeMatrixLoadKHR,
        [CooperativeMatrixKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrOne),
            (MemoryAccess, ZeroOrOne)
        ]
    ),
    inst!(
        CooperativeMatrixStoreKHR,
        [CooperativeMatrixKHR],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrOne),
            (MemoryAccess, ZeroOrOne)
        ]
    ),
    inst!(
        CooperativeMatrixMulAddKHR,
        [CooperativeMatrixKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (CooperativeMatrixOperands, ZeroOrOne)
        ]
    ),
    inst!(
        CooperativeMatrixLengthKHR,
        [CooperativeMatrixKHR],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        TypeRayQueryKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdResult, One)]
    ),
    inst!(
        RayQueryInitializeKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
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
    inst!(
        RayQueryTerminateKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdRef, One)]
    ),
    inst!(
        RayQueryGenerateIntersectionKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        RayQueryConfirmIntersectionKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdRef, One)]
    ),
    inst!(
        RayQueryProceedKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        RayQueryGetIntersectionTypeKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ImageSampleWeightedQCOM,
        [TextureSampleWeightedQCOM],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ImageBoxFilterQCOM,
        [TextureBoxFilterQCOM],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ImageBlockMatchSSDQCOM,
        [TextureBlockMatchQCOM],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ImageBlockMatchSADQCOM,
        [TextureBlockMatchQCOM],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupIAddNonUniformAMD,
        [Groups],
        ["SPV_AMD_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupFAddNonUniformAMD,
        [Groups],
        ["SPV_AMD_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupFMinNonUniformAMD,
        [Groups],
        ["SPV_AMD_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupUMinNonUniformAMD,
        [Groups],
        ["SPV_AMD_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupSMinNonUniformAMD,
        [Groups],
        ["SPV_AMD_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupFMaxNonUniformAMD,
        [Groups],
        ["SPV_AMD_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupUMaxNonUniformAMD,
        [Groups],
        ["SPV_AMD_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupSMaxNonUniformAMD,
        [Groups],
        ["SPV_AMD_shader_ballot"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FragmentMaskFetchAMD,
        [FragmentMaskAMD],
        ["SPV_AMD_shader_fragment_mask"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FragmentFetchAMD,
        [FragmentMaskAMD],
        ["SPV_AMD_shader_fragment_mask"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ReadClockKHR,
        [ShaderClockKHR],
        [],
        [(IdResultType, One), (IdResult, One), (IdScope, One)]
    ),
    inst!(
        FinalizeNodePayloadsAMDX,
        [ShaderEnqueueAMDX],
        [],
        [(IdRef, One)]
    ),
    inst!(
        FinishWritingNodePayloadAMDX,
        [ShaderEnqueueAMDX],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        InitializeNodePayloadsAMDX,
        [ShaderEnqueueAMDX],
        [],
        [(IdRef, One), (IdScope, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        HitObjectRecordHitMotionNV,
        [ShaderInvocationReorderNV, RayTracingMotionBlurNV],
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
            (IdRef, One)
        ]
    ),
    inst!(
        HitObjectRecordHitWithIndexMotionNV,
        [ShaderInvocationReorderNV, RayTracingMotionBlurNV],
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
            (IdRef, One)
        ]
    ),
    inst!(
        HitObjectRecordMissMotionNV,
        [ShaderInvocationReorderNV, RayTracingMotionBlurNV],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        HitObjectGetWorldToObjectNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetObjectToWorldNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetObjectRayDirectionNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetObjectRayOriginNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectTraceRayMotionNV,
        [ShaderInvocationReorderNV, RayTracingMotionBlurNV],
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
            (IdRef, One)
        ]
    ),
    inst!(
        HitObjectGetShaderRecordBufferHandleNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetShaderBindingTableRecordIndexNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectRecordEmptyNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdRef, One)]
    ),
    inst!(
        HitObjectTraceRayNV,
        [ShaderInvocationReorderNV],
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
            (IdRef, One)
        ]
    ),
    inst!(
        HitObjectRecordHitNV,
        [ShaderInvocationReorderNV],
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
            (IdRef, One)
        ]
    ),
    inst!(
        HitObjectRecordHitWithIndexNV,
        [ShaderInvocationReorderNV],
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
            (IdRef, One)
        ]
    ),
    inst!(
        HitObjectRecordMissNV,
        [ShaderInvocationReorderNV],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        HitObjectExecuteShaderNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetCurrentTimeNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetAttributesNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetHitKindNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetPrimitiveIndexNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetGeometryIndexNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetInstanceIdNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetInstanceCustomIndexNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetWorldRayDirectionNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetWorldRayOriginNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetRayTMaxNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectGetRayTMinNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectIsEmptyNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectIsHitNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        HitObjectIsMissNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ReorderThreadWithHitObjectNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdRef, One), (IdRef, ZeroOrOne), (IdRef, ZeroOrOne)]
    ),
    inst!(
        ReorderThreadWithHintNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        TypeHitObjectNV,
        [ShaderInvocationReorderNV],
        [],
        [(IdResult, One)]
    ),
    inst!(
        ImageSampleFootprintNV,
        [ImageFootprintNV],
        ["SPV_NV_shader_image_footprint"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (ImageOperands, ZeroOrOne)
        ]
    ),
    inst!(
        EmitMeshTasksEXT,
        [MeshShadingEXT],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, ZeroOrOne)]
    ),
    inst!(
        SetMeshOutputsEXT,
        [MeshShadingEXT],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        GroupNonUniformPartitionNV,
        [GroupNonUniformPartitionedNV],
        ["SPV_NV_shader_subgroup_partitioned"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        WritePackedPrimitiveIndices4x8NV,
        [MeshShadingNV],
        ["SPV_NV_mesh_shader"],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        FetchMicroTriangleVertexPositionNV,
        [DisplacementMicromapNV],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FetchMicroTriangleVertexBarycentricNV,
        [DisplacementMicromapNV],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ReportIntersectionKHR,
        [RayTracingNV, RayTracingKHR],
        ["SPV_NV_ray_tracing", "SPV_KHR_ray_tracing"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ReportIntersectionNV,
        [RayTracingNV, RayTracingKHR],
        ["SPV_NV_ray_tracing", "SPV_KHR_ray_tracing"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        IgnoreIntersectionNV,
        [RayTracingNV],
        ["SPV_NV_ray_tracing"],
        []
    ),
    inst!(TerminateRayNV, [RayTracingNV], ["SPV_NV_ray_tracing"], []),
    inst!(
        TraceNV,
        [RayTracingNV],
        ["SPV_NV_ray_tracing"],
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
            (IdRef, One)
        ]
    ),
    inst!(
        TraceMotionNV,
        [RayTracingMotionBlurNV],
        ["SPV_NV_ray_tracing_motion_blur"],
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
            (IdRef, One)
        ]
    ),
    inst!(
        TraceRayMotionNV,
        [RayTracingMotionBlurNV],
        ["SPV_NV_ray_tracing_motion_blur"],
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
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionTriangleVertexPositionsKHR,
        [RayQueryPositionFetchKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        TypeAccelerationStructureKHR,
        [RayTracingNV, RayTracingKHR, RayQueryKHR],
        [
            "SPV_NV_ray_tracing",
            "SPV_KHR_ray_tracing",
            "SPV_KHR_ray_query"
        ],
        [(IdResult, One)]
    ),
    inst!(
        TypeAccelerationStructureNV,
        [RayTracingNV, RayTracingKHR, RayQueryKHR],
        [
            "SPV_NV_ray_tracing",
            "SPV_KHR_ray_tracing",
            "SPV_KHR_ray_query"
        ],
        [(IdResult, One)]
    ),
    inst!(
        ExecuteCallableNV,
        [RayTracingNV],
        ["SPV_NV_ray_tracing"],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        TypeCooperativeMatrixNV,
        [CooperativeMatrixNV],
        ["SPV_NV_cooperative_matrix"],
        [
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        CooperativeMatrixLoadNV,
        [CooperativeMatrixNV],
        ["SPV_NV_cooperative_matrix"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (MemoryAccess, ZeroOrOne)
        ]
    ),
    inst!(
        CooperativeMatrixStoreNV,
        [CooperativeMatrixNV],
        ["SPV_NV_cooperative_matrix"],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (MemoryAccess, ZeroOrOne)
        ]
    ),
    inst!(
        CooperativeMatrixMulAddNV,
        [CooperativeMatrixNV],
        ["SPV_NV_cooperative_matrix"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        CooperativeMatrixLengthNV,
        [CooperativeMatrixNV],
        ["SPV_NV_cooperative_matrix"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        BeginInvocationInterlockEXT,
        [
            FragmentShaderSampleInterlockEXT,
            FragmentShaderPixelInterlockEXT,
            FragmentShaderShadingRateInterlockEXT
        ],
        ["SPV_EXT_fragment_shader_interlock"],
        []
    ),
    inst!(
        EndInvocationInterlockEXT,
        [
            FragmentShaderSampleInterlockEXT,
            FragmentShaderPixelInterlockEXT,
            FragmentShaderShadingRateInterlockEXT
        ],
        ["SPV_EXT_fragment_shader_interlock"],
        []
    ),
    inst!(DemoteToHelperInvocation, [DemoteToHelperInvocation], [], []),
    inst!(
        DemoteToHelperInvocationEXT,
        [DemoteToHelperInvocationEXT],
        [],
        []
    ),
    inst!(
        IsHelperInvocationEXT,
        [DemoteToHelperInvocationEXT],
        ["SPV_EXT_demote_to_helper_invocation"],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        ConvertUToImageNV,
        [BindlessTextureNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertUToSamplerNV,
        [BindlessTextureNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertImageToUNV,
        [BindlessTextureNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertSamplerToUNV,
        [BindlessTextureNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertUToSampledImageNV,
        [BindlessTextureNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertSampledImageToUNV,
        [BindlessTextureNV],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SamplerImageAddressingModeNV,
        [BindlessTextureNV],
        [],
        [(LiteralInteger, One)]
    ),
    inst!(
        SubgroupShuffleINTEL,
        [SubgroupShuffleINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupShuffleDownINTEL,
        [SubgroupShuffleINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupShuffleUpINTEL,
        [SubgroupShuffleINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupShuffleXorINTEL,
        [SubgroupShuffleINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupBlockReadINTEL,
        [SubgroupBufferBlockIOINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupBlockWriteINTEL,
        [SubgroupBufferBlockIOINTEL],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        SubgroupImageBlockReadINTEL,
        [SubgroupImageBlockIOINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupImageBlockWriteINTEL,
        [SubgroupImageBlockIOINTEL],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        SubgroupImageMediaBlockReadINTEL,
        [SubgroupImageMediaBlockIOINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupImageMediaBlockWriteINTEL,
        [SubgroupImageMediaBlockIOINTEL],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UCountLeadingZerosINTEL,
        [IntegerFunctions2INTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        UCountTrailingZerosINTEL,
        [IntegerFunctions2INTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        AbsISubINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AbsUSubINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        IAddSatINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UAddSatINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        IAverageINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UAverageINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        IAverageRoundedINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UAverageRoundedINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ISubSatINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        USubSatINTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        IMul32x16INTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        UMul32x16INTEL,
        [IntegerFunctions2INTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        ConstantFunctionPointerINTEL,
        [FunctionPointersINTEL],
        ["SPV_INTEL_function_pointers"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FunctionPointerCallINTEL,
        [FunctionPointersINTEL],
        ["SPV_INTEL_function_pointers"],
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        AsmTargetINTEL,
        [AsmINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (LiteralString, One)]
    ),
    inst!(
        AsmINTEL,
        [AsmINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralString, One),
            (LiteralString, One)
        ]
    ),
    inst!(
        AsmCallINTEL,
        [AsmINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(
        AtomicFMinEXT,
        [
            AtomicFloat16MinMaxEXT,
            AtomicFloat32MinMaxEXT,
            AtomicFloat64MinMaxEXT
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicFMaxEXT,
        [
            AtomicFloat16MinMaxEXT,
            AtomicFloat32MinMaxEXT,
            AtomicFloat64MinMaxEXT
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AssumeTrueKHR,
        [ExpectAssumeKHR],
        ["SPV_KHR_expect_assume"],
        [(IdRef, One)]
    ),
    inst!(
        ExpectKHR,
        [ExpectAssumeKHR],
        ["SPV_KHR_expect_assume"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        DecorateString,
        [],
        [
            "SPV_GOOGLE_decorate_string",
            "SPV_GOOGLE_hlsl_functionality1"
        ],
        [(IdRef, One), (Decoration, One)]
    ),
    inst!(
        DecorateStringGOOGLE,
        [],
        [
            "SPV_GOOGLE_decorate_string",
            "SPV_GOOGLE_hlsl_functionality1"
        ],
        [(IdRef, One), (Decoration, One)]
    ),
    inst!(
        MemberDecorateString,
        [],
        [
            "SPV_GOOGLE_decorate_string",
            "SPV_GOOGLE_hlsl_functionality1"
        ],
        [(IdRef, One), (LiteralInteger, One), (Decoration, One)]
    ),
    inst!(
        MemberDecorateStringGOOGLE,
        [],
        [
            "SPV_GOOGLE_decorate_string",
            "SPV_GOOGLE_hlsl_functionality1"
        ],
        [(IdRef, One), (LiteralInteger, One), (Decoration, One)]
    ),
    inst!(
        VmeImageINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        TypeVmeImageINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One), (IdRef, One)]
    ),
    inst!(
        TypeAvcImePayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcRefPayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcSicPayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcMcePayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcMceResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcImeResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcImeResultSingleReferenceStreamoutINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcImeResultDualReferenceStreamoutINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcImeSingleReferenceStreaminINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcImeDualReferenceStreaminINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcRefResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        TypeAvcSicResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResult, One)]
    ),
    inst!(
        SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceGetDefaultInterShapePenaltyINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceSetInterShapePenaltyINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceSetInterDirectionPenaltyINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        SubgroupAvcMceSetMotionVectorCostFunctionINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationChromaINTEL
        ],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        SubgroupAvcMceSetAcOnlyHaarINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcMceConvertToImePayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceConvertToImeResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceConvertToRefPayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceConvertToRefResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceConvertToSicPayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceConvertToSicResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetMotionVectorsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetInterDistortionsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetBestInterDistortionsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetInterMajorShapeINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetInterMinorShapeINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetInterDirectionsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetInterMotionVectorCountINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetInterReferenceIdsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeInitializeINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeSetSingleReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeSetDualReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeRefWindowSizeINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeAdjustRefOffsetINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeConvertToMcePayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeSetMaxMotionVectorCountINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeSetUnidirectionalMixDisableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeSetWeightedSadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeEvaluateWithSingleReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeEvaluateWithDualReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeConvertToMceResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeGetSingleReferenceStreaminINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeGetDualReferenceStreaminINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeStripSingleReferenceStreamoutINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeStripDualReferenceStreamoutINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeGetBorderReachedINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcImeGetTruncatedSearchIndicationINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcFmeInitializeINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcBmeInitializeINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
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
    inst!(
        SubgroupAvcRefConvertToMcePayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcRefSetBidirectionalMixDisableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcRefSetBilinearFilterEnableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcRefEvaluateWithSingleReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcRefEvaluateWithDualReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcRefEvaluateWithMultiReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcRefConvertToMceResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicInitializeINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicConfigureSkcINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicConfigureIpeLumaINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
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
    inst!(
        SubgroupAvcSicConfigureIpeLumaChromaINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationChromaINTEL
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
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
    inst!(
        SubgroupAvcSicGetMotionVectorMaskINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicConvertToMcePayloadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicSetIntraLumaShapePenaltyINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationChromaINTEL
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicSetBilinearFilterEnableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicSetSkcForwardTransformEnableINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicSetBlockBasedRawSkipSadINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicEvaluateIpeINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicEvaluateWithSingleReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicEvaluateWithDualReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicEvaluateWithMultiReferenceINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupAvcSicConvertToMceResultINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicGetIpeLumaShapeINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicGetBestIpeLumaDistortionINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicGetBestIpeChromaDistortionINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicGetPackedIpeLumaModesINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicGetIpeChromaModeINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationChromaINTEL
        ],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL,
        [
            SubgroupAvcMotionEstimationINTEL,
            SubgroupAvcMotionEstimationIntraINTEL
        ],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAvcSicGetInterRawSadsINTEL,
        [SubgroupAvcMotionEstimationINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        VariableLengthArrayINTEL,
        [VariableLengthArrayINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SaveMemoryINTEL,
        [VariableLengthArrayINTEL],
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        RestoreMemoryINTEL,
        [VariableLengthArrayINTEL],
        [],
        [(IdRef, One)]
    ),
    inst!(
        ArbitraryFloatSinCosPiINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatCastINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatCastFromIntINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatCastToIntINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatAddINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatSubINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatMulINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatDivINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatGTINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatGEINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatLTINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatLEINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatEQINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatRecipINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatRSqrtINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatCbrtINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatHypotINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatSqrtINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatLogINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatLog2INTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatLog10INTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatLog1pINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatExpINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatExp2INTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatExp10INTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatExpm1INTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatSinINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatCosINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatSinCosINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatSinPiINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatCosPiINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatASinINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatASinPiINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatACosINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatACosPiINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatATanINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatATanPiINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatATan2INTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatPowINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatPowRINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        ArbitraryFloatPowNINTEL,
        [ArbitraryPrecisionFloatingPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (LiteralInteger, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        LoopControlINTEL,
        [UnstructuredLoopControlsINTEL],
        ["SPV_INTEL_unstructured_loop_controls"],
        [(LiteralInteger, ZeroOrMore)]
    ),
    inst!(
        AliasDomainDeclINTEL,
        [MemoryAccessAliasingINTEL],
        ["SPV_INTEL_memory_access_aliasing"],
        [(IdResult, One), (IdRef, ZeroOrOne)]
    ),
    inst!(
        AliasScopeDeclINTEL,
        [MemoryAccessAliasingINTEL],
        ["SPV_INTEL_memory_access_aliasing"],
        [(IdResult, One), (IdRef, One), (IdRef, ZeroOrOne)]
    ),
    inst!(
        AliasScopeListDeclINTEL,
        [MemoryAccessAliasingINTEL],
        ["SPV_INTEL_memory_access_aliasing"],
        [(IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        FixedSqrtINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedRecipINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedRsqrtINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedSinINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedCosINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedSinCosINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedSinPiINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedCosPiINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedSinCosPiINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedLogINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        FixedExpINTEL,
        [ArbitraryPrecisionFixedPointINTEL],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(
        PtrCastToCrossWorkgroupINTEL,
        [USMStorageClassesINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        CrossWorkgroupCastToPtrINTEL,
        [USMStorageClassesINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ReadPipeBlockingINTEL,
        [BlockingPipesINTEL],
        ["SPV_INTEL_blocking_pipes"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        WritePipeBlockingINTEL,
        [BlockingPipesINTEL],
        ["SPV_INTEL_blocking_pipes"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        FPGARegINTEL,
        [FPGARegINTEL],
        ["SPV_INTEL_fpga_reg"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetRayTMinKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        RayQueryGetRayFlagsKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        RayQueryGetIntersectionTKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionInstanceCustomIndexKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionInstanceIdKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionGeometryIndexKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionPrimitiveIndexKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionBarycentricsKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionFrontFaceKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionCandidateAABBOpaqueKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        RayQueryGetIntersectionObjectRayDirectionKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionObjectRayOriginKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetWorldRayDirectionKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        RayQueryGetWorldRayOriginKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        RayQueryGetIntersectionObjectToWorldKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        RayQueryGetIntersectionWorldToObjectKHR,
        [RayQueryKHR],
        ["SPV_KHR_ray_query"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        AtomicFAddEXT,
        [
            AtomicFloat16AddEXT,
            AtomicFloat32AddEXT,
            AtomicFloat64AddEXT
        ],
        ["SPV_EXT_shader_atomic_float_add"],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdScope, One),
            (IdMemorySemantics, One),
            (IdRef, One)
        ]
    ),
    inst!(
        TypeBufferSurfaceINTEL,
        [VectorComputeINTEL],
        [],
        [(IdResult, One), (AccessQualifier, One)]
    ),
    inst!(
        TypeStructContinuedINTEL,
        [LongCompositesINTEL],
        [],
        [(IdRef, ZeroOrMore)]
    ),
    inst!(
        ConstantCompositeContinuedINTEL,
        [LongCompositesINTEL],
        [],
        [(IdRef, ZeroOrMore)]
    ),
    inst!(
        SpecConstantCompositeContinuedINTEL,
        [LongCompositesINTEL],
        [],
        [(IdRef, ZeroOrMore)]
    ),
    inst!(
        CompositeConstructContinuedINTEL,
        [LongCompositesINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        ConvertFToBF16INTEL,
        [BFloat16ConversionINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertBF16ToFINTEL,
        [BFloat16ConversionINTEL],
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ControlBarrierArriveINTEL,
        [SplitBarrierINTEL],
        [],
        [(IdScope, One), (IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(
        ControlBarrierWaitINTEL,
        [SplitBarrierINTEL],
        [],
        [(IdScope, One), (IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(
        GroupIMulKHR,
        [GroupUniformArithmeticKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupFMulKHR,
        [GroupUniformArithmeticKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupBitwiseAndKHR,
        [GroupUniformArithmeticKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupBitwiseOrKHR,
        [GroupUniformArithmeticKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupBitwiseXorKHR,
        [GroupUniformArithmeticKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupLogicalAndKHR,
        [GroupUniformArithmeticKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupLogicalOrKHR,
        [GroupUniformArithmeticKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupLogicalXorKHR,
        [GroupUniformArithmeticKHR],
        [],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (GroupOperation, One),
            (IdRef, One)
        ]
    ),
];
