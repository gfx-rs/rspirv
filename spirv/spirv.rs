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

pub type Word = u32;
pub const MAGIC_NUMBER: u32 = 0x07230203;
pub const MAJOR_VERSION: u32 = 1;
pub const MINOR_VERSION: u32 = 1;
pub const REVISION: u32 = 8;

bitflags!{
    /// SPIR-V operand kind: [ImageOperands](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_image_operands_a_image_operands)
    pub struct ImageOperands : u32 {
        const NONE = 0x0000;
        const BIAS = 0x0001;
        const LOD = 0x0002;
        const GRAD = 0x0004;
        const CONST_OFFSET = 0x0008;
        const OFFSET = 0x0010;
        const CONST_OFFSETS = 0x0020;
        const SAMPLE = 0x0040;
        const MIN_LOD = 0x0080;
    }
}

bitflags!{
    /// SPIR-V operand kind: [FPFastMathMode](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_fp_fast_math_mode_a_fp_fast_math_mode)
    pub struct FPFastMathMode : u32 {
        const NONE = 0x0000;
        const NOT_NAN = 0x0001;
        const NOT_INF = 0x0002;
        const NSZ = 0x0004;
        const ALLOW_RECIP = 0x0008;
        const FAST = 0x0010;
    }
}

bitflags!{
    /// SPIR-V operand kind: [SelectionControl](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_selection_control_a_selection_control)
    pub struct SelectionControl : u32 {
        const NONE = 0x0000;
        const FLATTEN = 0x0001;
        const DONT_FLATTEN = 0x0002;
    }
}

bitflags!{
    /// SPIR-V operand kind: [LoopControl](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_loop_control_a_loop_control)
    pub struct LoopControl : u32 {
        const NONE = 0x0000;
        const UNROLL = 0x0001;
        const DONT_UNROLL = 0x0002;
        const DEPENDENCY_INFINITE = 0x0004;
        const DEPENDENCY_LENGTH = 0x0008;
    }
}

bitflags!{
    /// SPIR-V operand kind: [FunctionControl](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_function_control_a_function_control)
    pub struct FunctionControl : u32 {
        const NONE = 0x0000;
        const INLINE = 0x0001;
        const DONT_INLINE = 0x0002;
        const PURE = 0x0004;
        const CONST = 0x0008;
    }
}

bitflags!{
    /// SPIR-V operand kind: [MemorySemantics](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_memory_semantics_a_memory_semantics)
    pub struct MemorySemantics : u32 {
        const RELAXED = 0x0000;
        const NONE = 0x0000;
        const ACQUIRE = 0x0002;
        const RELEASE = 0x0004;
        const ACQUIRE_RELEASE = 0x0008;
        const SEQUENTIALLY_CONSISTENT = 0x0010;
        const UNIFORM_MEMORY = 0x0040;
        const SUBGROUP_MEMORY = 0x0080;
        const WORKGROUP_MEMORY = 0x0100;
        const CROSS_WORKGROUP_MEMORY = 0x0200;
        const ATOMIC_COUNTER_MEMORY = 0x0400;
        const IMAGE_MEMORY = 0x0800;
    }
}

bitflags!{
    /// SPIR-V operand kind: [MemoryAccess](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_memory_access_a_memory_access)
    pub struct MemoryAccess : u32 {
        const NONE = 0x0000;
        const VOLATILE = 0x0001;
        const ALIGNED = 0x0002;
        const NONTEMPORAL = 0x0004;
    }
}

bitflags!{
    /// SPIR-V operand kind: [KernelProfilingInfo](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_kernel_profiling_info_a_kernel_profiling_info)
    pub struct KernelProfilingInfo : u32 {
        const NONE = 0x0000;
        const CMD_EXEC_TIME = 0x0001;
    }
}

/// SPIR-V operand kind: [SourceLanguage](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_source_language_a_source_language)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum SourceLanguage {
    Unknown = 0,
    ESSL = 1,
    GLSL = 2,
    OpenCL_C = 3,
    OpenCL_CPP = 4,
    HLSL = 5,
}

/// SPIR-V operand kind: [ExecutionModel](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_execution_model_a_execution_model)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum ExecutionModel {
    Vertex = 0,
    TessellationControl = 1,
    TessellationEvaluation = 2,
    Geometry = 3,
    Fragment = 4,
    GLCompute = 5,
    Kernel = 6,
}

/// SPIR-V operand kind: [AddressingModel](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_addressing_model_a_addressing_model)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum AddressingModel {
    Logical = 0,
    Physical32 = 1,
    Physical64 = 2,
}

