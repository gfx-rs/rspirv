// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::sr::{module::Jump, storage::Token, Type};
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Branch {
    Phi {
        variable_parent: Vec<(spirv::Word, spirv::Word)>,
    },
    LoopMerge {
        merge_block: spirv::Word,
        continue_target: spirv::Word,
        loop_control: spirv::LoopControl,
    },
    SelectionMerge {
        merge_block: spirv::Word,
        selection_control: spirv::SelectionControl,
    },
    Label,
    Branch {
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
        target: Vec<(u32, Jump)>,
    },
    Kill,
    Return,
    ReturnValue {
        value: spirv::Word,
    },
    Unreachable,
    LifetimeStart {
        pointer: spirv::Word,
        size: u32,
    },
    LifetimeStop {
        pointer: spirv::Word,
        size: u32,
    },
    TerminateInvocation,
    DemoteToHelperInvocation,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Terminator {
    Branch(Branch),
    IgnoreIntersectionKHR,
    TerminateRayKHR,
    EmitMeshTasksEXT {
        group_count_x: spirv::Word,
        group_count_y: spirv::Word,
        group_count_z: spirv::Word,
        payload: Option<spirv::Word>,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Op {
    Nop,
    Undef,
    SourceContinued {
        continued_source: String,
    },
    Source {
        source_language: spirv::SourceLanguage,
        version: u32,
        file: Option<spirv::Word>,
        source: Option<String>,
    },
    SourceExtension {
        extension: String,
    },
    Name {
        target: spirv::Word,
        name: String,
    },
    MemberName {
        ty: Token<Type>,
        member: u32,
        name: String,
    },
    String {
        string: String,
    },
    Line {
        file: spirv::Word,
        line: u32,
        column: u32,
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
        memory_access_2: Option<spirv::MemoryAccess>,
    },
    CopyMemorySized {
        target: spirv::Word,
        source: spirv::Word,
        size: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        memory_access_2: Option<spirv::MemoryAccess>,
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
        structure_type: Token<Type>,
        member: u32,
        decoration: spirv::Decoration,
    },
    DecorationGroup,
    GroupDecorate {
        decoration_group: spirv::Word,
        targets: Vec<spirv::Word>,
    },
    GroupMemberDecorate {
        decoration_group: spirv::Word,
        targets: Vec<(Jump, u32)>,
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
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSampleExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: (spirv::ImageOperands, Vec<spirv::Word>),
    },
    ImageSampleDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSampleDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: (spirv::ImageOperands, Vec<spirv::Word>),
    },
    ImageSampleProjImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSampleProjExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: (spirv::ImageOperands, Vec<spirv::Word>),
    },
    ImageSampleProjDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSampleProjDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: (spirv::ImageOperands, Vec<spirv::Word>),
    },
    ImageFetch {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageDrefGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageRead {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageWrite {
        image: spirv::Word,
        coordinate: spirv::Word,
        texel: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
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
    },
    EmitVertex,
    EndPrimitive,
    EmitStreamVertex {
        stream: spirv::Word,
    },
    EndStreamPrimitive {
        stream: spirv::Word,
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
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicStore {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicExchange {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicCompareExchange {
        pointer: spirv::Word,
        memory: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    },
    AtomicCompareExchangeWeak {
        pointer: spirv::Word,
        memory: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    },
    AtomicIIncrement {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicIDecrement {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicIAdd {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicISub {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicSMin {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicUMin {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicSMax {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicUMax {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicAnd {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicOr {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicXor {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    GroupAsyncCopy {
        execution: spirv::Word,
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
    },
    GroupIAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupUMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupSMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupUMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupSMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
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
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSparseSampleExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: (spirv::ImageOperands, Vec<spirv::Word>),
    },
    ImageSparseSampleDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSparseSampleDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: (spirv::ImageOperands, Vec<spirv::Word>),
    },
    ImageSparseSampleProjImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSparseSampleProjExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: (spirv::ImageOperands, Vec<spirv::Word>),
    },
    ImageSparseSampleProjDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSparseSampleProjDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: (spirv::ImageOperands, Vec<spirv::Word>),
    },
    ImageSparseFetch {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSparseGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSparseDrefGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    ImageSparseTexelsResident {
        resident_code: spirv::Word,
    },
    NoLine,
    AtomicFlagTestAndSet {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicFlagClear {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    ImageSparseRead {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    SizeOf {
        pointer: spirv::Word,
    },
    ConstantPipeStorage {
        packet_size: u32,
        packet_alignment: u32,
        capacity: u32,
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
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    ModuleProcessed {
        process: String,
    },
    DecorateId {
        target: spirv::Word,
        decoration: spirv::Decoration,
    },
    GroupNonUniformElect {
        execution: spirv::Word,
    },
    GroupNonUniformAll {
        execution: spirv::Word,
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
    },
    GroupNonUniformBallotBitCount {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
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
    },
    GroupNonUniformIAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformFAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformIMul {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformFMul {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformSMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformUMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformFMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformSMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformUMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformFMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformBitwiseAnd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformBitwiseOr {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformBitwiseXor {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformLogicalAnd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformLogicalOr {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformLogicalXor {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
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
    CopyLogical {
        operand: spirv::Word,
    },
    PtrEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    PtrNotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    PtrDiff {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ColorAttachmentReadEXT {
        attachment: spirv::Word,
        sample: Option<spirv::Word>,
    },
    DepthAttachmentReadEXT {
        sample: Option<spirv::Word>,
    },
    StencilAttachmentReadEXT {
        sample: Option<spirv::Word>,
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
    GroupNonUniformRotateKHR {
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    SubgroupReadInvocationKHR {
        value: spirv::Word,
        index: spirv::Word,
    },
    TraceRayKHR {
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        payload: spirv::Word,
    },
    ExecuteCallableKHR {
        sbt_index: spirv::Word,
        callable_data: spirv::Word,
    },
    ConvertUToAccelerationStructureKHR {
        accel: spirv::Word,
    },
    SDot {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    },
    UDot {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    },
    SUDot {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    },
    SDotAccSat {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    },
    UDotAccSat {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    },
    SUDotAccSat {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    },
    CooperativeMatrixLoadKHR {
        pointer: spirv::Word,
        memory_layout: spirv::Word,
        stride: Option<spirv::Word>,
        memory_operand: Option<spirv::MemoryAccess>,
    },
    CooperativeMatrixStoreKHR {
        pointer: spirv::Word,
        object: spirv::Word,
        memory_layout: spirv::Word,
        stride: Option<spirv::Word>,
        memory_operand: Option<spirv::MemoryAccess>,
    },
    CooperativeMatrixMulAddKHR {
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
        cooperative_matrix_operands: Option<spirv::CooperativeMatrixOperands>,
    },
    CooperativeMatrixLengthKHR {
        ty: Token<Type>,
    },
    RayQueryInitializeKHR {
        ray_query: spirv::Word,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        ray_origin: spirv::Word,
        ray_t_min: spirv::Word,
        ray_direction: spirv::Word,
        ray_t_max: spirv::Word,
    },
    RayQueryTerminateKHR {
        ray_query: spirv::Word,
    },
    RayQueryGenerateIntersectionKHR {
        ray_query: spirv::Word,
        hit_t: spirv::Word,
    },
    RayQueryConfirmIntersectionKHR {
        ray_query: spirv::Word,
    },
    RayQueryProceedKHR {
        ray_query: spirv::Word,
    },
    RayQueryGetIntersectionTypeKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    ImageSampleWeightedQCOM {
        texture: spirv::Word,
        coordinates: spirv::Word,
        weights: spirv::Word,
    },
    ImageBoxFilterQCOM {
        texture: spirv::Word,
        coordinates: spirv::Word,
        box_size: spirv::Word,
    },
    ImageBlockMatchSSDQCOM {
        target: spirv::Word,
        target_coordinates: spirv::Word,
        reference: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    },
    ImageBlockMatchSADQCOM {
        target: spirv::Word,
        target_coordinates: spirv::Word,
        reference: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    },
    GroupIAddNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFAddNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupUMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupSMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupUMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupSMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
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
    ReadClockKHR {
        scope: spirv::Word,
    },
    FinalizeNodePayloadsAMDX {
        payload_array: spirv::Word,
    },
    FinishWritingNodePayloadAMDX {
        payload: spirv::Word,
    },
    InitializeNodePayloadsAMDX {
        payload_array: spirv::Word,
        visibility: spirv::Word,
        payload_count: spirv::Word,
        node_index: spirv::Word,
    },
    HitObjectRecordHitMotionNV {
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
        hit_object_attributes: spirv::Word,
    },
    HitObjectRecordHitWithIndexMotionNV {
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
        hit_object_attributes: spirv::Word,
    },
    HitObjectRecordMissMotionNV {
        hit_object: spirv::Word,
        sbt_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
    },
    HitObjectGetWorldToObjectNV {
        hit_object: spirv::Word,
    },
    HitObjectGetObjectToWorldNV {
        hit_object: spirv::Word,
    },
    HitObjectGetObjectRayDirectionNV {
        hit_object: spirv::Word,
    },
    HitObjectGetObjectRayOriginNV {
        hit_object: spirv::Word,
    },
    HitObjectTraceRayMotionNV {
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        ray_flags: spirv::Word,
        cullmask: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        miss_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        time: spirv::Word,
        payload: spirv::Word,
    },
    HitObjectGetShaderRecordBufferHandleNV {
        hit_object: spirv::Word,
    },
    HitObjectGetShaderBindingTableRecordIndexNV {
        hit_object: spirv::Word,
    },
    HitObjectRecordEmptyNV {
        hit_object: spirv::Word,
    },
    HitObjectTraceRayNV {
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        ray_flags: spirv::Word,
        cullmask: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        miss_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        payload: spirv::Word,
    },
    HitObjectRecordHitNV {
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        hit_object_attributes: spirv::Word,
    },
    HitObjectRecordHitWithIndexNV {
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        hit_object_attributes: spirv::Word,
    },
    HitObjectRecordMissNV {
        hit_object: spirv::Word,
        sbt_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
    },
    HitObjectExecuteShaderNV {
        hit_object: spirv::Word,
        payload: spirv::Word,
    },
    HitObjectGetCurrentTimeNV {
        hit_object: spirv::Word,
    },
    HitObjectGetAttributesNV {
        hit_object: spirv::Word,
        hit_object_attribute: spirv::Word,
    },
    HitObjectGetHitKindNV {
        hit_object: spirv::Word,
    },
    HitObjectGetPrimitiveIndexNV {
        hit_object: spirv::Word,
    },
    HitObjectGetGeometryIndexNV {
        hit_object: spirv::Word,
    },
    HitObjectGetInstanceIdNV {
        hit_object: spirv::Word,
    },
    HitObjectGetInstanceCustomIndexNV {
        hit_object: spirv::Word,
    },
    HitObjectGetWorldRayDirectionNV {
        hit_object: spirv::Word,
    },
    HitObjectGetWorldRayOriginNV {
        hit_object: spirv::Word,
    },
    HitObjectGetRayTMaxNV {
        hit_object: spirv::Word,
    },
    HitObjectGetRayTMinNV {
        hit_object: spirv::Word,
    },
    HitObjectIsEmptyNV {
        hit_object: spirv::Word,
    },
    HitObjectIsHitNV {
        hit_object: spirv::Word,
    },
    HitObjectIsMissNV {
        hit_object: spirv::Word,
    },
    ReorderThreadWithHitObjectNV {
        hit_object: spirv::Word,
        hint: Option<spirv::Word>,
        bits: Option<spirv::Word>,
    },
    ReorderThreadWithHintNV {
        hint: spirv::Word,
        bits: spirv::Word,
    },
    ImageSampleFootprintNV {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        granularity: spirv::Word,
        coarse: spirv::Word,
        image_operands: Option<(spirv::ImageOperands, Vec<spirv::Word>)>,
    },
    SetMeshOutputsEXT {
        vertex_count: spirv::Word,
        primitive_count: spirv::Word,
    },
    GroupNonUniformPartitionNV {
        value: spirv::Word,
    },
    WritePackedPrimitiveIndices4x8NV {
        index_offset: spirv::Word,
        packed_indices: spirv::Word,
    },
    FetchMicroTriangleVertexPositionNV {
        accel: spirv::Word,
        instance_id: spirv::Word,
        geometry_index: spirv::Word,
        primitive_index: spirv::Word,
        barycentric: spirv::Word,
    },
    FetchMicroTriangleVertexBarycentricNV {
        accel: spirv::Word,
        instance_id: spirv::Word,
        geometry_index: spirv::Word,
        primitive_index: spirv::Word,
        barycentric: spirv::Word,
    },
    ReportIntersectionKHR {
        hit: spirv::Word,
        hit_kind: spirv::Word,
    },
    IgnoreIntersectionNV,
    TerminateRayNV,
    TraceNV {
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        payload_id: spirv::Word,
    },
    TraceMotionNV {
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        time: spirv::Word,
        payload_id: spirv::Word,
    },
    TraceRayMotionNV {
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        time: spirv::Word,
        payload: spirv::Word,
    },
    RayQueryGetIntersectionTriangleVertexPositionsKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    ExecuteCallableNV {
        sbt_index: spirv::Word,
        callable_data_id: spirv::Word,
    },
    CooperativeMatrixLoadNV {
        pointer: spirv::Word,
        stride: spirv::Word,
        column_major: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CooperativeMatrixStoreNV {
        pointer: spirv::Word,
        object: spirv::Word,
        stride: spirv::Word,
        column_major: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CooperativeMatrixMulAddNV {
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    },
    CooperativeMatrixLengthNV {
        ty: Token<Type>,
    },
    BeginInvocationInterlockEXT,
    EndInvocationInterlockEXT,
    IsHelperInvocationEXT,
    ConvertUToImageNV {
        operand: spirv::Word,
    },
    ConvertUToSamplerNV {
        operand: spirv::Word,
    },
    ConvertImageToUNV {
        operand: spirv::Word,
    },
    ConvertSamplerToUNV {
        operand: spirv::Word,
    },
    ConvertUToSampledImageNV {
        operand: spirv::Word,
    },
    ConvertSampledImageToUNV {
        operand: spirv::Word,
    },
    SamplerImageAddressingModeNV {
        bit_width: u32,
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
    SubgroupImageMediaBlockReadINTEL {
        image: spirv::Word,
        coordinate: spirv::Word,
        width: spirv::Word,
        height: spirv::Word,
    },
    SubgroupImageMediaBlockWriteINTEL {
        image: spirv::Word,
        coordinate: spirv::Word,
        width: spirv::Word,
        height: spirv::Word,
        data: spirv::Word,
    },
    UCountLeadingZerosINTEL {
        operand: spirv::Word,
    },
    UCountTrailingZerosINTEL {
        operand: spirv::Word,
    },
    AbsISubINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    AbsUSubINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    IAddSatINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UAddSatINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    IAverageINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UAverageINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    IAverageRoundedINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UAverageRoundedINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ISubSatINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    USubSatINTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    IMul32x16INTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UMul32x16INTEL {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ConstantFunctionPointerINTEL {
        function: spirv::Word,
    },
    FunctionPointerCallINTEL {
        operand_1: Vec<spirv::Word>,
    },
    AsmTargetINTEL {
        asm_target: String,
    },
    AsmINTEL {
        asm_type: spirv::Word,
        target: spirv::Word,
        asm_instructions: String,
        constraints: String,
    },
    AsmCallINTEL {
        asm: spirv::Word,
        argument_0: Vec<spirv::Word>,
    },
    AtomicFMinEXT {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicFMaxEXT {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AssumeTrueKHR {
        condition: spirv::Word,
    },
    ExpectKHR {
        value: spirv::Word,
        expected_value: spirv::Word,
    },
    DecorateString {
        target: spirv::Word,
        decoration: spirv::Decoration,
    },
    MemberDecorateString {
        struct_type: Token<Type>,
        member: u32,
        decoration: spirv::Decoration,
    },
    VmeImageINTEL {
        image_type: Token<Type>,
        sampler: spirv::Word,
    },
    TypeVmeImageINTEL {
        image_type: Token<Type>,
    },
    TypeAvcImePayloadINTEL,
    TypeAvcRefPayloadINTEL,
    TypeAvcSicPayloadINTEL,
    TypeAvcMcePayloadINTEL,
    TypeAvcMceResultINTEL,
    TypeAvcImeResultINTEL,
    TypeAvcImeResultSingleReferenceStreamoutINTEL,
    TypeAvcImeResultDualReferenceStreamoutINTEL,
    TypeAvcImeSingleReferenceStreaminINTEL,
    TypeAvcImeDualReferenceStreaminINTEL,
    TypeAvcRefResultINTEL,
    TypeAvcSicResultINTEL,
    SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL {
        slice_type: Token<Type>,
        qp: spirv::Word,
    },
    SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL {
        reference_base_penalty: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcMceGetDefaultInterShapePenaltyINTEL {
        slice_type: Token<Type>,
        qp: spirv::Word,
    },
    SubgroupAvcMceSetInterShapePenaltyINTEL {
        packed_shape_penalty: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL {
        slice_type: Token<Type>,
        qp: spirv::Word,
    },
    SubgroupAvcMceSetInterDirectionPenaltyINTEL {
        direction_cost: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL {
        slice_type: Token<Type>,
        qp: spirv::Word,
    },
    SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL {
        slice_type: Token<Type>,
        qp: spirv::Word,
    },
    SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL,
    SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL,
    SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL,
    SubgroupAvcMceSetMotionVectorCostFunctionINTEL {
        packed_cost_center_delta: spirv::Word,
        packed_cost_table: spirv::Word,
        cost_precision: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL {
        slice_type: Token<Type>,
        qp: spirv::Word,
    },
    SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL,
    SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL,
    SubgroupAvcMceSetAcOnlyHaarINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL {
        source_field_polarity: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL {
        reference_field_polarity: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL {
        forward_reference_field_polarity: spirv::Word,
        backward_reference_field_polarity: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcMceConvertToImePayloadINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceConvertToImeResultINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceConvertToRefPayloadINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceConvertToRefResultINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceConvertToSicPayloadINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceConvertToSicResultINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetMotionVectorsINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetInterDistortionsINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetBestInterDistortionsINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetInterMajorShapeINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetInterMinorShapeINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetInterDirectionsINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetInterMotionVectorCountINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetInterReferenceIdsINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL {
        packed_reference_ids: spirv::Word,
        packed_reference_parameter_field_polarities: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeInitializeINTEL {
        src_coord: spirv::Word,
        partition_mask: spirv::Word,
        sad_adjustment: spirv::Word,
    },
    SubgroupAvcImeSetSingleReferenceINTEL {
        ref_offset: spirv::Word,
        search_window_config: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeSetDualReferenceINTEL {
        fwd_ref_offset: spirv::Word,
        bwd_ref_offset: spirv::Word,
        id_search_window_config: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeRefWindowSizeINTEL {
        search_window_config: spirv::Word,
        dual_ref: spirv::Word,
    },
    SubgroupAvcImeAdjustRefOffsetINTEL {
        ref_offset: spirv::Word,
        src_coord: spirv::Word,
        ref_window_size: spirv::Word,
        image_size: spirv::Word,
    },
    SubgroupAvcImeConvertToMcePayloadINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeSetMaxMotionVectorCountINTEL {
        max_motion_vector_count: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeSetUnidirectionalMixDisableINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL {
        threshold: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeSetWeightedSadINTEL {
        packed_sad_weights: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeEvaluateWithSingleReferenceINTEL {
        src_image: spirv::Word,
        ref_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeEvaluateWithDualReferenceINTEL {
        src_image: spirv::Word,
        fwd_ref_image: spirv::Word,
        bwd_ref_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL {
        src_image: spirv::Word,
        ref_image: spirv::Word,
        payload: spirv::Word,
        streamin_components: spirv::Word,
    },
    SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL {
        src_image: spirv::Word,
        fwd_ref_image: spirv::Word,
        bwd_ref_image: spirv::Word,
        payload: spirv::Word,
        streamin_components: spirv::Word,
    },
    SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL {
        src_image: spirv::Word,
        ref_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL {
        src_image: spirv::Word,
        fwd_ref_image: spirv::Word,
        bwd_ref_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL {
        src_image: spirv::Word,
        ref_image: spirv::Word,
        payload: spirv::Word,
        streamin_components: spirv::Word,
    },
    SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL {
        src_image: spirv::Word,
        fwd_ref_image: spirv::Word,
        bwd_ref_image: spirv::Word,
        payload: spirv::Word,
        streamin_components: spirv::Word,
    },
    SubgroupAvcImeConvertToMceResultINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeGetSingleReferenceStreaminINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeGetDualReferenceStreaminINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeStripSingleReferenceStreamoutINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeStripDualReferenceStreamoutINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL {
        payload: spirv::Word,
        major_shape: spirv::Word,
    },
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL {
        payload: spirv::Word,
        major_shape: spirv::Word,
    },
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL {
        payload: spirv::Word,
        major_shape: spirv::Word,
    },
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL {
        payload: spirv::Word,
        major_shape: spirv::Word,
        direction: spirv::Word,
    },
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL {
        payload: spirv::Word,
        major_shape: spirv::Word,
        direction: spirv::Word,
    },
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL {
        payload: spirv::Word,
        major_shape: spirv::Word,
        direction: spirv::Word,
    },
    SubgroupAvcImeGetBorderReachedINTEL {
        image_select: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcImeGetTruncatedSearchIndicationINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcFmeInitializeINTEL {
        src_coord: spirv::Word,
        motion_vectors: spirv::Word,
        major_shapes: spirv::Word,
        minor_shapes: spirv::Word,
        direction: spirv::Word,
        pixel_resolution: spirv::Word,
        sad_adjustment: spirv::Word,
    },
    SubgroupAvcBmeInitializeINTEL {
        src_coord: spirv::Word,
        motion_vectors: spirv::Word,
        major_shapes: spirv::Word,
        minor_shapes: spirv::Word,
        direction: spirv::Word,
        pixel_resolution: spirv::Word,
        bidirectional_weight: spirv::Word,
        sad_adjustment: spirv::Word,
    },
    SubgroupAvcRefConvertToMcePayloadINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcRefSetBidirectionalMixDisableINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcRefSetBilinearFilterEnableINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcRefEvaluateWithSingleReferenceINTEL {
        src_image: spirv::Word,
        ref_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcRefEvaluateWithDualReferenceINTEL {
        src_image: spirv::Word,
        fwd_ref_image: spirv::Word,
        bwd_ref_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcRefEvaluateWithMultiReferenceINTEL {
        src_image: spirv::Word,
        packed_reference_ids: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL {
        src_image: spirv::Word,
        packed_reference_ids: spirv::Word,
        packed_reference_field_polarities: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcRefConvertToMceResultINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicInitializeINTEL {
        src_coord: spirv::Word,
    },
    SubgroupAvcSicConfigureSkcINTEL {
        skip_block_partition_type: Token<Type>,
        skip_motion_vector_mask: spirv::Word,
        motion_vectors: spirv::Word,
        bidirectional_weight: spirv::Word,
        sad_adjustment: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicConfigureIpeLumaINTEL {
        luma_intra_partition_mask: spirv::Word,
        intra_neighbour_availabilty: spirv::Word,
        left_edge_luma_pixels: spirv::Word,
        upper_left_corner_luma_pixel: spirv::Word,
        upper_edge_luma_pixels: spirv::Word,
        upper_right_edge_luma_pixels: spirv::Word,
        sad_adjustment: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicConfigureIpeLumaChromaINTEL {
        luma_intra_partition_mask: spirv::Word,
        intra_neighbour_availabilty: spirv::Word,
        left_edge_luma_pixels: spirv::Word,
        upper_left_corner_luma_pixel: spirv::Word,
        upper_edge_luma_pixels: spirv::Word,
        upper_right_edge_luma_pixels: spirv::Word,
        left_edge_chroma_pixels: spirv::Word,
        upper_left_corner_chroma_pixel: spirv::Word,
        upper_edge_chroma_pixels: spirv::Word,
        sad_adjustment: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicGetMotionVectorMaskINTEL {
        skip_block_partition_type: Token<Type>,
        direction: spirv::Word,
    },
    SubgroupAvcSicConvertToMcePayloadINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicSetIntraLumaShapePenaltyINTEL {
        packed_shape_penalty: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL {
        luma_mode_penalty: spirv::Word,
        luma_packed_neighbor_modes: spirv::Word,
        luma_packed_non_dc_penalty: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL {
        chroma_mode_base_penalty: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicSetBilinearFilterEnableINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicSetSkcForwardTransformEnableINTEL {
        packed_sad_coefficients: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicSetBlockBasedRawSkipSadINTEL {
        block_based_skip_type: Token<Type>,
        payload: spirv::Word,
    },
    SubgroupAvcSicEvaluateIpeINTEL {
        src_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicEvaluateWithSingleReferenceINTEL {
        src_image: spirv::Word,
        ref_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicEvaluateWithDualReferenceINTEL {
        src_image: spirv::Word,
        fwd_ref_image: spirv::Word,
        bwd_ref_image: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicEvaluateWithMultiReferenceINTEL {
        src_image: spirv::Word,
        packed_reference_ids: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL {
        src_image: spirv::Word,
        packed_reference_ids: spirv::Word,
        packed_reference_field_polarities: spirv::Word,
        payload: spirv::Word,
    },
    SubgroupAvcSicConvertToMceResultINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicGetIpeLumaShapeINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicGetBestIpeLumaDistortionINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicGetBestIpeChromaDistortionINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicGetPackedIpeLumaModesINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicGetIpeChromaModeINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL {
        payload: spirv::Word,
    },
    SubgroupAvcSicGetInterRawSadsINTEL {
        payload: spirv::Word,
    },
    VariableLengthArrayINTEL {
        lenght: spirv::Word,
    },
    SaveMemoryINTEL,
    RestoreMemoryINTEL {
        ptr: spirv::Word,
    },
    ArbitraryFloatSinCosPiINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        from_sign: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatCastINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatCastFromIntINTEL {
        a: spirv::Word,
        mout: u32,
        from_sign: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatCastToIntINTEL {
        a: spirv::Word,
        m1: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatAddINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatSubINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatMulINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatDivINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatGTINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
    },
    ArbitraryFloatGEINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
    },
    ArbitraryFloatLTINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
    },
    ArbitraryFloatLEINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
    },
    ArbitraryFloatEQINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
    },
    ArbitraryFloatRecipINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatRSqrtINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatCbrtINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatHypotINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatSqrtINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatLogINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatLog2INTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatLog10INTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatLog1pINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatExpINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatExp2INTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatExp10INTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatExpm1INTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatSinINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatCosINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatSinCosINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatSinPiINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatCosPiINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatASinINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatASinPiINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatACosINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatACosPiINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatATanINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatATanPiINTEL {
        a: spirv::Word,
        m1: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatATan2INTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatPowINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatPowRINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        m2: u32,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    ArbitraryFloatPowNINTEL {
        a: spirv::Word,
        m1: u32,
        b: spirv::Word,
        mout: u32,
        enable_subnormals: u32,
        rounding_mode: u32,
        rounding_accuracy: u32,
    },
    LoopControlINTEL {
        loop_control_parameters: Vec<u32>,
    },
    AliasDomainDeclINTEL {
        name: Option<spirv::Word>,
    },
    AliasScopeDeclINTEL {
        alias_domain: spirv::Word,
        name: Option<spirv::Word>,
    },
    AliasScopeListDeclINTEL {
        alias_scope1_alias_scope2: Vec<spirv::Word>,
    },
    FixedSqrtINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedRecipINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedRsqrtINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedSinINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedCosINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedSinCosINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedSinPiINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedCosPiINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedSinCosPiINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedLogINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    FixedExpINTEL {
        input_type: Token<Type>,
        input: spirv::Word,
        s: u32,
        i: u32,
        r_i: u32,
        q: u32,
        o: u32,
    },
    PtrCastToCrossWorkgroupINTEL {
        pointer: spirv::Word,
    },
    CrossWorkgroupCastToPtrINTEL {
        pointer: spirv::Word,
    },
    ReadPipeBlockingINTEL {
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    WritePipeBlockingINTEL {
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    FPGARegINTEL {
        result: spirv::Word,
        input: spirv::Word,
    },
    RayQueryGetRayTMinKHR {
        ray_query: spirv::Word,
    },
    RayQueryGetRayFlagsKHR {
        ray_query: spirv::Word,
    },
    RayQueryGetIntersectionTKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionInstanceCustomIndexKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionInstanceIdKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionGeometryIndexKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionPrimitiveIndexKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionBarycentricsKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionFrontFaceKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionCandidateAABBOpaqueKHR {
        ray_query: spirv::Word,
    },
    RayQueryGetIntersectionObjectRayDirectionKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionObjectRayOriginKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetWorldRayDirectionKHR {
        ray_query: spirv::Word,
    },
    RayQueryGetWorldRayOriginKHR {
        ray_query: spirv::Word,
    },
    RayQueryGetIntersectionObjectToWorldKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    RayQueryGetIntersectionWorldToObjectKHR {
        ray_query: spirv::Word,
        intersection: spirv::Word,
    },
    AtomicFAddEXT {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    CompositeConstructContinuedINTEL {
        constituents: Vec<spirv::Word>,
    },
    ConvertFToBF16INTEL {
        float_value: spirv::Word,
    },
    ConvertBF16ToFINTEL {
        b_float16_value: spirv::Word,
    },
    ControlBarrierArriveINTEL {
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    ControlBarrierWaitINTEL {
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    GroupIMulKHR {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMulKHR {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupBitwiseAndKHR {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupBitwiseOrKHR {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupBitwiseXorKHR {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupLogicalAndKHR {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupLogicalOrKHR {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupLogicalXorKHR {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
}
