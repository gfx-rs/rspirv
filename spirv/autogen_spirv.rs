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
pub const MAJOR_VERSION: u8 = 1u8;
pub const MINOR_VERSION: u8 = 3u8;
pub const REVISION: u8 = 1u8;
bitflags! { # [ doc = "SPIR-V operand kind: [ImageOperands](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_image_operands_a_image_operands)" ] pub struct ImageOperands : u32 { const NONE = 0u32 ; const BIAS = 1u32 ; const LOD = 2u32 ; const GRAD = 4u32 ; const CONST_OFFSET = 8u32 ; const OFFSET = 16u32 ; const CONST_OFFSETS = 32u32 ; const SAMPLE = 64u32 ; const MIN_LOD = 128u32 ; const MAKE_TEXEL_AVAILABLE_KHR = 256u32 ; const MAKE_TEXEL_VISIBLE_KHR = 512u32 ; const NON_PRIVATE_TEXEL_KHR = 1024u32 ; const VOLATILE_TEXEL_KHR = 2048u32 ; } }
bitflags! { # [ doc = "SPIR-V operand kind: [FPFastMathMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_fp_fast_math_mode_a_fp_fast_math_mode)" ] pub struct FPFastMathMode : u32 { const NONE = 0u32 ; const NOT_NAN = 1u32 ; const NOT_INF = 2u32 ; const NSZ = 4u32 ; const ALLOW_RECIP = 8u32 ; const FAST = 16u32 ; } }
bitflags! { # [ doc = "SPIR-V operand kind: [SelectionControl](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_selection_control_a_selection_control)" ] pub struct SelectionControl : u32 { const NONE = 0u32 ; const FLATTEN = 1u32 ; const DONT_FLATTEN = 2u32 ; } }
bitflags! { # [ doc = "SPIR-V operand kind: [LoopControl](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_loop_control_a_loop_control)" ] pub struct LoopControl : u32 { const NONE = 0u32 ; const UNROLL = 1u32 ; const DONT_UNROLL = 2u32 ; const DEPENDENCY_INFINITE = 4u32 ; const DEPENDENCY_LENGTH = 8u32 ; } }
bitflags! { # [ doc = "SPIR-V operand kind: [FunctionControl](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_function_control_a_function_control)" ] pub struct FunctionControl : u32 { const NONE = 0u32 ; const INLINE = 1u32 ; const DONT_INLINE = 2u32 ; const PURE = 4u32 ; const CONST = 8u32 ; } }
bitflags! { # [ doc = "SPIR-V operand kind: [MemorySemantics](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_memory_semantics_a_memory_semantics)" ] pub struct MemorySemantics : u32 { const RELAXED = 0u32 ; const NONE = 0u32 ; const ACQUIRE = 2u32 ; const RELEASE = 4u32 ; const ACQUIRE_RELEASE = 8u32 ; const SEQUENTIALLY_CONSISTENT = 16u32 ; const UNIFORM_MEMORY = 64u32 ; const SUBGROUP_MEMORY = 128u32 ; const WORKGROUP_MEMORY = 256u32 ; const CROSS_WORKGROUP_MEMORY = 512u32 ; const ATOMIC_COUNTER_MEMORY = 1024u32 ; const IMAGE_MEMORY = 2048u32 ; const OUTPUT_MEMORY_KHR = 4096u32 ; const MAKE_AVAILABLE_KHR = 8192u32 ; const MAKE_VISIBLE_KHR = 16384u32 ; } }
bitflags! { # [ doc = "SPIR-V operand kind: [MemoryAccess](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_memory_access_a_memory_access)" ] pub struct MemoryAccess : u32 { const NONE = 0u32 ; const VOLATILE = 1u32 ; const ALIGNED = 2u32 ; const NONTEMPORAL = 4u32 ; const MAKE_POINTER_AVAILABLE_KHR = 8u32 ; const MAKE_POINTER_VISIBLE_KHR = 16u32 ; const NON_PRIVATE_POINTER_KHR = 32u32 ; } }
bitflags! { # [ doc = "SPIR-V operand kind: [KernelProfilingInfo](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_kernel_profiling_info_a_kernel_profiling_info)" ] pub struct KernelProfilingInfo : u32 { const NONE = 0u32 ; const CMD_EXEC_TIME = 1u32 ; } }
#[doc = "/// SPIR-V operand kind: [SourceLanguage](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_source_language_a_source_language)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceLanguage {
    Unknown = 0u32,
    ESSL = 1u32,
    GLSL = 2u32,
    OpenCL_C = 3u32,
    OpenCL_CPP = 4u32,
    HLSL = 5u32,
}
#[allow(non_upper_case_globals)]
impl SourceLanguage {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            SourceLanguage::Unknown
            | SourceLanguage::ESSL
            | SourceLanguage::GLSL
            | SourceLanguage::OpenCL_C
            | SourceLanguage::OpenCL_CPP
            | SourceLanguage::HLSL => &[],
        }
    }
}
impl num_traits::FromPrimitive for SourceLanguage {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(SourceLanguage::Unknown),
            1u32 => Some(SourceLanguage::ESSL),
            2u32 => Some(SourceLanguage::GLSL),
            3u32 => Some(SourceLanguage::OpenCL_C),
            4u32 => Some(SourceLanguage::OpenCL_CPP),
            5u32 => Some(SourceLanguage::HLSL),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [ExecutionModel](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_execution_model_a_execution_model)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExecutionModel {
    Vertex = 0u32,
    TessellationControl = 1u32,
    TessellationEvaluation = 2u32,
    Geometry = 3u32,
    Fragment = 4u32,
    GLCompute = 5u32,
    Kernel = 6u32,
}
#[allow(non_upper_case_globals)]
impl ExecutionModel {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            ExecutionModel::Geometry => &[Capability::Geometry],
            ExecutionModel::Kernel => &[Capability::Kernel],
            ExecutionModel::Vertex | ExecutionModel::Fragment | ExecutionModel::GLCompute => {
                &[Capability::Shader]
            }
            ExecutionModel::TessellationControl | ExecutionModel::TessellationEvaluation => {
                &[Capability::Tessellation]
            }
        }
    }
}
impl num_traits::FromPrimitive for ExecutionModel {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(ExecutionModel::Vertex),
            1u32 => Some(ExecutionModel::TessellationControl),
            2u32 => Some(ExecutionModel::TessellationEvaluation),
            3u32 => Some(ExecutionModel::Geometry),
            4u32 => Some(ExecutionModel::Fragment),
            5u32 => Some(ExecutionModel::GLCompute),
            6u32 => Some(ExecutionModel::Kernel),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [AddressingModel](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_addressing_model_a_addressing_model)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddressingModel {
    Logical = 0u32,
    Physical32 = 1u32,
    Physical64 = 2u32,
}
#[allow(non_upper_case_globals)]
impl AddressingModel {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            AddressingModel::Logical => &[],
            AddressingModel::Physical32 | AddressingModel::Physical64 => &[Capability::Addresses],
        }
    }
}
impl num_traits::FromPrimitive for AddressingModel {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(AddressingModel::Logical),
            1u32 => Some(AddressingModel::Physical32),
            2u32 => Some(AddressingModel::Physical64),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [MemoryModel](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_memory_model_a_memory_model)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryModel {
    Simple = 0u32,
    GLSL450 = 1u32,
    OpenCL = 2u32,
    VulkanKHR = 3u32,
}
#[allow(non_upper_case_globals)]
impl MemoryModel {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            MemoryModel::OpenCL => &[Capability::Kernel],
            MemoryModel::Simple | MemoryModel::GLSL450 => &[Capability::Shader],
            MemoryModel::VulkanKHR => &[Capability::VulkanMemoryModelKHR],
        }
    }
}
impl num_traits::FromPrimitive for MemoryModel {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(MemoryModel::Simple),
            1u32 => Some(MemoryModel::GLSL450),
            2u32 => Some(MemoryModel::OpenCL),
            3u32 => Some(MemoryModel::VulkanKHR),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [ExecutionMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_execution_mode_a_execution_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExecutionMode {
    Invocations = 0u32,
    SpacingEqual = 1u32,
    SpacingFractionalEven = 2u32,
    SpacingFractionalOdd = 3u32,
    VertexOrderCw = 4u32,
    VertexOrderCcw = 5u32,
    PixelCenterInteger = 6u32,
    OriginUpperLeft = 7u32,
    OriginLowerLeft = 8u32,
    EarlyFragmentTests = 9u32,
    PointMode = 10u32,
    Xfb = 11u32,
    DepthReplacing = 12u32,
    DepthGreater = 14u32,
    DepthLess = 15u32,
    DepthUnchanged = 16u32,
    LocalSize = 17u32,
    LocalSizeHint = 18u32,
    InputPoints = 19u32,
    InputLines = 20u32,
    InputLinesAdjacency = 21u32,
    Triangles = 22u32,
    InputTrianglesAdjacency = 23u32,
    Quads = 24u32,
    Isolines = 25u32,
    OutputVertices = 26u32,
    OutputPoints = 27u32,
    OutputLineStrip = 28u32,
    OutputTriangleStrip = 29u32,
    VecTypeHint = 30u32,
    ContractionOff = 31u32,
    Initializer = 33u32,
    Finalizer = 34u32,
    SubgroupSize = 35u32,
    SubgroupsPerWorkgroup = 36u32,
    SubgroupsPerWorkgroupId = 37u32,
    LocalSizeId = 38u32,
    LocalSizeHintId = 39u32,
    PostDepthCoverage = 4446u32,
    StencilRefReplacingEXT = 5027u32,
}
#[allow(non_upper_case_globals)]
impl ExecutionMode {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            ExecutionMode::LocalSize | ExecutionMode::LocalSizeId => &[],
            ExecutionMode::Invocations
            | ExecutionMode::InputPoints
            | ExecutionMode::InputLines
            | ExecutionMode::InputLinesAdjacency
            | ExecutionMode::InputTrianglesAdjacency
            | ExecutionMode::OutputPoints
            | ExecutionMode::OutputLineStrip
            | ExecutionMode::OutputTriangleStrip => &[Capability::Geometry],
            ExecutionMode::Triangles | ExecutionMode::OutputVertices => {
                &[Capability::Geometry, Capability::Tessellation]
            }
            ExecutionMode::LocalSizeHint
            | ExecutionMode::VecTypeHint
            | ExecutionMode::ContractionOff
            | ExecutionMode::Initializer
            | ExecutionMode::Finalizer
            | ExecutionMode::LocalSizeHintId => &[Capability::Kernel],
            ExecutionMode::PostDepthCoverage => &[Capability::SampleMaskPostDepthCoverage],
            ExecutionMode::PixelCenterInteger
            | ExecutionMode::OriginUpperLeft
            | ExecutionMode::OriginLowerLeft
            | ExecutionMode::EarlyFragmentTests
            | ExecutionMode::DepthReplacing
            | ExecutionMode::DepthGreater
            | ExecutionMode::DepthLess
            | ExecutionMode::DepthUnchanged => &[Capability::Shader],
            ExecutionMode::StencilRefReplacingEXT => &[Capability::StencilExportEXT],
            ExecutionMode::SubgroupSize
            | ExecutionMode::SubgroupsPerWorkgroup
            | ExecutionMode::SubgroupsPerWorkgroupId => &[Capability::SubgroupDispatch],
            ExecutionMode::SpacingEqual
            | ExecutionMode::SpacingFractionalEven
            | ExecutionMode::SpacingFractionalOdd
            | ExecutionMode::VertexOrderCw
            | ExecutionMode::VertexOrderCcw
            | ExecutionMode::PointMode
            | ExecutionMode::Quads
            | ExecutionMode::Isolines => &[Capability::Tessellation],
            ExecutionMode::Xfb => &[Capability::TransformFeedback],
        }
    }
}
impl num_traits::FromPrimitive for ExecutionMode {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(ExecutionMode::Invocations),
            1u32 => Some(ExecutionMode::SpacingEqual),
            2u32 => Some(ExecutionMode::SpacingFractionalEven),
            3u32 => Some(ExecutionMode::SpacingFractionalOdd),
            4u32 => Some(ExecutionMode::VertexOrderCw),
            5u32 => Some(ExecutionMode::VertexOrderCcw),
            6u32 => Some(ExecutionMode::PixelCenterInteger),
            7u32 => Some(ExecutionMode::OriginUpperLeft),
            8u32 => Some(ExecutionMode::OriginLowerLeft),
            9u32 => Some(ExecutionMode::EarlyFragmentTests),
            10u32 => Some(ExecutionMode::PointMode),
            11u32 => Some(ExecutionMode::Xfb),
            12u32 => Some(ExecutionMode::DepthReplacing),
            14u32 => Some(ExecutionMode::DepthGreater),
            15u32 => Some(ExecutionMode::DepthLess),
            16u32 => Some(ExecutionMode::DepthUnchanged),
            17u32 => Some(ExecutionMode::LocalSize),
            18u32 => Some(ExecutionMode::LocalSizeHint),
            19u32 => Some(ExecutionMode::InputPoints),
            20u32 => Some(ExecutionMode::InputLines),
            21u32 => Some(ExecutionMode::InputLinesAdjacency),
            22u32 => Some(ExecutionMode::Triangles),
            23u32 => Some(ExecutionMode::InputTrianglesAdjacency),
            24u32 => Some(ExecutionMode::Quads),
            25u32 => Some(ExecutionMode::Isolines),
            26u32 => Some(ExecutionMode::OutputVertices),
            27u32 => Some(ExecutionMode::OutputPoints),
            28u32 => Some(ExecutionMode::OutputLineStrip),
            29u32 => Some(ExecutionMode::OutputTriangleStrip),
            30u32 => Some(ExecutionMode::VecTypeHint),
            31u32 => Some(ExecutionMode::ContractionOff),
            33u32 => Some(ExecutionMode::Initializer),
            34u32 => Some(ExecutionMode::Finalizer),
            35u32 => Some(ExecutionMode::SubgroupSize),
            36u32 => Some(ExecutionMode::SubgroupsPerWorkgroup),
            37u32 => Some(ExecutionMode::SubgroupsPerWorkgroupId),
            38u32 => Some(ExecutionMode::LocalSizeId),
            39u32 => Some(ExecutionMode::LocalSizeHintId),
            4446u32 => Some(ExecutionMode::PostDepthCoverage),
            5027u32 => Some(ExecutionMode::StencilRefReplacingEXT),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [StorageClass](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_storage_class_a_storage_class)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StorageClass {
    UniformConstant = 0u32,
    Input = 1u32,
    Uniform = 2u32,
    Output = 3u32,
    Workgroup = 4u32,
    CrossWorkgroup = 5u32,
    Private = 6u32,
    Function = 7u32,
    Generic = 8u32,
    PushConstant = 9u32,
    AtomicCounter = 10u32,
    Image = 11u32,
    StorageBuffer = 12u32,
}
#[allow(non_upper_case_globals)]
impl StorageClass {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            StorageClass::UniformConstant
            | StorageClass::Input
            | StorageClass::Workgroup
            | StorageClass::CrossWorkgroup
            | StorageClass::Function
            | StorageClass::Image => &[],
            StorageClass::AtomicCounter => &[Capability::AtomicStorage],
            StorageClass::Generic => &[Capability::GenericPointer],
            StorageClass::Uniform
            | StorageClass::Output
            | StorageClass::Private
            | StorageClass::PushConstant
            | StorageClass::StorageBuffer => &[Capability::Shader],
        }
    }
}
impl num_traits::FromPrimitive for StorageClass {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(StorageClass::UniformConstant),
            1u32 => Some(StorageClass::Input),
            2u32 => Some(StorageClass::Uniform),
            3u32 => Some(StorageClass::Output),
            4u32 => Some(StorageClass::Workgroup),
            5u32 => Some(StorageClass::CrossWorkgroup),
            6u32 => Some(StorageClass::Private),
            7u32 => Some(StorageClass::Function),
            8u32 => Some(StorageClass::Generic),
            9u32 => Some(StorageClass::PushConstant),
            10u32 => Some(StorageClass::AtomicCounter),
            11u32 => Some(StorageClass::Image),
            12u32 => Some(StorageClass::StorageBuffer),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [Dim](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_dim_a_dim)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dim {
    Dim1D = 0u32,
    Dim2D = 1u32,
    Dim3D = 2u32,
    DimCube = 3u32,
    DimRect = 4u32,
    DimBuffer = 5u32,
    DimSubpassData = 6u32,
}
#[allow(non_upper_case_globals)]
impl Dim {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            Dim::Dim3D => &[],
            Dim::DimSubpassData => &[Capability::InputAttachment],
            Dim::Dim1D => &[Capability::Sampled1D, Capability::Image1D],
            Dim::DimBuffer => &[Capability::SampledBuffer, Capability::ImageBuffer],
            Dim::DimRect => &[Capability::SampledRect, Capability::ImageRect],
            Dim::DimCube => &[Capability::Shader, Capability::ImageCubeArray],
            Dim::Dim2D => &[
                Capability::Shader,
                Capability::Kernel,
                Capability::ImageMSArray,
            ],
        }
    }
}
impl num_traits::FromPrimitive for Dim {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(Dim::Dim1D),
            1u32 => Some(Dim::Dim2D),
            2u32 => Some(Dim::Dim3D),
            3u32 => Some(Dim::DimCube),
            4u32 => Some(Dim::DimRect),
            5u32 => Some(Dim::DimBuffer),
            6u32 => Some(Dim::DimSubpassData),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [SamplerAddressingMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_sampler_addressing_mode_a_sampler_addressing_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SamplerAddressingMode {
    None = 0u32,
    ClampToEdge = 1u32,
    Clamp = 2u32,
    Repeat = 3u32,
    RepeatMirrored = 4u32,
}
#[allow(non_upper_case_globals)]
impl SamplerAddressingMode {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            SamplerAddressingMode::None
            | SamplerAddressingMode::ClampToEdge
            | SamplerAddressingMode::Clamp
            | SamplerAddressingMode::Repeat
            | SamplerAddressingMode::RepeatMirrored => &[Capability::Kernel],
        }
    }
}
impl num_traits::FromPrimitive for SamplerAddressingMode {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(SamplerAddressingMode::None),
            1u32 => Some(SamplerAddressingMode::ClampToEdge),
            2u32 => Some(SamplerAddressingMode::Clamp),
            3u32 => Some(SamplerAddressingMode::Repeat),
            4u32 => Some(SamplerAddressingMode::RepeatMirrored),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [SamplerFilterMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_sampler_filter_mode_a_sampler_filter_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SamplerFilterMode {
    Nearest = 0u32,
    Linear = 1u32,
}
#[allow(non_upper_case_globals)]
impl SamplerFilterMode {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            SamplerFilterMode::Nearest | SamplerFilterMode::Linear => &[Capability::Kernel],
        }
    }
}
impl num_traits::FromPrimitive for SamplerFilterMode {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(SamplerFilterMode::Nearest),
            1u32 => Some(SamplerFilterMode::Linear),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [ImageFormat](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_image_format_a_image_format)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageFormat {
    Unknown = 0u32,
    Rgba32f = 1u32,
    Rgba16f = 2u32,
    R32f = 3u32,
    Rgba8 = 4u32,
    Rgba8Snorm = 5u32,
    Rg32f = 6u32,
    Rg16f = 7u32,
    R11fG11fB10f = 8u32,
    R16f = 9u32,
    Rgba16 = 10u32,
    Rgb10A2 = 11u32,
    Rg16 = 12u32,
    Rg8 = 13u32,
    R16 = 14u32,
    R8 = 15u32,
    Rgba16Snorm = 16u32,
    Rg16Snorm = 17u32,
    Rg8Snorm = 18u32,
    R16Snorm = 19u32,
    R8Snorm = 20u32,
    Rgba32i = 21u32,
    Rgba16i = 22u32,
    Rgba8i = 23u32,
    R32i = 24u32,
    Rg32i = 25u32,
    Rg16i = 26u32,
    Rg8i = 27u32,
    R16i = 28u32,
    R8i = 29u32,
    Rgba32ui = 30u32,
    Rgba16ui = 31u32,
    Rgba8ui = 32u32,
    R32ui = 33u32,
    Rgb10a2ui = 34u32,
    Rg32ui = 35u32,
    Rg16ui = 36u32,
    Rg8ui = 37u32,
    R16ui = 38u32,
    R8ui = 39u32,
}
#[allow(non_upper_case_globals)]
impl ImageFormat {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            ImageFormat::Unknown => &[],
            ImageFormat::Rgba32f
            | ImageFormat::Rgba16f
            | ImageFormat::R32f
            | ImageFormat::Rgba8
            | ImageFormat::Rgba8Snorm
            | ImageFormat::Rgba32i
            | ImageFormat::Rgba16i
            | ImageFormat::Rgba8i
            | ImageFormat::R32i
            | ImageFormat::Rgba32ui
            | ImageFormat::Rgba16ui
            | ImageFormat::Rgba8ui
            | ImageFormat::R32ui => &[Capability::Shader],
            ImageFormat::Rg32f
            | ImageFormat::Rg16f
            | ImageFormat::R11fG11fB10f
            | ImageFormat::R16f
            | ImageFormat::Rgba16
            | ImageFormat::Rgb10A2
            | ImageFormat::Rg16
            | ImageFormat::Rg8
            | ImageFormat::R16
            | ImageFormat::R8
            | ImageFormat::Rgba16Snorm
            | ImageFormat::Rg16Snorm
            | ImageFormat::Rg8Snorm
            | ImageFormat::R16Snorm
            | ImageFormat::R8Snorm
            | ImageFormat::Rg32i
            | ImageFormat::Rg16i
            | ImageFormat::Rg8i
            | ImageFormat::R16i
            | ImageFormat::R8i
            | ImageFormat::Rgb10a2ui
            | ImageFormat::Rg32ui
            | ImageFormat::Rg16ui
            | ImageFormat::Rg8ui
            | ImageFormat::R16ui
            | ImageFormat::R8ui => &[Capability::StorageImageExtendedFormats],
        }
    }
}
impl num_traits::FromPrimitive for ImageFormat {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(ImageFormat::Unknown),
            1u32 => Some(ImageFormat::Rgba32f),
            2u32 => Some(ImageFormat::Rgba16f),
            3u32 => Some(ImageFormat::R32f),
            4u32 => Some(ImageFormat::Rgba8),
            5u32 => Some(ImageFormat::Rgba8Snorm),
            6u32 => Some(ImageFormat::Rg32f),
            7u32 => Some(ImageFormat::Rg16f),
            8u32 => Some(ImageFormat::R11fG11fB10f),
            9u32 => Some(ImageFormat::R16f),
            10u32 => Some(ImageFormat::Rgba16),
            11u32 => Some(ImageFormat::Rgb10A2),
            12u32 => Some(ImageFormat::Rg16),
            13u32 => Some(ImageFormat::Rg8),
            14u32 => Some(ImageFormat::R16),
            15u32 => Some(ImageFormat::R8),
            16u32 => Some(ImageFormat::Rgba16Snorm),
            17u32 => Some(ImageFormat::Rg16Snorm),
            18u32 => Some(ImageFormat::Rg8Snorm),
            19u32 => Some(ImageFormat::R16Snorm),
            20u32 => Some(ImageFormat::R8Snorm),
            21u32 => Some(ImageFormat::Rgba32i),
            22u32 => Some(ImageFormat::Rgba16i),
            23u32 => Some(ImageFormat::Rgba8i),
            24u32 => Some(ImageFormat::R32i),
            25u32 => Some(ImageFormat::Rg32i),
            26u32 => Some(ImageFormat::Rg16i),
            27u32 => Some(ImageFormat::Rg8i),
            28u32 => Some(ImageFormat::R16i),
            29u32 => Some(ImageFormat::R8i),
            30u32 => Some(ImageFormat::Rgba32ui),
            31u32 => Some(ImageFormat::Rgba16ui),
            32u32 => Some(ImageFormat::Rgba8ui),
            33u32 => Some(ImageFormat::R32ui),
            34u32 => Some(ImageFormat::Rgb10a2ui),
            35u32 => Some(ImageFormat::Rg32ui),
            36u32 => Some(ImageFormat::Rg16ui),
            37u32 => Some(ImageFormat::Rg8ui),
            38u32 => Some(ImageFormat::R16ui),
            39u32 => Some(ImageFormat::R8ui),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [ImageChannelOrder](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_image_channel_order_a_image_channel_order)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageChannelOrder {
    R = 0u32,
    A = 1u32,
    RG = 2u32,
    RA = 3u32,
    RGB = 4u32,
    RGBA = 5u32,
    BGRA = 6u32,
    ARGB = 7u32,
    Intensity = 8u32,
    Luminance = 9u32,
    Rx = 10u32,
    RGx = 11u32,
    RGBx = 12u32,
    Depth = 13u32,
    DepthStencil = 14u32,
    sRGB = 15u32,
    sRGBx = 16u32,
    sRGBA = 17u32,
    sBGRA = 18u32,
    ABGR = 19u32,
}
#[allow(non_upper_case_globals)]
impl ImageChannelOrder {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            ImageChannelOrder::R
            | ImageChannelOrder::A
            | ImageChannelOrder::RG
            | ImageChannelOrder::RA
            | ImageChannelOrder::RGB
            | ImageChannelOrder::RGBA
            | ImageChannelOrder::BGRA
            | ImageChannelOrder::ARGB
            | ImageChannelOrder::Intensity
            | ImageChannelOrder::Luminance
            | ImageChannelOrder::Rx
            | ImageChannelOrder::RGx
            | ImageChannelOrder::RGBx
            | ImageChannelOrder::Depth
            | ImageChannelOrder::DepthStencil
            | ImageChannelOrder::sRGB
            | ImageChannelOrder::sRGBx
            | ImageChannelOrder::sRGBA
            | ImageChannelOrder::sBGRA
            | ImageChannelOrder::ABGR => &[Capability::Kernel],
        }
    }
}
impl num_traits::FromPrimitive for ImageChannelOrder {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(ImageChannelOrder::R),
            1u32 => Some(ImageChannelOrder::A),
            2u32 => Some(ImageChannelOrder::RG),
            3u32 => Some(ImageChannelOrder::RA),
            4u32 => Some(ImageChannelOrder::RGB),
            5u32 => Some(ImageChannelOrder::RGBA),
            6u32 => Some(ImageChannelOrder::BGRA),
            7u32 => Some(ImageChannelOrder::ARGB),
            8u32 => Some(ImageChannelOrder::Intensity),
            9u32 => Some(ImageChannelOrder::Luminance),
            10u32 => Some(ImageChannelOrder::Rx),
            11u32 => Some(ImageChannelOrder::RGx),
            12u32 => Some(ImageChannelOrder::RGBx),
            13u32 => Some(ImageChannelOrder::Depth),
            14u32 => Some(ImageChannelOrder::DepthStencil),
            15u32 => Some(ImageChannelOrder::sRGB),
            16u32 => Some(ImageChannelOrder::sRGBx),
            17u32 => Some(ImageChannelOrder::sRGBA),
            18u32 => Some(ImageChannelOrder::sBGRA),
            19u32 => Some(ImageChannelOrder::ABGR),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [ImageChannelDataType](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_image_channel_data_type_a_image_channel_data_type)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageChannelDataType {
    SnormInt8 = 0u32,
    SnormInt16 = 1u32,
    UnormInt8 = 2u32,
    UnormInt16 = 3u32,
    UnormShort565 = 4u32,
    UnormShort555 = 5u32,
    UnormInt101010 = 6u32,
    SignedInt8 = 7u32,
    SignedInt16 = 8u32,
    SignedInt32 = 9u32,
    UnsignedInt8 = 10u32,
    UnsignedInt16 = 11u32,
    UnsignedInt32 = 12u32,
    HalfFloat = 13u32,
    Float = 14u32,
    UnormInt24 = 15u32,
    UnormInt101010_2 = 16u32,
}
#[allow(non_upper_case_globals)]
impl ImageChannelDataType {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            ImageChannelDataType::SnormInt8
            | ImageChannelDataType::SnormInt16
            | ImageChannelDataType::UnormInt8
            | ImageChannelDataType::UnormInt16
            | ImageChannelDataType::UnormShort565
            | ImageChannelDataType::UnormShort555
            | ImageChannelDataType::UnormInt101010
            | ImageChannelDataType::SignedInt8
            | ImageChannelDataType::SignedInt16
            | ImageChannelDataType::SignedInt32
            | ImageChannelDataType::UnsignedInt8
            | ImageChannelDataType::UnsignedInt16
            | ImageChannelDataType::UnsignedInt32
            | ImageChannelDataType::HalfFloat
            | ImageChannelDataType::Float
            | ImageChannelDataType::UnormInt24
            | ImageChannelDataType::UnormInt101010_2 => &[Capability::Kernel],
        }
    }
}
impl num_traits::FromPrimitive for ImageChannelDataType {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(ImageChannelDataType::SnormInt8),
            1u32 => Some(ImageChannelDataType::SnormInt16),
            2u32 => Some(ImageChannelDataType::UnormInt8),
            3u32 => Some(ImageChannelDataType::UnormInt16),
            4u32 => Some(ImageChannelDataType::UnormShort565),
            5u32 => Some(ImageChannelDataType::UnormShort555),
            6u32 => Some(ImageChannelDataType::UnormInt101010),
            7u32 => Some(ImageChannelDataType::SignedInt8),
            8u32 => Some(ImageChannelDataType::SignedInt16),
            9u32 => Some(ImageChannelDataType::SignedInt32),
            10u32 => Some(ImageChannelDataType::UnsignedInt8),
            11u32 => Some(ImageChannelDataType::UnsignedInt16),
            12u32 => Some(ImageChannelDataType::UnsignedInt32),
            13u32 => Some(ImageChannelDataType::HalfFloat),
            14u32 => Some(ImageChannelDataType::Float),
            15u32 => Some(ImageChannelDataType::UnormInt24),
            16u32 => Some(ImageChannelDataType::UnormInt101010_2),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [FPRoundingMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_fp_rounding_mode_a_fp_rounding_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FPRoundingMode {
    RTE = 0u32,
    RTZ = 1u32,
    RTP = 2u32,
    RTN = 3u32,
}
#[allow(non_upper_case_globals)]
impl FPRoundingMode {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            FPRoundingMode::RTE
            | FPRoundingMode::RTZ
            | FPRoundingMode::RTP
            | FPRoundingMode::RTN => &[],
        }
    }
}
impl num_traits::FromPrimitive for FPRoundingMode {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(FPRoundingMode::RTE),
            1u32 => Some(FPRoundingMode::RTZ),
            2u32 => Some(FPRoundingMode::RTP),
            3u32 => Some(FPRoundingMode::RTN),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [LinkageType](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_linkage_type_a_linkage_type)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LinkageType {
    Export = 0u32,
    Import = 1u32,
}
#[allow(non_upper_case_globals)]
impl LinkageType {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            LinkageType::Export | LinkageType::Import => &[Capability::Linkage],
        }
    }
}
impl num_traits::FromPrimitive for LinkageType {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(LinkageType::Export),
            1u32 => Some(LinkageType::Import),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [AccessQualifier](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_access_qualifier_a_access_qualifier)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccessQualifier {
    ReadOnly = 0u32,
    WriteOnly = 1u32,
    ReadWrite = 2u32,
}
#[allow(non_upper_case_globals)]
impl AccessQualifier {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            AccessQualifier::ReadOnly | AccessQualifier::WriteOnly | AccessQualifier::ReadWrite => {
                &[Capability::Kernel]
            }
        }
    }
}
impl num_traits::FromPrimitive for AccessQualifier {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(AccessQualifier::ReadOnly),
            1u32 => Some(AccessQualifier::WriteOnly),
            2u32 => Some(AccessQualifier::ReadWrite),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [FunctionParameterAttribute](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_function_parameter_attribute_a_function_parameter_attribute)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionParameterAttribute {
    Zext = 0u32,
    Sext = 1u32,
    ByVal = 2u32,
    Sret = 3u32,
    NoAlias = 4u32,
    NoCapture = 5u32,
    NoWrite = 6u32,
    NoReadWrite = 7u32,
}
#[allow(non_upper_case_globals)]
impl FunctionParameterAttribute {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            FunctionParameterAttribute::Zext
            | FunctionParameterAttribute::Sext
            | FunctionParameterAttribute::ByVal
            | FunctionParameterAttribute::Sret
            | FunctionParameterAttribute::NoAlias
            | FunctionParameterAttribute::NoCapture
            | FunctionParameterAttribute::NoWrite
            | FunctionParameterAttribute::NoReadWrite => &[Capability::Kernel],
        }
    }
}
impl num_traits::FromPrimitive for FunctionParameterAttribute {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(FunctionParameterAttribute::Zext),
            1u32 => Some(FunctionParameterAttribute::Sext),
            2u32 => Some(FunctionParameterAttribute::ByVal),
            3u32 => Some(FunctionParameterAttribute::Sret),
            4u32 => Some(FunctionParameterAttribute::NoAlias),
            5u32 => Some(FunctionParameterAttribute::NoCapture),
            6u32 => Some(FunctionParameterAttribute::NoWrite),
            7u32 => Some(FunctionParameterAttribute::NoReadWrite),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [Decoration](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_decoration_a_decoration)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Decoration {
    RelaxedPrecision = 0u32,
    SpecId = 1u32,
    Block = 2u32,
    BufferBlock = 3u32,
    RowMajor = 4u32,
    ColMajor = 5u32,
    ArrayStride = 6u32,
    MatrixStride = 7u32,
    GLSLShared = 8u32,
    GLSLPacked = 9u32,
    CPacked = 10u32,
    BuiltIn = 11u32,
    NoPerspective = 13u32,
    Flat = 14u32,
    Patch = 15u32,
    Centroid = 16u32,
    Sample = 17u32,
    Invariant = 18u32,
    Restrict = 19u32,
    Aliased = 20u32,
    Volatile = 21u32,
    Constant = 22u32,
    Coherent = 23u32,
    NonWritable = 24u32,
    NonReadable = 25u32,
    Uniform = 26u32,
    SaturatedConversion = 28u32,
    Stream = 29u32,
    Location = 30u32,
    Component = 31u32,
    Index = 32u32,
    Binding = 33u32,
    DescriptorSet = 34u32,
    Offset = 35u32,
    XfbBuffer = 36u32,
    XfbStride = 37u32,
    FuncParamAttr = 38u32,
    FPRoundingMode = 39u32,
    FPFastMathMode = 40u32,
    LinkageAttributes = 41u32,
    NoContraction = 42u32,
    InputAttachmentIndex = 43u32,
    Alignment = 44u32,
    MaxByteOffset = 45u32,
    AlignmentId = 46u32,
    MaxByteOffsetId = 47u32,
    ExplicitInterpAMD = 4999u32,
    OverrideCoverageNV = 5248u32,
    PassthroughNV = 5250u32,
    ViewportRelativeNV = 5252u32,
    SecondaryViewportRelativeNV = 5256u32,
    NonUniformEXT = 5300u32,
    HlslCounterBufferGOOGLE = 5634u32,
    HlslSemanticGOOGLE = 5635u32,
}
#[allow(non_upper_case_globals)]
impl Decoration {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            Decoration::BuiltIn
            | Decoration::Restrict
            | Decoration::Aliased
            | Decoration::Volatile
            | Decoration::Coherent
            | Decoration::NonWritable
            | Decoration::NonReadable
            | Decoration::FPRoundingMode
            | Decoration::ExplicitInterpAMD
            | Decoration::HlslCounterBufferGOOGLE
            | Decoration::HlslSemanticGOOGLE => &[],
            Decoration::MaxByteOffset | Decoration::MaxByteOffsetId => &[Capability::Addresses],
            Decoration::PassthroughNV => &[Capability::GeometryShaderPassthroughNV],
            Decoration::Stream => &[Capability::GeometryStreams],
            Decoration::InputAttachmentIndex => &[Capability::InputAttachment],
            Decoration::CPacked
            | Decoration::Constant
            | Decoration::SaturatedConversion
            | Decoration::FuncParamAttr
            | Decoration::FPFastMathMode
            | Decoration::Alignment
            | Decoration::AlignmentId => &[Capability::Kernel],
            Decoration::LinkageAttributes => &[Capability::Linkage],
            Decoration::RowMajor | Decoration::ColMajor | Decoration::MatrixStride => {
                &[Capability::Matrix]
            }
            Decoration::OverrideCoverageNV => &[Capability::SampleMaskOverrideCoverageNV],
            Decoration::Sample => &[Capability::SampleRateShading],
            Decoration::RelaxedPrecision
            | Decoration::Block
            | Decoration::BufferBlock
            | Decoration::ArrayStride
            | Decoration::GLSLShared
            | Decoration::GLSLPacked
            | Decoration::NoPerspective
            | Decoration::Flat
            | Decoration::Centroid
            | Decoration::Invariant
            | Decoration::Uniform
            | Decoration::Location
            | Decoration::Component
            | Decoration::Index
            | Decoration::Binding
            | Decoration::DescriptorSet
            | Decoration::Offset
            | Decoration::NoContraction => &[Capability::Shader],
            Decoration::SpecId => &[Capability::Shader, Capability::Kernel],
            Decoration::NonUniformEXT => &[Capability::ShaderNonUniformEXT],
            Decoration::SecondaryViewportRelativeNV => &[Capability::ShaderStereoViewNV],
            Decoration::ViewportRelativeNV => &[Capability::ShaderViewportMaskNV],
            Decoration::Patch => &[Capability::Tessellation],
            Decoration::XfbBuffer | Decoration::XfbStride => &[Capability::TransformFeedback],
        }
    }
}
impl num_traits::FromPrimitive for Decoration {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(Decoration::RelaxedPrecision),
            1u32 => Some(Decoration::SpecId),
            2u32 => Some(Decoration::Block),
            3u32 => Some(Decoration::BufferBlock),
            4u32 => Some(Decoration::RowMajor),
            5u32 => Some(Decoration::ColMajor),
            6u32 => Some(Decoration::ArrayStride),
            7u32 => Some(Decoration::MatrixStride),
            8u32 => Some(Decoration::GLSLShared),
            9u32 => Some(Decoration::GLSLPacked),
            10u32 => Some(Decoration::CPacked),
            11u32 => Some(Decoration::BuiltIn),
            13u32 => Some(Decoration::NoPerspective),
            14u32 => Some(Decoration::Flat),
            15u32 => Some(Decoration::Patch),
            16u32 => Some(Decoration::Centroid),
            17u32 => Some(Decoration::Sample),
            18u32 => Some(Decoration::Invariant),
            19u32 => Some(Decoration::Restrict),
            20u32 => Some(Decoration::Aliased),
            21u32 => Some(Decoration::Volatile),
            22u32 => Some(Decoration::Constant),
            23u32 => Some(Decoration::Coherent),
            24u32 => Some(Decoration::NonWritable),
            25u32 => Some(Decoration::NonReadable),
            26u32 => Some(Decoration::Uniform),
            28u32 => Some(Decoration::SaturatedConversion),
            29u32 => Some(Decoration::Stream),
            30u32 => Some(Decoration::Location),
            31u32 => Some(Decoration::Component),
            32u32 => Some(Decoration::Index),
            33u32 => Some(Decoration::Binding),
            34u32 => Some(Decoration::DescriptorSet),
            35u32 => Some(Decoration::Offset),
            36u32 => Some(Decoration::XfbBuffer),
            37u32 => Some(Decoration::XfbStride),
            38u32 => Some(Decoration::FuncParamAttr),
            39u32 => Some(Decoration::FPRoundingMode),
            40u32 => Some(Decoration::FPFastMathMode),
            41u32 => Some(Decoration::LinkageAttributes),
            42u32 => Some(Decoration::NoContraction),
            43u32 => Some(Decoration::InputAttachmentIndex),
            44u32 => Some(Decoration::Alignment),
            45u32 => Some(Decoration::MaxByteOffset),
            46u32 => Some(Decoration::AlignmentId),
            47u32 => Some(Decoration::MaxByteOffsetId),
            4999u32 => Some(Decoration::ExplicitInterpAMD),
            5248u32 => Some(Decoration::OverrideCoverageNV),
            5250u32 => Some(Decoration::PassthroughNV),
            5252u32 => Some(Decoration::ViewportRelativeNV),
            5256u32 => Some(Decoration::SecondaryViewportRelativeNV),
            5300u32 => Some(Decoration::NonUniformEXT),
            5634u32 => Some(Decoration::HlslCounterBufferGOOGLE),
            5635u32 => Some(Decoration::HlslSemanticGOOGLE),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [BuiltIn](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_built_in_a_built_in)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltIn {
    Position = 0u32,
    PointSize = 1u32,
    ClipDistance = 3u32,
    CullDistance = 4u32,
    VertexId = 5u32,
    InstanceId = 6u32,
    PrimitiveId = 7u32,
    InvocationId = 8u32,
    Layer = 9u32,
    ViewportIndex = 10u32,
    TessLevelOuter = 11u32,
    TessLevelInner = 12u32,
    TessCoord = 13u32,
    PatchVertices = 14u32,
    FragCoord = 15u32,
    PointCoord = 16u32,
    FrontFacing = 17u32,
    SampleId = 18u32,
    SamplePosition = 19u32,
    SampleMask = 20u32,
    FragDepth = 22u32,
    HelperInvocation = 23u32,
    NumWorkgroups = 24u32,
    WorkgroupSize = 25u32,
    WorkgroupId = 26u32,
    LocalInvocationId = 27u32,
    GlobalInvocationId = 28u32,
    LocalInvocationIndex = 29u32,
    WorkDim = 30u32,
    GlobalSize = 31u32,
    EnqueuedWorkgroupSize = 32u32,
    GlobalOffset = 33u32,
    GlobalLinearId = 34u32,
    SubgroupSize = 36u32,
    SubgroupMaxSize = 37u32,
    NumSubgroups = 38u32,
    NumEnqueuedSubgroups = 39u32,
    SubgroupId = 40u32,
    SubgroupLocalInvocationId = 41u32,
    VertexIndex = 42u32,
    InstanceIndex = 43u32,
    SubgroupEqMask = 4416u32,
    SubgroupGeMask = 4417u32,
    SubgroupGtMask = 4418u32,
    SubgroupLeMask = 4419u32,
    SubgroupLtMask = 4420u32,
    BaseVertex = 4424u32,
    BaseInstance = 4425u32,
    DrawIndex = 4426u32,
    DeviceIndex = 4438u32,
    ViewIndex = 4440u32,
    BaryCoordNoPerspAMD = 4992u32,
    BaryCoordNoPerspCentroidAMD = 4993u32,
    BaryCoordNoPerspSampleAMD = 4994u32,
    BaryCoordSmoothAMD = 4995u32,
    BaryCoordSmoothCentroidAMD = 4996u32,
    BaryCoordSmoothSampleAMD = 4997u32,
    BaryCoordPullModelAMD = 4998u32,
    FragStencilRefEXT = 5014u32,
    ViewportMaskNV = 5253u32,
    SecondaryPositionNV = 5257u32,
    SecondaryViewportMaskNV = 5258u32,
    PositionPerViewNV = 5261u32,
    ViewportMaskPerViewNV = 5262u32,
    FullyCoveredEXT = 5264u32,
}
#[allow(non_upper_case_globals)]
impl BuiltIn {
    pub const SubgroupEqMaskKHR: BuiltIn = BuiltIn::SubgroupEqMask;
    pub const SubgroupGeMaskKHR: BuiltIn = BuiltIn::SubgroupGeMask;
    pub const SubgroupGtMaskKHR: BuiltIn = BuiltIn::SubgroupGtMask;
    pub const SubgroupLeMaskKHR: BuiltIn = BuiltIn::SubgroupLeMask;
    pub const SubgroupLtMaskKHR: BuiltIn = BuiltIn::SubgroupLtMask;
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            BuiltIn::NumWorkgroups
            | BuiltIn::WorkgroupSize
            | BuiltIn::WorkgroupId
            | BuiltIn::LocalInvocationId
            | BuiltIn::GlobalInvocationId
            | BuiltIn::LocalInvocationIndex
            | BuiltIn::BaryCoordNoPerspAMD
            | BuiltIn::BaryCoordNoPerspCentroidAMD
            | BuiltIn::BaryCoordNoPerspSampleAMD
            | BuiltIn::BaryCoordSmoothAMD
            | BuiltIn::BaryCoordSmoothCentroidAMD
            | BuiltIn::BaryCoordSmoothSampleAMD
            | BuiltIn::BaryCoordPullModelAMD => &[],
            BuiltIn::ClipDistance => &[Capability::ClipDistance],
            BuiltIn::CullDistance => &[Capability::CullDistance],
            BuiltIn::DeviceIndex => &[Capability::DeviceGroup],
            BuiltIn::BaseVertex | BuiltIn::BaseInstance | BuiltIn::DrawIndex => {
                &[Capability::DrawParameters]
            }
            BuiltIn::FullyCoveredEXT => &[Capability::FragmentFullyCoveredEXT],
            BuiltIn::Layer => &[Capability::Geometry],
            BuiltIn::PrimitiveId | BuiltIn::InvocationId => {
                &[Capability::Geometry, Capability::Tessellation]
            }
            BuiltIn::WorkDim
            | BuiltIn::GlobalSize
            | BuiltIn::EnqueuedWorkgroupSize
            | BuiltIn::GlobalOffset
            | BuiltIn::GlobalLinearId
            | BuiltIn::SubgroupMaxSize
            | BuiltIn::NumEnqueuedSubgroups => &[Capability::Kernel],
            BuiltIn::NumSubgroups | BuiltIn::SubgroupId => {
                &[Capability::Kernel, Capability::GroupNonUniform]
            }
            BuiltIn::SubgroupSize | BuiltIn::SubgroupLocalInvocationId => &[
                Capability::Kernel,
                Capability::GroupNonUniform,
                Capability::SubgroupBallotKHR,
            ],
            BuiltIn::ViewIndex => &[Capability::MultiView],
            BuiltIn::ViewportIndex => &[Capability::MultiViewport],
            BuiltIn::PositionPerViewNV | BuiltIn::ViewportMaskPerViewNV => {
                &[Capability::PerViewAttributesNV]
            }
            BuiltIn::SampleId | BuiltIn::SamplePosition => &[Capability::SampleRateShading],
            BuiltIn::Position
            | BuiltIn::PointSize
            | BuiltIn::VertexId
            | BuiltIn::InstanceId
            | BuiltIn::FragCoord
            | BuiltIn::PointCoord
            | BuiltIn::FrontFacing
            | BuiltIn::SampleMask
            | BuiltIn::FragDepth
            | BuiltIn::HelperInvocation
            | BuiltIn::VertexIndex
            | BuiltIn::InstanceIndex => &[Capability::Shader],
            BuiltIn::SecondaryPositionNV | BuiltIn::SecondaryViewportMaskNV => {
                &[Capability::ShaderStereoViewNV]
            }
            BuiltIn::ViewportMaskNV => &[Capability::ShaderViewportMaskNV],
            BuiltIn::FragStencilRefEXT => &[Capability::StencilExportEXT],
            BuiltIn::SubgroupEqMask
            | BuiltIn::SubgroupGeMask
            | BuiltIn::SubgroupGtMask
            | BuiltIn::SubgroupLeMask
            | BuiltIn::SubgroupLtMask => &[
                Capability::SubgroupBallotKHR,
                Capability::GroupNonUniformBallot,
            ],
            BuiltIn::TessLevelOuter
            | BuiltIn::TessLevelInner
            | BuiltIn::TessCoord
            | BuiltIn::PatchVertices => &[Capability::Tessellation],
        }
    }
}
impl num_traits::FromPrimitive for BuiltIn {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(BuiltIn::Position),
            1u32 => Some(BuiltIn::PointSize),
            3u32 => Some(BuiltIn::ClipDistance),
            4u32 => Some(BuiltIn::CullDistance),
            5u32 => Some(BuiltIn::VertexId),
            6u32 => Some(BuiltIn::InstanceId),
            7u32 => Some(BuiltIn::PrimitiveId),
            8u32 => Some(BuiltIn::InvocationId),
            9u32 => Some(BuiltIn::Layer),
            10u32 => Some(BuiltIn::ViewportIndex),
            11u32 => Some(BuiltIn::TessLevelOuter),
            12u32 => Some(BuiltIn::TessLevelInner),
            13u32 => Some(BuiltIn::TessCoord),
            14u32 => Some(BuiltIn::PatchVertices),
            15u32 => Some(BuiltIn::FragCoord),
            16u32 => Some(BuiltIn::PointCoord),
            17u32 => Some(BuiltIn::FrontFacing),
            18u32 => Some(BuiltIn::SampleId),
            19u32 => Some(BuiltIn::SamplePosition),
            20u32 => Some(BuiltIn::SampleMask),
            22u32 => Some(BuiltIn::FragDepth),
            23u32 => Some(BuiltIn::HelperInvocation),
            24u32 => Some(BuiltIn::NumWorkgroups),
            25u32 => Some(BuiltIn::WorkgroupSize),
            26u32 => Some(BuiltIn::WorkgroupId),
            27u32 => Some(BuiltIn::LocalInvocationId),
            28u32 => Some(BuiltIn::GlobalInvocationId),
            29u32 => Some(BuiltIn::LocalInvocationIndex),
            30u32 => Some(BuiltIn::WorkDim),
            31u32 => Some(BuiltIn::GlobalSize),
            32u32 => Some(BuiltIn::EnqueuedWorkgroupSize),
            33u32 => Some(BuiltIn::GlobalOffset),
            34u32 => Some(BuiltIn::GlobalLinearId),
            36u32 => Some(BuiltIn::SubgroupSize),
            37u32 => Some(BuiltIn::SubgroupMaxSize),
            38u32 => Some(BuiltIn::NumSubgroups),
            39u32 => Some(BuiltIn::NumEnqueuedSubgroups),
            40u32 => Some(BuiltIn::SubgroupId),
            41u32 => Some(BuiltIn::SubgroupLocalInvocationId),
            42u32 => Some(BuiltIn::VertexIndex),
            43u32 => Some(BuiltIn::InstanceIndex),
            4416u32 => Some(BuiltIn::SubgroupEqMask),
            4417u32 => Some(BuiltIn::SubgroupGeMask),
            4418u32 => Some(BuiltIn::SubgroupGtMask),
            4419u32 => Some(BuiltIn::SubgroupLeMask),
            4420u32 => Some(BuiltIn::SubgroupLtMask),
            4424u32 => Some(BuiltIn::BaseVertex),
            4425u32 => Some(BuiltIn::BaseInstance),
            4426u32 => Some(BuiltIn::DrawIndex),
            4438u32 => Some(BuiltIn::DeviceIndex),
            4440u32 => Some(BuiltIn::ViewIndex),
            4992u32 => Some(BuiltIn::BaryCoordNoPerspAMD),
            4993u32 => Some(BuiltIn::BaryCoordNoPerspCentroidAMD),
            4994u32 => Some(BuiltIn::BaryCoordNoPerspSampleAMD),
            4995u32 => Some(BuiltIn::BaryCoordSmoothAMD),
            4996u32 => Some(BuiltIn::BaryCoordSmoothCentroidAMD),
            4997u32 => Some(BuiltIn::BaryCoordSmoothSampleAMD),
            4998u32 => Some(BuiltIn::BaryCoordPullModelAMD),
            5014u32 => Some(BuiltIn::FragStencilRefEXT),
            5253u32 => Some(BuiltIn::ViewportMaskNV),
            5257u32 => Some(BuiltIn::SecondaryPositionNV),
            5258u32 => Some(BuiltIn::SecondaryViewportMaskNV),
            5261u32 => Some(BuiltIn::PositionPerViewNV),
            5262u32 => Some(BuiltIn::ViewportMaskPerViewNV),
            5264u32 => Some(BuiltIn::FullyCoveredEXT),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [Scope](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_scope_a_scope)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    CrossDevice = 0u32,
    Device = 1u32,
    Workgroup = 2u32,
    Subgroup = 3u32,
    Invocation = 4u32,
    QueueFamilyKHR = 5u32,
}
#[allow(non_upper_case_globals)]
impl Scope {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            Scope::CrossDevice
            | Scope::Device
            | Scope::Workgroup
            | Scope::Subgroup
            | Scope::Invocation => &[],
            Scope::QueueFamilyKHR => &[Capability::VulkanMemoryModelKHR],
        }
    }
}
impl num_traits::FromPrimitive for Scope {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(Scope::CrossDevice),
            1u32 => Some(Scope::Device),
            2u32 => Some(Scope::Workgroup),
            3u32 => Some(Scope::Subgroup),
            4u32 => Some(Scope::Invocation),
            5u32 => Some(Scope::QueueFamilyKHR),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [GroupOperation](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_group_operation_a_group_operation)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroupOperation {
    Reduce = 0u32,
    InclusiveScan = 1u32,
    ExclusiveScan = 2u32,
    ClusteredReduce = 3u32,
    PartitionedReduceNV = 6u32,
    PartitionedInclusiveScanNV = 7u32,
    PartitionedExclusiveScanNV = 8u32,
}
#[allow(non_upper_case_globals)]
impl GroupOperation {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            GroupOperation::ClusteredReduce => &[Capability::GroupNonUniformClustered],
            GroupOperation::PartitionedReduceNV
            | GroupOperation::PartitionedInclusiveScanNV
            | GroupOperation::PartitionedExclusiveScanNV => {
                &[Capability::GroupNonUniformPartitionedNV]
            }
            GroupOperation::Reduce
            | GroupOperation::InclusiveScan
            | GroupOperation::ExclusiveScan => &[
                Capability::Kernel,
                Capability::GroupNonUniformArithmetic,
                Capability::GroupNonUniformBallot,
            ],
        }
    }
}
impl num_traits::FromPrimitive for GroupOperation {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(GroupOperation::Reduce),
            1u32 => Some(GroupOperation::InclusiveScan),
            2u32 => Some(GroupOperation::ExclusiveScan),
            3u32 => Some(GroupOperation::ClusteredReduce),
            6u32 => Some(GroupOperation::PartitionedReduceNV),
            7u32 => Some(GroupOperation::PartitionedInclusiveScanNV),
            8u32 => Some(GroupOperation::PartitionedExclusiveScanNV),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [KernelEnqueueFlags](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_kernel_enqueue_flags_a_kernel_enqueue_flags)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KernelEnqueueFlags {
    NoWait = 0u32,
    WaitKernel = 1u32,
    WaitWorkGroup = 2u32,
}
#[allow(non_upper_case_globals)]
impl KernelEnqueueFlags {
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            KernelEnqueueFlags::NoWait
            | KernelEnqueueFlags::WaitKernel
            | KernelEnqueueFlags::WaitWorkGroup => &[Capability::Kernel],
        }
    }
}
impl num_traits::FromPrimitive for KernelEnqueueFlags {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(KernelEnqueueFlags::NoWait),
            1u32 => Some(KernelEnqueueFlags::WaitKernel),
            2u32 => Some(KernelEnqueueFlags::WaitWorkGroup),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "/// SPIR-V operand kind: [Capability](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_capability_a_capability)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Capability {
    Matrix = 0u32,
    Shader = 1u32,
    Geometry = 2u32,
    Tessellation = 3u32,
    Addresses = 4u32,
    Linkage = 5u32,
    Kernel = 6u32,
    Vector16 = 7u32,
    Float16Buffer = 8u32,
    Float16 = 9u32,
    Float64 = 10u32,
    Int64 = 11u32,
    Int64Atomics = 12u32,
    ImageBasic = 13u32,
    ImageReadWrite = 14u32,
    ImageMipmap = 15u32,
    Pipes = 17u32,
    Groups = 18u32,
    DeviceEnqueue = 19u32,
    LiteralSampler = 20u32,
    AtomicStorage = 21u32,
    Int16 = 22u32,
    TessellationPointSize = 23u32,
    GeometryPointSize = 24u32,
    ImageGatherExtended = 25u32,
    StorageImageMultisample = 27u32,
    UniformBufferArrayDynamicIndexing = 28u32,
    SampledImageArrayDynamicIndexing = 29u32,
    StorageBufferArrayDynamicIndexing = 30u32,
    StorageImageArrayDynamicIndexing = 31u32,
    ClipDistance = 32u32,
    CullDistance = 33u32,
    ImageCubeArray = 34u32,
    SampleRateShading = 35u32,
    ImageRect = 36u32,
    SampledRect = 37u32,
    GenericPointer = 38u32,
    Int8 = 39u32,
    InputAttachment = 40u32,
    SparseResidency = 41u32,
    MinLod = 42u32,
    Sampled1D = 43u32,
    Image1D = 44u32,
    SampledCubeArray = 45u32,
    SampledBuffer = 46u32,
    ImageBuffer = 47u32,
    ImageMSArray = 48u32,
    StorageImageExtendedFormats = 49u32,
    ImageQuery = 50u32,
    DerivativeControl = 51u32,
    InterpolationFunction = 52u32,
    TransformFeedback = 53u32,
    GeometryStreams = 54u32,
    StorageImageReadWithoutFormat = 55u32,
    StorageImageWriteWithoutFormat = 56u32,
    MultiViewport = 57u32,
    SubgroupDispatch = 58u32,
    NamedBarrier = 59u32,
    PipeStorage = 60u32,
    GroupNonUniform = 61u32,
    GroupNonUniformVote = 62u32,
    GroupNonUniformArithmetic = 63u32,
    GroupNonUniformBallot = 64u32,
    GroupNonUniformShuffle = 65u32,
    GroupNonUniformShuffleRelative = 66u32,
    GroupNonUniformClustered = 67u32,
    GroupNonUniformQuad = 68u32,
    SubgroupBallotKHR = 4423u32,
    DrawParameters = 4427u32,
    SubgroupVoteKHR = 4431u32,
    StorageBuffer16BitAccess = 4433u32,
    UniformAndStorageBuffer16BitAccess = 4434u32,
    StoragePushConstant16 = 4435u32,
    StorageInputOutput16 = 4436u32,
    DeviceGroup = 4437u32,
    MultiView = 4439u32,
    VariablePointersStorageBuffer = 4441u32,
    VariablePointers = 4442u32,
    AtomicStorageOps = 4445u32,
    SampleMaskPostDepthCoverage = 4447u32,
    StorageBuffer8BitAccess = 4448u32,
    UniformAndStorageBuffer8BitAccess = 4449u32,
    StoragePushConstant8 = 4450u32,
    Float16ImageAMD = 5008u32,
    ImageGatherBiasLodAMD = 5009u32,
    FragmentMaskAMD = 5010u32,
    StencilExportEXT = 5013u32,
    ImageReadWriteLodAMD = 5015u32,
    SampleMaskOverrideCoverageNV = 5249u32,
    GeometryShaderPassthroughNV = 5251u32,
    ShaderViewportIndexLayerEXT = 5254u32,
    ShaderViewportMaskNV = 5255u32,
    ShaderStereoViewNV = 5259u32,
    PerViewAttributesNV = 5260u32,
    FragmentFullyCoveredEXT = 5265u32,
    ShaderNonUniformEXT = 5301u32,
    RuntimeDescriptorArrayEXT = 5302u32,
    InputAttachmentArrayDynamicIndexingEXT = 5303u32,
    UniformTexelBufferArrayDynamicIndexingEXT = 5304u32,
    StorageTexelBufferArrayDynamicIndexingEXT = 5305u32,
    UniformBufferArrayNonUniformIndexingEXT = 5306u32,
    SampledImageArrayNonUniformIndexingEXT = 5307u32,
    StorageBufferArrayNonUniformIndexingEXT = 5308u32,
    StorageImageArrayNonUniformIndexingEXT = 5309u32,
    InputAttachmentArrayNonUniformIndexingEXT = 5310u32,
    UniformTexelBufferArrayNonUniformIndexingEXT = 5311u32,
    StorageTexelBufferArrayNonUniformIndexingEXT = 5312u32,
    SubgroupShuffleINTEL = 5568u32,
    SubgroupBufferBlockIOINTEL = 5569u32,
    SubgroupImageBlockIOINTEL = 5570u32,
    GroupNonUniformPartitionedNV = 5297u32,
    VulkanMemoryModelKHR = 5345u32,
    VulkanMemoryModelDeviceScopeKHR = 5346u32,
}
#[allow(non_upper_case_globals)]
impl Capability {
    pub const StorageUniformBufferBlock16: Capability = Capability::StorageBuffer16BitAccess;
    pub const StorageUniform16: Capability = Capability::UniformAndStorageBuffer16BitAccess;
    pub const ShaderViewportIndexLayerNV: Capability = Capability::ShaderViewportIndexLayerEXT;
    pub fn required_capabilities(self) -> &'static [Capability] {
        match self {
            Capability::Matrix
            | Capability::Addresses
            | Capability::Linkage
            | Capability::Kernel
            | Capability::Float16
            | Capability::Float64
            | Capability::Int64
            | Capability::Groups
            | Capability::Int16
            | Capability::Int8
            | Capability::Sampled1D
            | Capability::SampledBuffer
            | Capability::GroupNonUniform
            | Capability::SubgroupBallotKHR
            | Capability::SubgroupVoteKHR
            | Capability::StorageBuffer16BitAccess
            | Capability::StoragePushConstant16
            | Capability::StorageInputOutput16
            | Capability::DeviceGroup
            | Capability::AtomicStorageOps
            | Capability::SampleMaskPostDepthCoverage
            | Capability::StorageBuffer8BitAccess
            | Capability::StoragePushConstant8
            | Capability::SubgroupShuffleINTEL
            | Capability::SubgroupBufferBlockIOINTEL
            | Capability::SubgroupImageBlockIOINTEL
            | Capability::GroupNonUniformPartitionedNV
            | Capability::VulkanMemoryModelKHR
            | Capability::VulkanMemoryModelDeviceScopeKHR => &[],
            Capability::GenericPointer => &[Capability::Addresses],
            Capability::SubgroupDispatch => &[Capability::DeviceEnqueue],
            Capability::GeometryPointSize
            | Capability::GeometryStreams
            | Capability::MultiViewport
            | Capability::GeometryShaderPassthroughNV => &[Capability::Geometry],
            Capability::GroupNonUniformVote
            | Capability::GroupNonUniformArithmetic
            | Capability::GroupNonUniformBallot
            | Capability::GroupNonUniformShuffle
            | Capability::GroupNonUniformShuffleRelative
            | Capability::GroupNonUniformClustered
            | Capability::GroupNonUniformQuad => &[Capability::GroupNonUniform],
            Capability::ImageReadWrite | Capability::ImageMipmap => &[Capability::ImageBasic],
            Capability::StorageTexelBufferArrayDynamicIndexingEXT => &[Capability::ImageBuffer],
            Capability::StorageTexelBufferArrayNonUniformIndexingEXT => {
                &[Capability::ImageBuffer, Capability::ShaderNonUniformEXT]
            }
            Capability::InputAttachmentArrayDynamicIndexingEXT => &[Capability::InputAttachment],
            Capability::InputAttachmentArrayNonUniformIndexingEXT => {
                &[Capability::InputAttachment, Capability::ShaderNonUniformEXT]
            }
            Capability::Int64Atomics => &[Capability::Int64],
            Capability::Vector16
            | Capability::Float16Buffer
            | Capability::ImageBasic
            | Capability::Pipes
            | Capability::DeviceEnqueue
            | Capability::LiteralSampler
            | Capability::NamedBarrier => &[Capability::Kernel],
            Capability::Shader => &[Capability::Matrix],
            Capability::PerViewAttributesNV => &[Capability::MultiView],
            Capability::ShaderViewportIndexLayerEXT => &[Capability::MultiViewport],
            Capability::PipeStorage => &[Capability::Pipes],
            Capability::SampleMaskOverrideCoverageNV => &[Capability::SampleRateShading],
            Capability::Image1D => &[Capability::Sampled1D],
            Capability::ImageBuffer | Capability::UniformTexelBufferArrayDynamicIndexingEXT => {
                &[Capability::SampledBuffer]
            }
            Capability::UniformTexelBufferArrayNonUniformIndexingEXT => {
                &[Capability::SampledBuffer, Capability::ShaderNonUniformEXT]
            }
            Capability::ImageCubeArray => &[Capability::SampledCubeArray],
            Capability::ImageRect => &[Capability::SampledRect],
            Capability::Geometry
            | Capability::Tessellation
            | Capability::AtomicStorage
            | Capability::ImageGatherExtended
            | Capability::StorageImageMultisample
            | Capability::UniformBufferArrayDynamicIndexing
            | Capability::SampledImageArrayDynamicIndexing
            | Capability::StorageBufferArrayDynamicIndexing
            | Capability::StorageImageArrayDynamicIndexing
            | Capability::ClipDistance
            | Capability::CullDistance
            | Capability::SampleRateShading
            | Capability::SampledRect
            | Capability::InputAttachment
            | Capability::SparseResidency
            | Capability::MinLod
            | Capability::SampledCubeArray
            | Capability::ImageMSArray
            | Capability::StorageImageExtendedFormats
            | Capability::ImageQuery
            | Capability::DerivativeControl
            | Capability::InterpolationFunction
            | Capability::TransformFeedback
            | Capability::StorageImageReadWithoutFormat
            | Capability::StorageImageWriteWithoutFormat
            | Capability::DrawParameters
            | Capability::MultiView
            | Capability::VariablePointersStorageBuffer
            | Capability::Float16ImageAMD
            | Capability::ImageGatherBiasLodAMD
            | Capability::FragmentMaskAMD
            | Capability::StencilExportEXT
            | Capability::ImageReadWriteLodAMD
            | Capability::FragmentFullyCoveredEXT
            | Capability::ShaderNonUniformEXT
            | Capability::RuntimeDescriptorArrayEXT => &[Capability::Shader],
            Capability::UniformBufferArrayNonUniformIndexingEXT
            | Capability::SampledImageArrayNonUniformIndexingEXT
            | Capability::StorageBufferArrayNonUniformIndexingEXT
            | Capability::StorageImageArrayNonUniformIndexingEXT => {
                &[Capability::ShaderNonUniformEXT]
            }
            Capability::ShaderViewportMaskNV => &[Capability::ShaderViewportIndexLayerNV],
            Capability::ShaderStereoViewNV => &[Capability::ShaderViewportMaskNV],
            Capability::UniformAndStorageBuffer16BitAccess => &[
                Capability::StorageBuffer16BitAccess,
                Capability::StorageUniformBufferBlock16,
            ],
            Capability::UniformAndStorageBuffer8BitAccess => &[Capability::StorageBuffer8BitAccess],
            Capability::TessellationPointSize => &[Capability::Tessellation],
            Capability::VariablePointers => &[Capability::VariablePointersStorageBuffer],
        }
    }
}
impl num_traits::FromPrimitive for Capability {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(Capability::Matrix),
            1u32 => Some(Capability::Shader),
            2u32 => Some(Capability::Geometry),
            3u32 => Some(Capability::Tessellation),
            4u32 => Some(Capability::Addresses),
            5u32 => Some(Capability::Linkage),
            6u32 => Some(Capability::Kernel),
            7u32 => Some(Capability::Vector16),
            8u32 => Some(Capability::Float16Buffer),
            9u32 => Some(Capability::Float16),
            10u32 => Some(Capability::Float64),
            11u32 => Some(Capability::Int64),
            12u32 => Some(Capability::Int64Atomics),
            13u32 => Some(Capability::ImageBasic),
            14u32 => Some(Capability::ImageReadWrite),
            15u32 => Some(Capability::ImageMipmap),
            17u32 => Some(Capability::Pipes),
            18u32 => Some(Capability::Groups),
            19u32 => Some(Capability::DeviceEnqueue),
            20u32 => Some(Capability::LiteralSampler),
            21u32 => Some(Capability::AtomicStorage),
            22u32 => Some(Capability::Int16),
            23u32 => Some(Capability::TessellationPointSize),
            24u32 => Some(Capability::GeometryPointSize),
            25u32 => Some(Capability::ImageGatherExtended),
            27u32 => Some(Capability::StorageImageMultisample),
            28u32 => Some(Capability::UniformBufferArrayDynamicIndexing),
            29u32 => Some(Capability::SampledImageArrayDynamicIndexing),
            30u32 => Some(Capability::StorageBufferArrayDynamicIndexing),
            31u32 => Some(Capability::StorageImageArrayDynamicIndexing),
            32u32 => Some(Capability::ClipDistance),
            33u32 => Some(Capability::CullDistance),
            34u32 => Some(Capability::ImageCubeArray),
            35u32 => Some(Capability::SampleRateShading),
            36u32 => Some(Capability::ImageRect),
            37u32 => Some(Capability::SampledRect),
            38u32 => Some(Capability::GenericPointer),
            39u32 => Some(Capability::Int8),
            40u32 => Some(Capability::InputAttachment),
            41u32 => Some(Capability::SparseResidency),
            42u32 => Some(Capability::MinLod),
            43u32 => Some(Capability::Sampled1D),
            44u32 => Some(Capability::Image1D),
            45u32 => Some(Capability::SampledCubeArray),
            46u32 => Some(Capability::SampledBuffer),
            47u32 => Some(Capability::ImageBuffer),
            48u32 => Some(Capability::ImageMSArray),
            49u32 => Some(Capability::StorageImageExtendedFormats),
            50u32 => Some(Capability::ImageQuery),
            51u32 => Some(Capability::DerivativeControl),
            52u32 => Some(Capability::InterpolationFunction),
            53u32 => Some(Capability::TransformFeedback),
            54u32 => Some(Capability::GeometryStreams),
            55u32 => Some(Capability::StorageImageReadWithoutFormat),
            56u32 => Some(Capability::StorageImageWriteWithoutFormat),
            57u32 => Some(Capability::MultiViewport),
            58u32 => Some(Capability::SubgroupDispatch),
            59u32 => Some(Capability::NamedBarrier),
            60u32 => Some(Capability::PipeStorage),
            61u32 => Some(Capability::GroupNonUniform),
            62u32 => Some(Capability::GroupNonUniformVote),
            63u32 => Some(Capability::GroupNonUniformArithmetic),
            64u32 => Some(Capability::GroupNonUniformBallot),
            65u32 => Some(Capability::GroupNonUniformShuffle),
            66u32 => Some(Capability::GroupNonUniformShuffleRelative),
            67u32 => Some(Capability::GroupNonUniformClustered),
            68u32 => Some(Capability::GroupNonUniformQuad),
            4423u32 => Some(Capability::SubgroupBallotKHR),
            4427u32 => Some(Capability::DrawParameters),
            4431u32 => Some(Capability::SubgroupVoteKHR),
            4433u32 => Some(Capability::StorageBuffer16BitAccess),
            4434u32 => Some(Capability::UniformAndStorageBuffer16BitAccess),
            4435u32 => Some(Capability::StoragePushConstant16),
            4436u32 => Some(Capability::StorageInputOutput16),
            4437u32 => Some(Capability::DeviceGroup),
            4439u32 => Some(Capability::MultiView),
            4441u32 => Some(Capability::VariablePointersStorageBuffer),
            4442u32 => Some(Capability::VariablePointers),
            4445u32 => Some(Capability::AtomicStorageOps),
            4447u32 => Some(Capability::SampleMaskPostDepthCoverage),
            4448u32 => Some(Capability::StorageBuffer8BitAccess),
            4449u32 => Some(Capability::UniformAndStorageBuffer8BitAccess),
            4450u32 => Some(Capability::StoragePushConstant8),
            5008u32 => Some(Capability::Float16ImageAMD),
            5009u32 => Some(Capability::ImageGatherBiasLodAMD),
            5010u32 => Some(Capability::FragmentMaskAMD),
            5013u32 => Some(Capability::StencilExportEXT),
            5015u32 => Some(Capability::ImageReadWriteLodAMD),
            5249u32 => Some(Capability::SampleMaskOverrideCoverageNV),
            5251u32 => Some(Capability::GeometryShaderPassthroughNV),
            5254u32 => Some(Capability::ShaderViewportIndexLayerEXT),
            5255u32 => Some(Capability::ShaderViewportMaskNV),
            5259u32 => Some(Capability::ShaderStereoViewNV),
            5260u32 => Some(Capability::PerViewAttributesNV),
            5265u32 => Some(Capability::FragmentFullyCoveredEXT),
            5301u32 => Some(Capability::ShaderNonUniformEXT),
            5302u32 => Some(Capability::RuntimeDescriptorArrayEXT),
            5303u32 => Some(Capability::InputAttachmentArrayDynamicIndexingEXT),
            5304u32 => Some(Capability::UniformTexelBufferArrayDynamicIndexingEXT),
            5305u32 => Some(Capability::StorageTexelBufferArrayDynamicIndexingEXT),
            5306u32 => Some(Capability::UniformBufferArrayNonUniformIndexingEXT),
            5307u32 => Some(Capability::SampledImageArrayNonUniformIndexingEXT),
            5308u32 => Some(Capability::StorageBufferArrayNonUniformIndexingEXT),
            5309u32 => Some(Capability::StorageImageArrayNonUniformIndexingEXT),
            5310u32 => Some(Capability::InputAttachmentArrayNonUniformIndexingEXT),
            5311u32 => Some(Capability::UniformTexelBufferArrayNonUniformIndexingEXT),
            5312u32 => Some(Capability::StorageTexelBufferArrayNonUniformIndexingEXT),
            5568u32 => Some(Capability::SubgroupShuffleINTEL),
            5569u32 => Some(Capability::SubgroupBufferBlockIOINTEL),
            5570u32 => Some(Capability::SubgroupImageBlockIOINTEL),
            5297u32 => Some(Capability::GroupNonUniformPartitionedNV),
            5345u32 => Some(Capability::VulkanMemoryModelKHR),
            5346u32 => Some(Capability::VulkanMemoryModelDeviceScopeKHR),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "SPIR-V [instructions](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_instructions_a_instructions) opcodes"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Nop = 0u32,
    Undef = 1u32,
    SourceContinued = 2u32,
    Source = 3u32,
    SourceExtension = 4u32,
    Name = 5u32,
    MemberName = 6u32,
    String = 7u32,
    Line = 8u32,
    Extension = 10u32,
    ExtInstImport = 11u32,
    ExtInst = 12u32,
    MemoryModel = 14u32,
    EntryPoint = 15u32,
    ExecutionMode = 16u32,
    Capability = 17u32,
    TypeVoid = 19u32,
    TypeBool = 20u32,
    TypeInt = 21u32,
    TypeFloat = 22u32,
    TypeVector = 23u32,
    TypeMatrix = 24u32,
    TypeImage = 25u32,
    TypeSampler = 26u32,
    TypeSampledImage = 27u32,
    TypeArray = 28u32,
    TypeRuntimeArray = 29u32,
    TypeStruct = 30u32,
    TypeOpaque = 31u32,
    TypePointer = 32u32,
    TypeFunction = 33u32,
    TypeEvent = 34u32,
    TypeDeviceEvent = 35u32,
    TypeReserveId = 36u32,
    TypeQueue = 37u32,
    TypePipe = 38u32,
    TypeForwardPointer = 39u32,
    ConstantTrue = 41u32,
    ConstantFalse = 42u32,
    Constant = 43u32,
    ConstantComposite = 44u32,
    ConstantSampler = 45u32,
    ConstantNull = 46u32,
    SpecConstantTrue = 48u32,
    SpecConstantFalse = 49u32,
    SpecConstant = 50u32,
    SpecConstantComposite = 51u32,
    SpecConstantOp = 52u32,
    Function = 54u32,
    FunctionParameter = 55u32,
    FunctionEnd = 56u32,
    FunctionCall = 57u32,
    Variable = 59u32,
    ImageTexelPointer = 60u32,
    Load = 61u32,
    Store = 62u32,
    CopyMemory = 63u32,
    CopyMemorySized = 64u32,
    AccessChain = 65u32,
    InBoundsAccessChain = 66u32,
    PtrAccessChain = 67u32,
    ArrayLength = 68u32,
    GenericPtrMemSemantics = 69u32,
    InBoundsPtrAccessChain = 70u32,
    Decorate = 71u32,
    MemberDecorate = 72u32,
    DecorationGroup = 73u32,
    GroupDecorate = 74u32,
    GroupMemberDecorate = 75u32,
    VectorExtractDynamic = 77u32,
    VectorInsertDynamic = 78u32,
    VectorShuffle = 79u32,
    CompositeConstruct = 80u32,
    CompositeExtract = 81u32,
    CompositeInsert = 82u32,
    CopyObject = 83u32,
    Transpose = 84u32,
    SampledImage = 86u32,
    ImageSampleImplicitLod = 87u32,
    ImageSampleExplicitLod = 88u32,
    ImageSampleDrefImplicitLod = 89u32,
    ImageSampleDrefExplicitLod = 90u32,
    ImageSampleProjImplicitLod = 91u32,
    ImageSampleProjExplicitLod = 92u32,
    ImageSampleProjDrefImplicitLod = 93u32,
    ImageSampleProjDrefExplicitLod = 94u32,
    ImageFetch = 95u32,
    ImageGather = 96u32,
    ImageDrefGather = 97u32,
    ImageRead = 98u32,
    ImageWrite = 99u32,
    Image = 100u32,
    ImageQueryFormat = 101u32,
    ImageQueryOrder = 102u32,
    ImageQuerySizeLod = 103u32,
    ImageQuerySize = 104u32,
    ImageQueryLod = 105u32,
    ImageQueryLevels = 106u32,
    ImageQuerySamples = 107u32,
    ConvertFToU = 109u32,
    ConvertFToS = 110u32,
    ConvertSToF = 111u32,
    ConvertUToF = 112u32,
    UConvert = 113u32,
    SConvert = 114u32,
    FConvert = 115u32,
    QuantizeToF16 = 116u32,
    ConvertPtrToU = 117u32,
    SatConvertSToU = 118u32,
    SatConvertUToS = 119u32,
    ConvertUToPtr = 120u32,
    PtrCastToGeneric = 121u32,
    GenericCastToPtr = 122u32,
    GenericCastToPtrExplicit = 123u32,
    Bitcast = 124u32,
    SNegate = 126u32,
    FNegate = 127u32,
    IAdd = 128u32,
    FAdd = 129u32,
    ISub = 130u32,
    FSub = 131u32,
    IMul = 132u32,
    FMul = 133u32,
    UDiv = 134u32,
    SDiv = 135u32,
    FDiv = 136u32,
    UMod = 137u32,
    SRem = 138u32,
    SMod = 139u32,
    FRem = 140u32,
    FMod = 141u32,
    VectorTimesScalar = 142u32,
    MatrixTimesScalar = 143u32,
    VectorTimesMatrix = 144u32,
    MatrixTimesVector = 145u32,
    MatrixTimesMatrix = 146u32,
    OuterProduct = 147u32,
    Dot = 148u32,
    IAddCarry = 149u32,
    ISubBorrow = 150u32,
    UMulExtended = 151u32,
    SMulExtended = 152u32,
    Any = 154u32,
    All = 155u32,
    IsNan = 156u32,
    IsInf = 157u32,
    IsFinite = 158u32,
    IsNormal = 159u32,
    SignBitSet = 160u32,
    LessOrGreater = 161u32,
    Ordered = 162u32,
    Unordered = 163u32,
    LogicalEqual = 164u32,
    LogicalNotEqual = 165u32,
    LogicalOr = 166u32,
    LogicalAnd = 167u32,
    LogicalNot = 168u32,
    Select = 169u32,
    IEqual = 170u32,
    INotEqual = 171u32,
    UGreaterThan = 172u32,
    SGreaterThan = 173u32,
    UGreaterThanEqual = 174u32,
    SGreaterThanEqual = 175u32,
    ULessThan = 176u32,
    SLessThan = 177u32,
    ULessThanEqual = 178u32,
    SLessThanEqual = 179u32,
    FOrdEqual = 180u32,
    FUnordEqual = 181u32,
    FOrdNotEqual = 182u32,
    FUnordNotEqual = 183u32,
    FOrdLessThan = 184u32,
    FUnordLessThan = 185u32,
    FOrdGreaterThan = 186u32,
    FUnordGreaterThan = 187u32,
    FOrdLessThanEqual = 188u32,
    FUnordLessThanEqual = 189u32,
    FOrdGreaterThanEqual = 190u32,
    FUnordGreaterThanEqual = 191u32,
    ShiftRightLogical = 194u32,
    ShiftRightArithmetic = 195u32,
    ShiftLeftLogical = 196u32,
    BitwiseOr = 197u32,
    BitwiseXor = 198u32,
    BitwiseAnd = 199u32,
    Not = 200u32,
    BitFieldInsert = 201u32,
    BitFieldSExtract = 202u32,
    BitFieldUExtract = 203u32,
    BitReverse = 204u32,
    BitCount = 205u32,
    DPdx = 207u32,
    DPdy = 208u32,
    Fwidth = 209u32,
    DPdxFine = 210u32,
    DPdyFine = 211u32,
    FwidthFine = 212u32,
    DPdxCoarse = 213u32,
    DPdyCoarse = 214u32,
    FwidthCoarse = 215u32,
    EmitVertex = 218u32,
    EndPrimitive = 219u32,
    EmitStreamVertex = 220u32,
    EndStreamPrimitive = 221u32,
    ControlBarrier = 224u32,
    MemoryBarrier = 225u32,
    AtomicLoad = 227u32,
    AtomicStore = 228u32,
    AtomicExchange = 229u32,
    AtomicCompareExchange = 230u32,
    AtomicCompareExchangeWeak = 231u32,
    AtomicIIncrement = 232u32,
    AtomicIDecrement = 233u32,
    AtomicIAdd = 234u32,
    AtomicISub = 235u32,
    AtomicSMin = 236u32,
    AtomicUMin = 237u32,
    AtomicSMax = 238u32,
    AtomicUMax = 239u32,
    AtomicAnd = 240u32,
    AtomicOr = 241u32,
    AtomicXor = 242u32,
    Phi = 245u32,
    LoopMerge = 246u32,
    SelectionMerge = 247u32,
    Label = 248u32,
    Branch = 249u32,
    BranchConditional = 250u32,
    Switch = 251u32,
    Kill = 252u32,
    Return = 253u32,
    ReturnValue = 254u32,
    Unreachable = 255u32,
    LifetimeStart = 256u32,
    LifetimeStop = 257u32,
    GroupAsyncCopy = 259u32,
    GroupWaitEvents = 260u32,
    GroupAll = 261u32,
    GroupAny = 262u32,
    GroupBroadcast = 263u32,
    GroupIAdd = 264u32,
    GroupFAdd = 265u32,
    GroupFMin = 266u32,
    GroupUMin = 267u32,
    GroupSMin = 268u32,
    GroupFMax = 269u32,
    GroupUMax = 270u32,
    GroupSMax = 271u32,
    ReadPipe = 274u32,
    WritePipe = 275u32,
    ReservedReadPipe = 276u32,
    ReservedWritePipe = 277u32,
    ReserveReadPipePackets = 278u32,
    ReserveWritePipePackets = 279u32,
    CommitReadPipe = 280u32,
    CommitWritePipe = 281u32,
    IsValidReserveId = 282u32,
    GetNumPipePackets = 283u32,
    GetMaxPipePackets = 284u32,
    GroupReserveReadPipePackets = 285u32,
    GroupReserveWritePipePackets = 286u32,
    GroupCommitReadPipe = 287u32,
    GroupCommitWritePipe = 288u32,
    EnqueueMarker = 291u32,
    EnqueueKernel = 292u32,
    GetKernelNDrangeSubGroupCount = 293u32,
    GetKernelNDrangeMaxSubGroupSize = 294u32,
    GetKernelWorkGroupSize = 295u32,
    GetKernelPreferredWorkGroupSizeMultiple = 296u32,
    RetainEvent = 297u32,
    ReleaseEvent = 298u32,
    CreateUserEvent = 299u32,
    IsValidEvent = 300u32,
    SetUserEventStatus = 301u32,
    CaptureEventProfilingInfo = 302u32,
    GetDefaultQueue = 303u32,
    BuildNDRange = 304u32,
    ImageSparseSampleImplicitLod = 305u32,
    ImageSparseSampleExplicitLod = 306u32,
    ImageSparseSampleDrefImplicitLod = 307u32,
    ImageSparseSampleDrefExplicitLod = 308u32,
    ImageSparseSampleProjImplicitLod = 309u32,
    ImageSparseSampleProjExplicitLod = 310u32,
    ImageSparseSampleProjDrefImplicitLod = 311u32,
    ImageSparseSampleProjDrefExplicitLod = 312u32,
    ImageSparseFetch = 313u32,
    ImageSparseGather = 314u32,
    ImageSparseDrefGather = 315u32,
    ImageSparseTexelsResident = 316u32,
    NoLine = 317u32,
    AtomicFlagTestAndSet = 318u32,
    AtomicFlagClear = 319u32,
    ImageSparseRead = 320u32,
    SizeOf = 321u32,
    TypePipeStorage = 322u32,
    ConstantPipeStorage = 323u32,
    CreatePipeFromPipeStorage = 324u32,
    GetKernelLocalSizeForSubgroupCount = 325u32,
    GetKernelMaxNumSubgroups = 326u32,
    TypeNamedBarrier = 327u32,
    NamedBarrierInitialize = 328u32,
    MemoryNamedBarrier = 329u32,
    ModuleProcessed = 330u32,
    ExecutionModeId = 331u32,
    DecorateId = 332u32,
    GroupNonUniformElect = 333u32,
    GroupNonUniformAll = 334u32,
    GroupNonUniformAny = 335u32,
    GroupNonUniformAllEqual = 336u32,
    GroupNonUniformBroadcast = 337u32,
    GroupNonUniformBroadcastFirst = 338u32,
    GroupNonUniformBallot = 339u32,
    GroupNonUniformInverseBallot = 340u32,
    GroupNonUniformBallotBitExtract = 341u32,
    GroupNonUniformBallotBitCount = 342u32,
    GroupNonUniformBallotFindLSB = 343u32,
    GroupNonUniformBallotFindMSB = 344u32,
    GroupNonUniformShuffle = 345u32,
    GroupNonUniformShuffleXor = 346u32,
    GroupNonUniformShuffleUp = 347u32,
    GroupNonUniformShuffleDown = 348u32,
    GroupNonUniformIAdd = 349u32,
    GroupNonUniformFAdd = 350u32,
    GroupNonUniformIMul = 351u32,
    GroupNonUniformFMul = 352u32,
    GroupNonUniformSMin = 353u32,
    GroupNonUniformUMin = 354u32,
    GroupNonUniformFMin = 355u32,
    GroupNonUniformSMax = 356u32,
    GroupNonUniformUMax = 357u32,
    GroupNonUniformFMax = 358u32,
    GroupNonUniformBitwiseAnd = 359u32,
    GroupNonUniformBitwiseOr = 360u32,
    GroupNonUniformBitwiseXor = 361u32,
    GroupNonUniformLogicalAnd = 362u32,
    GroupNonUniformLogicalOr = 363u32,
    GroupNonUniformLogicalXor = 364u32,
    GroupNonUniformQuadBroadcast = 365u32,
    GroupNonUniformQuadSwap = 366u32,
    SubgroupBallotKHR = 4421u32,
    SubgroupFirstInvocationKHR = 4422u32,
    SubgroupAllKHR = 4428u32,
    SubgroupAnyKHR = 4429u32,
    SubgroupAllEqualKHR = 4430u32,
    SubgroupReadInvocationKHR = 4432u32,
    GroupIAddNonUniformAMD = 5000u32,
    GroupFAddNonUniformAMD = 5001u32,
    GroupFMinNonUniformAMD = 5002u32,
    GroupUMinNonUniformAMD = 5003u32,
    GroupSMinNonUniformAMD = 5004u32,
    GroupFMaxNonUniformAMD = 5005u32,
    GroupUMaxNonUniformAMD = 5006u32,
    GroupSMaxNonUniformAMD = 5007u32,
    FragmentMaskFetchAMD = 5011u32,
    FragmentFetchAMD = 5012u32,
    SubgroupShuffleINTEL = 5571u32,
    SubgroupShuffleDownINTEL = 5572u32,
    SubgroupShuffleUpINTEL = 5573u32,
    SubgroupShuffleXorINTEL = 5574u32,
    SubgroupBlockReadINTEL = 5575u32,
    SubgroupBlockWriteINTEL = 5576u32,
    SubgroupImageBlockReadINTEL = 5577u32,
    SubgroupImageBlockWriteINTEL = 5578u32,
    DecorateStringGOOGLE = 5632u32,
    MemberDecorateStringGOOGLE = 5633u32,
    GroupNonUniformPartitionNV = 5296u32,
}
impl num_traits::FromPrimitive for Op {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(Op::Nop),
            1u32 => Some(Op::Undef),
            2u32 => Some(Op::SourceContinued),
            3u32 => Some(Op::Source),
            4u32 => Some(Op::SourceExtension),
            5u32 => Some(Op::Name),
            6u32 => Some(Op::MemberName),
            7u32 => Some(Op::String),
            8u32 => Some(Op::Line),
            10u32 => Some(Op::Extension),
            11u32 => Some(Op::ExtInstImport),
            12u32 => Some(Op::ExtInst),
            14u32 => Some(Op::MemoryModel),
            15u32 => Some(Op::EntryPoint),
            16u32 => Some(Op::ExecutionMode),
            17u32 => Some(Op::Capability),
            19u32 => Some(Op::TypeVoid),
            20u32 => Some(Op::TypeBool),
            21u32 => Some(Op::TypeInt),
            22u32 => Some(Op::TypeFloat),
            23u32 => Some(Op::TypeVector),
            24u32 => Some(Op::TypeMatrix),
            25u32 => Some(Op::TypeImage),
            26u32 => Some(Op::TypeSampler),
            27u32 => Some(Op::TypeSampledImage),
            28u32 => Some(Op::TypeArray),
            29u32 => Some(Op::TypeRuntimeArray),
            30u32 => Some(Op::TypeStruct),
            31u32 => Some(Op::TypeOpaque),
            32u32 => Some(Op::TypePointer),
            33u32 => Some(Op::TypeFunction),
            34u32 => Some(Op::TypeEvent),
            35u32 => Some(Op::TypeDeviceEvent),
            36u32 => Some(Op::TypeReserveId),
            37u32 => Some(Op::TypeQueue),
            38u32 => Some(Op::TypePipe),
            39u32 => Some(Op::TypeForwardPointer),
            41u32 => Some(Op::ConstantTrue),
            42u32 => Some(Op::ConstantFalse),
            43u32 => Some(Op::Constant),
            44u32 => Some(Op::ConstantComposite),
            45u32 => Some(Op::ConstantSampler),
            46u32 => Some(Op::ConstantNull),
            48u32 => Some(Op::SpecConstantTrue),
            49u32 => Some(Op::SpecConstantFalse),
            50u32 => Some(Op::SpecConstant),
            51u32 => Some(Op::SpecConstantComposite),
            52u32 => Some(Op::SpecConstantOp),
            54u32 => Some(Op::Function),
            55u32 => Some(Op::FunctionParameter),
            56u32 => Some(Op::FunctionEnd),
            57u32 => Some(Op::FunctionCall),
            59u32 => Some(Op::Variable),
            60u32 => Some(Op::ImageTexelPointer),
            61u32 => Some(Op::Load),
            62u32 => Some(Op::Store),
            63u32 => Some(Op::CopyMemory),
            64u32 => Some(Op::CopyMemorySized),
            65u32 => Some(Op::AccessChain),
            66u32 => Some(Op::InBoundsAccessChain),
            67u32 => Some(Op::PtrAccessChain),
            68u32 => Some(Op::ArrayLength),
            69u32 => Some(Op::GenericPtrMemSemantics),
            70u32 => Some(Op::InBoundsPtrAccessChain),
            71u32 => Some(Op::Decorate),
            72u32 => Some(Op::MemberDecorate),
            73u32 => Some(Op::DecorationGroup),
            74u32 => Some(Op::GroupDecorate),
            75u32 => Some(Op::GroupMemberDecorate),
            77u32 => Some(Op::VectorExtractDynamic),
            78u32 => Some(Op::VectorInsertDynamic),
            79u32 => Some(Op::VectorShuffle),
            80u32 => Some(Op::CompositeConstruct),
            81u32 => Some(Op::CompositeExtract),
            82u32 => Some(Op::CompositeInsert),
            83u32 => Some(Op::CopyObject),
            84u32 => Some(Op::Transpose),
            86u32 => Some(Op::SampledImage),
            87u32 => Some(Op::ImageSampleImplicitLod),
            88u32 => Some(Op::ImageSampleExplicitLod),
            89u32 => Some(Op::ImageSampleDrefImplicitLod),
            90u32 => Some(Op::ImageSampleDrefExplicitLod),
            91u32 => Some(Op::ImageSampleProjImplicitLod),
            92u32 => Some(Op::ImageSampleProjExplicitLod),
            93u32 => Some(Op::ImageSampleProjDrefImplicitLod),
            94u32 => Some(Op::ImageSampleProjDrefExplicitLod),
            95u32 => Some(Op::ImageFetch),
            96u32 => Some(Op::ImageGather),
            97u32 => Some(Op::ImageDrefGather),
            98u32 => Some(Op::ImageRead),
            99u32 => Some(Op::ImageWrite),
            100u32 => Some(Op::Image),
            101u32 => Some(Op::ImageQueryFormat),
            102u32 => Some(Op::ImageQueryOrder),
            103u32 => Some(Op::ImageQuerySizeLod),
            104u32 => Some(Op::ImageQuerySize),
            105u32 => Some(Op::ImageQueryLod),
            106u32 => Some(Op::ImageQueryLevels),
            107u32 => Some(Op::ImageQuerySamples),
            109u32 => Some(Op::ConvertFToU),
            110u32 => Some(Op::ConvertFToS),
            111u32 => Some(Op::ConvertSToF),
            112u32 => Some(Op::ConvertUToF),
            113u32 => Some(Op::UConvert),
            114u32 => Some(Op::SConvert),
            115u32 => Some(Op::FConvert),
            116u32 => Some(Op::QuantizeToF16),
            117u32 => Some(Op::ConvertPtrToU),
            118u32 => Some(Op::SatConvertSToU),
            119u32 => Some(Op::SatConvertUToS),
            120u32 => Some(Op::ConvertUToPtr),
            121u32 => Some(Op::PtrCastToGeneric),
            122u32 => Some(Op::GenericCastToPtr),
            123u32 => Some(Op::GenericCastToPtrExplicit),
            124u32 => Some(Op::Bitcast),
            126u32 => Some(Op::SNegate),
            127u32 => Some(Op::FNegate),
            128u32 => Some(Op::IAdd),
            129u32 => Some(Op::FAdd),
            130u32 => Some(Op::ISub),
            131u32 => Some(Op::FSub),
            132u32 => Some(Op::IMul),
            133u32 => Some(Op::FMul),
            134u32 => Some(Op::UDiv),
            135u32 => Some(Op::SDiv),
            136u32 => Some(Op::FDiv),
            137u32 => Some(Op::UMod),
            138u32 => Some(Op::SRem),
            139u32 => Some(Op::SMod),
            140u32 => Some(Op::FRem),
            141u32 => Some(Op::FMod),
            142u32 => Some(Op::VectorTimesScalar),
            143u32 => Some(Op::MatrixTimesScalar),
            144u32 => Some(Op::VectorTimesMatrix),
            145u32 => Some(Op::MatrixTimesVector),
            146u32 => Some(Op::MatrixTimesMatrix),
            147u32 => Some(Op::OuterProduct),
            148u32 => Some(Op::Dot),
            149u32 => Some(Op::IAddCarry),
            150u32 => Some(Op::ISubBorrow),
            151u32 => Some(Op::UMulExtended),
            152u32 => Some(Op::SMulExtended),
            154u32 => Some(Op::Any),
            155u32 => Some(Op::All),
            156u32 => Some(Op::IsNan),
            157u32 => Some(Op::IsInf),
            158u32 => Some(Op::IsFinite),
            159u32 => Some(Op::IsNormal),
            160u32 => Some(Op::SignBitSet),
            161u32 => Some(Op::LessOrGreater),
            162u32 => Some(Op::Ordered),
            163u32 => Some(Op::Unordered),
            164u32 => Some(Op::LogicalEqual),
            165u32 => Some(Op::LogicalNotEqual),
            166u32 => Some(Op::LogicalOr),
            167u32 => Some(Op::LogicalAnd),
            168u32 => Some(Op::LogicalNot),
            169u32 => Some(Op::Select),
            170u32 => Some(Op::IEqual),
            171u32 => Some(Op::INotEqual),
            172u32 => Some(Op::UGreaterThan),
            173u32 => Some(Op::SGreaterThan),
            174u32 => Some(Op::UGreaterThanEqual),
            175u32 => Some(Op::SGreaterThanEqual),
            176u32 => Some(Op::ULessThan),
            177u32 => Some(Op::SLessThan),
            178u32 => Some(Op::ULessThanEqual),
            179u32 => Some(Op::SLessThanEqual),
            180u32 => Some(Op::FOrdEqual),
            181u32 => Some(Op::FUnordEqual),
            182u32 => Some(Op::FOrdNotEqual),
            183u32 => Some(Op::FUnordNotEqual),
            184u32 => Some(Op::FOrdLessThan),
            185u32 => Some(Op::FUnordLessThan),
            186u32 => Some(Op::FOrdGreaterThan),
            187u32 => Some(Op::FUnordGreaterThan),
            188u32 => Some(Op::FOrdLessThanEqual),
            189u32 => Some(Op::FUnordLessThanEqual),
            190u32 => Some(Op::FOrdGreaterThanEqual),
            191u32 => Some(Op::FUnordGreaterThanEqual),
            194u32 => Some(Op::ShiftRightLogical),
            195u32 => Some(Op::ShiftRightArithmetic),
            196u32 => Some(Op::ShiftLeftLogical),
            197u32 => Some(Op::BitwiseOr),
            198u32 => Some(Op::BitwiseXor),
            199u32 => Some(Op::BitwiseAnd),
            200u32 => Some(Op::Not),
            201u32 => Some(Op::BitFieldInsert),
            202u32 => Some(Op::BitFieldSExtract),
            203u32 => Some(Op::BitFieldUExtract),
            204u32 => Some(Op::BitReverse),
            205u32 => Some(Op::BitCount),
            207u32 => Some(Op::DPdx),
            208u32 => Some(Op::DPdy),
            209u32 => Some(Op::Fwidth),
            210u32 => Some(Op::DPdxFine),
            211u32 => Some(Op::DPdyFine),
            212u32 => Some(Op::FwidthFine),
            213u32 => Some(Op::DPdxCoarse),
            214u32 => Some(Op::DPdyCoarse),
            215u32 => Some(Op::FwidthCoarse),
            218u32 => Some(Op::EmitVertex),
            219u32 => Some(Op::EndPrimitive),
            220u32 => Some(Op::EmitStreamVertex),
            221u32 => Some(Op::EndStreamPrimitive),
            224u32 => Some(Op::ControlBarrier),
            225u32 => Some(Op::MemoryBarrier),
            227u32 => Some(Op::AtomicLoad),
            228u32 => Some(Op::AtomicStore),
            229u32 => Some(Op::AtomicExchange),
            230u32 => Some(Op::AtomicCompareExchange),
            231u32 => Some(Op::AtomicCompareExchangeWeak),
            232u32 => Some(Op::AtomicIIncrement),
            233u32 => Some(Op::AtomicIDecrement),
            234u32 => Some(Op::AtomicIAdd),
            235u32 => Some(Op::AtomicISub),
            236u32 => Some(Op::AtomicSMin),
            237u32 => Some(Op::AtomicUMin),
            238u32 => Some(Op::AtomicSMax),
            239u32 => Some(Op::AtomicUMax),
            240u32 => Some(Op::AtomicAnd),
            241u32 => Some(Op::AtomicOr),
            242u32 => Some(Op::AtomicXor),
            245u32 => Some(Op::Phi),
            246u32 => Some(Op::LoopMerge),
            247u32 => Some(Op::SelectionMerge),
            248u32 => Some(Op::Label),
            249u32 => Some(Op::Branch),
            250u32 => Some(Op::BranchConditional),
            251u32 => Some(Op::Switch),
            252u32 => Some(Op::Kill),
            253u32 => Some(Op::Return),
            254u32 => Some(Op::ReturnValue),
            255u32 => Some(Op::Unreachable),
            256u32 => Some(Op::LifetimeStart),
            257u32 => Some(Op::LifetimeStop),
            259u32 => Some(Op::GroupAsyncCopy),
            260u32 => Some(Op::GroupWaitEvents),
            261u32 => Some(Op::GroupAll),
            262u32 => Some(Op::GroupAny),
            263u32 => Some(Op::GroupBroadcast),
            264u32 => Some(Op::GroupIAdd),
            265u32 => Some(Op::GroupFAdd),
            266u32 => Some(Op::GroupFMin),
            267u32 => Some(Op::GroupUMin),
            268u32 => Some(Op::GroupSMin),
            269u32 => Some(Op::GroupFMax),
            270u32 => Some(Op::GroupUMax),
            271u32 => Some(Op::GroupSMax),
            274u32 => Some(Op::ReadPipe),
            275u32 => Some(Op::WritePipe),
            276u32 => Some(Op::ReservedReadPipe),
            277u32 => Some(Op::ReservedWritePipe),
            278u32 => Some(Op::ReserveReadPipePackets),
            279u32 => Some(Op::ReserveWritePipePackets),
            280u32 => Some(Op::CommitReadPipe),
            281u32 => Some(Op::CommitWritePipe),
            282u32 => Some(Op::IsValidReserveId),
            283u32 => Some(Op::GetNumPipePackets),
            284u32 => Some(Op::GetMaxPipePackets),
            285u32 => Some(Op::GroupReserveReadPipePackets),
            286u32 => Some(Op::GroupReserveWritePipePackets),
            287u32 => Some(Op::GroupCommitReadPipe),
            288u32 => Some(Op::GroupCommitWritePipe),
            291u32 => Some(Op::EnqueueMarker),
            292u32 => Some(Op::EnqueueKernel),
            293u32 => Some(Op::GetKernelNDrangeSubGroupCount),
            294u32 => Some(Op::GetKernelNDrangeMaxSubGroupSize),
            295u32 => Some(Op::GetKernelWorkGroupSize),
            296u32 => Some(Op::GetKernelPreferredWorkGroupSizeMultiple),
            297u32 => Some(Op::RetainEvent),
            298u32 => Some(Op::ReleaseEvent),
            299u32 => Some(Op::CreateUserEvent),
            300u32 => Some(Op::IsValidEvent),
            301u32 => Some(Op::SetUserEventStatus),
            302u32 => Some(Op::CaptureEventProfilingInfo),
            303u32 => Some(Op::GetDefaultQueue),
            304u32 => Some(Op::BuildNDRange),
            305u32 => Some(Op::ImageSparseSampleImplicitLod),
            306u32 => Some(Op::ImageSparseSampleExplicitLod),
            307u32 => Some(Op::ImageSparseSampleDrefImplicitLod),
            308u32 => Some(Op::ImageSparseSampleDrefExplicitLod),
            309u32 => Some(Op::ImageSparseSampleProjImplicitLod),
            310u32 => Some(Op::ImageSparseSampleProjExplicitLod),
            311u32 => Some(Op::ImageSparseSampleProjDrefImplicitLod),
            312u32 => Some(Op::ImageSparseSampleProjDrefExplicitLod),
            313u32 => Some(Op::ImageSparseFetch),
            314u32 => Some(Op::ImageSparseGather),
            315u32 => Some(Op::ImageSparseDrefGather),
            316u32 => Some(Op::ImageSparseTexelsResident),
            317u32 => Some(Op::NoLine),
            318u32 => Some(Op::AtomicFlagTestAndSet),
            319u32 => Some(Op::AtomicFlagClear),
            320u32 => Some(Op::ImageSparseRead),
            321u32 => Some(Op::SizeOf),
            322u32 => Some(Op::TypePipeStorage),
            323u32 => Some(Op::ConstantPipeStorage),
            324u32 => Some(Op::CreatePipeFromPipeStorage),
            325u32 => Some(Op::GetKernelLocalSizeForSubgroupCount),
            326u32 => Some(Op::GetKernelMaxNumSubgroups),
            327u32 => Some(Op::TypeNamedBarrier),
            328u32 => Some(Op::NamedBarrierInitialize),
            329u32 => Some(Op::MemoryNamedBarrier),
            330u32 => Some(Op::ModuleProcessed),
            331u32 => Some(Op::ExecutionModeId),
            332u32 => Some(Op::DecorateId),
            333u32 => Some(Op::GroupNonUniformElect),
            334u32 => Some(Op::GroupNonUniformAll),
            335u32 => Some(Op::GroupNonUniformAny),
            336u32 => Some(Op::GroupNonUniformAllEqual),
            337u32 => Some(Op::GroupNonUniformBroadcast),
            338u32 => Some(Op::GroupNonUniformBroadcastFirst),
            339u32 => Some(Op::GroupNonUniformBallot),
            340u32 => Some(Op::GroupNonUniformInverseBallot),
            341u32 => Some(Op::GroupNonUniformBallotBitExtract),
            342u32 => Some(Op::GroupNonUniformBallotBitCount),
            343u32 => Some(Op::GroupNonUniformBallotFindLSB),
            344u32 => Some(Op::GroupNonUniformBallotFindMSB),
            345u32 => Some(Op::GroupNonUniformShuffle),
            346u32 => Some(Op::GroupNonUniformShuffleXor),
            347u32 => Some(Op::GroupNonUniformShuffleUp),
            348u32 => Some(Op::GroupNonUniformShuffleDown),
            349u32 => Some(Op::GroupNonUniformIAdd),
            350u32 => Some(Op::GroupNonUniformFAdd),
            351u32 => Some(Op::GroupNonUniformIMul),
            352u32 => Some(Op::GroupNonUniformFMul),
            353u32 => Some(Op::GroupNonUniformSMin),
            354u32 => Some(Op::GroupNonUniformUMin),
            355u32 => Some(Op::GroupNonUniformFMin),
            356u32 => Some(Op::GroupNonUniformSMax),
            357u32 => Some(Op::GroupNonUniformUMax),
            358u32 => Some(Op::GroupNonUniformFMax),
            359u32 => Some(Op::GroupNonUniformBitwiseAnd),
            360u32 => Some(Op::GroupNonUniformBitwiseOr),
            361u32 => Some(Op::GroupNonUniformBitwiseXor),
            362u32 => Some(Op::GroupNonUniformLogicalAnd),
            363u32 => Some(Op::GroupNonUniformLogicalOr),
            364u32 => Some(Op::GroupNonUniformLogicalXor),
            365u32 => Some(Op::GroupNonUniformQuadBroadcast),
            366u32 => Some(Op::GroupNonUniformQuadSwap),
            4421u32 => Some(Op::SubgroupBallotKHR),
            4422u32 => Some(Op::SubgroupFirstInvocationKHR),
            4428u32 => Some(Op::SubgroupAllKHR),
            4429u32 => Some(Op::SubgroupAnyKHR),
            4430u32 => Some(Op::SubgroupAllEqualKHR),
            4432u32 => Some(Op::SubgroupReadInvocationKHR),
            5000u32 => Some(Op::GroupIAddNonUniformAMD),
            5001u32 => Some(Op::GroupFAddNonUniformAMD),
            5002u32 => Some(Op::GroupFMinNonUniformAMD),
            5003u32 => Some(Op::GroupUMinNonUniformAMD),
            5004u32 => Some(Op::GroupSMinNonUniformAMD),
            5005u32 => Some(Op::GroupFMaxNonUniformAMD),
            5006u32 => Some(Op::GroupUMaxNonUniformAMD),
            5007u32 => Some(Op::GroupSMaxNonUniformAMD),
            5011u32 => Some(Op::FragmentMaskFetchAMD),
            5012u32 => Some(Op::FragmentFetchAMD),
            5571u32 => Some(Op::SubgroupShuffleINTEL),
            5572u32 => Some(Op::SubgroupShuffleDownINTEL),
            5573u32 => Some(Op::SubgroupShuffleUpINTEL),
            5574u32 => Some(Op::SubgroupShuffleXorINTEL),
            5575u32 => Some(Op::SubgroupBlockReadINTEL),
            5576u32 => Some(Op::SubgroupBlockWriteINTEL),
            5577u32 => Some(Op::SubgroupImageBlockReadINTEL),
            5578u32 => Some(Op::SubgroupImageBlockWriteINTEL),
            5632u32 => Some(Op::DecorateStringGOOGLE),
            5633u32 => Some(Op::MemberDecorateStringGOOGLE),
            5296u32 => Some(Op::GroupNonUniformPartitionNV),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "[GLSL.std.450](https://www.khronos.org/registry/spir-v/specs/unified1/GLSL.std.450.html) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GLOp {
    Round = 1u32,
    RoundEven = 2u32,
    Trunc = 3u32,
    FAbs = 4u32,
    SAbs = 5u32,
    FSign = 6u32,
    SSign = 7u32,
    Floor = 8u32,
    Ceil = 9u32,
    Fract = 10u32,
    Radians = 11u32,
    Degrees = 12u32,
    Sin = 13u32,
    Cos = 14u32,
    Tan = 15u32,
    Asin = 16u32,
    Acos = 17u32,
    Atan = 18u32,
    Sinh = 19u32,
    Cosh = 20u32,
    Tanh = 21u32,
    Asinh = 22u32,
    Acosh = 23u32,
    Atanh = 24u32,
    Atan2 = 25u32,
    Pow = 26u32,
    Exp = 27u32,
    Log = 28u32,
    Exp2 = 29u32,
    Log2 = 30u32,
    Sqrt = 31u32,
    InverseSqrt = 32u32,
    Determinant = 33u32,
    MatrixInverse = 34u32,
    Modf = 35u32,
    ModfStruct = 36u32,
    FMin = 37u32,
    UMin = 38u32,
    SMin = 39u32,
    FMax = 40u32,
    UMax = 41u32,
    SMax = 42u32,
    FClamp = 43u32,
    UClamp = 44u32,
    SClamp = 45u32,
    FMix = 46u32,
    IMix = 47u32,
    Step = 48u32,
    SmoothStep = 49u32,
    Fma = 50u32,
    Frexp = 51u32,
    FrexpStruct = 52u32,
    Ldexp = 53u32,
    PackSnorm4x8 = 54u32,
    PackUnorm4x8 = 55u32,
    PackSnorm2x16 = 56u32,
    PackUnorm2x16 = 57u32,
    PackHalf2x16 = 58u32,
    PackDouble2x32 = 59u32,
    UnpackSnorm2x16 = 60u32,
    UnpackUnorm2x16 = 61u32,
    UnpackHalf2x16 = 62u32,
    UnpackSnorm4x8 = 63u32,
    UnpackUnorm4x8 = 64u32,
    UnpackDouble2x32 = 65u32,
    Length = 66u32,
    Distance = 67u32,
    Cross = 68u32,
    Normalize = 69u32,
    FaceForward = 70u32,
    Reflect = 71u32,
    Refract = 72u32,
    FindILsb = 73u32,
    FindSMsb = 74u32,
    FindUMsb = 75u32,
    InterpolateAtCentroid = 76u32,
    InterpolateAtSample = 77u32,
    InterpolateAtOffset = 78u32,
    NMin = 79u32,
    NMax = 80u32,
    NClamp = 81u32,
}
impl num_traits::FromPrimitive for GLOp {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            1u32 => Some(GLOp::Round),
            2u32 => Some(GLOp::RoundEven),
            3u32 => Some(GLOp::Trunc),
            4u32 => Some(GLOp::FAbs),
            5u32 => Some(GLOp::SAbs),
            6u32 => Some(GLOp::FSign),
            7u32 => Some(GLOp::SSign),
            8u32 => Some(GLOp::Floor),
            9u32 => Some(GLOp::Ceil),
            10u32 => Some(GLOp::Fract),
            11u32 => Some(GLOp::Radians),
            12u32 => Some(GLOp::Degrees),
            13u32 => Some(GLOp::Sin),
            14u32 => Some(GLOp::Cos),
            15u32 => Some(GLOp::Tan),
            16u32 => Some(GLOp::Asin),
            17u32 => Some(GLOp::Acos),
            18u32 => Some(GLOp::Atan),
            19u32 => Some(GLOp::Sinh),
            20u32 => Some(GLOp::Cosh),
            21u32 => Some(GLOp::Tanh),
            22u32 => Some(GLOp::Asinh),
            23u32 => Some(GLOp::Acosh),
            24u32 => Some(GLOp::Atanh),
            25u32 => Some(GLOp::Atan2),
            26u32 => Some(GLOp::Pow),
            27u32 => Some(GLOp::Exp),
            28u32 => Some(GLOp::Log),
            29u32 => Some(GLOp::Exp2),
            30u32 => Some(GLOp::Log2),
            31u32 => Some(GLOp::Sqrt),
            32u32 => Some(GLOp::InverseSqrt),
            33u32 => Some(GLOp::Determinant),
            34u32 => Some(GLOp::MatrixInverse),
            35u32 => Some(GLOp::Modf),
            36u32 => Some(GLOp::ModfStruct),
            37u32 => Some(GLOp::FMin),
            38u32 => Some(GLOp::UMin),
            39u32 => Some(GLOp::SMin),
            40u32 => Some(GLOp::FMax),
            41u32 => Some(GLOp::UMax),
            42u32 => Some(GLOp::SMax),
            43u32 => Some(GLOp::FClamp),
            44u32 => Some(GLOp::UClamp),
            45u32 => Some(GLOp::SClamp),
            46u32 => Some(GLOp::FMix),
            47u32 => Some(GLOp::IMix),
            48u32 => Some(GLOp::Step),
            49u32 => Some(GLOp::SmoothStep),
            50u32 => Some(GLOp::Fma),
            51u32 => Some(GLOp::Frexp),
            52u32 => Some(GLOp::FrexpStruct),
            53u32 => Some(GLOp::Ldexp),
            54u32 => Some(GLOp::PackSnorm4x8),
            55u32 => Some(GLOp::PackUnorm4x8),
            56u32 => Some(GLOp::PackSnorm2x16),
            57u32 => Some(GLOp::PackUnorm2x16),
            58u32 => Some(GLOp::PackHalf2x16),
            59u32 => Some(GLOp::PackDouble2x32),
            60u32 => Some(GLOp::UnpackSnorm2x16),
            61u32 => Some(GLOp::UnpackUnorm2x16),
            62u32 => Some(GLOp::UnpackHalf2x16),
            63u32 => Some(GLOp::UnpackSnorm4x8),
            64u32 => Some(GLOp::UnpackUnorm4x8),
            65u32 => Some(GLOp::UnpackDouble2x32),
            66u32 => Some(GLOp::Length),
            67u32 => Some(GLOp::Distance),
            68u32 => Some(GLOp::Cross),
            69u32 => Some(GLOp::Normalize),
            70u32 => Some(GLOp::FaceForward),
            71u32 => Some(GLOp::Reflect),
            72u32 => Some(GLOp::Refract),
            73u32 => Some(GLOp::FindILsb),
            74u32 => Some(GLOp::FindSMsb),
            75u32 => Some(GLOp::FindUMsb),
            76u32 => Some(GLOp::InterpolateAtCentroid),
            77u32 => Some(GLOp::InterpolateAtSample),
            78u32 => Some(GLOp::InterpolateAtOffset),
            79u32 => Some(GLOp::NMin),
            80u32 => Some(GLOp::NMax),
            81u32 => Some(GLOp::NClamp),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
#[doc = "[OpenCL.std](https://www.khronos.org/registry/spir-v/specs/unified1/OpenCL.ExtendedInstructionSet.100.html) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CLOp {
    acos = 0u32,
    acosh = 1u32,
    acospi = 2u32,
    asin = 3u32,
    asinh = 4u32,
    asinpi = 5u32,
    atan = 6u32,
    atan2 = 7u32,
    atanh = 8u32,
    atanpi = 9u32,
    atan2pi = 10u32,
    cbrt = 11u32,
    ceil = 12u32,
    copysign = 13u32,
    cos = 14u32,
    cosh = 15u32,
    cospi = 16u32,
    erfc = 17u32,
    erf = 18u32,
    exp = 19u32,
    exp2 = 20u32,
    exp10 = 21u32,
    expm1 = 22u32,
    fabs = 23u32,
    fdim = 24u32,
    floor = 25u32,
    fma = 26u32,
    fmax = 27u32,
    fmin = 28u32,
    fmod = 29u32,
    fract = 30u32,
    frexp = 31u32,
    hypot = 32u32,
    ilogb = 33u32,
    ldexp = 34u32,
    lgamma = 35u32,
    lgamma_r = 36u32,
    log = 37u32,
    log2 = 38u32,
    log10 = 39u32,
    log1p = 40u32,
    logb = 41u32,
    mad = 42u32,
    maxmag = 43u32,
    minmag = 44u32,
    modf = 45u32,
    nan = 46u32,
    nextafter = 47u32,
    pow = 48u32,
    pown = 49u32,
    powr = 50u32,
    remainder = 51u32,
    remquo = 52u32,
    rint = 53u32,
    rootn = 54u32,
    round = 55u32,
    rsqrt = 56u32,
    sin = 57u32,
    sincos = 58u32,
    sinh = 59u32,
    sinpi = 60u32,
    sqrt = 61u32,
    tan = 62u32,
    tanh = 63u32,
    tanpi = 64u32,
    tgamma = 65u32,
    trunc = 66u32,
    half_cos = 67u32,
    half_divide = 68u32,
    half_exp = 69u32,
    half_exp2 = 70u32,
    half_exp10 = 71u32,
    half_log = 72u32,
    half_log2 = 73u32,
    half_log10 = 74u32,
    half_powr = 75u32,
    half_recip = 76u32,
    half_rsqrt = 77u32,
    half_sin = 78u32,
    half_sqrt = 79u32,
    half_tan = 80u32,
    native_cos = 81u32,
    native_divide = 82u32,
    native_exp = 83u32,
    native_exp2 = 84u32,
    native_exp10 = 85u32,
    native_log = 86u32,
    native_log2 = 87u32,
    native_log10 = 88u32,
    native_powr = 89u32,
    native_recip = 90u32,
    native_rsqrt = 91u32,
    native_sin = 92u32,
    native_sqrt = 93u32,
    native_tan = 94u32,
    s_abs = 141u32,
    s_abs_diff = 142u32,
    s_add_sat = 143u32,
    u_add_sat = 144u32,
    s_hadd = 145u32,
    u_hadd = 146u32,
    s_rhadd = 147u32,
    u_rhadd = 148u32,
    s_clamp = 149u32,
    u_clamp = 150u32,
    clz = 151u32,
    ctz = 152u32,
    s_mad_hi = 153u32,
    u_mad_sat = 154u32,
    s_mad_sat = 155u32,
    s_max = 156u32,
    u_max = 157u32,
    s_min = 158u32,
    u_min = 159u32,
    s_mul_hi = 160u32,
    rotate = 161u32,
    s_sub_sat = 162u32,
    u_sub_sat = 163u32,
    u_upsample = 164u32,
    s_upsample = 165u32,
    popcount = 166u32,
    s_mad24 = 167u32,
    u_mad24 = 168u32,
    s_mul24 = 169u32,
    u_mul24 = 170u32,
    u_abs = 201u32,
    u_abs_diff = 202u32,
    u_mul_hi = 203u32,
    u_mad_hi = 204u32,
    fclamp = 95u32,
    degrees = 96u32,
    fmax_common = 97u32,
    fmin_common = 98u32,
    mix = 99u32,
    radians = 100u32,
    step = 101u32,
    smoothstep = 102u32,
    sign = 103u32,
    cross = 104u32,
    distance = 105u32,
    length = 106u32,
    normalize = 107u32,
    fast_distance = 108u32,
    fast_length = 109u32,
    fast_normalize = 110u32,
    bitselect = 186u32,
    select = 187u32,
    vloadn = 171u32,
    vstoren = 172u32,
    vload_half = 173u32,
    vload_halfn = 174u32,
    vstore_half = 175u32,
    vstore_half_r = 176u32,
    vstore_halfn = 177u32,
    vstore_halfn_r = 178u32,
    vloada_halfn = 179u32,
    vstorea_halfn = 180u32,
    vstorea_halfn_r = 181u32,
    shuffle = 182u32,
    shuffle2 = 183u32,
    printf = 184u32,
    prefetch = 185u32,
}
impl num_traits::FromPrimitive for CLOp {
    #[allow(trivial_numeric_casts)]
    fn from_i64(n: i64) -> Option<Self> {
        match n as u32 {
            0u32 => Some(CLOp::acos),
            1u32 => Some(CLOp::acosh),
            2u32 => Some(CLOp::acospi),
            3u32 => Some(CLOp::asin),
            4u32 => Some(CLOp::asinh),
            5u32 => Some(CLOp::asinpi),
            6u32 => Some(CLOp::atan),
            7u32 => Some(CLOp::atan2),
            8u32 => Some(CLOp::atanh),
            9u32 => Some(CLOp::atanpi),
            10u32 => Some(CLOp::atan2pi),
            11u32 => Some(CLOp::cbrt),
            12u32 => Some(CLOp::ceil),
            13u32 => Some(CLOp::copysign),
            14u32 => Some(CLOp::cos),
            15u32 => Some(CLOp::cosh),
            16u32 => Some(CLOp::cospi),
            17u32 => Some(CLOp::erfc),
            18u32 => Some(CLOp::erf),
            19u32 => Some(CLOp::exp),
            20u32 => Some(CLOp::exp2),
            21u32 => Some(CLOp::exp10),
            22u32 => Some(CLOp::expm1),
            23u32 => Some(CLOp::fabs),
            24u32 => Some(CLOp::fdim),
            25u32 => Some(CLOp::floor),
            26u32 => Some(CLOp::fma),
            27u32 => Some(CLOp::fmax),
            28u32 => Some(CLOp::fmin),
            29u32 => Some(CLOp::fmod),
            30u32 => Some(CLOp::fract),
            31u32 => Some(CLOp::frexp),
            32u32 => Some(CLOp::hypot),
            33u32 => Some(CLOp::ilogb),
            34u32 => Some(CLOp::ldexp),
            35u32 => Some(CLOp::lgamma),
            36u32 => Some(CLOp::lgamma_r),
            37u32 => Some(CLOp::log),
            38u32 => Some(CLOp::log2),
            39u32 => Some(CLOp::log10),
            40u32 => Some(CLOp::log1p),
            41u32 => Some(CLOp::logb),
            42u32 => Some(CLOp::mad),
            43u32 => Some(CLOp::maxmag),
            44u32 => Some(CLOp::minmag),
            45u32 => Some(CLOp::modf),
            46u32 => Some(CLOp::nan),
            47u32 => Some(CLOp::nextafter),
            48u32 => Some(CLOp::pow),
            49u32 => Some(CLOp::pown),
            50u32 => Some(CLOp::powr),
            51u32 => Some(CLOp::remainder),
            52u32 => Some(CLOp::remquo),
            53u32 => Some(CLOp::rint),
            54u32 => Some(CLOp::rootn),
            55u32 => Some(CLOp::round),
            56u32 => Some(CLOp::rsqrt),
            57u32 => Some(CLOp::sin),
            58u32 => Some(CLOp::sincos),
            59u32 => Some(CLOp::sinh),
            60u32 => Some(CLOp::sinpi),
            61u32 => Some(CLOp::sqrt),
            62u32 => Some(CLOp::tan),
            63u32 => Some(CLOp::tanh),
            64u32 => Some(CLOp::tanpi),
            65u32 => Some(CLOp::tgamma),
            66u32 => Some(CLOp::trunc),
            67u32 => Some(CLOp::half_cos),
            68u32 => Some(CLOp::half_divide),
            69u32 => Some(CLOp::half_exp),
            70u32 => Some(CLOp::half_exp2),
            71u32 => Some(CLOp::half_exp10),
            72u32 => Some(CLOp::half_log),
            73u32 => Some(CLOp::half_log2),
            74u32 => Some(CLOp::half_log10),
            75u32 => Some(CLOp::half_powr),
            76u32 => Some(CLOp::half_recip),
            77u32 => Some(CLOp::half_rsqrt),
            78u32 => Some(CLOp::half_sin),
            79u32 => Some(CLOp::half_sqrt),
            80u32 => Some(CLOp::half_tan),
            81u32 => Some(CLOp::native_cos),
            82u32 => Some(CLOp::native_divide),
            83u32 => Some(CLOp::native_exp),
            84u32 => Some(CLOp::native_exp2),
            85u32 => Some(CLOp::native_exp10),
            86u32 => Some(CLOp::native_log),
            87u32 => Some(CLOp::native_log2),
            88u32 => Some(CLOp::native_log10),
            89u32 => Some(CLOp::native_powr),
            90u32 => Some(CLOp::native_recip),
            91u32 => Some(CLOp::native_rsqrt),
            92u32 => Some(CLOp::native_sin),
            93u32 => Some(CLOp::native_sqrt),
            94u32 => Some(CLOp::native_tan),
            141u32 => Some(CLOp::s_abs),
            142u32 => Some(CLOp::s_abs_diff),
            143u32 => Some(CLOp::s_add_sat),
            144u32 => Some(CLOp::u_add_sat),
            145u32 => Some(CLOp::s_hadd),
            146u32 => Some(CLOp::u_hadd),
            147u32 => Some(CLOp::s_rhadd),
            148u32 => Some(CLOp::u_rhadd),
            149u32 => Some(CLOp::s_clamp),
            150u32 => Some(CLOp::u_clamp),
            151u32 => Some(CLOp::clz),
            152u32 => Some(CLOp::ctz),
            153u32 => Some(CLOp::s_mad_hi),
            154u32 => Some(CLOp::u_mad_sat),
            155u32 => Some(CLOp::s_mad_sat),
            156u32 => Some(CLOp::s_max),
            157u32 => Some(CLOp::u_max),
            158u32 => Some(CLOp::s_min),
            159u32 => Some(CLOp::u_min),
            160u32 => Some(CLOp::s_mul_hi),
            161u32 => Some(CLOp::rotate),
            162u32 => Some(CLOp::s_sub_sat),
            163u32 => Some(CLOp::u_sub_sat),
            164u32 => Some(CLOp::u_upsample),
            165u32 => Some(CLOp::s_upsample),
            166u32 => Some(CLOp::popcount),
            167u32 => Some(CLOp::s_mad24),
            168u32 => Some(CLOp::u_mad24),
            169u32 => Some(CLOp::s_mul24),
            170u32 => Some(CLOp::u_mul24),
            201u32 => Some(CLOp::u_abs),
            202u32 => Some(CLOp::u_abs_diff),
            203u32 => Some(CLOp::u_mul_hi),
            204u32 => Some(CLOp::u_mad_hi),
            95u32 => Some(CLOp::fclamp),
            96u32 => Some(CLOp::degrees),
            97u32 => Some(CLOp::fmax_common),
            98u32 => Some(CLOp::fmin_common),
            99u32 => Some(CLOp::mix),
            100u32 => Some(CLOp::radians),
            101u32 => Some(CLOp::step),
            102u32 => Some(CLOp::smoothstep),
            103u32 => Some(CLOp::sign),
            104u32 => Some(CLOp::cross),
            105u32 => Some(CLOp::distance),
            106u32 => Some(CLOp::length),
            107u32 => Some(CLOp::normalize),
            108u32 => Some(CLOp::fast_distance),
            109u32 => Some(CLOp::fast_length),
            110u32 => Some(CLOp::fast_normalize),
            186u32 => Some(CLOp::bitselect),
            187u32 => Some(CLOp::select),
            171u32 => Some(CLOp::vloadn),
            172u32 => Some(CLOp::vstoren),
            173u32 => Some(CLOp::vload_half),
            174u32 => Some(CLOp::vload_halfn),
            175u32 => Some(CLOp::vstore_half),
            176u32 => Some(CLOp::vstore_half_r),
            177u32 => Some(CLOp::vstore_halfn),
            178u32 => Some(CLOp::vstore_halfn_r),
            179u32 => Some(CLOp::vloada_halfn),
            180u32 => Some(CLOp::vstorea_halfn),
            181u32 => Some(CLOp::vstorea_halfn_r),
            182u32 => Some(CLOp::shuffle),
            183u32 => Some(CLOp::shuffle2),
            184u32 => Some(CLOp::printf),
            185u32 => Some(CLOp::prefetch),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