/// SPIR-V operand kind: [MemoryModel](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_memory_model_a_memory_model)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum MemoryModel {
    Simple = 0,
    GLSL450 = 1,
    OpenCL = 2,
}

/// SPIR-V operand kind: [ExecutionMode](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_execution_mode_a_execution_mode)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum ExecutionMode {
    Invocations = 0,
    SpacingEqual = 1,
    SpacingFractionalEven = 2,
    SpacingFractionalOdd = 3,
    VertexOrderCw = 4,
    VertexOrderCcw = 5,
    PixelCenterInteger = 6,
    OriginUpperLeft = 7,
    OriginLowerLeft = 8,
    EarlyFragmentTests = 9,
    PointMode = 10,
    Xfb = 11,
    DepthReplacing = 12,
    DepthGreater = 14,
    DepthLess = 15,
    DepthUnchanged = 16,
    LocalSize = 17,
    LocalSizeHint = 18,
    InputPoints = 19,
    InputLines = 20,
    InputLinesAdjacency = 21,
    Triangles = 22,
    InputTrianglesAdjacency = 23,
    Quads = 24,
    Isolines = 25,
    OutputVertices = 26,
    OutputPoints = 27,
    OutputLineStrip = 28,
    OutputTriangleStrip = 29,
    VecTypeHint = 30,
    ContractionOff = 31,
    Initializer = 33,
    Finalizer = 34,
    SubgroupSize = 35,
    SubgroupsPerWorkgroup = 36,
    PostDepthCoverage = 4446,
    StencilRefReplacingEXT = 5027,
}

/// SPIR-V operand kind: [StorageClass](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_storage_class_a_storage_class)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum StorageClass {
    UniformConstant = 0,
    Input = 1,
    Uniform = 2,
    Output = 3,
    Workgroup = 4,
    CrossWorkgroup = 5,
    Private = 6,
    Function = 7,
    Generic = 8,
    PushConstant = 9,
    AtomicCounter = 10,
    Image = 11,
    StorageBuffer = 12,
}

/// SPIR-V operand kind: [Dim](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_dim_a_dim)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum Dim {
    Dim1D = 0,
    Dim2D = 1,
    Dim3D = 2,
    DimCube = 3,
    DimRect = 4,
    DimBuffer = 5,
    DimSubpassData = 6,
}

/// SPIR-V operand kind: [SamplerAddressingMode](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_sampler_addressing_mode_a_sampler_addressing_mode)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum SamplerAddressingMode {
    None = 0,
    ClampToEdge = 1,
    Clamp = 2,
    Repeat = 3,
    RepeatMirrored = 4,
}

/// SPIR-V operand kind: [SamplerFilterMode](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_sampler_filter_mode_a_sampler_filter_mode)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum SamplerFilterMode {
    Nearest = 0,
    Linear = 1,
}

/// SPIR-V operand kind: [ImageFormat](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_image_format_a_image_format)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum ImageFormat {
    Unknown = 0,
    Rgba32f = 1,
    Rgba16f = 2,
    R32f = 3,
    Rgba8 = 4,
    Rgba8Snorm = 5,
    Rg32f = 6,
    Rg16f = 7,
    R11fG11fB10f = 8,
    R16f = 9,
    Rgba16 = 10,
    Rgb10A2 = 11,
    Rg16 = 12,
    Rg8 = 13,
    R16 = 14,
    R8 = 15,
    Rgba16Snorm = 16,
    Rg16Snorm = 17,
    Rg8Snorm = 18,
    R16Snorm = 19,
    R8Snorm = 20,
    Rgba32i = 21,
    Rgba16i = 22,
    Rgba8i = 23,
    R32i = 24,
    Rg32i = 25,
    Rg16i = 26,
    Rg8i = 27,
    R16i = 28,
    R8i = 29,
    Rgba32ui = 30,
    Rgba16ui = 31,
    Rgba8ui = 32,
    R32ui = 33,
    Rgb10a2ui = 34,
    Rg32ui = 35,
    Rg16ui = 36,
    Rg8ui = 37,
    R16ui = 38,
    R8ui = 39,
}

/// SPIR-V operand kind: [ImageChannelOrder](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_image_channel_order_a_image_channel_order)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum ImageChannelOrder {
    R = 0,
    A = 1,
    RG = 2,
    RA = 3,
    RGB = 4,
    RGBA = 5,
    BGRA = 6,
    ARGB = 7,
    Intensity = 8,
    Luminance = 9,
    Rx = 10,
    RGx = 11,
    RGBx = 12,
    Depth = 13,
    DepthStencil = 14,
    sRGB = 15,
    sRGBx = 16,
    sRGBA = 17,
    sBGRA = 18,
    ABGR = 19,
}

