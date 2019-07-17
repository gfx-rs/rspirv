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
<<<<<<< HEAD
        target_label: spirv::Word,
    },
    BranchConditional {
        condition: spirv::Word,
        true_label: spirv::Word,
        false_label: spirv::Word,
        branch_weights: Vec<u32>,
    },
    Switch {
        selector: spirv::Word,
        default: spirv::Word,
=======
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
>>>>>>> Separate type structs
        target: Vec<(u32, spirv::Word)>,
    },
    Kill {},
    Return {},
    ReturnValue {
<<<<<<< HEAD
        value: spirv::Word,
=======
        value: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    Unreachable {},
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
<<<<<<< HEAD
        file: Option<spirv::Word>,
=======
        file: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
        source: Option<String>,
    },
    SourceExtension {
        extension: String,
    },
    Name {
<<<<<<< HEAD
        target: spirv::Word,
        name: String,
    },
    MemberName {
        target_type: spirv::Word,
=======
        target: Token<super::types::Type>,
        name: String,
    },
    MemberName {
        target_type: Token<super::types::Type>,
>>>>>>> Separate type structs
        member: u32,
        name: String,
    },
    String {
        string: String,
    },
    Line {
<<<<<<< HEAD
        file: spirv::Word,
=======
        file: Token<super::types::Type>,
>>>>>>> Separate type structs
        line: u32,
        column: u32,
    },
    ExtInst {
<<<<<<< HEAD
        set: spirv::Word,
        instruction: u32,
        operands: Vec<spirv::Word>,
    },
    FunctionCall {
        function: spirv::Word,
        arguments: Vec<spirv::Word>,
    },
    Variable {
        storage_class: spirv::StorageClass,
        initializer: Option<spirv::Word>,
    },
    ImageTexelPointer {
        image: spirv::Word,
        coordinate: spirv::Word,
        sample: spirv::Word,
    },
    Load {
        pointer: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    Store {
        pointer: spirv::Word,
        object: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CopyMemory {
        target: spirv::Word,
        source: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CopyMemorySized {
        target: spirv::Word,
        source: spirv::Word,
        size: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    AccessChain {
        base: spirv::Word,
        indexes: Vec<spirv::Word>,
    },
    InBoundsAccessChain {
        base: spirv::Word,
        indexes: Vec<spirv::Word>,
    },
    PtrAccessChain {
        base: spirv::Word,
        element: spirv::Word,
        indexes: Vec<spirv::Word>,
    },
    ArrayLength {
        structure: spirv::Word,
        array_member: u32,
    },
    GenericPtrMemSemantics {
        pointer: spirv::Word,
    },
    InBoundsPtrAccessChain {
        base: spirv::Word,
        element: spirv::Word,
        indexes: Vec<spirv::Word>,
    },
    Decorate {
        target: spirv::Word,
        decoration: spirv::Decoration,
    },
    MemberDecorate {
        structure_type: spirv::Word,
=======
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
>>>>>>> Separate type structs
        member: u32,
        decoration: spirv::Decoration,
    },
    DecorationGroup,
    GroupDecorate {
<<<<<<< HEAD
        decoration_group: spirv::Word,
        targets: Vec<spirv::Word>,
    },
    GroupMemberDecorate {
        decoration_group: spirv::Word,
        targets: Vec<(spirv::Word, u32)>,
    },
    VectorExtractDynamic {
        vector: spirv::Word,
        index: spirv::Word,
    },
    VectorInsertDynamic {
        vector: spirv::Word,
        component: spirv::Word,
        index: spirv::Word,
    },
    VectorShuffle {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        components: Vec<u32>,
    },
    CompositeConstruct {
        constituents: Vec<spirv::Word>,
    },
    CompositeExtract {
        composite: spirv::Word,
        indexes: Vec<u32>,
    },
    CompositeInsert {
        object: spirv::Word,
        composite: spirv::Word,
        indexes: Vec<u32>,
    },
    CopyObject {
        operand: spirv::Word,
    },
    Transpose {
        matrix: spirv::Word,
    },
    SampledImage {
        image: spirv::Word,
        sampler: spirv::Word,
    },
    ImageSampleImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleProjImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleProjExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleProjDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleProjDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageFetch {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageDrefGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageRead {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageWrite {
        image: spirv::Word,
        coordinate: spirv::Word,
        texel: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    Image {
        sampled_image: spirv::Word,
    },
    ImageQueryFormat {
        image: spirv::Word,
    },
    ImageQueryOrder {
        image: spirv::Word,
    },
    ImageQuerySizeLod {
        image: spirv::Word,
        level_of_detail: spirv::Word,
    },
    ImageQuerySize {
        image: spirv::Word,
    },
    ImageQueryLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
    },
    ImageQueryLevels {
        image: spirv::Word,
    },
    ImageQuerySamples {
        image: spirv::Word,
    },
    ConvertFToU {
        float_value: spirv::Word,
    },
    ConvertFToS {
        float_value: spirv::Word,
    },
    ConvertSToF {
        signed_value: spirv::Word,
    },
    ConvertUToF {
        unsigned_value: spirv::Word,
    },
    UConvert {
        unsigned_value: spirv::Word,
    },
    SConvert {
        signed_value: spirv::Word,
    },
    FConvert {
        float_value: spirv::Word,
    },
    QuantizeToF16 {
        value: spirv::Word,
    },
    ConvertPtrToU {
        pointer: spirv::Word,
    },
    SatConvertSToU {
        signed_value: spirv::Word,
    },
    SatConvertUToS {
        unsigned_value: spirv::Word,
    },
    ConvertUToPtr {
        integer_value: spirv::Word,
    },
    PtrCastToGeneric {
        pointer: spirv::Word,
    },
    GenericCastToPtr {
        pointer: spirv::Word,
    },
    GenericCastToPtrExplicit {
        pointer: spirv::Word,
        storage: spirv::StorageClass,
    },
    Bitcast {
        operand: spirv::Word,
    },
    SNegate {
        operand: spirv::Word,
    },
    FNegate {
        operand: spirv::Word,
    },
    IAdd {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FAdd {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ISub {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FSub {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    IMul {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FMul {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UDiv {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SDiv {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FDiv {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UMod {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SRem {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SMod {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FRem {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FMod {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    VectorTimesScalar {
        vector: spirv::Word,
        scalar: spirv::Word,
    },
    MatrixTimesScalar {
        matrix: spirv::Word,
        scalar: spirv::Word,
    },
    VectorTimesMatrix {
        vector: spirv::Word,
        matrix: spirv::Word,
    },
    MatrixTimesVector {
        matrix: spirv::Word,
        vector: spirv::Word,
    },
    MatrixTimesMatrix {
        left_matrix: spirv::Word,
        right_matrix: spirv::Word,
    },
    OuterProduct {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    },
    Dot {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    },
    IAddCarry {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ISubBorrow {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UMulExtended {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SMulExtended {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    Any {
        vector: spirv::Word,
    },
    All {
        vector: spirv::Word,
    },
    IsNan {
        x: spirv::Word,
    },
    IsInf {
        x: spirv::Word,
    },
    IsFinite {
        x: spirv::Word,
    },
    IsNormal {
        x: spirv::Word,
    },
    SignBitSet {
        x: spirv::Word,
    },
    LessOrGreater {
        x: spirv::Word,
        y: spirv::Word,
    },
    Ordered {
        x: spirv::Word,
        y: spirv::Word,
    },
    Unordered {
        x: spirv::Word,
        y: spirv::Word,
    },
    LogicalEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    LogicalNotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    LogicalOr {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    LogicalAnd {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    LogicalNot {
        operand: spirv::Word,
    },
    Select {
        condition: spirv::Word,
        object_1: spirv::Word,
        object_2: spirv::Word,
    },
    IEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    INotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UGreaterThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SGreaterThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UGreaterThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SGreaterThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ULessThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SLessThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ULessThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SLessThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdNotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordNotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdLessThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordLessThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdGreaterThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordGreaterThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdLessThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordLessThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdGreaterThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordGreaterThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ShiftRightLogical {
        base: spirv::Word,
        shift: spirv::Word,
    },
    ShiftRightArithmetic {
        base: spirv::Word,
        shift: spirv::Word,
    },
    ShiftLeftLogical {
        base: spirv::Word,
        shift: spirv::Word,
    },
    BitwiseOr {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    BitwiseXor {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    BitwiseAnd {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    Not {
        operand: spirv::Word,
    },
    BitFieldInsert {
        base: spirv::Word,
        insert: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    },
    BitFieldSExtract {
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    },
    BitFieldUExtract {
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    },
    BitReverse {
        base: spirv::Word,
    },
    BitCount {
        base: spirv::Word,
    },
    DPdx {
        p: spirv::Word,
    },
    DPdy {
        p: spirv::Word,
    },
    Fwidth {
        p: spirv::Word,
    },
    DPdxFine {
        p: spirv::Word,
    },
    DPdyFine {
        p: spirv::Word,
    },
    FwidthFine {
        p: spirv::Word,
    },
    DPdxCoarse {
        p: spirv::Word,
    },
    DPdyCoarse {
        p: spirv::Word,
    },
    FwidthCoarse {
        p: spirv::Word,
=======
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
>>>>>>> Separate type structs
    },
    EmitVertex,
    EndPrimitive,
    EmitStreamVertex {
<<<<<<< HEAD
        stream: spirv::Word,
    },
    EndStreamPrimitive {
        stream: spirv::Word,
=======
        stream: Token<super::types::Type>,
    },
    EndStreamPrimitive {
        stream: Token<super::types::Type>,
>>>>>>> Separate type structs
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
<<<<<<< HEAD
        pointer: spirv::Word,
=======
        pointer: Token<super::types::Type>,
>>>>>>> Separate type structs
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicStore {
<<<<<<< HEAD
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicExchange {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicCompareExchange {
        pointer: spirv::Word,
        scope: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    },
    AtomicCompareExchangeWeak {
        pointer: spirv::Word,
        scope: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    },
    AtomicIIncrement {
        pointer: spirv::Word,
=======
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
>>>>>>> Separate type structs
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicIDecrement {
<<<<<<< HEAD
        pointer: spirv::Word,
=======
        pointer: Token<super::types::Type>,
>>>>>>> Separate type structs
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicIAdd {
<<<<<<< HEAD
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicISub {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicSMin {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicUMin {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicSMax {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicUMax {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicAnd {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicOr {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicXor {
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
=======
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
>>>>>>> Separate type structs
    },
    Phi {
        value_label_pairs: Vec<(spirv::Word, spirv::Word)>,
    },
    LoopMerge {
<<<<<<< HEAD
        merge_block: spirv::Word,
        continue_target: spirv::Word,
        loop_control: spirv::LoopControl,
    },
    SelectionMerge {
        merge_block: spirv::Word,
        selection_control: spirv::SelectionControl,
    },
    LifetimeStart {
        pointer: spirv::Word,
        size: u32,
    },
    LifetimeStop {
        pointer: spirv::Word,
=======
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
>>>>>>> Separate type structs
        size: u32,
    },
    GroupAsyncCopy {
        execution: spirv::Word,
<<<<<<< HEAD
        destination: spirv::Word,
        source: spirv::Word,
        num_elements: spirv::Word,
        stride: spirv::Word,
        event: spirv::Word,
    },
    GroupWaitEvents {
        execution: spirv::Word,
        num_events: spirv::Word,
        events_list: spirv::Word,
    },
    GroupAll {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupAny {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupBroadcast {
        execution: spirv::Word,
        value: spirv::Word,
        local_id: spirv::Word,
=======
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
>>>>>>> Separate type structs
    },
    GroupIAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupFAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupFMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupUMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupSMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupFMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupUMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupSMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
    },
    ReadPipe {
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    WritePipe {
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    ReservedReadPipe {
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        index: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    ReservedWritePipe {
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        index: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    ReserveReadPipePackets {
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    ReserveWritePipePackets {
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    CommitReadPipe {
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    CommitWritePipe {
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    IsValidReserveId {
        reserve_id: spirv::Word,
    },
    GetNumPipePackets {
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GetMaxPipePackets {
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GroupReserveReadPipePackets {
        execution: spirv::Word,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GroupReserveWritePipePackets {
        execution: spirv::Word,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GroupCommitReadPipe {
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GroupCommitWritePipe {
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    EnqueueMarker {
        queue: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
    },
    EnqueueKernel {
        queue: spirv::Word,
        flags: spirv::Word,
        nd_range: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
        local_size: Vec<spirv::Word>,
    },
    GetKernelNDrangeSubGroupCount {
        nd_range: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    GetKernelNDrangeMaxSubGroupSize {
        nd_range: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    GetKernelWorkGroupSize {
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    GetKernelPreferredWorkGroupSizeMultiple {
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    RetainEvent {
        event: spirv::Word,
    },
    ReleaseEvent {
        event: spirv::Word,
    },
    CreateUserEvent,
    IsValidEvent {
        event: spirv::Word,
    },
    SetUserEventStatus {
        event: spirv::Word,
        status: spirv::Word,
    },
    CaptureEventProfilingInfo {
        event: spirv::Word,
        profiling_info: spirv::Word,
        value: spirv::Word,
    },
    GetDefaultQueue,
    BuildNDRange {
        global_work_size: spirv::Word,
        local_work_size: spirv::Word,
        global_work_offset: spirv::Word,
    },
    ImageSparseSampleImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleProjImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleProjExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleProjDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleProjDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseFetch {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseDrefGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        dref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseTexelsResident {
        resident_code: spirv::Word,
    },
    NoLine,
    AtomicFlagTestAndSet {
        pointer: spirv::Word,
=======
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
>>>>>>> Separate type structs
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicFlagClear {
<<<<<<< HEAD
        pointer: spirv::Word,
=======
        pointer: Token<super::types::Type>,
>>>>>>> Separate type structs
        scope: spirv::Word,
        semantics: spirv::Word,
    },
    ImageSparseRead {
<<<<<<< HEAD
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    SizeOf {
        pointer: spirv::Word,
    },
    CreatePipeFromPipeStorage {
        pipe_storage: spirv::Word,
    },
    GetKernelLocalSizeForSubgroupCount {
        subgroup_count: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    GetKernelMaxNumSubgroups {
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    NamedBarrierInitialize {
        subgroup_count: spirv::Word,
    },
    MemoryNamedBarrier {
        named_barrier: spirv::Word,
=======
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
>>>>>>> Separate type structs
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    ModuleProcessed {
        process: String,
    },
    DecorateId {
<<<<<<< HEAD
        target: spirv::Word,
=======
        target: Token<super::types::Type>,
>>>>>>> Separate type structs
        decoration: spirv::Decoration,
    },
    GroupNonUniformElect {
        execution: spirv::Word,
    },
    GroupNonUniformAll {
        execution: spirv::Word,
<<<<<<< HEAD
        predicate: spirv::Word,
    },
    GroupNonUniformAny {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupNonUniformAllEqual {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformBroadcast {
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    },
    GroupNonUniformBroadcastFirst {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformBallot {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupNonUniformInverseBallot {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformBallotBitExtract {
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
=======
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
>>>>>>> Separate type structs
    },
    GroupNonUniformBallotBitCount {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
    },
    GroupNonUniformBallotFindLSB {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformBallotFindMSB {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformShuffle {
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    },
    GroupNonUniformShuffleXor {
        execution: spirv::Word,
        value: spirv::Word,
        mask: spirv::Word,
    },
    GroupNonUniformShuffleUp {
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    },
    GroupNonUniformShuffleDown {
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
=======
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
>>>>>>> Separate type structs
    },
    GroupNonUniformIAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformFAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformIMul {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformFMul {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformSMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformUMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformFMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformSMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformUMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformFMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformBitwiseAnd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformBitwiseOr {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformBitwiseXor {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformLogicalAnd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformLogicalOr {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
=======
        value: Token<super::types::Type>,
        cluster_size: Option<Token<super::types::Type>>,
>>>>>>> Separate type structs
    },
    GroupNonUniformLogicalXor {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformQuadBroadcast {
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    },
    GroupNonUniformQuadSwap {
        execution: spirv::Word,
        value: spirv::Word,
        direction: spirv::Word,
    },
    SubgroupBallotKHR {
        predicate: spirv::Word,
    },
    SubgroupFirstInvocationKHR {
        value: spirv::Word,
    },
    SubgroupAllKHR {
        predicate: spirv::Word,
    },
    SubgroupAnyKHR {
        predicate: spirv::Word,
    },
    SubgroupAllEqualKHR {
        predicate: spirv::Word,
    },
    SubgroupReadInvocationKHR {
        value: spirv::Word,
        index: spirv::Word,
=======
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
>>>>>>> Separate type structs
    },
    GroupIAddNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupFAddNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupFMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupUMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupSMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupFMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupUMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
=======
        x: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
    GroupSMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
<<<<<<< HEAD
        x: spirv::Word,
    },
    FragmentMaskFetchAMD {
        image: spirv::Word,
        coordinate: spirv::Word,
    },
    FragmentFetchAMD {
        image: spirv::Word,
        coordinate: spirv::Word,
        fragment_index: spirv::Word,
    },
    SubgroupShuffleINTEL {
        data: spirv::Word,
        invocation_id: spirv::Word,
    },
    SubgroupShuffleDownINTEL {
        current: spirv::Word,
        next: spirv::Word,
        delta: spirv::Word,
    },
    SubgroupShuffleUpINTEL {
        previous: spirv::Word,
        current: spirv::Word,
        delta: spirv::Word,
    },
    SubgroupShuffleXorINTEL {
        data: spirv::Word,
        value: spirv::Word,
    },
    SubgroupBlockReadINTEL {
        ptr: spirv::Word,
    },
    SubgroupBlockWriteINTEL {
        ptr: spirv::Word,
        data: spirv::Word,
    },
    SubgroupImageBlockReadINTEL {
        image: spirv::Word,
        coordinate: spirv::Word,
    },
    SubgroupImageBlockWriteINTEL {
        image: spirv::Word,
        coordinate: spirv::Word,
        data: spirv::Word,
    },
    DecorateStringGOOGLE {
        target: spirv::Word,
        decoration: spirv::Decoration,
    },
    MemberDecorateStringGOOGLE {
        struct_type: spirv::Word,
=======
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
>>>>>>> Separate type structs
        member: u32,
        decoration: spirv::Decoration,
    },
    GroupNonUniformPartitionNV {
<<<<<<< HEAD
        value: spirv::Word,
=======
        value: Token<super::types::Type>,
>>>>>>> Separate type structs
    },
}
