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

#[doc = "All operand kinds in the SPIR-V grammar."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperandKind {
    ImageOperands,
    FPFastMathMode,
    SelectionControl,
    LoopControl,
    FunctionControl,
    MemorySemantics,
    MemoryAccess,
    KernelProfilingInfo,
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
    LinkageType,
    AccessQualifier,
    FunctionParameterAttribute,
    Decoration,
    BuiltIn,
    Scope,
    GroupOperation,
    KernelEnqueueFlags,
    Capability,
    IdResultType,
    IdResult,
    IdMemorySemantics,
    IdScope,
    IdRef,
    LiteralInteger,
    LiteralString,
    LiteralContextDependentNumber,
    LiteralExtInstInteger,
    LiteralSpecConstantOpInteger,
    PairLiteralIntegerIdRef,
    PairIdRefLiteralInteger,
    PairIdRefIdRef,
}
static INSTRUCTION_TABLE: &'static [Instruction<'static>] = &[
    inst!(Nop, [], []),
    inst!(Undef, [], [(IdResultType, One), (IdResult, One)]),
    inst!(SourceContinued, [], [(LiteralString, One)]),
    inst!(
        Source,
        [],
        [
            (SourceLanguage, One),
            (LiteralInteger, One),
            (IdRef, ZeroOrOne),
            (LiteralString, ZeroOrOne)
        ]
    ),
    inst!(SourceExtension, [], [(LiteralString, One)]),
    inst!(Name, [], [(IdRef, One), (LiteralString, One)]),
    inst!(
        MemberName,
        [],
        [(IdRef, One), (LiteralInteger, One), (LiteralString, One)]
    ),
    inst!(String, [], [(IdResult, One), (LiteralString, One)]),
    inst!(
        Line,
        [],
        [(IdRef, One), (LiteralInteger, One), (LiteralInteger, One)]
    ),
    inst!(Extension, [], [(LiteralString, One)]),
    inst!(ExtInstImport, [], [(IdResult, One), (LiteralString, One)]),
    inst!(
        ExtInst,
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
        [(AddressingModel, One), (MemoryModel, One)]
    ),
    inst!(
        EntryPoint,
        [],
        [
            (ExecutionModel, One),
            (IdRef, One),
            (LiteralString, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(ExecutionMode, [], [(IdRef, One), (ExecutionMode, One)]),
    inst!(Capability, [], [(Capability, One)]),
    inst!(TypeVoid, [], [(IdResult, One)]),
    inst!(TypeBool, [], [(IdResult, One)]),
    inst!(
        TypeInt,
        [],
        [
            (IdResult, One),
            (LiteralInteger, One),
            (LiteralInteger, One)
        ]
    ),
    inst!(TypeFloat, [], [(IdResult, One), (LiteralInteger, One)]),
    inst!(
        TypeVector,
        [],
        [(IdResult, One), (IdRef, One), (LiteralInteger, One)]
    ),
    inst!(
        TypeMatrix,
        [Matrix],
        [(IdResult, One), (IdRef, One), (LiteralInteger, One)]
    ),
    inst!(
        TypeImage,
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
    inst!(TypeSampler, [], [(IdResult, One)]),
    inst!(TypeSampledImage, [], [(IdResult, One), (IdRef, One)]),
    inst!(TypeArray, [], [(IdResult, One), (IdRef, One), (IdRef, One)]),
    inst!(TypeRuntimeArray, [Shader], [(IdResult, One), (IdRef, One)]),
    inst!(TypeStruct, [], [(IdResult, One), (IdRef, ZeroOrMore)]),
    inst!(
        TypeOpaque,
        [Kernel],
        [(IdResult, One), (LiteralString, One)]
    ),
    inst!(
        TypePointer,
        [],
        [(IdResult, One), (StorageClass, One), (IdRef, One)]
    ),
    inst!(
        TypeFunction,
        [],
        [(IdResult, One), (IdRef, One), (IdRef, ZeroOrMore)]
    ),
    inst!(TypeEvent, [Kernel], [(IdResult, One)]),
    inst!(TypeDeviceEvent, [DeviceEnqueue], [(IdResult, One)]),
    inst!(TypeReserveId, [Pipes], [(IdResult, One)]),
    inst!(TypeQueue, [DeviceEnqueue], [(IdResult, One)]),
    inst!(TypePipe, [Pipes], [(IdResult, One), (AccessQualifier, One)]),
    inst!(
        TypeForwardPointer,
        [Addresses],
        [(IdRef, One), (StorageClass, One)]
    ),
    inst!(ConstantTrue, [], [(IdResultType, One), (IdResult, One)]),
    inst!(ConstantFalse, [], [(IdResultType, One), (IdResult, One)]),
    inst!(
        Constant,
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
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        ConstantSampler,
        [LiteralSampler],
        [
            (IdResultType, One),
            (IdResult, One),
            (SamplerAddressingMode, One),
            (LiteralInteger, One),
            (SamplerFilterMode, One)
        ]
    ),
    inst!(ConstantNull, [], [(IdResultType, One), (IdResult, One)]),
    inst!(SpecConstantTrue, [], [(IdResultType, One), (IdResult, One)]),
    inst!(
        SpecConstantFalse,
        [],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        SpecConstant,
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
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        SpecConstantOp,
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
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(FunctionEnd, [], []),
    inst!(
        FunctionCall,
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
        [(IdRef, One), (IdRef, One), (MemoryAccess, ZeroOrOne)]
    ),
    inst!(
        CopyMemory,
        [],
        [(IdRef, One), (IdRef, One), (MemoryAccess, ZeroOrOne)]
    ),
    inst!(
        CopyMemorySized,
        [Addresses],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (MemoryAccess, ZeroOrOne)
        ]
    ),
    inst!(
        AccessChain,
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
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(
        PtrAccessChain,
        [Addresses, VariablePointers, VariablePointersStorageBuffer],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        InBoundsPtrAccessChain,
        [Addresses],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    inst!(Decorate, [], [(IdRef, One), (Decoration, One)]),
    inst!(
        MemberDecorate,
        [],
        [(IdRef, One), (LiteralInteger, One), (Decoration, One)]
    ),
    inst!(DecorationGroup, [], [(IdResult, One)]),
    inst!(GroupDecorate, [], [(IdRef, One), (IdRef, ZeroOrMore)]),
    inst!(
        GroupMemberDecorate,
        [],
        [(IdRef, One), (PairIdRefLiteralInteger, ZeroOrMore)]
    ),
    inst!(
        VectorExtractDynamic,
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
        [(IdResultType, One), (IdResult, One), (IdRef, ZeroOrMore)]
    ),
    inst!(
        CompositeExtract,
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        Transpose,
        [Matrix],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SampledImage,
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQueryFormat,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQueryOrder,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQuerySizeLod,
        [Kernel, ImageQuery],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQueryLod,
        [ImageQuery],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ImageQuerySamples,
        [Kernel, ImageQuery],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertFToU,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertFToS,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertSToF,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertUToF,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        UConvert,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SConvert,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FConvert,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        QuantizeToF16,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertPtrToU,
        [Addresses],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SatConvertSToU,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SatConvertUToS,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        ConvertUToPtr,
        [Addresses],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        PtrCastToGeneric,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GenericCastToPtr,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GenericCastToPtrExplicit,
        [Kernel],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SNegate,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FNegate,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IAdd,
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        All,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IsNan,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IsInf,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IsFinite,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        IsNormal,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SignBitSet,
        [Kernel],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        LessOrGreater,
        [Kernel],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        Select,
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        BitFieldInsert,
        [Shader],
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
        [Shader],
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
        [Shader],
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
        [Shader],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        BitCount,
        [],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdx,
        [Shader],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdy,
        [Shader],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        Fwidth,
        [Shader],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdxFine,
        [DerivativeControl],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdyFine,
        [DerivativeControl],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FwidthFine,
        [DerivativeControl],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdxCoarse,
        [DerivativeControl],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        DPdyCoarse,
        [DerivativeControl],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        FwidthCoarse,
        [DerivativeControl],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(EmitVertex, [Geometry], []),
    inst!(EndPrimitive, [Geometry], []),
    inst!(EmitStreamVertex, [GeometryStreams], [(IdRef, One)]),
    inst!(EndStreamPrimitive, [GeometryStreams], [(IdRef, One)]),
    inst!(
        ControlBarrier,
        [],
        [(IdScope, One), (IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(
        MemoryBarrier,
        [],
        [(IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(
        AtomicLoad,
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
        [
            (IdResultType, One),
            (IdResult, One),
            (PairIdRefIdRef, ZeroOrMore)
        ]
    ),
    inst!(
        LoopMerge,
        [],
        [(IdRef, One), (IdRef, One), (LoopControl, One)]
    ),
    inst!(SelectionMerge, [], [(IdRef, One), (SelectionControl, One)]),
    inst!(Label, [], [(IdResult, One)]),
    inst!(Branch, [], [(IdRef, One)]),
    inst!(
        BranchConditional,
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
        [
            (IdRef, One),
            (IdRef, One),
            (PairLiteralIntegerIdRef, ZeroOrMore)
        ]
    ),
    inst!(Kill, [Shader], []),
    inst!(Return, [], []),
    inst!(ReturnValue, [], [(IdRef, One)]),
    inst!(Unreachable, [], []),
    inst!(
        LifetimeStart,
        [Kernel],
        [(IdRef, One), (LiteralInteger, One)]
    ),
    inst!(
        LifetimeStop,
        [Kernel],
        [(IdRef, One), (LiteralInteger, One)]
    ),
    inst!(
        GroupAsyncCopy,
        [Kernel],
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
        [(IdScope, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        GroupAll,
        [Groups],
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
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        CommitWritePipe,
        [Pipes],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        IsValidReserveId,
        [Pipes],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GetNumPipePackets,
        [Pipes],
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
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(RetainEvent, [DeviceEnqueue], [(IdRef, One)]),
    inst!(ReleaseEvent, [DeviceEnqueue], [(IdRef, One)]),
    inst!(
        CreateUserEvent,
        [DeviceEnqueue],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        IsValidEvent,
        [DeviceEnqueue],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SetUserEventStatus,
        [DeviceEnqueue],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        CaptureEventProfilingInfo,
        [DeviceEnqueue],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(
        GetDefaultQueue,
        [DeviceEnqueue],
        [(IdResultType, One), (IdResult, One)]
    ),
    inst!(
        BuildNDRange,
        [DeviceEnqueue],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(NoLine, [], []),
    inst!(
        AtomicFlagTestAndSet,
        [Kernel],
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
        [(IdRef, One), (IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(
        ImageSparseRead,
        [SparseResidency],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(TypePipeStorage, [PipeStorage], [(IdResult, One)]),
    inst!(
        ConstantPipeStorage,
        [PipeStorage],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        GetKernelLocalSizeForSubgroupCount,
        [SubgroupDispatch],
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
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(TypeNamedBarrier, [NamedBarrier], [(IdResult, One)]),
    inst!(
        NamedBarrierInitialize,
        [NamedBarrier],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        MemoryNamedBarrier,
        [NamedBarrier],
        [(IdRef, One), (IdScope, One), (IdMemorySemantics, One)]
    ),
    inst!(ModuleProcessed, [], [(LiteralString, One)]),
    inst!(ExecutionModeId, [], [(IdRef, One), (ExecutionMode, One)]),
    inst!(DecorateId, [], [(IdRef, One), (Decoration, One)]),
    inst!(
        GroupNonUniformElect,
        [GroupNonUniform],
        [(IdResultType, One), (IdResult, One), (IdScope, One)]
    ),
    inst!(
        GroupNonUniformAll,
        [GroupNonUniformVote],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [GroupNonUniformArithmetic, GroupNonUniformClustered],
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
        [
            (IdResultType, One),
            (IdResult, One),
            (IdScope, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupBallotKHR,
        [SubgroupBallotKHR],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupFirstInvocationKHR,
        [SubgroupBallotKHR],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAllKHR,
        [SubgroupVoteKHR],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAnyKHR,
        [SubgroupVoteKHR],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupAllEqualKHR,
        [SubgroupVoteKHR],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupReadInvocationKHR,
        [SubgroupBallotKHR],
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        GroupIAddNonUniformAMD,
        [Groups],
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
        [
            (IdResultType, One),
            (IdResult, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    inst!(
        SubgroupShuffleINTEL,
        [SubgroupShuffleINTEL],
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
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
    inst!(
        SubgroupBlockWriteINTEL,
        [SubgroupBufferBlockIOINTEL],
        [(IdRef, One), (IdRef, One)]
    ),
    inst!(
        SubgroupImageBlockReadINTEL,
        [SubgroupImageBlockIOINTEL],
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
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    inst!(DecorateStringGOOGLE, [], [(IdRef, One), (Decoration, One)]),
    inst!(
        MemberDecorateStringGOOGLE,
        [],
        [(IdRef, One), (LiteralInteger, One), (Decoration, One)]
    ),
    inst!(
        GroupNonUniformPartitionNV,
        [GroupNonUniformPartitionedNV],
        [(IdResultType, One), (IdResult, One), (IdRef, One)]
    ),
];