/// SPIR-V operand kind: [ImageChannelDataType](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_image_channel_data_type_a_image_channel_data_type)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum ImageChannelDataType {
    SnormInt8 = 0,
    SnormInt16 = 1,
    UnormInt8 = 2,
    UnormInt16 = 3,
    UnormShort565 = 4,
    UnormShort555 = 5,
    UnormInt101010 = 6,
    SignedInt8 = 7,
    SignedInt16 = 8,
    SignedInt32 = 9,
    UnsignedInt8 = 10,
    UnsignedInt16 = 11,
    UnsignedInt32 = 12,
    HalfFloat = 13,
    Float = 14,
    UnormInt24 = 15,
    UnormInt101010_2 = 16,
}

/// SPIR-V operand kind: [FPRoundingMode](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_fp_rounding_mode_a_fp_rounding_mode)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum FPRoundingMode {
    RTE = 0,
    RTZ = 1,
    RTP = 2,
    RTN = 3,
}

/// SPIR-V operand kind: [LinkageType](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_linkage_type_a_linkage_type)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum LinkageType {
    Export = 0,
    Import = 1,
}

/// SPIR-V operand kind: [AccessQualifier](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_access_qualifier_a_access_qualifier)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum AccessQualifier {
    ReadOnly = 0,
    WriteOnly = 1,
    ReadWrite = 2,
}

/// SPIR-V operand kind: [FunctionParameterAttribute](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_function_parameter_attribute_a_function_parameter_attribute)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum FunctionParameterAttribute {
    Zext = 0,
    Sext = 1,
    ByVal = 2,
    Sret = 3,
    NoAlias = 4,
    NoCapture = 5,
    NoWrite = 6,
    NoReadWrite = 7,
}

/// SPIR-V operand kind: [Decoration](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_decoration_a_decoration)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum Decoration {
    RelaxedPrecision = 0,
    SpecId = 1,
    Block = 2,
    BufferBlock = 3,
    RowMajor = 4,
    ColMajor = 5,
    ArrayStride = 6,
    MatrixStride = 7,
    GLSLShared = 8,
    GLSLPacked = 9,
    CPacked = 10,
    BuiltIn = 11,
    NoPerspective = 13,
    Flat = 14,
    Patch = 15,
    Centroid = 16,
    Sample = 17,
    Invariant = 18,
    Restrict = 19,
    Aliased = 20,
    Volatile = 21,
    Constant = 22,
    Coherent = 23,
    NonWritable = 24,
    NonReadable = 25,
    Uniform = 26,
    SaturatedConversion = 28,
    Stream = 29,
    Location = 30,
    Component = 31,
    Index = 32,
    Binding = 33,
    DescriptorSet = 34,
    Offset = 35,
    XfbBuffer = 36,
    XfbStride = 37,
    FuncParamAttr = 38,
    FPRoundingMode = 39,
    FPFastMathMode = 40,
    LinkageAttributes = 41,
    NoContraction = 42,
    InputAttachmentIndex = 43,
    Alignment = 44,
    MaxByteOffset = 45,
    ExplicitInterpAMD = 4999,
    OverrideCoverageNV = 5248,
    PassthroughNV = 5250,
    ViewportRelativeNV = 5252,
    SecondaryViewportRelativeNV = 5256,
}

/// SPIR-V operand kind: [BuiltIn](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_built_in_a_built_in)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum BuiltIn {
    Position = 0,
    PointSize = 1,
    ClipDistance = 3,
    CullDistance = 4,
    VertexId = 5,
    InstanceId = 6,
    PrimitiveId = 7,
    InvocationId = 8,
    Layer = 9,
    ViewportIndex = 10,
    TessLevelOuter = 11,
    TessLevelInner = 12,
    TessCoord = 13,
    PatchVertices = 14,
    FragCoord = 15,
    PointCoord = 16,
    FrontFacing = 17,
    SampleId = 18,
    SamplePosition = 19,
    SampleMask = 20,
    FragDepth = 22,
    HelperInvocation = 23,
    NumWorkgroups = 24,
    WorkgroupSize = 25,
    WorkgroupId = 26,
    LocalInvocationId = 27,
    GlobalInvocationId = 28,
    LocalInvocationIndex = 29,
    WorkDim = 30,
    GlobalSize = 31,
    EnqueuedWorkgroupSize = 32,
    GlobalOffset = 33,
    GlobalLinearId = 34,
    SubgroupSize = 36,
    SubgroupMaxSize = 37,
    NumSubgroups = 38,
    NumEnqueuedSubgroups = 39,
    SubgroupId = 40,
    SubgroupLocalInvocationId = 41,
    VertexIndex = 42,
    InstanceIndex = 43,
    SubgroupEqMaskKHR = 4416,
    SubgroupGeMaskKHR = 4417,
    SubgroupGtMaskKHR = 4418,
    SubgroupLeMaskKHR = 4419,
    SubgroupLtMaskKHR = 4420,
    BaseVertex = 4424,
    BaseInstance = 4425,
    DrawIndex = 4426,
    DeviceIndex = 4438,
    ViewIndex = 4440,
    BaryCoordNoPerspAMD = 4992,
    BaryCoordNoPerspCentroidAMD = 4993,
    BaryCoordNoPerspSampleAMD = 4994,
    BaryCoordSmoothAMD = 4995,
    BaryCoordSmoothCentroidAMD = 4996,
    BaryCoordSmoothSampleAMD = 4997,
    BaryCoordPullModelAMD = 4998,
    FragStencilRefEXT = 5014,
    ViewportMaskNV = 5253,
    SecondaryPositionNV = 5257,
    SecondaryViewportMaskNV = 5258,
    PositionPerViewNV = 5261,
    ViewportMaskPerViewNV = 5262,
}

