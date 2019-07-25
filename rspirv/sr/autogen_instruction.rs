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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Terminator {
    Branch {
        target_label: Token<super::types::Type>,
    },
    BranchConditional {
        condition: Token<super::types::Type>,
        true_label: Token<super::types::Type>,
        false_label: Token<super::types::Type>,
        branch_weights: Vec<u32>,
    },
    Switch {
        selector: Token<super::types::Type>,
        default: Token<super::types::Type>,
        target: Vec<(u32, spirv::Word)>,
    },
    Kill,
    Return,
    ReturnValue {
        value: Token<super::types::Type>,
    },
    Unreachable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Nop,
    Undef,
    SourceContinued {
        continued_source: String,
    },
    Source {
        source_language: spirv::SourceLanguage,
        version: u32,
        file: Option<Token<super::types::Type>>,
        source: Option<String>,
    },
    SourceExtension {
        extension: String,
    },
    Name {
        target: Token<super::types::Type>,
        name: String,
    },
    MemberName {
        target_type: Token<super::types::Type>,
        member: u32,
        name: String,
    },
    String {
        string: String,
    },
    Line {
        file: Token<super::types::Type>,
        line: u32,
        column: u32,
    },
    ExtInst {
        set: Token<super::types::Type>,
        instruction: u32,
        operands: Vec<Token<super::types::Type>>,
    },
    FunctionCall {
        function: Token<super::types::Type>,
        arguments: Vec<Token<super::types::Type>>,
    },
    Variable {
        storage_class: spirv::StorageClass,
        initializer: Option<Token<super::types::Type>>,
    },
    ImageTexelPointer {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        sample: Token<super::types::Type>,
    },
    Load {
        pointer: Token<super::types::Type>,
        memory_access: Option<spirv::MemoryAccess>,
    },
    Store {
        pointer: Token<super::types::Type>,
        object: Token<super::types::Type>,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CopyMemory {
        target: Token<super::types::Type>,
        source: Token<super::types::Type>,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CopyMemorySized {
        target: Token<super::types::Type>,
        source: Token<super::types::Type>,
        size: Token<super::types::Type>,
        memory_access: Option<spirv::MemoryAccess>,
    },
    AccessChain {
        base: Token<super::types::Type>,
        indexes: Vec<Token<super::types::Type>>,
    },
    InBoundsAccessChain {
        base: Token<super::types::Type>,
        indexes: Vec<Token<super::types::Type>>,
    },
    PtrAccessChain {
        base: Token<super::types::Type>,
        element: Token<super::types::Type>,
        indexes: Vec<Token<super::types::Type>>,
    },
    ArrayLength {
        structure: Token<super::types::Type>,
        array_member: u32,
    },
    GenericPtrMemSemantics {
        pointer: Token<super::types::Type>,
    },
    InBoundsPtrAccessChain {
        base: Token<super::types::Type>,
        element: Token<super::types::Type>,
        indexes: Vec<Token<super::types::Type>>,
    },
    Decorate {
        target: Token<super::types::Type>,
        decoration: spirv::Decoration,
    },
    MemberDecorate {
        structure_type: Token<super::types::Type>,
        member: u32,
        decoration: spirv::Decoration,
    },
    DecorationGroup,
    GroupDecorate {
        decoration_group: Token<super::types::Type>,
        targets: Vec<Token<super::types::Type>>,
    },
    GroupMemberDecorate {
        decoration_group: Token<super::types::Type>,
        targets: Vec<(spirv::Word, u32)>,
    },
    VectorExtractDynamic {
        vector: Token<super::types::Type>,
        index: Token<super::types::Type>,
    },
    VectorInsertDynamic {
        vector: Token<super::types::Type>,
        component: Token<super::types::Type>,
        index: Token<super::types::Type>,
    },
    VectorShuffle {
        vector_1: Token<super::types::Type>,
        vector_2: Token<super::types::Type>,
        components: Vec<u32>,
    },
    CompositeConstruct {
        constituents: Vec<Token<super::types::Type>>,
    },
    CompositeExtract {
        composite: Token<super::types::Type>,
        indexes: Vec<u32>,
    },
    CompositeInsert {
        object: Token<super::types::Type>,
        composite: Token<super::types::Type>,
        indexes: Vec<u32>,
    },
    CopyObject {
        operand: Token<super::types::Type>,
    },
    Transpose {
        matrix: Token<super::types::Type>,
    },
    SampledImage {
        image: Token<super::types::Type>,
        sampler: Token<super::types::Type>,
    },
    ImageSampleImplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleExplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleDrefImplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleDrefExplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleProjImplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleProjExplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleProjDrefImplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleProjDrefExplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: spirv::ImageOperands,
    },
    ImageFetch {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageGather {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        component: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageDrefGather {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageRead {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageWrite {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        texel: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    Image {
        sampled_image: Token<super::types::Type>,
    },
    ImageQueryFormat {
        image: Token<super::types::Type>,
    },
    ImageQueryOrder {
        image: Token<super::types::Type>,
    },
    ImageQuerySizeLod {
        image: Token<super::types::Type>,
        level_of_detail: Token<super::types::Type>,
    },
    ImageQuerySize {
        image: Token<super::types::Type>,
    },
    ImageQueryLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
    },
    ImageQueryLevels {
        image: Token<super::types::Type>,
    },
    ImageQuerySamples {
        image: Token<super::types::Type>,
    },
    ConvertFToU {
        float_value: Token<super::types::Type>,
    },
    ConvertFToS {
        float_value: Token<super::types::Type>,
    },
    ConvertSToF {
        signed_value: Token<super::types::Type>,
    },
    ConvertUToF {
        unsigned_value: Token<super::types::Type>,
    },
    UConvert {
        unsigned_value: Token<super::types::Type>,
    },
    SConvert {
        signed_value: Token<super::types::Type>,
    },
    FConvert {
        float_value: Token<super::types::Type>,
    },
    QuantizeToF16 {
        value: Token<super::types::Type>,
    },
    ConvertPtrToU {
        pointer: Token<super::types::Type>,
    },
    SatConvertSToU {
        signed_value: Token<super::types::Type>,
    },
    SatConvertUToS {
        unsigned_value: Token<super::types::Type>,
    },
    ConvertUToPtr {
        integer_value: Token<super::types::Type>,
    },
    PtrCastToGeneric {
        pointer: Token<super::types::Type>,
    },
    GenericCastToPtr {
        pointer: Token<super::types::Type>,
    },
    GenericCastToPtrExplicit {
        pointer: Token<super::types::Type>,
        storage: spirv::StorageClass,
    },
    Bitcast {
        operand: Token<super::types::Type>,
    },
    SNegate {
        operand: Token<super::types::Type>,
    },
    FNegate {
        operand: Token<super::types::Type>,
    },
    IAdd {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FAdd {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    ISub {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FSub {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    IMul {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FMul {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    UDiv {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    SDiv {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FDiv {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    UMod {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    SRem {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    SMod {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FRem {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FMod {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    VectorTimesScalar {
        vector: Token<super::types::Type>,
        scalar: Token<super::types::Type>,
    },
    MatrixTimesScalar {
        matrix: Token<super::types::Type>,
        scalar: Token<super::types::Type>,
    },
    VectorTimesMatrix {
        vector: Token<super::types::Type>,
        matrix: Token<super::types::Type>,
    },
    MatrixTimesVector {
        matrix: Token<super::types::Type>,
        vector: Token<super::types::Type>,
    },
    MatrixTimesMatrix {
        left_matrix: Token<super::types::Type>,
        right_matrix: Token<super::types::Type>,
    },
    OuterProduct {
        vector_1: Token<super::types::Type>,
        vector_2: Token<super::types::Type>,
    },
    Dot {
        vector_1: Token<super::types::Type>,
        vector_2: Token<super::types::Type>,
    },
    IAddCarry {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    ISubBorrow {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    UMulExtended {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    SMulExtended {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    Any {
        vector: Token<super::types::Type>,
    },
    All {
        vector: Token<super::types::Type>,
    },
    IsNan {
        x: Token<super::types::Type>,
    },
    IsInf {
        x: Token<super::types::Type>,
    },
    IsFinite {
        x: Token<super::types::Type>,
    },
    IsNormal {
        x: Token<super::types::Type>,
    },
    SignBitSet {
        x: Token<super::types::Type>,
    },
    LessOrGreater {
        x: Token<super::types::Type>,
        y: Token<super::types::Type>,
    },
    Ordered {
        x: Token<super::types::Type>,
        y: Token<super::types::Type>,
    },
    Unordered {
        x: Token<super::types::Type>,
        y: Token<super::types::Type>,
    },
    LogicalEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    LogicalNotEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    LogicalOr {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    LogicalAnd {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    LogicalNot {
        operand: Token<super::types::Type>,
    },
    Select {
        condition: Token<super::types::Type>,
        object_1: Token<super::types::Type>,
        object_2: Token<super::types::Type>,
    },
    IEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    INotEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    UGreaterThan {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    SGreaterThan {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    UGreaterThanEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    SGreaterThanEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    ULessThan {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    SLessThan {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    ULessThanEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    SLessThanEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FOrdEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FUnordEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FOrdNotEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FUnordNotEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FOrdLessThan {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FUnordLessThan {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FOrdGreaterThan {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FUnordGreaterThan {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FOrdLessThanEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FUnordLessThanEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FOrdGreaterThanEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    FUnordGreaterThanEqual {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    ShiftRightLogical {
        base: Token<super::types::Type>,
        shift: Token<super::types::Type>,
    },
    ShiftRightArithmetic {
        base: Token<super::types::Type>,
        shift: Token<super::types::Type>,
    },
    ShiftLeftLogical {
        base: Token<super::types::Type>,
        shift: Token<super::types::Type>,
    },
    BitwiseOr {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    BitwiseXor {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    BitwiseAnd {
        operand_1: Token<super::types::Type>,
        operand_2: Token<super::types::Type>,
    },
    Not {
        operand: Token<super::types::Type>,
    },
    BitFieldInsert {
        base: Token<super::types::Type>,
        insert: Token<super::types::Type>,
        offset: Token<super::types::Type>,
        count: Token<super::types::Type>,
    },
    BitFieldSExtract {
        base: Token<super::types::Type>,
        offset: Token<super::types::Type>,
        count: Token<super::types::Type>,
    },
    BitFieldUExtract {
        base: Token<super::types::Type>,
        offset: Token<super::types::Type>,
        count: Token<super::types::Type>,
    },
    BitReverse {
        base: Token<super::types::Type>,
    },
    BitCount {
        base: Token<super::types::Type>,
    },
    DPdx {
        p: Token<super::types::Type>,
    },
    DPdy {
        p: Token<super::types::Type>,
    },
    Fwidth {
        p: Token<super::types::Type>,
    },
    DPdxFine {
        p: Token<super::types::Type>,
    },
    DPdyFine {
        p: Token<super::types::Type>,
    },
    FwidthFine {
        p: Token<super::types::Type>,
    },
    DPdxCoarse {
        p: Token<super::types::Type>,
    },
    DPdyCoarse {
        p: Token<super::types::Type>,
    },
    FwidthCoarse {
        p: Token<super::types::Type>,
    },
    EmitVertex,
    EndPrimitive,
    EmitStreamVertex {
        stream: Token<super::types::Type>,
    },
    EndStreamPrimitive {
        stream: Token<super::types::Type>,
    },
    ControlBarrier {
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    MemoryBarrier {
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicLoad {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicStore {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicExchange {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicCompareExchange {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: Token<super::types::Type>,
        comparator: Token<super::types::Type>,
    },
    AtomicCompareExchangeWeak {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: Token<super::types::Type>,
        comparator: Token<super::types::Type>,
    },
    AtomicIIncrement {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicIDecrement {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicIAdd {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicISub {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicSMin {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicUMin {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicSMax {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicUMax {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicAnd {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicOr {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    AtomicXor {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: Token<super::types::Type>,
    },
    Phi {
        value_label_pairs: Vec<(spirv::Word, spirv::Word)>,
    },
    LoopMerge {
        merge_block: Token<super::types::Type>,
        continue_target: Token<super::types::Type>,
        loop_control: spirv::LoopControl,
    },
    SelectionMerge {
        merge_block: Token<super::types::Type>,
        selection_control: spirv::SelectionControl,
    },
    LifetimeStart {
        pointer: Token<super::types::Type>,
        size: u32,
    },
    LifetimeStop {
        pointer: Token<super::types::Type>,
        size: u32,
    },
    GroupAsyncCopy {
        execution: spirv::Word,
        destination: Token<super::types::Type>,
        source: Token<super::types::Type>,
        num_elements: Token<super::types::Type>,
        stride: Token<super::types::Type>,
        event: Token<super::types::Type>,
    },
    GroupWaitEvents {
        execution: spirv::Word,
        num_events: Token<super::types::Type>,
        events_list: Token<super::types::Type>,
    },
    GroupAll {
        execution: spirv::Word,
        predicate: Token<super::types::Type>,
    },
    GroupAny {
        execution: spirv::Word,
        predicate: Token<super::types::Type>,
    },
    GroupBroadcast {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        local_id: Token<super::types::Type>,
    },
    GroupIAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupFAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupFMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupUMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupSMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupFMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupUMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupSMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    ReadPipe {
        pipe: Token<super::types::Type>,
        pointer: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    WritePipe {
        pipe: Token<super::types::Type>,
        pointer: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    ReservedReadPipe {
        pipe: Token<super::types::Type>,
        reserve_id: Token<super::types::Type>,
        index: Token<super::types::Type>,
        pointer: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    ReservedWritePipe {
        pipe: Token<super::types::Type>,
        reserve_id: Token<super::types::Type>,
        index: Token<super::types::Type>,
        pointer: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    ReserveReadPipePackets {
        pipe: Token<super::types::Type>,
        num_packets: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    ReserveWritePipePackets {
        pipe: Token<super::types::Type>,
        num_packets: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    CommitReadPipe {
        pipe: Token<super::types::Type>,
        reserve_id: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    CommitWritePipe {
        pipe: Token<super::types::Type>,
        reserve_id: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    IsValidReserveId {
        reserve_id: Token<super::types::Type>,
    },
    GetNumPipePackets {
        pipe: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    GetMaxPipePackets {
        pipe: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    GroupReserveReadPipePackets {
        execution: spirv::Word,
        pipe: Token<super::types::Type>,
        num_packets: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    GroupReserveWritePipePackets {
        execution: spirv::Word,
        pipe: Token<super::types::Type>,
        num_packets: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    GroupCommitReadPipe {
        execution: spirv::Word,
        pipe: Token<super::types::Type>,
        reserve_id: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    GroupCommitWritePipe {
        execution: spirv::Word,
        pipe: Token<super::types::Type>,
        reserve_id: Token<super::types::Type>,
        packet_size: Token<super::types::Type>,
        packet_alignment: Token<super::types::Type>,
    },
    EnqueueMarker {
        queue: Token<super::types::Type>,
        num_events: Token<super::types::Type>,
        wait_events: Token<super::types::Type>,
        ret_event: Token<super::types::Type>,
    },
    EnqueueKernel {
        queue: Token<super::types::Type>,
        flags: Token<super::types::Type>,
        nd_range: Token<super::types::Type>,
        num_events: Token<super::types::Type>,
        wait_events: Token<super::types::Type>,
        ret_event: Token<super::types::Type>,
        invoke: Token<super::types::Type>,
        param: Token<super::types::Type>,
        param_size: Token<super::types::Type>,
        param_align: Token<super::types::Type>,
        local_size: Vec<Token<super::types::Type>>,
    },
    GetKernelNDrangeSubGroupCount {
        nd_range: Token<super::types::Type>,
        invoke: Token<super::types::Type>,
        param: Token<super::types::Type>,
        param_size: Token<super::types::Type>,
        param_align: Token<super::types::Type>,
    },
    GetKernelNDrangeMaxSubGroupSize {
        nd_range: Token<super::types::Type>,
        invoke: Token<super::types::Type>,
        param: Token<super::types::Type>,
        param_size: Token<super::types::Type>,
        param_align: Token<super::types::Type>,
    },
    GetKernelWorkGroupSize {
        invoke: Token<super::types::Type>,
        param: Token<super::types::Type>,
        param_size: Token<super::types::Type>,
        param_align: Token<super::types::Type>,
    },
    GetKernelPreferredWorkGroupSizeMultiple {
        invoke: Token<super::types::Type>,
        param: Token<super::types::Type>,
        param_size: Token<super::types::Type>,
        param_align: Token<super::types::Type>,
    },
    RetainEvent {
        event: Token<super::types::Type>,
    },
    ReleaseEvent {
        event: Token<super::types::Type>,
    },
    CreateUserEvent,
    IsValidEvent {
        event: Token<super::types::Type>,
    },
    SetUserEventStatus {
        event: Token<super::types::Type>,
        status: Token<super::types::Type>,
    },
    CaptureEventProfilingInfo {
        event: Token<super::types::Type>,
        profiling_info: Token<super::types::Type>,
        value: Token<super::types::Type>,
    },
    GetDefaultQueue,
    BuildNDRange {
        global_work_size: Token<super::types::Type>,
        local_work_size: Token<super::types::Type>,
        global_work_offset: Token<super::types::Type>,
    },
    ImageSparseSampleImplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleExplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleDrefImplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleDrefExplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleProjImplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleProjExplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleProjDrefImplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleProjDrefExplicitLod {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseFetch {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseGather {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        component: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseDrefGather {
        sampled_image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        dref: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseTexelsResident {
        resident_code: Token<super::types::Type>,
    },
    NoLine,
    AtomicFlagTestAndSet {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicFlagClear {
        pointer: Token<super::types::Type>,
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    ImageSparseRead {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        image_operands: Option<spirv::ImageOperands>,
    },
    SizeOf {
        pointer: Token<super::types::Type>,
    },
    CreatePipeFromPipeStorage {
        pipe_storage: Token<super::types::Type>,
    },
    GetKernelLocalSizeForSubgroupCount {
        subgroup_count: Token<super::types::Type>,
        invoke: Token<super::types::Type>,
        param: Token<super::types::Type>,
        param_size: Token<super::types::Type>,
        param_align: Token<super::types::Type>,
    },
    GetKernelMaxNumSubgroups {
        invoke: Token<super::types::Type>,
        param: Token<super::types::Type>,
        param_size: Token<super::types::Type>,
        param_align: Token<super::types::Type>,
    },
    NamedBarrierInitialize {
        subgroup_count: Token<super::types::Type>,
    },
    MemoryNamedBarrier {
        named_barrier: Token<super::types::Type>,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    ModuleProcessed {
        process: String,
    },
    DecorateId {
        target: Token<super::types::Type>,
        decoration: spirv::Decoration,
    },
    GroupNonUniformElect {
        execution: spirv::Word,
    },
    GroupNonUniformAll {
        execution: spirv::Word,
        predicate: Token<super::types::Type>,
    },
    GroupNonUniformAny {
        execution: spirv::Word,
        predicate: Token<super::types::Type>,
    },
    GroupNonUniformAllEqual {
        execution: spirv::Word,
        value: Token<super::types::Type>,
    },
    GroupNonUniformBroadcast {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        id: Token<super::types::Type>,
    },
    GroupNonUniformBroadcastFirst {
        execution: spirv::Word,
        value: Token<super::types::Type>,
    },
    GroupNonUniformBallot {
        execution: spirv::Word,
        predicate: Token<super::types::Type>,
    },
    GroupNonUniformInverseBallot {
        execution: spirv::Word,
        value: Token<super::types::Type>,
    },
    GroupNonUniformBallotBitExtract {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        index: Token<super::types::Type>,
    },
    GroupNonUniformBallotBitCount {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
    },
    GroupNonUniformBallotFindLSB {
        execution: spirv::Word,
        value: Token<super::types::Type>,
    },
    GroupNonUniformBallotFindMSB {
        execution: spirv::Word,
        value: Token<super::types::Type>,
    },
    GroupNonUniformShuffle {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        id: Token<super::types::Type>,
    },
    GroupNonUniformShuffleXor {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        mask: Token<super::types::Type>,
    },
    GroupNonUniformShuffleUp {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        delta: Token<super::types::Type>,
    },
    GroupNonUniformShuffleDown {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        delta: Token<super::types::Type>,
    },
    GroupNonUniformIAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformFAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformIMul {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformFMul {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformSMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformUMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformFMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformSMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformUMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformFMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformBitwiseAnd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformBitwiseOr {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformBitwiseXor {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformLogicalAnd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformLogicalOr {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformLogicalXor {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
    },
    GroupNonUniformQuadBroadcast {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        index: Token<super::types::Type>,
    },
    GroupNonUniformQuadSwap {
        execution: spirv::Word,
        value: Token<super::types::Type>,
        direction: Token<super::types::Type>,
    },
    SubgroupBallotKHR {
        predicate: Token<super::types::Type>,
    },
    SubgroupFirstInvocationKHR {
        value: Token<super::types::Type>,
    },
    SubgroupAllKHR {
        predicate: Token<super::types::Type>,
    },
    SubgroupAnyKHR {
        predicate: Token<super::types::Type>,
    },
    SubgroupAllEqualKHR {
        predicate: Token<super::types::Type>,
    },
    SubgroupReadInvocationKHR {
        value: Token<super::types::Type>,
        index: Token<super::types::Type>,
    },
    GroupIAddNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupFAddNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupFMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupUMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupSMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupFMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupUMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    GroupSMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: Token<super::types::Type>,
    },
    FragmentMaskFetchAMD {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
    },
    FragmentFetchAMD {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        fragment_index: Token<super::types::Type>,
    },
    SubgroupShuffleINTEL {
        data: Token<super::types::Type>,
        invocation_id: Token<super::types::Type>,
    },
    SubgroupShuffleDownINTEL {
        current: Token<super::types::Type>,
        next: Token<super::types::Type>,
        delta: Token<super::types::Type>,
    },
    SubgroupShuffleUpINTEL {
        previous: Token<super::types::Type>,
        current: Token<super::types::Type>,
        delta: Token<super::types::Type>,
    },
    SubgroupShuffleXorINTEL {
        data: Token<super::types::Type>,
        value: Token<super::types::Type>,
    },
    SubgroupBlockReadINTEL {
        ptr: Token<super::types::Type>,
    },
    SubgroupBlockWriteINTEL {
        ptr: Token<super::types::Type>,
        data: Token<super::types::Type>,
    },
    SubgroupImageBlockReadINTEL {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
    },
    SubgroupImageBlockWriteINTEL {
        image: Token<super::types::Type>,
        coordinate: Token<super::types::Type>,
        data: Token<super::types::Type>,
    },
    DecorateStringGOOGLE {
        target: Token<super::types::Type>,
        decoration: spirv::Decoration,
    },
    MemberDecorateStringGOOGLE {
        struct_type: Token<super::types::Type>,
        member: u32,
        decoration: spirv::Decoration,
    },
    GroupNonUniformPartitionNV {
        value: Token<super::types::Type>,
    },
}