/// SPIR-V operand kind: [Scope](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_scope_a_scope)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum Scope {
    CrossDevice = 0,
    Device = 1,
    Workgroup = 2,
    Subgroup = 3,
    Invocation = 4,
}

/// SPIR-V operand kind: [GroupOperation](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_group_operation_a_group_operation)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum GroupOperation {
    Reduce = 0,
    InclusiveScan = 1,
    ExclusiveScan = 2,
}

/// SPIR-V operand kind: [KernelEnqueueFlags](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_kernel_enqueue_flags_a_kernel_enqueue_flags)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum KernelEnqueueFlags {
    NoWait = 0,
    WaitKernel = 1,
    WaitWorkGroup = 2,
}

/// SPIR-V operand kind: [Capability](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_capability_a_capability)
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum Capability {
    Matrix = 0,
    Shader = 1,
    Geometry = 2,
    Tessellation = 3,
    Addresses = 4,
    Linkage = 5,
    Kernel = 6,
    Vector16 = 7,
    Float16Buffer = 8,
    Float16 = 9,
    Float64 = 10,
    Int64 = 11,
    Int64Atomics = 12,
    ImageBasic = 13,
    ImageReadWrite = 14,
    ImageMipmap = 15,
    Pipes = 17,
    Groups = 18,
    DeviceEnqueue = 19,
    LiteralSampler = 20,
    AtomicStorage = 21,
    Int16 = 22,
    TessellationPointSize = 23,
    GeometryPointSize = 24,
    ImageGatherExtended = 25,
    StorageImageMultisample = 27,
    UniformBufferArrayDynamicIndexing = 28,
    SampledImageArrayDynamicIndexing = 29,
    StorageBufferArrayDynamicIndexing = 30,
    StorageImageArrayDynamicIndexing = 31,
    ClipDistance = 32,
    CullDistance = 33,
    ImageCubeArray = 34,
    SampleRateShading = 35,
    ImageRect = 36,
    SampledRect = 37,
    GenericPointer = 38,
    Int8 = 39,
    InputAttachment = 40,
    SparseResidency = 41,
    MinLod = 42,
    Sampled1D = 43,
    Image1D = 44,
    SampledCubeArray = 45,
    SampledBuffer = 46,
    ImageBuffer = 47,
    ImageMSArray = 48,
    StorageImageExtendedFormats = 49,
    ImageQuery = 50,
    DerivativeControl = 51,
    InterpolationFunction = 52,
    TransformFeedback = 53,
    GeometryStreams = 54,
    StorageImageReadWithoutFormat = 55,
    StorageImageWriteWithoutFormat = 56,
    MultiViewport = 57,
    SubgroupDispatch = 58,
    NamedBarrier = 59,
    PipeStorage = 60,
    SubgroupBallotKHR = 4423,
    DrawParameters = 4427,
    SubgroupVoteKHR = 4431,
    StorageBuffer16BitAccess = 4433,
    UniformAndStorageBuffer16BitAccess = 4434,
    StoragePushConstant16 = 4435,
    StorageInputOutput16 = 4436,
    DeviceGroup = 4437,
    MultiView = 4439,
    VariablePointersStorageBuffer = 4441,
    VariablePointers = 4442,
    AtomicStorageOps = 4445,
    SampleMaskPostDepthCoverage = 4447,
    ImageGatherBiasLodAMD = 5009,
    FragmentMaskAMD = 5010,
    StencilExportEXT = 5013,
    ImageReadWriteLodAMD = 5015,
    SampleMaskOverrideCoverageNV = 5249,
    GeometryShaderPassthroughNV = 5251,
    ShaderViewportIndexLayerEXT = 5254,
    ShaderViewportMaskNV = 5255,
    ShaderStereoViewNV = 5259,
    PerViewAttributesNV = 5260,
}

#[allow(non_upper_case_globals)]
impl Capability {
    pub const StorageUniformBufferBlock16: Capability = Capability::StorageBuffer16BitAccess;
    pub const StorageUniform16: Capability = Capability::UniformAndStorageBuffer16BitAccess;
    pub const ShaderViewportIndexLayerNV: Capability = Capability::ShaderViewportIndexLayerEXT;
}
/// SPIR-V [instructions](https://www.khronos.org/registry/spir-v/specs/1.1/SPIRV.html#_a_id_instructions_a_instructions) opcodes
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum Op {
    Nop = 0,
    Undef = 1,
    SourceContinued = 2,
    Source = 3,
    SourceExtension = 4,
    Name = 5,
    MemberName = 6,
    String = 7,
    Line = 8,
    Extension = 10,
    ExtInstImport = 11,
    ExtInst = 12,
    MemoryModel = 14,
    EntryPoint = 15,
    ExecutionMode = 16,
    Capability = 17,
    TypeVoid = 19,
    TypeBool = 20,
    TypeInt = 21,
    TypeFloat = 22,
    TypeVector = 23,
    TypeMatrix = 24,
    TypeImage = 25,
    TypeSampler = 26,
    TypeSampledImage = 27,
    TypeArray = 28,
    TypeRuntimeArray = 29,
    TypeStruct = 30,
    TypeOpaque = 31,
    TypePointer = 32,
    TypeFunction = 33,
    TypeEvent = 34,
    TypeDeviceEvent = 35,
    TypeReserveId = 36,
    TypeQueue = 37,
    TypePipe = 38,
    TypeForwardPointer = 39,
    ConstantTrue = 41,
    ConstantFalse = 42,
    Constant = 43,
    ConstantComposite = 44,
    ConstantSampler = 45,
    ConstantNull = 46,
    SpecConstantTrue = 48,
    SpecConstantFalse = 49,
    SpecConstant = 50,
    SpecConstantComposite = 51,
    SpecConstantOp = 52,
    Function = 54,
    FunctionParameter = 55,
    FunctionEnd = 56,
    FunctionCall = 57,
    Variable = 59,
    ImageTexelPointer = 60,
    Load = 61,
    Store = 62,
    CopyMemory = 63,
    CopyMemorySized = 64,
    AccessChain = 65,
    InBoundsAccessChain = 66,
    PtrAccessChain = 67,
    ArrayLength = 68,
    GenericPtrMemSemantics = 69,
    InBoundsPtrAccessChain = 70,
    Decorate = 71,
    MemberDecorate = 72,
    DecorationGroup = 73,
    GroupDecorate = 74,
    GroupMemberDecorate = 75,
    VectorExtractDynamic = 77,
    VectorInsertDynamic = 78,
    VectorShuffle = 79,
    CompositeConstruct = 80,
    CompositeExtract = 81,
    CompositeInsert = 82,
    CopyObject = 83,
    Transpose = 84,
    SampledImage = 86,
    ImageSampleImplicitLod = 87,
    ImageSampleExplicitLod = 88,
    ImageSampleDrefImplicitLod = 89,
    ImageSampleDrefExplicitLod = 90,
    ImageSampleProjImplicitLod = 91,
    ImageSampleProjExplicitLod = 92,
    ImageSampleProjDrefImplicitLod = 93,
    ImageSampleProjDrefExplicitLod = 94,
    ImageFetch = 95,
    ImageGather = 96,
    ImageDrefGather = 97,
    ImageRead = 98,
    ImageWrite = 99,
    Image = 100,
    ImageQueryFormat = 101,
    ImageQueryOrder = 102,
    ImageQuerySizeLod = 103,
    ImageQuerySize = 104,
    ImageQueryLod = 105,
    ImageQueryLevels = 106,
    ImageQuerySamples = 107,
    ConvertFToU = 109,
    ConvertFToS = 110,
    ConvertSToF = 111,
    ConvertUToF = 112,
    UConvert = 113,
    SConvert = 114,
    FConvert = 115,
    QuantizeToF16 = 116,
    ConvertPtrToU = 117,
    SatConvertSToU = 118,
    SatConvertUToS = 119,
    ConvertUToPtr = 120,
    PtrCastToGeneric = 121,
    GenericCastToPtr = 122,
    GenericCastToPtrExplicit = 123,
    Bitcast = 124,
    SNegate = 126,
    FNegate = 127,
    IAdd = 128,
    FAdd = 129,
    ISub = 130,
    FSub = 131,
    IMul = 132,
    FMul = 133,
    UDiv = 134,
    SDiv = 135,
    FDiv = 136,
    UMod = 137,
    SRem = 138,
    SMod = 139,
    FRem = 140,
    FMod = 141,
    VectorTimesScalar = 142,
    MatrixTimesScalar = 143,
    VectorTimesMatrix = 144,
    MatrixTimesVector = 145,
    MatrixTimesMatrix = 146,
    OuterProduct = 147,
    Dot = 148,
    IAddCarry = 149,
    ISubBorrow = 150,
    UMulExtended = 151,
    SMulExtended = 152,
    Any = 154,
    All = 155,
    IsNan = 156,
    IsInf = 157,
    IsFinite = 158,
    IsNormal = 159,
    SignBitSet = 160,
    LessOrGreater = 161,
    Ordered = 162,
    Unordered = 163,
    LogicalEqual = 164,
    LogicalNotEqual = 165,
    LogicalOr = 166,
    LogicalAnd = 167,
    LogicalNot = 168,
    Select = 169,
    IEqual = 170,
    INotEqual = 171,
    UGreaterThan = 172,
    SGreaterThan = 173,
    UGreaterThanEqual = 174,
    SGreaterThanEqual = 175,
    ULessThan = 176,
    SLessThan = 177,
    ULessThanEqual = 178,
    SLessThanEqual = 179,
    FOrdEqual = 180,
    FUnordEqual = 181,
    FOrdNotEqual = 182,
    FUnordNotEqual = 183,
    FOrdLessThan = 184,
    FUnordLessThan = 185,
    FOrdGreaterThan = 186,
    FUnordGreaterThan = 187,
    FOrdLessThanEqual = 188,
    FUnordLessThanEqual = 189,
    FOrdGreaterThanEqual = 190,
    FUnordGreaterThanEqual = 191,
    ShiftRightLogical = 194,
    ShiftRightArithmetic = 195,
    ShiftLeftLogical = 196,
    BitwiseOr = 197,
    BitwiseXor = 198,
    BitwiseAnd = 199,
    Not = 200,
    BitFieldInsert = 201,
    BitFieldSExtract = 202,
    BitFieldUExtract = 203,
    BitReverse = 204,
    BitCount = 205,
    DPdx = 207,
    DPdy = 208,
    Fwidth = 209,
    DPdxFine = 210,
    DPdyFine = 211,
    FwidthFine = 212,
    DPdxCoarse = 213,
    DPdyCoarse = 214,
    FwidthCoarse = 215,
    EmitVertex = 218,
    EndPrimitive = 219,
    EmitStreamVertex = 220,
    EndStreamPrimitive = 221,
    ControlBarrier = 224,
    MemoryBarrier = 225,
    AtomicLoad = 227,
    AtomicStore = 228,
    AtomicExchange = 229,
    AtomicCompareExchange = 230,
    AtomicCompareExchangeWeak = 231,
    AtomicIIncrement = 232,
    AtomicIDecrement = 233,
    AtomicIAdd = 234,
    AtomicISub = 235,
    AtomicSMin = 236,
    AtomicUMin = 237,
    AtomicSMax = 238,
    AtomicUMax = 239,
    AtomicAnd = 240,
    AtomicOr = 241,
    AtomicXor = 242,
    Phi = 245,
    LoopMerge = 246,
    SelectionMerge = 247,
    Label = 248,
    Branch = 249,
    BranchConditional = 250,
    Switch = 251,
    Kill = 252,
    Return = 253,
    ReturnValue = 254,
    Unreachable = 255,
    LifetimeStart = 256,
    LifetimeStop = 257,
    GroupAsyncCopy = 259,
    GroupWaitEvents = 260,
    GroupAll = 261,
    GroupAny = 262,
    GroupBroadcast = 263,
    GroupIAdd = 264,
    GroupFAdd = 265,
    GroupFMin = 266,
    GroupUMin = 267,
    GroupSMin = 268,
    GroupFMax = 269,
    GroupUMax = 270,
    GroupSMax = 271,
    ReadPipe = 274,
    WritePipe = 275,
    ReservedReadPipe = 276,
    ReservedWritePipe = 277,
    ReserveReadPipePackets = 278,
    ReserveWritePipePackets = 279,
    CommitReadPipe = 280,
    CommitWritePipe = 281,
    IsValidReserveId = 282,
    GetNumPipePackets = 283,
    GetMaxPipePackets = 284,
    GroupReserveReadPipePackets = 285,
    GroupReserveWritePipePackets = 286,
    GroupCommitReadPipe = 287,
    GroupCommitWritePipe = 288,
    EnqueueMarker = 291,
    EnqueueKernel = 292,
    GetKernelNDrangeSubGroupCount = 293,
    GetKernelNDrangeMaxSubGroupSize = 294,
    GetKernelWorkGroupSize = 295,
    GetKernelPreferredWorkGroupSizeMultiple = 296,
    RetainEvent = 297,
    ReleaseEvent = 298,
    CreateUserEvent = 299,
    IsValidEvent = 300,
    SetUserEventStatus = 301,
    CaptureEventProfilingInfo = 302,
    GetDefaultQueue = 303,
    BuildNDRange = 304,
    ImageSparseSampleImplicitLod = 305,
    ImageSparseSampleExplicitLod = 306,
    ImageSparseSampleDrefImplicitLod = 307,
    ImageSparseSampleDrefExplicitLod = 308,
    ImageSparseSampleProjImplicitLod = 309,
    ImageSparseSampleProjExplicitLod = 310,
    ImageSparseSampleProjDrefImplicitLod = 311,
    ImageSparseSampleProjDrefExplicitLod = 312,
    ImageSparseFetch = 313,
    ImageSparseGather = 314,
    ImageSparseDrefGather = 315,
    ImageSparseTexelsResident = 316,
    NoLine = 317,
    AtomicFlagTestAndSet = 318,
    AtomicFlagClear = 319,
    ImageSparseRead = 320,
    SizeOf = 321,
    TypePipeStorage = 322,
    ConstantPipeStorage = 323,
    CreatePipeFromPipeStorage = 324,
    GetKernelLocalSizeForSubgroupCount = 325,
    GetKernelMaxNumSubgroups = 326,
    TypeNamedBarrier = 327,
    NamedBarrierInitialize = 328,
    MemoryNamedBarrier = 329,
    ModuleProcessed = 330,
    SubgroupBallotKHR = 4421,
    SubgroupFirstInvocationKHR = 4422,
    SubgroupAllKHR = 4428,
    SubgroupAnyKHR = 4429,
    SubgroupAllEqualKHR = 4430,
    SubgroupReadInvocationKHR = 4432,
    GroupIAddNonUniformAMD = 5000,
    GroupFAddNonUniformAMD = 5001,
    GroupFMinNonUniformAMD = 5002,
    GroupUMinNonUniformAMD = 5003,
    GroupSMinNonUniformAMD = 5004,
    GroupFMaxNonUniformAMD = 5005,
    GroupUMaxNonUniformAMD = 5006,
    GroupSMaxNonUniformAMD = 5007,
    FragmentMaskFetchAMD = 5011,
    FragmentFetchAMD = 5012,
}

/// [GLSL.std.450](https://www.khronos.org/registry/spir-v/specs/1.0/GLSL.std.450.html) extended instruction opcode
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum GLOp {
    Round = 1,
    RoundEven = 2,
    Trunc = 3,
    FAbs = 4,
    SAbs = 5,
    FSign = 6,
    SSign = 7,
    Floor = 8,
    Ceil = 9,
    Fract = 10,
    Radians = 11,
    Degrees = 12,
    Sin = 13,
    Cos = 14,
    Tan = 15,
    Asin = 16,
    Acos = 17,
    Atan = 18,
    Sinh = 19,
    Cosh = 20,
    Tanh = 21,
    Asinh = 22,
    Acosh = 23,
    Atanh = 24,
    Atan2 = 25,
    Pow = 26,
    Exp = 27,
    Log = 28,
    Exp2 = 29,
    Log2 = 30,
    Sqrt = 31,
    InverseSqrt = 32,
    Determinant = 33,
    MatrixInverse = 34,
    Modf = 35,
    ModfStruct = 36,
    FMin = 37,
    UMin = 38,
    SMin = 39,
    FMax = 40,
    UMax = 41,
    SMax = 42,
    FClamp = 43,
    UClamp = 44,
    SClamp = 45,
    FMix = 46,
    IMix = 47,
    Step = 48,
    SmoothStep = 49,
    Fma = 50,
    Frexp = 51,
    FrexpStruct = 52,
    Ldexp = 53,
    PackSnorm4x8 = 54,
    PackUnorm4x8 = 55,
    PackSnorm2x16 = 56,
    PackUnorm2x16 = 57,
    PackHalf2x16 = 58,
    PackDouble2x32 = 59,
    UnpackSnorm2x16 = 60,
    UnpackUnorm2x16 = 61,
    UnpackHalf2x16 = 62,
    UnpackSnorm4x8 = 63,
    UnpackUnorm4x8 = 64,
    UnpackDouble2x32 = 65,
    Length = 66,
    Distance = 67,
    Cross = 68,
    Normalize = 69,
    FaceForward = 70,
    Reflect = 71,
    Refract = 72,
    FindILsb = 73,
    FindSMsb = 74,
    FindUMsb = 75,
    InterpolateAtCentroid = 76,
    InterpolateAtSample = 77,
    InterpolateAtOffset = 78,
    NMin = 79,
    NMax = 80,
    NClamp = 81,
}

/// [OpenCL.std](https://www.khronos.org/registry/spir-v/specs/1.2/OpenCL.ExtendedInstructionSet.100.html) extended instruction opcode
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, Hash)]
pub enum CLOp {
    acos = 0,
    acosh = 1,
    acospi = 2,
    asin = 3,
    asinh = 4,
    asinpi = 5,
    atan = 6,
    atan2 = 7,
    atanh = 8,
    atanpi = 9,
    atan2pi = 10,
    cbrt = 11,
    ceil = 12,
    copysign = 13,
    cos = 14,
    cosh = 15,
    cospi = 16,
    erfc = 17,
    erf = 18,
    exp = 19,
    exp2 = 20,
    exp10 = 21,
    expm1 = 22,
    fabs = 23,
    fdim = 24,
    floor = 25,
    fma = 26,
    fmax = 27,
    fmin = 28,
    fmod = 29,
    fract = 30,
    frexp = 31,
    hypot = 32,
    ilogb = 33,
    ldexp = 34,
    lgamma = 35,
    lgamma_r = 36,
    log = 37,
    log2 = 38,
    log10 = 39,
    log1p = 40,
    logb = 41,
    mad = 42,
    maxmag = 43,
    minmag = 44,
    modf = 45,
    nan = 46,
    nextafter = 47,
    pow = 48,
    pown = 49,
    powr = 50,
    remainder = 51,
    remquo = 52,
    rint = 53,
    rootn = 54,
    round = 55,
    rsqrt = 56,
    sin = 57,
    sincos = 58,
    sinh = 59,
    sinpi = 60,
    sqrt = 61,
    tan = 62,
    tanh = 63,
    tanpi = 64,
    tgamma = 65,
    trunc = 66,
    half_cos = 67,
    half_divide = 68,
    half_exp = 69,
    half_exp2 = 70,
    half_exp10 = 71,
    half_log = 72,
    half_log2 = 73,
    half_log10 = 74,
    half_powr = 75,
    half_recip = 76,
    half_rsqrt = 77,
    half_sin = 78,
    half_sqrt = 79,
    half_tan = 80,
    native_cos = 81,
    native_divide = 82,
    native_exp = 83,
    native_exp2 = 84,
    native_exp10 = 85,
    native_log = 86,
    native_log2 = 87,
    native_log10 = 88,
    native_powr = 89,
    native_recip = 90,
    native_rsqrt = 91,
    native_sin = 92,
    native_sqrt = 93,
    native_tan = 94,
    s_abs = 141,
    s_abs_diff = 142,
    s_add_sat = 143,
    u_add_sat = 144,
    s_hadd = 145,
    u_hadd = 146,
    s_rhadd = 147,
    u_rhadd = 148,
    s_clamp = 149,
    u_clamp = 150,
    clz = 151,
    ctz = 152,
    s_mad_hi = 153,
    u_mad_sat = 154,
    s_mad_sat = 155,
    s_max = 156,
    u_max = 157,
    s_min = 158,
    u_min = 159,
    s_mul_hi = 160,
    rotate = 161,
    s_sub_sat = 162,
    u_sub_sat = 163,
    u_upsample = 164,
    s_upsample = 165,
    popcount = 166,
    s_mad24 = 167,
    u_mad24 = 168,
    s_mul24 = 169,
    u_mul24 = 170,
    u_abs = 201,
    u_abs_diff = 202,
    u_mul_hi = 203,
    u_mad_hi = 204,
    fclamp = 95,
    degrees = 96,
    fmax_common = 97,
    fmin_common = 98,
    mix = 99,
    radians = 100,
    step = 101,
    smoothstep = 102,
    sign = 103,
    cross = 104,
    distance = 105,
    length = 106,
    normalize = 107,
    fast_distance = 108,
    fast_length = 109,
    fast_normalize = 110,
    bitselect = 186,
    select = 187,
    vloadn = 171,
    vstoren = 172,
    vload_half = 173,
    vload_halfn = 174,
    vstore_half = 175,
    vstore_half_r = 176,
    vstore_halfn = 177,
    vstore_halfn_r = 178,
    vloada_halfn = 179,
    vstorea_halfn = 180,
    vstorea_halfn_r = 181,
    shuffle = 182,
    shuffle2 = 183,
    printf = 184,
    prefetch = 185,
}
