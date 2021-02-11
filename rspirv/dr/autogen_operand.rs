// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[doc = "Data representation of a SPIR-V operand."]
#[derive(Clone, Debug, PartialEq, From)]
pub enum Operand {
    ImageOperands(spirv::ImageOperands),
    FPFastMathMode(spirv::FPFastMathMode),
    SelectionControl(spirv::SelectionControl),
    LoopControl(spirv::LoopControl),
    FunctionControl(spirv::FunctionControl),
    MemorySemantics(spirv::MemorySemantics),
    MemoryAccess(spirv::MemoryAccess),
    KernelProfilingInfo(spirv::KernelProfilingInfo),
    RayFlags(spirv::RayFlags),
    FragmentShadingRate(spirv::FragmentShadingRate),
    SourceLanguage(spirv::SourceLanguage),
    ExecutionModel(spirv::ExecutionModel),
    AddressingModel(spirv::AddressingModel),
    MemoryModel(spirv::MemoryModel),
    ExecutionMode(spirv::ExecutionMode),
    StorageClass(spirv::StorageClass),
    Dim(spirv::Dim),
    SamplerAddressingMode(spirv::SamplerAddressingMode),
    SamplerFilterMode(spirv::SamplerFilterMode),
    ImageFormat(spirv::ImageFormat),
    ImageChannelOrder(spirv::ImageChannelOrder),
    ImageChannelDataType(spirv::ImageChannelDataType),
    FPRoundingMode(spirv::FPRoundingMode),
    LinkageType(spirv::LinkageType),
    AccessQualifier(spirv::AccessQualifier),
    FunctionParameterAttribute(spirv::FunctionParameterAttribute),
    Decoration(spirv::Decoration),
    BuiltIn(spirv::BuiltIn),
    Scope(spirv::Scope),
    GroupOperation(spirv::GroupOperation),
    KernelEnqueueFlags(spirv::KernelEnqueueFlags),
    Capability(spirv::Capability),
    RayQueryIntersection(spirv::RayQueryIntersection),
    RayQueryCommittedIntersectionType(spirv::RayQueryCommittedIntersectionType),
    RayQueryCandidateIntersectionType(spirv::RayQueryCandidateIntersectionType),
    #[from(ignore)]
    IdMemorySemantics(spirv::Word),
    #[from(ignore)]
    IdScope(spirv::Word),
    #[from(ignore)]
    IdRef(spirv::Word),
    LiteralInt32(u32),
    LiteralInt64(u64),
    LiteralFloat32(f32),
    LiteralFloat64(f64),
    #[from(ignore)]
    LiteralExtInstInteger(u32),
    LiteralSpecConstantOpInteger(spirv::Op),
    LiteralString(String),
}
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::ImageOperands(ref v) => write!(f, "{:?}", v),
            Operand::FPFastMathMode(ref v) => write!(f, "{:?}", v),
            Operand::SelectionControl(ref v) => write!(f, "{:?}", v),
            Operand::LoopControl(ref v) => write!(f, "{:?}", v),
            Operand::FunctionControl(ref v) => write!(f, "{:?}", v),
            Operand::MemorySemantics(ref v) => write!(f, "{:?}", v),
            Operand::MemoryAccess(ref v) => write!(f, "{:?}", v),
            Operand::KernelProfilingInfo(ref v) => write!(f, "{:?}", v),
            Operand::RayFlags(ref v) => write!(f, "{:?}", v),
            Operand::FragmentShadingRate(ref v) => write!(f, "{:?}", v),
            Operand::SourceLanguage(ref v) => write!(f, "{:?}", v),
            Operand::ExecutionModel(ref v) => write!(f, "{:?}", v),
            Operand::AddressingModel(ref v) => write!(f, "{:?}", v),
            Operand::MemoryModel(ref v) => write!(f, "{:?}", v),
            Operand::ExecutionMode(ref v) => write!(f, "{:?}", v),
            Operand::StorageClass(ref v) => write!(f, "{:?}", v),
            Operand::Dim(ref v) => write!(f, "{}", &format!("{:?}", v)[3..]),
            Operand::SamplerAddressingMode(ref v) => write!(f, "{:?}", v),
            Operand::SamplerFilterMode(ref v) => write!(f, "{:?}", v),
            Operand::ImageFormat(ref v) => write!(f, "{:?}", v),
            Operand::ImageChannelOrder(ref v) => write!(f, "{:?}", v),
            Operand::ImageChannelDataType(ref v) => write!(f, "{:?}", v),
            Operand::FPRoundingMode(ref v) => write!(f, "{:?}", v),
            Operand::LinkageType(ref v) => write!(f, "{:?}", v),
            Operand::AccessQualifier(ref v) => write!(f, "{:?}", v),
            Operand::FunctionParameterAttribute(ref v) => write!(f, "{:?}", v),
            Operand::Decoration(ref v) => write!(f, "{:?}", v),
            Operand::BuiltIn(ref v) => write!(f, "{:?}", v),
            Operand::Scope(ref v) => write!(f, "{:?}", v),
            Operand::GroupOperation(ref v) => write!(f, "{:?}", v),
            Operand::KernelEnqueueFlags(ref v) => write!(f, "{:?}", v),
            Operand::Capability(ref v) => write!(f, "{:?}", v),
            Operand::RayQueryIntersection(ref v) => write!(f, "{:?}", v),
            Operand::RayQueryCommittedIntersectionType(ref v) => write!(f, "{:?}", v),
            Operand::RayQueryCandidateIntersectionType(ref v) => write!(f, "{:?}", v),
            Operand::IdMemorySemantics(ref v) => write!(f, "%{}", v),
            Operand::IdScope(ref v) => write!(f, "%{}", v),
            Operand::IdRef(ref v) => write!(f, "%{}", v),
            Operand::LiteralString(ref v) => write!(f, "{:?}", v),
            Operand::LiteralExtInstInteger(ref v) => write!(f, "{:?}", v),
            Operand::LiteralSpecConstantOpInteger(ref v) => write!(f, "{:?}", v),
            Operand::LiteralInt32(ref v) => write!(f, "{:?}", v),
            Operand::LiteralInt64(ref v) => write!(f, "{:?}", v),
            Operand::LiteralFloat32(ref v) => write!(f, "{:?}", v),
            Operand::LiteralFloat64(ref v) => write!(f, "{:?}", v),
        }
    }
}
impl Operand {
    pub fn unwrap_image_operands(&self) -> spirv::ImageOperands {
        match *self {
            Self::ImageOperands(v) => v,
            ref other => panic!("Expected Operand::ImageOperands, got {} instead", other),
        }
    }
    pub fn unwrap_fp_fast_math_mode(&self) -> spirv::FPFastMathMode {
        match *self {
            Self::FPFastMathMode(v) => v,
            ref other => panic!("Expected Operand::FPFastMathMode, got {} instead", other),
        }
    }
    pub fn unwrap_selection_control(&self) -> spirv::SelectionControl {
        match *self {
            Self::SelectionControl(v) => v,
            ref other => panic!("Expected Operand::SelectionControl, got {} instead", other),
        }
    }
    pub fn unwrap_loop_control(&self) -> spirv::LoopControl {
        match *self {
            Self::LoopControl(v) => v,
            ref other => panic!("Expected Operand::LoopControl, got {} instead", other),
        }
    }
    pub fn unwrap_function_control(&self) -> spirv::FunctionControl {
        match *self {
            Self::FunctionControl(v) => v,
            ref other => panic!("Expected Operand::FunctionControl, got {} instead", other),
        }
    }
    pub fn unwrap_memory_semantics(&self) -> spirv::MemorySemantics {
        match *self {
            Self::MemorySemantics(v) => v,
            ref other => panic!("Expected Operand::MemorySemantics, got {} instead", other),
        }
    }
    pub fn unwrap_memory_access(&self) -> spirv::MemoryAccess {
        match *self {
            Self::MemoryAccess(v) => v,
            ref other => panic!("Expected Operand::MemoryAccess, got {} instead", other),
        }
    }
    pub fn unwrap_kernel_profiling_info(&self) -> spirv::KernelProfilingInfo {
        match *self {
            Self::KernelProfilingInfo(v) => v,
            ref other => panic!(
                "Expected Operand::KernelProfilingInfo, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_ray_flags(&self) -> spirv::RayFlags {
        match *self {
            Self::RayFlags(v) => v,
            ref other => panic!("Expected Operand::RayFlags, got {} instead", other),
        }
    }
    pub fn unwrap_fragment_shading_rate(&self) -> spirv::FragmentShadingRate {
        match *self {
            Self::FragmentShadingRate(v) => v,
            ref other => panic!(
                "Expected Operand::FragmentShadingRate, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_source_language(&self) -> spirv::SourceLanguage {
        match *self {
            Self::SourceLanguage(v) => v,
            ref other => panic!("Expected Operand::SourceLanguage, got {} instead", other),
        }
    }
    pub fn unwrap_execution_model(&self) -> spirv::ExecutionModel {
        match *self {
            Self::ExecutionModel(v) => v,
            ref other => panic!("Expected Operand::ExecutionModel, got {} instead", other),
        }
    }
    pub fn unwrap_addressing_model(&self) -> spirv::AddressingModel {
        match *self {
            Self::AddressingModel(v) => v,
            ref other => panic!("Expected Operand::AddressingModel, got {} instead", other),
        }
    }
    pub fn unwrap_memory_model(&self) -> spirv::MemoryModel {
        match *self {
            Self::MemoryModel(v) => v,
            ref other => panic!("Expected Operand::MemoryModel, got {} instead", other),
        }
    }
    pub fn unwrap_execution_mode(&self) -> spirv::ExecutionMode {
        match *self {
            Self::ExecutionMode(v) => v,
            ref other => panic!("Expected Operand::ExecutionMode, got {} instead", other),
        }
    }
    pub fn unwrap_storage_class(&self) -> spirv::StorageClass {
        match *self {
            Self::StorageClass(v) => v,
            ref other => panic!("Expected Operand::StorageClass, got {} instead", other),
        }
    }
    pub fn unwrap_dim(&self) -> spirv::Dim {
        match *self {
            Self::Dim(v) => v,
            ref other => panic!("Expected Operand::Dim, got {} instead", other),
        }
    }
    pub fn unwrap_sampler_addressing_mode(&self) -> spirv::SamplerAddressingMode {
        match *self {
            Self::SamplerAddressingMode(v) => v,
            ref other => panic!(
                "Expected Operand::SamplerAddressingMode, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_sampler_filter_mode(&self) -> spirv::SamplerFilterMode {
        match *self {
            Self::SamplerFilterMode(v) => v,
            ref other => panic!("Expected Operand::SamplerFilterMode, got {} instead", other),
        }
    }
    pub fn unwrap_image_format(&self) -> spirv::ImageFormat {
        match *self {
            Self::ImageFormat(v) => v,
            ref other => panic!("Expected Operand::ImageFormat, got {} instead", other),
        }
    }
    pub fn unwrap_image_channel_order(&self) -> spirv::ImageChannelOrder {
        match *self {
            Self::ImageChannelOrder(v) => v,
            ref other => panic!("Expected Operand::ImageChannelOrder, got {} instead", other),
        }
    }
    pub fn unwrap_image_channel_data_type(&self) -> spirv::ImageChannelDataType {
        match *self {
            Self::ImageChannelDataType(v) => v,
            ref other => panic!(
                "Expected Operand::ImageChannelDataType, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_fp_rounding_mode(&self) -> spirv::FPRoundingMode {
        match *self {
            Self::FPRoundingMode(v) => v,
            ref other => panic!("Expected Operand::FPRoundingMode, got {} instead", other),
        }
    }
    pub fn unwrap_linkage_type(&self) -> spirv::LinkageType {
        match *self {
            Self::LinkageType(v) => v,
            ref other => panic!("Expected Operand::LinkageType, got {} instead", other),
        }
    }
    pub fn unwrap_access_qualifier(&self) -> spirv::AccessQualifier {
        match *self {
            Self::AccessQualifier(v) => v,
            ref other => panic!("Expected Operand::AccessQualifier, got {} instead", other),
        }
    }
    pub fn unwrap_function_parameter_attribute(&self) -> spirv::FunctionParameterAttribute {
        match *self {
            Self::FunctionParameterAttribute(v) => v,
            ref other => panic!(
                "Expected Operand::FunctionParameterAttribute, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_decoration(&self) -> spirv::Decoration {
        match *self {
            Self::Decoration(v) => v,
            ref other => panic!("Expected Operand::Decoration, got {} instead", other),
        }
    }
    pub fn unwrap_built_in(&self) -> spirv::BuiltIn {
        match *self {
            Self::BuiltIn(v) => v,
            ref other => panic!("Expected Operand::BuiltIn, got {} instead", other),
        }
    }
    pub fn unwrap_scope(&self) -> spirv::Scope {
        match *self {
            Self::Scope(v) => v,
            ref other => panic!("Expected Operand::Scope, got {} instead", other),
        }
    }
    pub fn unwrap_group_operation(&self) -> spirv::GroupOperation {
        match *self {
            Self::GroupOperation(v) => v,
            ref other => panic!("Expected Operand::GroupOperation, got {} instead", other),
        }
    }
    pub fn unwrap_kernel_enqueue_flags(&self) -> spirv::KernelEnqueueFlags {
        match *self {
            Self::KernelEnqueueFlags(v) => v,
            ref other => panic!(
                "Expected Operand::KernelEnqueueFlags, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_capability(&self) -> spirv::Capability {
        match *self {
            Self::Capability(v) => v,
            ref other => panic!("Expected Operand::Capability, got {} instead", other),
        }
    }
    pub fn unwrap_ray_query_intersection(&self) -> spirv::RayQueryIntersection {
        match *self {
            Self::RayQueryIntersection(v) => v,
            ref other => panic!(
                "Expected Operand::RayQueryIntersection, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_ray_query_committed_intersection_type(
        &self,
    ) -> spirv::RayQueryCommittedIntersectionType {
        match *self {
            Self::RayQueryCommittedIntersectionType(v) => v,
            ref other => panic!(
                "Expected Operand::RayQueryCommittedIntersectionType, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_ray_query_candidate_intersection_type(
        &self,
    ) -> spirv::RayQueryCandidateIntersectionType {
        match *self {
            Self::RayQueryCandidateIntersectionType(v) => v,
            ref other => panic!(
                "Expected Operand::RayQueryCandidateIntersectionType, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_id_memory_semantics(&self) -> spirv::Word {
        match *self {
            Self::IdMemorySemantics(v) => v,
            ref other => panic!("Expected Operand::IdMemorySemantics, got {} instead", other),
        }
    }
    pub fn unwrap_id_scope(&self) -> spirv::Word {
        match *self {
            Self::IdScope(v) => v,
            ref other => panic!("Expected Operand::IdScope, got {} instead", other),
        }
    }
    pub fn unwrap_id_ref(&self) -> spirv::Word {
        match *self {
            Self::IdRef(v) => v,
            ref other => panic!("Expected Operand::IdRef, got {} instead", other),
        }
    }
    pub fn unwrap_literal_int32(&self) -> u32 {
        match *self {
            Self::LiteralInt32(v) => v,
            ref other => panic!("Expected Operand::LiteralInt32, got {} instead", other),
        }
    }
    pub fn unwrap_literal_int64(&self) -> u64 {
        match *self {
            Self::LiteralInt64(v) => v,
            ref other => panic!("Expected Operand::LiteralInt64, got {} instead", other),
        }
    }
    pub fn unwrap_literal_float32(&self) -> f32 {
        match *self {
            Self::LiteralFloat32(v) => v,
            ref other => panic!("Expected Operand::LiteralFloat32, got {} instead", other),
        }
    }
    pub fn unwrap_literal_float64(&self) -> f64 {
        match *self {
            Self::LiteralFloat64(v) => v,
            ref other => panic!("Expected Operand::LiteralFloat64, got {} instead", other),
        }
    }
    pub fn unwrap_literal_ext_inst_integer(&self) -> u32 {
        match *self {
            Self::LiteralExtInstInteger(v) => v,
            ref other => panic!(
                "Expected Operand::LiteralExtInstInteger, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_literal_spec_constant_op_integer(&self) -> spirv::Op {
        match *self {
            Self::LiteralSpecConstantOpInteger(v) => v,
            ref other => panic!(
                "Expected Operand::LiteralSpecConstantOpInteger, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_literal_string(&self) -> &str {
        match self {
            Self::LiteralString(v) => v,
            ref other => panic!("Expected Operand::LiteralString, got {} instead", other),
        }
    }
    pub fn id_ref_any(&self) -> Option<spirv::Word> {
        match *self {
            Self::IdRef(v) | Self::IdScope(v) | Self::IdMemorySemantics(v) => Some(v),
            _ => None,
        }
    }
    pub fn id_ref_any_mut(&mut self) -> Option<&mut spirv::Word> {
        match self {
            Self::IdRef(v) | Self::IdScope(v) | Self::IdMemorySemantics(v) => Some(v),
            _ => None,
        }
    }
    pub fn required_capabilities(&self) -> Vec<spirv::Capability> {
        use spirv as s;
        match self {
            Self::ImageOperands(v) => {
                let mut result = vec![];
                if v.intersects(s::ImageOperands::OFFSET | s::ImageOperands::CONST_OFFSETS) {
                    result.extend_from_slice(&[spirv::Capability::ImageGatherExtended])
                };
                if v.intersects(s::ImageOperands::MIN_LOD) {
                    result.extend_from_slice(&[spirv::Capability::MinLod])
                };
                if v.intersects(s::ImageOperands::BIAS) {
                    result.extend_from_slice(&[spirv::Capability::Shader])
                };
                if v.intersects(
                    s::ImageOperands::MAKE_TEXEL_AVAILABLE
                        | s::ImageOperands::MAKE_TEXEL_VISIBLE
                        | s::ImageOperands::NON_PRIVATE_TEXEL
                        | s::ImageOperands::VOLATILE_TEXEL,
                ) {
                    result.extend_from_slice(&[spirv::Capability::VulkanMemoryModel])
                };
                result
            }
            Self::FPFastMathMode(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::FPFastMathMode::NOT_NAN
                        | s::FPFastMathMode::NOT_INF
                        | s::FPFastMathMode::NSZ
                        | s::FPFastMathMode::ALLOW_RECIP
                        | s::FPFastMathMode::FAST,
                ) {
                    result.extend_from_slice(&[spirv::Capability::Kernel])
                };
                result
            }
            Self::LoopControl(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::LoopControl::INITIATION_INTERVAL_INTEL
                        | s::LoopControl::MAX_CONCURRENCY_INTEL
                        | s::LoopControl::DEPENDENCY_ARRAY_INTEL
                        | s::LoopControl::PIPELINE_ENABLE_INTEL
                        | s::LoopControl::LOOP_COALESCE_INTEL
                        | s::LoopControl::MAX_INTERLEAVING_INTEL
                        | s::LoopControl::SPECULATED_ITERATIONS_INTEL,
                ) {
                    result.extend_from_slice(&[spirv::Capability::FPGALoopControlsINTEL])
                };
                result
            }
            Self::MemorySemantics(v) => {
                let mut result = vec![];
                if v.intersects(s::MemorySemantics::ATOMIC_COUNTER_MEMORY) {
                    result.extend_from_slice(&[spirv::Capability::AtomicStorage])
                };
                if v.intersects(s::MemorySemantics::UNIFORM_MEMORY) {
                    result.extend_from_slice(&[spirv::Capability::Shader])
                };
                if v.intersects(
                    s::MemorySemantics::OUTPUT_MEMORY
                        | s::MemorySemantics::MAKE_AVAILABLE
                        | s::MemorySemantics::MAKE_VISIBLE
                        | s::MemorySemantics::VOLATILE,
                ) {
                    result.extend_from_slice(&[spirv::Capability::VulkanMemoryModel])
                };
                result
            }
            Self::MemoryAccess(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::MemoryAccess::MAKE_POINTER_AVAILABLE
                        | s::MemoryAccess::MAKE_POINTER_VISIBLE
                        | s::MemoryAccess::NON_PRIVATE_POINTER,
                ) {
                    result.extend_from_slice(&[spirv::Capability::VulkanMemoryModel])
                };
                result
            }
            Self::KernelProfilingInfo(v) => {
                let mut result = vec![];
                if v.intersects(s::KernelProfilingInfo::CMD_EXEC_TIME) {
                    result.extend_from_slice(&[spirv::Capability::Kernel])
                };
                result
            }
            Self::RayFlags(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::RayFlags::NONE_KHR
                        | s::RayFlags::OPAQUE_KHR
                        | s::RayFlags::NO_OPAQUE_KHR
                        | s::RayFlags::TERMINATE_ON_FIRST_HIT_KHR
                        | s::RayFlags::SKIP_CLOSEST_HIT_SHADER_KHR
                        | s::RayFlags::CULL_BACK_FACING_TRIANGLES_KHR
                        | s::RayFlags::CULL_FRONT_FACING_TRIANGLES_KHR
                        | s::RayFlags::CULL_OPAQUE_KHR
                        | s::RayFlags::CULL_NO_OPAQUE_KHR,
                ) {
                    result.extend_from_slice(&[
                        spirv::Capability::RayQueryKHR,
                        spirv::Capability::RayTracingKHR,
                    ])
                };
                if v.intersects(s::RayFlags::SKIP_TRIANGLES_KHR | s::RayFlags::SKIP_AAB_BS_KHR) {
                    result.extend_from_slice(&[spirv::Capability::RayTraversalPrimitiveCullingKHR])
                };
                result
            }
            Self::FragmentShadingRate(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::FragmentShadingRate::VERTICAL2_PIXELS
                        | s::FragmentShadingRate::VERTICAL4_PIXELS
                        | s::FragmentShadingRate::HORIZONTAL2_PIXELS
                        | s::FragmentShadingRate::HORIZONTAL4_PIXELS,
                ) {
                    result.extend_from_slice(&[spirv::Capability::FragmentShadingRateKHR])
                };
                result
            }
            Self::SourceLanguage(v) => match v {
                s::SourceLanguage::Unknown
                | s::SourceLanguage::ESSL
                | s::SourceLanguage::GLSL
                | s::SourceLanguage::OpenCL_C
                | s::SourceLanguage::OpenCL_CPP
                | s::SourceLanguage::HLSL => vec![],
            },
            Self::ExecutionModel(v) => match v {
                s::ExecutionModel::Geometry => vec![spirv::Capability::Geometry],
                s::ExecutionModel::Kernel => vec![spirv::Capability::Kernel],
                s::ExecutionModel::TaskNV | s::ExecutionModel::MeshNV => {
                    vec![spirv::Capability::MeshShadingNV]
                }
                s::ExecutionModel::RayGenerationNV
                | s::ExecutionModel::IntersectionNV
                | s::ExecutionModel::AnyHitNV
                | s::ExecutionModel::ClosestHitNV
                | s::ExecutionModel::MissNV
                | s::ExecutionModel::CallableNV => vec![
                    spirv::Capability::RayTracingNV,
                    spirv::Capability::RayTracingKHR,
                ],
                s::ExecutionModel::Vertex
                | s::ExecutionModel::Fragment
                | s::ExecutionModel::GLCompute => vec![spirv::Capability::Shader],
                s::ExecutionModel::TessellationControl
                | s::ExecutionModel::TessellationEvaluation => {
                    vec![spirv::Capability::Tessellation]
                }
            },
            Self::AddressingModel(v) => match v {
                s::AddressingModel::Logical => vec![],
                s::AddressingModel::Physical32 | s::AddressingModel::Physical64 => {
                    vec![spirv::Capability::Addresses]
                }
                s::AddressingModel::PhysicalStorageBuffer64 => {
                    vec![spirv::Capability::PhysicalStorageBufferAddresses]
                }
            },
            Self::MemoryModel(v) => match v {
                s::MemoryModel::OpenCL => vec![spirv::Capability::Kernel],
                s::MemoryModel::Simple | s::MemoryModel::GLSL450 => vec![spirv::Capability::Shader],
                s::MemoryModel::Vulkan => vec![spirv::Capability::VulkanMemoryModel],
            },
            Self::ExecutionMode(v) => match v {
                s::ExecutionMode::LocalSize | s::ExecutionMode::LocalSizeId => vec![],
                s::ExecutionMode::DerivativeGroupLinearNV => {
                    vec![spirv::Capability::ComputeDerivativeGroupLinearNV]
                }
                s::ExecutionMode::DerivativeGroupQuadsNV => {
                    vec![spirv::Capability::ComputeDerivativeGroupQuadsNV]
                }
                s::ExecutionMode::DenormFlushToZero => vec![spirv::Capability::DenormFlushToZero],
                s::ExecutionMode::DenormPreserve => vec![spirv::Capability::DenormPreserve],
                s::ExecutionMode::NumSIMDWorkitemsINTEL => {
                    vec![spirv::Capability::FPGAKernelAttributesINTEL]
                }
                s::ExecutionMode::PixelInterlockOrderedEXT
                | s::ExecutionMode::PixelInterlockUnorderedEXT => {
                    vec![spirv::Capability::FragmentShaderPixelInterlockEXT]
                }
                s::ExecutionMode::SampleInterlockOrderedEXT
                | s::ExecutionMode::SampleInterlockUnorderedEXT => {
                    vec![spirv::Capability::FragmentShaderSampleInterlockEXT]
                }
                s::ExecutionMode::ShadingRateInterlockOrderedEXT
                | s::ExecutionMode::ShadingRateInterlockUnorderedEXT => {
                    vec![spirv::Capability::FragmentShaderShadingRateInterlockEXT]
                }
                s::ExecutionMode::Invocations
                | s::ExecutionMode::InputPoints
                | s::ExecutionMode::InputLines
                | s::ExecutionMode::InputLinesAdjacency
                | s::ExecutionMode::InputTrianglesAdjacency
                | s::ExecutionMode::OutputLineStrip
                | s::ExecutionMode::OutputTriangleStrip => vec![spirv::Capability::Geometry],
                s::ExecutionMode::OutputPoints => vec![
                    spirv::Capability::Geometry,
                    spirv::Capability::MeshShadingNV,
                ],
                s::ExecutionMode::Triangles => {
                    vec![spirv::Capability::Geometry, spirv::Capability::Tessellation]
                }
                s::ExecutionMode::OutputVertices => vec![
                    spirv::Capability::Geometry,
                    spirv::Capability::Tessellation,
                    spirv::Capability::MeshShadingNV,
                ],
                s::ExecutionMode::LocalSizeHint
                | s::ExecutionMode::VecTypeHint
                | s::ExecutionMode::ContractionOff
                | s::ExecutionMode::Initializer
                | s::ExecutionMode::Finalizer
                | s::ExecutionMode::LocalSizeHintId => vec![spirv::Capability::Kernel],
                s::ExecutionMode::MaxWorkgroupSizeINTEL
                | s::ExecutionMode::MaxWorkDimINTEL
                | s::ExecutionMode::NoGlobalOffsetINTEL => {
                    vec![spirv::Capability::KernelAttributesINTEL]
                }
                s::ExecutionMode::OutputLinesNV
                | s::ExecutionMode::OutputPrimitivesNV
                | s::ExecutionMode::OutputTrianglesNV => vec![spirv::Capability::MeshShadingNV],
                s::ExecutionMode::RoundingModeRTE => vec![spirv::Capability::RoundingModeRTE],
                s::ExecutionMode::RoundingModeRTZ => vec![spirv::Capability::RoundingModeRTZ],
                s::ExecutionMode::PostDepthCoverage => {
                    vec![spirv::Capability::SampleMaskPostDepthCoverage]
                }
                s::ExecutionMode::PixelCenterInteger
                | s::ExecutionMode::OriginUpperLeft
                | s::ExecutionMode::OriginLowerLeft
                | s::ExecutionMode::EarlyFragmentTests
                | s::ExecutionMode::DepthReplacing
                | s::ExecutionMode::DepthGreater
                | s::ExecutionMode::DepthLess
                | s::ExecutionMode::DepthUnchanged => vec![spirv::Capability::Shader],
                s::ExecutionMode::SignedZeroInfNanPreserve => {
                    vec![spirv::Capability::SignedZeroInfNanPreserve]
                }
                s::ExecutionMode::StencilRefReplacingEXT => {
                    vec![spirv::Capability::StencilExportEXT]
                }
                s::ExecutionMode::SubgroupSize
                | s::ExecutionMode::SubgroupsPerWorkgroup
                | s::ExecutionMode::SubgroupsPerWorkgroupId => {
                    vec![spirv::Capability::SubgroupDispatch]
                }
                s::ExecutionMode::SpacingEqual
                | s::ExecutionMode::SpacingFractionalEven
                | s::ExecutionMode::SpacingFractionalOdd
                | s::ExecutionMode::VertexOrderCw
                | s::ExecutionMode::VertexOrderCcw
                | s::ExecutionMode::PointMode
                | s::ExecutionMode::Quads
                | s::ExecutionMode::Isolines => vec![spirv::Capability::Tessellation],
                s::ExecutionMode::Xfb => vec![spirv::Capability::TransformFeedback],
            },
            Self::StorageClass(v) => match v {
                s::StorageClass::UniformConstant
                | s::StorageClass::Input
                | s::StorageClass::Workgroup
                | s::StorageClass::CrossWorkgroup
                | s::StorageClass::Function
                | s::StorageClass::Image => vec![],
                s::StorageClass::AtomicCounter => vec![spirv::Capability::AtomicStorage],
                s::StorageClass::CodeSectionINTEL => vec![spirv::Capability::FunctionPointersINTEL],
                s::StorageClass::Generic => vec![spirv::Capability::GenericPointer],
                s::StorageClass::PhysicalStorageBuffer => {
                    vec![spirv::Capability::PhysicalStorageBufferAddresses]
                }
                s::StorageClass::CallableDataNV
                | s::StorageClass::IncomingCallableDataNV
                | s::StorageClass::RayPayloadNV
                | s::StorageClass::HitAttributeNV
                | s::StorageClass::IncomingRayPayloadNV
                | s::StorageClass::ShaderRecordBufferNV => vec![
                    spirv::Capability::RayTracingNV,
                    spirv::Capability::RayTracingKHR,
                ],
                s::StorageClass::Uniform
                | s::StorageClass::Output
                | s::StorageClass::Private
                | s::StorageClass::PushConstant
                | s::StorageClass::StorageBuffer => vec![spirv::Capability::Shader],
            },
            Self::Dim(v) => match v {
                s::Dim::Dim3D => vec![],
                s::Dim::DimSubpassData => vec![spirv::Capability::InputAttachment],
                s::Dim::Dim1D => vec![spirv::Capability::Sampled1D, spirv::Capability::Image1D],
                s::Dim::DimBuffer => vec![
                    spirv::Capability::SampledBuffer,
                    spirv::Capability::ImageBuffer,
                ],
                s::Dim::DimRect => {
                    vec![spirv::Capability::SampledRect, spirv::Capability::ImageRect]
                }
                s::Dim::DimCube => {
                    vec![spirv::Capability::Shader, spirv::Capability::ImageCubeArray]
                }
                s::Dim::Dim2D => vec![
                    spirv::Capability::Shader,
                    spirv::Capability::Kernel,
                    spirv::Capability::ImageMSArray,
                ],
            },
            Self::SamplerAddressingMode(v) => match v {
                s::SamplerAddressingMode::None
                | s::SamplerAddressingMode::ClampToEdge
                | s::SamplerAddressingMode::Clamp
                | s::SamplerAddressingMode::Repeat
                | s::SamplerAddressingMode::RepeatMirrored => vec![spirv::Capability::Kernel],
            },
            Self::SamplerFilterMode(v) => match v {
                s::SamplerFilterMode::Nearest | s::SamplerFilterMode::Linear => {
                    vec![spirv::Capability::Kernel]
                }
            },
            Self::ImageFormat(v) => match v {
                s::ImageFormat::Unknown => vec![],
                s::ImageFormat::R64ui | s::ImageFormat::R64i => {
                    vec![spirv::Capability::Int64ImageEXT]
                }
                s::ImageFormat::Rgba32f
                | s::ImageFormat::Rgba16f
                | s::ImageFormat::R32f
                | s::ImageFormat::Rgba8
                | s::ImageFormat::Rgba8Snorm
                | s::ImageFormat::Rgba32i
                | s::ImageFormat::Rgba16i
                | s::ImageFormat::Rgba8i
                | s::ImageFormat::R32i
                | s::ImageFormat::Rgba32ui
                | s::ImageFormat::Rgba16ui
                | s::ImageFormat::Rgba8ui
                | s::ImageFormat::R32ui => vec![spirv::Capability::Shader],
                s::ImageFormat::Rg32f
                | s::ImageFormat::Rg16f
                | s::ImageFormat::R11fG11fB10f
                | s::ImageFormat::R16f
                | s::ImageFormat::Rgba16
                | s::ImageFormat::Rgb10A2
                | s::ImageFormat::Rg16
                | s::ImageFormat::Rg8
                | s::ImageFormat::R16
                | s::ImageFormat::R8
                | s::ImageFormat::Rgba16Snorm
                | s::ImageFormat::Rg16Snorm
                | s::ImageFormat::Rg8Snorm
                | s::ImageFormat::R16Snorm
                | s::ImageFormat::R8Snorm
                | s::ImageFormat::Rg32i
                | s::ImageFormat::Rg16i
                | s::ImageFormat::Rg8i
                | s::ImageFormat::R16i
                | s::ImageFormat::R8i
                | s::ImageFormat::Rgb10a2ui
                | s::ImageFormat::Rg32ui
                | s::ImageFormat::Rg16ui
                | s::ImageFormat::Rg8ui
                | s::ImageFormat::R16ui
                | s::ImageFormat::R8ui => vec![spirv::Capability::StorageImageExtendedFormats],
            },
            Self::ImageChannelOrder(v) => match v {
                s::ImageChannelOrder::R
                | s::ImageChannelOrder::A
                | s::ImageChannelOrder::RG
                | s::ImageChannelOrder::RA
                | s::ImageChannelOrder::RGB
                | s::ImageChannelOrder::RGBA
                | s::ImageChannelOrder::BGRA
                | s::ImageChannelOrder::ARGB
                | s::ImageChannelOrder::Intensity
                | s::ImageChannelOrder::Luminance
                | s::ImageChannelOrder::Rx
                | s::ImageChannelOrder::RGx
                | s::ImageChannelOrder::RGBx
                | s::ImageChannelOrder::Depth
                | s::ImageChannelOrder::DepthStencil
                | s::ImageChannelOrder::sRGB
                | s::ImageChannelOrder::sRGBx
                | s::ImageChannelOrder::sRGBA
                | s::ImageChannelOrder::sBGRA
                | s::ImageChannelOrder::ABGR => vec![spirv::Capability::Kernel],
            },
            Self::ImageChannelDataType(v) => match v {
                s::ImageChannelDataType::SnormInt8
                | s::ImageChannelDataType::SnormInt16
                | s::ImageChannelDataType::UnormInt8
                | s::ImageChannelDataType::UnormInt16
                | s::ImageChannelDataType::UnormShort565
                | s::ImageChannelDataType::UnormShort555
                | s::ImageChannelDataType::UnormInt101010
                | s::ImageChannelDataType::SignedInt8
                | s::ImageChannelDataType::SignedInt16
                | s::ImageChannelDataType::SignedInt32
                | s::ImageChannelDataType::UnsignedInt8
                | s::ImageChannelDataType::UnsignedInt16
                | s::ImageChannelDataType::UnsignedInt32
                | s::ImageChannelDataType::HalfFloat
                | s::ImageChannelDataType::Float
                | s::ImageChannelDataType::UnormInt24
                | s::ImageChannelDataType::UnormInt101010_2 => vec![spirv::Capability::Kernel],
            },
            Self::FPRoundingMode(v) => match v {
                s::FPRoundingMode::RTE
                | s::FPRoundingMode::RTZ
                | s::FPRoundingMode::RTP
                | s::FPRoundingMode::RTN => vec![],
            },
            Self::LinkageType(v) => match v {
                s::LinkageType::Export | s::LinkageType::Import => vec![spirv::Capability::Linkage],
            },
            Self::AccessQualifier(v) => match v {
                s::AccessQualifier::ReadOnly
                | s::AccessQualifier::WriteOnly
                | s::AccessQualifier::ReadWrite => vec![spirv::Capability::Kernel],
            },
            Self::FunctionParameterAttribute(v) => match v {
                s::FunctionParameterAttribute::Zext
                | s::FunctionParameterAttribute::Sext
                | s::FunctionParameterAttribute::ByVal
                | s::FunctionParameterAttribute::Sret
                | s::FunctionParameterAttribute::NoAlias
                | s::FunctionParameterAttribute::NoCapture
                | s::FunctionParameterAttribute::NoWrite
                | s::FunctionParameterAttribute::NoReadWrite => vec![spirv::Capability::Kernel],
            },
            Self::Decoration(v) => match v {
                s::Decoration::BuiltIn
                | s::Decoration::Restrict
                | s::Decoration::Aliased
                | s::Decoration::Volatile
                | s::Decoration::Coherent
                | s::Decoration::NonWritable
                | s::Decoration::NonReadable
                | s::Decoration::FPRoundingMode
                | s::Decoration::NoSignedWrap
                | s::Decoration::NoUnsignedWrap
                | s::Decoration::ExplicitInterpAMD
                | s::Decoration::CounterBuffer
                | s::Decoration::UserSemantic
                | s::Decoration::UserTypeGOOGLE => vec![],
                s::Decoration::MaxByteOffset | s::Decoration::MaxByteOffsetId => {
                    vec![spirv::Capability::Addresses]
                }
                s::Decoration::RegisterINTEL
                | s::Decoration::MemoryINTEL
                | s::Decoration::NumbanksINTEL
                | s::Decoration::BankwidthINTEL
                | s::Decoration::MaxPrivateCopiesINTEL
                | s::Decoration::SinglepumpINTEL
                | s::Decoration::DoublepumpINTEL
                | s::Decoration::MaxReplicatesINTEL
                | s::Decoration::SimpleDualPortINTEL
                | s::Decoration::MergeINTEL
                | s::Decoration::BankBitsINTEL
                | s::Decoration::ForcePow2DepthINTEL => {
                    vec![spirv::Capability::FPGAMemoryAttributesINTEL]
                }
                s::Decoration::PerVertexNV => vec![spirv::Capability::FragmentBarycentricNV],
                s::Decoration::PassthroughNV => {
                    vec![spirv::Capability::GeometryShaderPassthroughNV]
                }
                s::Decoration::Stream => vec![spirv::Capability::GeometryStreams],
                s::Decoration::ReferencedIndirectlyINTEL => {
                    vec![spirv::Capability::IndirectReferencesINTEL]
                }
                s::Decoration::InputAttachmentIndex => vec![spirv::Capability::InputAttachment],
                s::Decoration::CPacked
                | s::Decoration::Constant
                | s::Decoration::SaturatedConversion
                | s::Decoration::FuncParamAttr
                | s::Decoration::FPFastMathMode
                | s::Decoration::Alignment
                | s::Decoration::AlignmentId => vec![spirv::Capability::Kernel],
                s::Decoration::LinkageAttributes => vec![spirv::Capability::Linkage],
                s::Decoration::RowMajor | s::Decoration::ColMajor | s::Decoration::MatrixStride => {
                    vec![spirv::Capability::Matrix]
                }
                s::Decoration::PerPrimitiveNV
                | s::Decoration::PerViewNV
                | s::Decoration::PerTaskNV => vec![spirv::Capability::MeshShadingNV],
                s::Decoration::RestrictPointer | s::Decoration::AliasedPointer => {
                    vec![spirv::Capability::PhysicalStorageBufferAddresses]
                }
                s::Decoration::OverrideCoverageNV => {
                    vec![spirv::Capability::SampleMaskOverrideCoverageNV]
                }
                s::Decoration::Sample => vec![spirv::Capability::SampleRateShading],
                s::Decoration::RelaxedPrecision
                | s::Decoration::Block
                | s::Decoration::BufferBlock
                | s::Decoration::ArrayStride
                | s::Decoration::GLSLShared
                | s::Decoration::GLSLPacked
                | s::Decoration::NoPerspective
                | s::Decoration::Flat
                | s::Decoration::Centroid
                | s::Decoration::Invariant
                | s::Decoration::Uniform
                | s::Decoration::UniformId
                | s::Decoration::Location
                | s::Decoration::Component
                | s::Decoration::Index
                | s::Decoration::Binding
                | s::Decoration::DescriptorSet
                | s::Decoration::Offset
                | s::Decoration::NoContraction => vec![spirv::Capability::Shader],
                s::Decoration::SpecId => vec![spirv::Capability::Shader, spirv::Capability::Kernel],
                s::Decoration::NonUniform => vec![spirv::Capability::ShaderNonUniform],
                s::Decoration::SecondaryViewportRelativeNV => {
                    vec![spirv::Capability::ShaderStereoViewNV]
                }
                s::Decoration::ViewportRelativeNV => vec![spirv::Capability::ShaderViewportMaskNV],
                s::Decoration::Patch => vec![spirv::Capability::Tessellation],
                s::Decoration::XfbBuffer | s::Decoration::XfbStride => {
                    vec![spirv::Capability::TransformFeedback]
                }
            },
            Self::BuiltIn(v) => match v {
                s::BuiltIn::NumWorkgroups
                | s::BuiltIn::WorkgroupSize
                | s::BuiltIn::WorkgroupId
                | s::BuiltIn::LocalInvocationId
                | s::BuiltIn::GlobalInvocationId
                | s::BuiltIn::LocalInvocationIndex
                | s::BuiltIn::BaryCoordNoPerspAMD
                | s::BuiltIn::BaryCoordNoPerspCentroidAMD
                | s::BuiltIn::BaryCoordNoPerspSampleAMD
                | s::BuiltIn::BaryCoordSmoothAMD
                | s::BuiltIn::BaryCoordSmoothCentroidAMD
                | s::BuiltIn::BaryCoordSmoothSampleAMD
                | s::BuiltIn::BaryCoordPullModelAMD => vec![],
                s::BuiltIn::ClipDistance => vec![spirv::Capability::ClipDistance],
                s::BuiltIn::CullDistance => vec![spirv::Capability::CullDistance],
                s::BuiltIn::DeviceIndex => vec![spirv::Capability::DeviceGroup],
                s::BuiltIn::BaseVertex | s::BuiltIn::BaseInstance => {
                    vec![spirv::Capability::DrawParameters]
                }
                s::BuiltIn::DrawIndex => vec![
                    spirv::Capability::DrawParameters,
                    spirv::Capability::MeshShadingNV,
                ],
                s::BuiltIn::BaryCoordNV | s::BuiltIn::BaryCoordNoPerspNV => {
                    vec![spirv::Capability::FragmentBarycentricNV]
                }
                s::BuiltIn::FragSizeEXT | s::BuiltIn::FragInvocationCountEXT => vec![
                    spirv::Capability::FragmentDensityEXT,
                    spirv::Capability::ShadingRateNV,
                ],
                s::BuiltIn::FullyCoveredEXT => vec![spirv::Capability::FragmentFullyCoveredEXT],
                s::BuiltIn::PrimitiveShadingRateKHR | s::BuiltIn::ShadingRateKHR => {
                    vec![spirv::Capability::FragmentShadingRateKHR]
                }
                s::BuiltIn::Layer => vec![
                    spirv::Capability::Geometry,
                    spirv::Capability::ShaderLayer,
                    spirv::Capability::ShaderViewportIndexLayerEXT,
                    spirv::Capability::MeshShadingNV,
                ],
                s::BuiltIn::InvocationId => {
                    vec![spirv::Capability::Geometry, spirv::Capability::Tessellation]
                }
                s::BuiltIn::PrimitiveId => vec![
                    spirv::Capability::Geometry,
                    spirv::Capability::Tessellation,
                    spirv::Capability::RayTracingNV,
                    spirv::Capability::RayTracingKHR,
                    spirv::Capability::MeshShadingNV,
                ],
                s::BuiltIn::WorkDim
                | s::BuiltIn::GlobalSize
                | s::BuiltIn::EnqueuedWorkgroupSize
                | s::BuiltIn::GlobalOffset
                | s::BuiltIn::GlobalLinearId
                | s::BuiltIn::SubgroupMaxSize
                | s::BuiltIn::NumEnqueuedSubgroups => vec![spirv::Capability::Kernel],
                s::BuiltIn::NumSubgroups | s::BuiltIn::SubgroupId => vec![
                    spirv::Capability::Kernel,
                    spirv::Capability::GroupNonUniform,
                ],
                s::BuiltIn::SubgroupSize | s::BuiltIn::SubgroupLocalInvocationId => vec![
                    spirv::Capability::Kernel,
                    spirv::Capability::GroupNonUniform,
                    spirv::Capability::SubgroupBallotKHR,
                ],
                s::BuiltIn::TaskCountNV
                | s::BuiltIn::PrimitiveCountNV
                | s::BuiltIn::PrimitiveIndicesNV
                | s::BuiltIn::ClipDistancePerViewNV
                | s::BuiltIn::CullDistancePerViewNV
                | s::BuiltIn::LayerPerViewNV
                | s::BuiltIn::MeshViewCountNV
                | s::BuiltIn::MeshViewIndicesNV => vec![spirv::Capability::MeshShadingNV],
                s::BuiltIn::ViewIndex => vec![spirv::Capability::MultiView],
                s::BuiltIn::ViewportIndex => vec![
                    spirv::Capability::MultiViewport,
                    spirv::Capability::ShaderViewportIndex,
                    spirv::Capability::ShaderViewportIndexLayerEXT,
                    spirv::Capability::MeshShadingNV,
                ],
                s::BuiltIn::PositionPerViewNV | s::BuiltIn::ViewportMaskPerViewNV => vec![
                    spirv::Capability::PerViewAttributesNV,
                    spirv::Capability::MeshShadingNV,
                ],
                s::BuiltIn::RayGeometryIndexKHR => vec![spirv::Capability::RayTracingKHR],
                s::BuiltIn::HitTNV => vec![spirv::Capability::RayTracingNV],
                s::BuiltIn::LaunchIdNV
                | s::BuiltIn::LaunchSizeNV
                | s::BuiltIn::WorldRayOriginNV
                | s::BuiltIn::WorldRayDirectionNV
                | s::BuiltIn::ObjectRayOriginNV
                | s::BuiltIn::ObjectRayDirectionNV
                | s::BuiltIn::RayTminNV
                | s::BuiltIn::RayTmaxNV
                | s::BuiltIn::InstanceCustomIndexNV
                | s::BuiltIn::ObjectToWorldNV
                | s::BuiltIn::WorldToObjectNV
                | s::BuiltIn::HitKindNV
                | s::BuiltIn::IncomingRayFlagsNV => vec![
                    spirv::Capability::RayTracingNV,
                    spirv::Capability::RayTracingKHR,
                ],
                s::BuiltIn::SampleId | s::BuiltIn::SamplePosition => {
                    vec![spirv::Capability::SampleRateShading]
                }
                s::BuiltIn::Position
                | s::BuiltIn::PointSize
                | s::BuiltIn::VertexId
                | s::BuiltIn::InstanceId
                | s::BuiltIn::FragCoord
                | s::BuiltIn::PointCoord
                | s::BuiltIn::FrontFacing
                | s::BuiltIn::SampleMask
                | s::BuiltIn::FragDepth
                | s::BuiltIn::HelperInvocation
                | s::BuiltIn::VertexIndex
                | s::BuiltIn::InstanceIndex => vec![spirv::Capability::Shader],
                s::BuiltIn::WarpsPerSMNV
                | s::BuiltIn::SMCountNV
                | s::BuiltIn::WarpIDNV
                | s::BuiltIn::SMIDNV => vec![spirv::Capability::ShaderSMBuiltinsNV],
                s::BuiltIn::SecondaryPositionNV | s::BuiltIn::SecondaryViewportMaskNV => {
                    vec![spirv::Capability::ShaderStereoViewNV]
                }
                s::BuiltIn::ViewportMaskNV => vec![
                    spirv::Capability::ShaderViewportMaskNV,
                    spirv::Capability::MeshShadingNV,
                ],
                s::BuiltIn::FragStencilRefEXT => vec![spirv::Capability::StencilExportEXT],
                s::BuiltIn::SubgroupEqMask
                | s::BuiltIn::SubgroupGeMask
                | s::BuiltIn::SubgroupGtMask
                | s::BuiltIn::SubgroupLeMask
                | s::BuiltIn::SubgroupLtMask => vec![
                    spirv::Capability::SubgroupBallotKHR,
                    spirv::Capability::GroupNonUniformBallot,
                ],
                s::BuiltIn::TessLevelOuter
                | s::BuiltIn::TessLevelInner
                | s::BuiltIn::TessCoord
                | s::BuiltIn::PatchVertices => vec![spirv::Capability::Tessellation],
            },
            Self::Scope(v) => match v {
                s::Scope::CrossDevice
                | s::Scope::Device
                | s::Scope::Workgroup
                | s::Scope::Subgroup
                | s::Scope::Invocation => vec![],
                s::Scope::ShaderCallKHR => vec![spirv::Capability::RayTracingKHR],
                s::Scope::QueueFamily => vec![spirv::Capability::VulkanMemoryModel],
            },
            Self::GroupOperation(v) => match v {
                s::GroupOperation::ClusteredReduce => {
                    vec![spirv::Capability::GroupNonUniformClustered]
                }
                s::GroupOperation::PartitionedReduceNV
                | s::GroupOperation::PartitionedInclusiveScanNV
                | s::GroupOperation::PartitionedExclusiveScanNV => {
                    vec![spirv::Capability::GroupNonUniformPartitionedNV]
                }
                s::GroupOperation::Reduce
                | s::GroupOperation::InclusiveScan
                | s::GroupOperation::ExclusiveScan => vec![
                    spirv::Capability::Kernel,
                    spirv::Capability::GroupNonUniformArithmetic,
                    spirv::Capability::GroupNonUniformBallot,
                ],
            },
            Self::KernelEnqueueFlags(v) => match v {
                s::KernelEnqueueFlags::NoWait
                | s::KernelEnqueueFlags::WaitKernel
                | s::KernelEnqueueFlags::WaitWorkGroup => vec![spirv::Capability::Kernel],
            },
            Self::Capability(v) => match v {
                s::Capability::Matrix
                | s::Capability::Addresses
                | s::Capability::Linkage
                | s::Capability::Kernel
                | s::Capability::Float16
                | s::Capability::Float64
                | s::Capability::Int64
                | s::Capability::Groups
                | s::Capability::Int16
                | s::Capability::Int8
                | s::Capability::Sampled1D
                | s::Capability::SampledBuffer
                | s::Capability::GroupNonUniform
                | s::Capability::ShaderLayer
                | s::Capability::ShaderViewportIndex
                | s::Capability::SubgroupBallotKHR
                | s::Capability::SubgroupVoteKHR
                | s::Capability::StorageBuffer16BitAccess
                | s::Capability::StoragePushConstant16
                | s::Capability::StorageInputOutput16
                | s::Capability::DeviceGroup
                | s::Capability::AtomicStorageOps
                | s::Capability::SampleMaskPostDepthCoverage
                | s::Capability::StorageBuffer8BitAccess
                | s::Capability::StoragePushConstant8
                | s::Capability::DenormPreserve
                | s::Capability::DenormFlushToZero
                | s::Capability::SignedZeroInfNanPreserve
                | s::Capability::RoundingModeRTE
                | s::Capability::RoundingModeRTZ
                | s::Capability::ImageFootprintNV
                | s::Capability::FragmentBarycentricNV
                | s::Capability::ComputeDerivativeGroupQuadsNV
                | s::Capability::GroupNonUniformPartitionedNV
                | s::Capability::VulkanMemoryModel
                | s::Capability::VulkanMemoryModelDeviceScope
                | s::Capability::ComputeDerivativeGroupLinearNV
                | s::Capability::SubgroupShuffleINTEL
                | s::Capability::SubgroupBufferBlockIOINTEL
                | s::Capability::SubgroupImageBlockIOINTEL
                | s::Capability::SubgroupImageMediaBlockIOINTEL
                | s::Capability::FunctionPointersINTEL
                | s::Capability::IndirectReferencesINTEL
                | s::Capability::SubgroupAvcMotionEstimationINTEL
                | s::Capability::SubgroupAvcMotionEstimationIntraINTEL
                | s::Capability::SubgroupAvcMotionEstimationChromaINTEL
                | s::Capability::FPGAMemoryAttributesINTEL
                | s::Capability::UnstructuredLoopControlsINTEL
                | s::Capability::FPGALoopControlsINTEL
                | s::Capability::KernelAttributesINTEL
                | s::Capability::FPGAKernelAttributesINTEL
                | s::Capability::BlockingPipesINTEL
                | s::Capability::FPGARegINTEL => vec![],
                s::Capability::GenericPointer => vec![spirv::Capability::Addresses],
                s::Capability::SubgroupDispatch => vec![spirv::Capability::DeviceEnqueue],
                s::Capability::GeometryPointSize
                | s::Capability::GeometryStreams
                | s::Capability::MultiViewport
                | s::Capability::GeometryShaderPassthroughNV => vec![spirv::Capability::Geometry],
                s::Capability::GroupNonUniformVote
                | s::Capability::GroupNonUniformArithmetic
                | s::Capability::GroupNonUniformBallot
                | s::Capability::GroupNonUniformShuffle
                | s::Capability::GroupNonUniformShuffleRelative
                | s::Capability::GroupNonUniformClustered
                | s::Capability::GroupNonUniformQuad => vec![spirv::Capability::GroupNonUniform],
                s::Capability::ImageReadWrite | s::Capability::ImageMipmap => {
                    vec![spirv::Capability::ImageBasic]
                }
                s::Capability::StorageTexelBufferArrayDynamicIndexing => {
                    vec![spirv::Capability::ImageBuffer]
                }
                s::Capability::StorageTexelBufferArrayNonUniformIndexing => vec![
                    spirv::Capability::ImageBuffer,
                    spirv::Capability::ShaderNonUniform,
                ],
                s::Capability::InputAttachmentArrayDynamicIndexing => {
                    vec![spirv::Capability::InputAttachment]
                }
                s::Capability::InputAttachmentArrayNonUniformIndexing => vec![
                    spirv::Capability::InputAttachment,
                    spirv::Capability::ShaderNonUniform,
                ],
                s::Capability::Int64Atomics => vec![spirv::Capability::Int64],
                s::Capability::Vector16
                | s::Capability::Float16Buffer
                | s::Capability::ImageBasic
                | s::Capability::Pipes
                | s::Capability::DeviceEnqueue
                | s::Capability::LiteralSampler
                | s::Capability::NamedBarrier => vec![spirv::Capability::Kernel],
                s::Capability::Shader => vec![spirv::Capability::Matrix],
                s::Capability::PerViewAttributesNV => vec![spirv::Capability::MultiView],
                s::Capability::ShaderViewportIndexLayerEXT => {
                    vec![spirv::Capability::MultiViewport]
                }
                s::Capability::PipeStorage => vec![spirv::Capability::Pipes],
                s::Capability::RayTraversalPrimitiveCullingKHR => vec![
                    spirv::Capability::RayQueryKHR,
                    spirv::Capability::RayTracingKHR,
                ],
                s::Capability::SampleMaskOverrideCoverageNV => {
                    vec![spirv::Capability::SampleRateShading]
                }
                s::Capability::Image1D => vec![spirv::Capability::Sampled1D],
                s::Capability::ImageBuffer
                | s::Capability::UniformTexelBufferArrayDynamicIndexing => {
                    vec![spirv::Capability::SampledBuffer]
                }
                s::Capability::UniformTexelBufferArrayNonUniformIndexing => vec![
                    spirv::Capability::SampledBuffer,
                    spirv::Capability::ShaderNonUniform,
                ],
                s::Capability::ImageCubeArray => vec![spirv::Capability::SampledCubeArray],
                s::Capability::ImageRect => vec![spirv::Capability::SampledRect],
                s::Capability::Geometry
                | s::Capability::Tessellation
                | s::Capability::AtomicStorage
                | s::Capability::ImageGatherExtended
                | s::Capability::StorageImageMultisample
                | s::Capability::UniformBufferArrayDynamicIndexing
                | s::Capability::SampledImageArrayDynamicIndexing
                | s::Capability::StorageBufferArrayDynamicIndexing
                | s::Capability::StorageImageArrayDynamicIndexing
                | s::Capability::ClipDistance
                | s::Capability::CullDistance
                | s::Capability::SampleRateShading
                | s::Capability::SampledRect
                | s::Capability::InputAttachment
                | s::Capability::SparseResidency
                | s::Capability::MinLod
                | s::Capability::SampledCubeArray
                | s::Capability::ImageMSArray
                | s::Capability::StorageImageExtendedFormats
                | s::Capability::ImageQuery
                | s::Capability::DerivativeControl
                | s::Capability::InterpolationFunction
                | s::Capability::TransformFeedback
                | s::Capability::StorageImageReadWithoutFormat
                | s::Capability::StorageImageWriteWithoutFormat
                | s::Capability::FragmentShadingRateKHR
                | s::Capability::DrawParameters
                | s::Capability::MultiView
                | s::Capability::VariablePointersStorageBuffer
                | s::Capability::RayQueryProvisionalKHR
                | s::Capability::RayQueryKHR
                | s::Capability::RayTracingKHR
                | s::Capability::Float16ImageAMD
                | s::Capability::ImageGatherBiasLodAMD
                | s::Capability::FragmentMaskAMD
                | s::Capability::StencilExportEXT
                | s::Capability::ImageReadWriteLodAMD
                | s::Capability::Int64ImageEXT
                | s::Capability::ShaderClockKHR
                | s::Capability::FragmentFullyCoveredEXT
                | s::Capability::MeshShadingNV
                | s::Capability::FragmentDensityEXT
                | s::Capability::ShaderNonUniform
                | s::Capability::RuntimeDescriptorArray
                | s::Capability::RayTracingNV
                | s::Capability::PhysicalStorageBufferAddresses
                | s::Capability::RayTracingProvisionalKHR
                | s::Capability::CooperativeMatrixNV
                | s::Capability::FragmentShaderSampleInterlockEXT
                | s::Capability::FragmentShaderShadingRateInterlockEXT
                | s::Capability::ShaderSMBuiltinsNV
                | s::Capability::FragmentShaderPixelInterlockEXT
                | s::Capability::DemoteToHelperInvocationEXT
                | s::Capability::IntegerFunctions2INTEL
                | s::Capability::AtomicFloat32AddEXT
                | s::Capability::AtomicFloat64AddEXT => vec![spirv::Capability::Shader],
                s::Capability::UniformBufferArrayNonUniformIndexing
                | s::Capability::SampledImageArrayNonUniformIndexing
                | s::Capability::StorageBufferArrayNonUniformIndexing
                | s::Capability::StorageImageArrayNonUniformIndexing => {
                    vec![spirv::Capability::ShaderNonUniform]
                }
                s::Capability::ShaderViewportMaskNV => {
                    vec![spirv::Capability::ShaderViewportIndexLayerNV]
                }
                s::Capability::ShaderStereoViewNV => vec![spirv::Capability::ShaderViewportMaskNV],
                s::Capability::UniformAndStorageBuffer16BitAccess => vec![
                    spirv::Capability::StorageBuffer16BitAccess,
                    spirv::Capability::StorageUniformBufferBlock16,
                ],
                s::Capability::UniformAndStorageBuffer8BitAccess => {
                    vec![spirv::Capability::StorageBuffer8BitAccess]
                }
                s::Capability::TessellationPointSize => vec![spirv::Capability::Tessellation],
                s::Capability::VariablePointers => {
                    vec![spirv::Capability::VariablePointersStorageBuffer]
                }
            },
            Self::RayQueryIntersection(v) => match v {
                s::RayQueryIntersection::RayQueryCandidateIntersectionKHR
                | s::RayQueryIntersection::RayQueryCommittedIntersectionKHR => {
                    vec![spirv::Capability::RayQueryKHR]
                }
            },
            Self::RayQueryCommittedIntersectionType(v) => match v {
                s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionNoneKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionTriangleKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionGeneratedKHR => {
                    vec![spirv::Capability::RayQueryKHR]
                }
            },
            Self::RayQueryCandidateIntersectionType(v) => match v {
                s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionTriangleKHR
                | s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionAABBKHR => {
                    vec![spirv::Capability::RayQueryKHR]
                }
            },
            _ => vec![],
        }
    }
    pub fn required_extensions(&self) -> Vec<&'static str> {
        use spirv as s;
        match self {
            Self::LoopControl(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::LoopControl::INITIATION_INTERVAL_INTEL
                        | s::LoopControl::MAX_CONCURRENCY_INTEL
                        | s::LoopControl::DEPENDENCY_ARRAY_INTEL
                        | s::LoopControl::PIPELINE_ENABLE_INTEL
                        | s::LoopControl::LOOP_COALESCE_INTEL
                        | s::LoopControl::MAX_INTERLEAVING_INTEL
                        | s::LoopControl::SPECULATED_ITERATIONS_INTEL,
                ) {
                    result.extend_from_slice(&["SPV_INTEL_fpga_loop_controls"])
                };
                result
            }
            Self::MemorySemantics(v) => {
                let mut result = vec![];
                if v.intersects(s::MemorySemantics::VOLATILE) {
                    result.extend_from_slice(&["SPV_KHR_vulkan_memory_model"])
                };
                result
            }
            Self::SourceLanguage(v) => match v {
                s::SourceLanguage::Unknown
                | s::SourceLanguage::ESSL
                | s::SourceLanguage::GLSL
                | s::SourceLanguage::OpenCL_C
                | s::SourceLanguage::OpenCL_CPP
                | s::SourceLanguage::HLSL => vec![],
            },
            Self::ExecutionModel(v) => match v {
                s::ExecutionModel::Vertex
                | s::ExecutionModel::TessellationControl
                | s::ExecutionModel::TessellationEvaluation
                | s::ExecutionModel::Geometry
                | s::ExecutionModel::Fragment
                | s::ExecutionModel::GLCompute
                | s::ExecutionModel::Kernel
                | s::ExecutionModel::TaskNV
                | s::ExecutionModel::MeshNV
                | s::ExecutionModel::RayGenerationNV
                | s::ExecutionModel::IntersectionNV
                | s::ExecutionModel::AnyHitNV
                | s::ExecutionModel::ClosestHitNV
                | s::ExecutionModel::MissNV
                | s::ExecutionModel::CallableNV => vec![],
            },
            Self::AddressingModel(v) => match v {
                s::AddressingModel::Logical
                | s::AddressingModel::Physical32
                | s::AddressingModel::Physical64 => vec![],
                s::AddressingModel::PhysicalStorageBuffer64 => vec![
                    "SPV_EXT_physical_storage_buffer",
                    "SPV_KHR_physical_storage_buffer",
                ],
            },
            Self::MemoryModel(v) => match v {
                s::MemoryModel::Simple
                | s::MemoryModel::GLSL450
                | s::MemoryModel::OpenCL
                | s::MemoryModel::Vulkan => vec![],
            },
            Self::ExecutionMode(v) => match v {
                s::ExecutionMode::Invocations
                | s::ExecutionMode::SpacingEqual
                | s::ExecutionMode::SpacingFractionalEven
                | s::ExecutionMode::SpacingFractionalOdd
                | s::ExecutionMode::VertexOrderCw
                | s::ExecutionMode::VertexOrderCcw
                | s::ExecutionMode::PixelCenterInteger
                | s::ExecutionMode::OriginUpperLeft
                | s::ExecutionMode::OriginLowerLeft
                | s::ExecutionMode::EarlyFragmentTests
                | s::ExecutionMode::PointMode
                | s::ExecutionMode::Xfb
                | s::ExecutionMode::DepthReplacing
                | s::ExecutionMode::DepthGreater
                | s::ExecutionMode::DepthLess
                | s::ExecutionMode::DepthUnchanged
                | s::ExecutionMode::LocalSize
                | s::ExecutionMode::LocalSizeHint
                | s::ExecutionMode::InputPoints
                | s::ExecutionMode::InputLines
                | s::ExecutionMode::InputLinesAdjacency
                | s::ExecutionMode::Triangles
                | s::ExecutionMode::InputTrianglesAdjacency
                | s::ExecutionMode::Quads
                | s::ExecutionMode::Isolines
                | s::ExecutionMode::OutputVertices
                | s::ExecutionMode::OutputPoints
                | s::ExecutionMode::OutputLineStrip
                | s::ExecutionMode::OutputTriangleStrip
                | s::ExecutionMode::VecTypeHint
                | s::ExecutionMode::ContractionOff
                | s::ExecutionMode::Initializer
                | s::ExecutionMode::Finalizer
                | s::ExecutionMode::SubgroupSize
                | s::ExecutionMode::SubgroupsPerWorkgroup
                | s::ExecutionMode::SubgroupsPerWorkgroupId
                | s::ExecutionMode::LocalSizeId
                | s::ExecutionMode::LocalSizeHintId => vec![],
                s::ExecutionMode::PixelInterlockOrderedEXT
                | s::ExecutionMode::PixelInterlockUnorderedEXT
                | s::ExecutionMode::SampleInterlockOrderedEXT
                | s::ExecutionMode::SampleInterlockUnorderedEXT
                | s::ExecutionMode::ShadingRateInterlockOrderedEXT
                | s::ExecutionMode::ShadingRateInterlockUnorderedEXT => {
                    vec!["SPV_EXT_fragment_shader_interlock"]
                }
                s::ExecutionMode::StencilRefReplacingEXT => vec!["SPV_EXT_shader_stencil_export"],
                s::ExecutionMode::MaxWorkgroupSizeINTEL
                | s::ExecutionMode::MaxWorkDimINTEL
                | s::ExecutionMode::NoGlobalOffsetINTEL
                | s::ExecutionMode::NumSIMDWorkitemsINTEL => vec!["SPV_INTEL_kernel_attributes"],
                s::ExecutionMode::DenormPreserve
                | s::ExecutionMode::DenormFlushToZero
                | s::ExecutionMode::SignedZeroInfNanPreserve
                | s::ExecutionMode::RoundingModeRTE
                | s::ExecutionMode::RoundingModeRTZ => vec!["SPV_KHR_float_controls"],
                s::ExecutionMode::PostDepthCoverage => vec!["SPV_KHR_post_depth_coverage"],
                s::ExecutionMode::DerivativeGroupQuadsNV
                | s::ExecutionMode::DerivativeGroupLinearNV => {
                    vec!["SPV_NV_compute_shader_derivatives"]
                }
                s::ExecutionMode::OutputLinesNV
                | s::ExecutionMode::OutputPrimitivesNV
                | s::ExecutionMode::OutputTrianglesNV => vec!["SPV_NV_mesh_shader"],
            },
            Self::StorageClass(v) => match v {
                s::StorageClass::UniformConstant
                | s::StorageClass::Input
                | s::StorageClass::Uniform
                | s::StorageClass::Output
                | s::StorageClass::Workgroup
                | s::StorageClass::CrossWorkgroup
                | s::StorageClass::Private
                | s::StorageClass::Function
                | s::StorageClass::Generic
                | s::StorageClass::PushConstant
                | s::StorageClass::AtomicCounter
                | s::StorageClass::Image => vec![],
                s::StorageClass::PhysicalStorageBuffer => vec![
                    "SPV_EXT_physical_storage_buffer",
                    "SPV_KHR_physical_storage_buffer",
                ],
                s::StorageClass::CodeSectionINTEL => vec!["SPV_INTEL_function_pointers"],
                s::StorageClass::StorageBuffer => vec![
                    "SPV_KHR_storage_buffer_storage_class",
                    "SPV_KHR_variable_pointers",
                ],
                s::StorageClass::CallableDataNV
                | s::StorageClass::IncomingCallableDataNV
                | s::StorageClass::RayPayloadNV
                | s::StorageClass::HitAttributeNV
                | s::StorageClass::IncomingRayPayloadNV
                | s::StorageClass::ShaderRecordBufferNV => {
                    vec!["SPV_NV_ray_tracing", "SPV_KHR_ray_tracing"]
                }
            },
            Self::Dim(v) => match v {
                s::Dim::Dim1D
                | s::Dim::Dim2D
                | s::Dim::Dim3D
                | s::Dim::DimCube
                | s::Dim::DimRect
                | s::Dim::DimBuffer
                | s::Dim::DimSubpassData => vec![],
            },
            Self::SamplerAddressingMode(v) => match v {
                s::SamplerAddressingMode::None
                | s::SamplerAddressingMode::ClampToEdge
                | s::SamplerAddressingMode::Clamp
                | s::SamplerAddressingMode::Repeat
                | s::SamplerAddressingMode::RepeatMirrored => vec![],
            },
            Self::SamplerFilterMode(v) => match v {
                s::SamplerFilterMode::Nearest | s::SamplerFilterMode::Linear => vec![],
            },
            Self::ImageFormat(v) => match v {
                s::ImageFormat::Unknown
                | s::ImageFormat::Rgba32f
                | s::ImageFormat::Rgba16f
                | s::ImageFormat::R32f
                | s::ImageFormat::Rgba8
                | s::ImageFormat::Rgba8Snorm
                | s::ImageFormat::Rg32f
                | s::ImageFormat::Rg16f
                | s::ImageFormat::R11fG11fB10f
                | s::ImageFormat::R16f
                | s::ImageFormat::Rgba16
                | s::ImageFormat::Rgb10A2
                | s::ImageFormat::Rg16
                | s::ImageFormat::Rg8
                | s::ImageFormat::R16
                | s::ImageFormat::R8
                | s::ImageFormat::Rgba16Snorm
                | s::ImageFormat::Rg16Snorm
                | s::ImageFormat::Rg8Snorm
                | s::ImageFormat::R16Snorm
                | s::ImageFormat::R8Snorm
                | s::ImageFormat::Rgba32i
                | s::ImageFormat::Rgba16i
                | s::ImageFormat::Rgba8i
                | s::ImageFormat::R32i
                | s::ImageFormat::Rg32i
                | s::ImageFormat::Rg16i
                | s::ImageFormat::Rg8i
                | s::ImageFormat::R16i
                | s::ImageFormat::R8i
                | s::ImageFormat::Rgba32ui
                | s::ImageFormat::Rgba16ui
                | s::ImageFormat::Rgba8ui
                | s::ImageFormat::R32ui
                | s::ImageFormat::Rgb10a2ui
                | s::ImageFormat::Rg32ui
                | s::ImageFormat::Rg16ui
                | s::ImageFormat::Rg8ui
                | s::ImageFormat::R16ui
                | s::ImageFormat::R8ui
                | s::ImageFormat::R64ui
                | s::ImageFormat::R64i => vec![],
            },
            Self::ImageChannelOrder(v) => match v {
                s::ImageChannelOrder::R
                | s::ImageChannelOrder::A
                | s::ImageChannelOrder::RG
                | s::ImageChannelOrder::RA
                | s::ImageChannelOrder::RGB
                | s::ImageChannelOrder::RGBA
                | s::ImageChannelOrder::BGRA
                | s::ImageChannelOrder::ARGB
                | s::ImageChannelOrder::Intensity
                | s::ImageChannelOrder::Luminance
                | s::ImageChannelOrder::Rx
                | s::ImageChannelOrder::RGx
                | s::ImageChannelOrder::RGBx
                | s::ImageChannelOrder::Depth
                | s::ImageChannelOrder::DepthStencil
                | s::ImageChannelOrder::sRGB
                | s::ImageChannelOrder::sRGBx
                | s::ImageChannelOrder::sRGBA
                | s::ImageChannelOrder::sBGRA
                | s::ImageChannelOrder::ABGR => vec![],
            },
            Self::ImageChannelDataType(v) => match v {
                s::ImageChannelDataType::SnormInt8
                | s::ImageChannelDataType::SnormInt16
                | s::ImageChannelDataType::UnormInt8
                | s::ImageChannelDataType::UnormInt16
                | s::ImageChannelDataType::UnormShort565
                | s::ImageChannelDataType::UnormShort555
                | s::ImageChannelDataType::UnormInt101010
                | s::ImageChannelDataType::SignedInt8
                | s::ImageChannelDataType::SignedInt16
                | s::ImageChannelDataType::SignedInt32
                | s::ImageChannelDataType::UnsignedInt8
                | s::ImageChannelDataType::UnsignedInt16
                | s::ImageChannelDataType::UnsignedInt32
                | s::ImageChannelDataType::HalfFloat
                | s::ImageChannelDataType::Float
                | s::ImageChannelDataType::UnormInt24
                | s::ImageChannelDataType::UnormInt101010_2 => vec![],
            },
            Self::FPRoundingMode(v) => match v {
                s::FPRoundingMode::RTE
                | s::FPRoundingMode::RTZ
                | s::FPRoundingMode::RTP
                | s::FPRoundingMode::RTN => vec![],
            },
            Self::LinkageType(v) => match v {
                s::LinkageType::Export | s::LinkageType::Import => vec![],
            },
            Self::AccessQualifier(v) => match v {
                s::AccessQualifier::ReadOnly
                | s::AccessQualifier::WriteOnly
                | s::AccessQualifier::ReadWrite => vec![],
            },
            Self::FunctionParameterAttribute(v) => match v {
                s::FunctionParameterAttribute::Zext
                | s::FunctionParameterAttribute::Sext
                | s::FunctionParameterAttribute::ByVal
                | s::FunctionParameterAttribute::Sret
                | s::FunctionParameterAttribute::NoAlias
                | s::FunctionParameterAttribute::NoCapture
                | s::FunctionParameterAttribute::NoWrite
                | s::FunctionParameterAttribute::NoReadWrite => vec![],
            },
            Self::Decoration(v) => match v {
                s::Decoration::RelaxedPrecision
                | s::Decoration::SpecId
                | s::Decoration::Block
                | s::Decoration::BufferBlock
                | s::Decoration::RowMajor
                | s::Decoration::ColMajor
                | s::Decoration::ArrayStride
                | s::Decoration::MatrixStride
                | s::Decoration::GLSLShared
                | s::Decoration::GLSLPacked
                | s::Decoration::CPacked
                | s::Decoration::BuiltIn
                | s::Decoration::NoPerspective
                | s::Decoration::Flat
                | s::Decoration::Patch
                | s::Decoration::Centroid
                | s::Decoration::Sample
                | s::Decoration::Invariant
                | s::Decoration::Restrict
                | s::Decoration::Aliased
                | s::Decoration::Volatile
                | s::Decoration::Constant
                | s::Decoration::Coherent
                | s::Decoration::NonWritable
                | s::Decoration::NonReadable
                | s::Decoration::Uniform
                | s::Decoration::UniformId
                | s::Decoration::SaturatedConversion
                | s::Decoration::Stream
                | s::Decoration::Location
                | s::Decoration::Component
                | s::Decoration::Index
                | s::Decoration::Binding
                | s::Decoration::DescriptorSet
                | s::Decoration::Offset
                | s::Decoration::XfbBuffer
                | s::Decoration::XfbStride
                | s::Decoration::FuncParamAttr
                | s::Decoration::FPRoundingMode
                | s::Decoration::FPFastMathMode
                | s::Decoration::LinkageAttributes
                | s::Decoration::NoContraction
                | s::Decoration::InputAttachmentIndex
                | s::Decoration::Alignment
                | s::Decoration::MaxByteOffset
                | s::Decoration::AlignmentId
                | s::Decoration::MaxByteOffsetId
                | s::Decoration::ViewportRelativeNV
                | s::Decoration::NonUniform
                | s::Decoration::CounterBuffer
                | s::Decoration::UserSemantic => vec![],
                s::Decoration::ExplicitInterpAMD => {
                    vec!["SPV_AMD_shader_explicit_vertex_parameter"]
                }
                s::Decoration::RestrictPointer | s::Decoration::AliasedPointer => vec![
                    "SPV_EXT_physical_storage_buffer",
                    "SPV_KHR_physical_storage_buffer",
                ],
                s::Decoration::UserTypeGOOGLE => vec!["SPV_GOOGLE_user_type"],
                s::Decoration::RegisterINTEL
                | s::Decoration::MemoryINTEL
                | s::Decoration::NumbanksINTEL
                | s::Decoration::BankwidthINTEL
                | s::Decoration::MaxPrivateCopiesINTEL
                | s::Decoration::SinglepumpINTEL
                | s::Decoration::DoublepumpINTEL
                | s::Decoration::MaxReplicatesINTEL
                | s::Decoration::SimpleDualPortINTEL
                | s::Decoration::MergeINTEL
                | s::Decoration::BankBitsINTEL
                | s::Decoration::ForcePow2DepthINTEL => vec!["SPV_INTEL_fpga_memory_attributes"],
                s::Decoration::ReferencedIndirectlyINTEL => vec!["SPV_INTEL_function_pointers"],
                s::Decoration::NoSignedWrap | s::Decoration::NoUnsignedWrap => {
                    vec!["SPV_KHR_no_integer_wrap_decoration"]
                }
                s::Decoration::PerVertexNV => vec!["SPV_NV_fragment_shader_barycentric"],
                s::Decoration::PassthroughNV => vec!["SPV_NV_geometry_shader_passthrough"],
                s::Decoration::PerPrimitiveNV
                | s::Decoration::PerViewNV
                | s::Decoration::PerTaskNV => vec!["SPV_NV_mesh_shader"],
                s::Decoration::OverrideCoverageNV => vec!["SPV_NV_sample_mask_override_coverage"],
                s::Decoration::SecondaryViewportRelativeNV => vec!["SPV_NV_stereo_view_rendering"],
            },
            Self::BuiltIn(v) => match v {
                s::BuiltIn::Position
                | s::BuiltIn::PointSize
                | s::BuiltIn::ClipDistance
                | s::BuiltIn::CullDistance
                | s::BuiltIn::VertexId
                | s::BuiltIn::InstanceId
                | s::BuiltIn::PrimitiveId
                | s::BuiltIn::InvocationId
                | s::BuiltIn::Layer
                | s::BuiltIn::ViewportIndex
                | s::BuiltIn::TessLevelOuter
                | s::BuiltIn::TessLevelInner
                | s::BuiltIn::TessCoord
                | s::BuiltIn::PatchVertices
                | s::BuiltIn::FragCoord
                | s::BuiltIn::PointCoord
                | s::BuiltIn::FrontFacing
                | s::BuiltIn::SampleId
                | s::BuiltIn::SamplePosition
                | s::BuiltIn::SampleMask
                | s::BuiltIn::FragDepth
                | s::BuiltIn::HelperInvocation
                | s::BuiltIn::NumWorkgroups
                | s::BuiltIn::WorkgroupSize
                | s::BuiltIn::WorkgroupId
                | s::BuiltIn::LocalInvocationId
                | s::BuiltIn::GlobalInvocationId
                | s::BuiltIn::LocalInvocationIndex
                | s::BuiltIn::WorkDim
                | s::BuiltIn::GlobalSize
                | s::BuiltIn::EnqueuedWorkgroupSize
                | s::BuiltIn::GlobalOffset
                | s::BuiltIn::GlobalLinearId
                | s::BuiltIn::SubgroupSize
                | s::BuiltIn::SubgroupMaxSize
                | s::BuiltIn::NumSubgroups
                | s::BuiltIn::NumEnqueuedSubgroups
                | s::BuiltIn::SubgroupId
                | s::BuiltIn::SubgroupLocalInvocationId
                | s::BuiltIn::VertexIndex
                | s::BuiltIn::InstanceIndex
                | s::BuiltIn::SubgroupEqMask
                | s::BuiltIn::SubgroupGeMask
                | s::BuiltIn::SubgroupGtMask
                | s::BuiltIn::SubgroupLeMask
                | s::BuiltIn::SubgroupLtMask => vec![],
                s::BuiltIn::BaryCoordNoPerspAMD
                | s::BuiltIn::BaryCoordNoPerspCentroidAMD
                | s::BuiltIn::BaryCoordNoPerspSampleAMD
                | s::BuiltIn::BaryCoordSmoothAMD
                | s::BuiltIn::BaryCoordSmoothCentroidAMD
                | s::BuiltIn::BaryCoordSmoothSampleAMD
                | s::BuiltIn::BaryCoordPullModelAMD => {
                    vec!["SPV_AMD_shader_explicit_vertex_parameter"]
                }
                s::BuiltIn::FullyCoveredEXT => vec!["SPV_EXT_fragment_fully_covered"],
                s::BuiltIn::FragSizeEXT | s::BuiltIn::FragInvocationCountEXT => {
                    vec!["SPV_EXT_fragment_invocation_density", "SPV_NV_shading_rate"]
                }
                s::BuiltIn::FragStencilRefEXT => vec!["SPV_EXT_shader_stencil_export"],
                s::BuiltIn::DeviceIndex => vec!["SPV_KHR_device_group"],
                s::BuiltIn::PrimitiveShadingRateKHR | s::BuiltIn::ShadingRateKHR => {
                    vec!["SPV_KHR_fragment_shading_rate"]
                }
                s::BuiltIn::ViewIndex => vec!["SPV_KHR_multiview"],
                s::BuiltIn::RayGeometryIndexKHR => vec!["SPV_KHR_ray_tracing"],
                s::BuiltIn::BaseVertex | s::BuiltIn::BaseInstance => {
                    vec!["SPV_KHR_shader_draw_parameters"]
                }
                s::BuiltIn::DrawIndex => {
                    vec!["SPV_KHR_shader_draw_parameters", "SPV_NV_mesh_shader"]
                }
                s::BuiltIn::PositionPerViewNV | s::BuiltIn::ViewportMaskPerViewNV => vec![
                    "SPV_NVX_multiview_per_view_attributes",
                    "SPV_NV_mesh_shader",
                ],
                s::BuiltIn::BaryCoordNV | s::BuiltIn::BaryCoordNoPerspNV => {
                    vec!["SPV_NV_fragment_shader_barycentric"]
                }
                s::BuiltIn::TaskCountNV
                | s::BuiltIn::PrimitiveCountNV
                | s::BuiltIn::PrimitiveIndicesNV
                | s::BuiltIn::ClipDistancePerViewNV
                | s::BuiltIn::CullDistancePerViewNV
                | s::BuiltIn::LayerPerViewNV
                | s::BuiltIn::MeshViewCountNV
                | s::BuiltIn::MeshViewIndicesNV => vec!["SPV_NV_mesh_shader"],
                s::BuiltIn::HitTNV => vec!["SPV_NV_ray_tracing"],
                s::BuiltIn::LaunchIdNV
                | s::BuiltIn::LaunchSizeNV
                | s::BuiltIn::WorldRayOriginNV
                | s::BuiltIn::WorldRayDirectionNV
                | s::BuiltIn::ObjectRayOriginNV
                | s::BuiltIn::ObjectRayDirectionNV
                | s::BuiltIn::RayTminNV
                | s::BuiltIn::RayTmaxNV
                | s::BuiltIn::InstanceCustomIndexNV
                | s::BuiltIn::ObjectToWorldNV
                | s::BuiltIn::WorldToObjectNV
                | s::BuiltIn::HitKindNV
                | s::BuiltIn::IncomingRayFlagsNV => {
                    vec!["SPV_NV_ray_tracing", "SPV_KHR_ray_tracing"]
                }
                s::BuiltIn::WarpsPerSMNV
                | s::BuiltIn::SMCountNV
                | s::BuiltIn::WarpIDNV
                | s::BuiltIn::SMIDNV => vec!["SPV_NV_shader_sm_builtins"],
                s::BuiltIn::SecondaryPositionNV | s::BuiltIn::SecondaryViewportMaskNV => {
                    vec!["SPV_NV_stereo_view_rendering"]
                }
                s::BuiltIn::ViewportMaskNV => vec!["SPV_NV_viewport_array2", "SPV_NV_mesh_shader"],
            },
            Self::Scope(v) => match v {
                s::Scope::CrossDevice
                | s::Scope::Device
                | s::Scope::Workgroup
                | s::Scope::Subgroup
                | s::Scope::Invocation
                | s::Scope::QueueFamily
                | s::Scope::ShaderCallKHR => vec![],
            },
            Self::GroupOperation(v) => match v {
                s::GroupOperation::Reduce
                | s::GroupOperation::InclusiveScan
                | s::GroupOperation::ExclusiveScan
                | s::GroupOperation::ClusteredReduce => vec![],
                s::GroupOperation::PartitionedReduceNV
                | s::GroupOperation::PartitionedInclusiveScanNV
                | s::GroupOperation::PartitionedExclusiveScanNV => {
                    vec!["SPV_NV_shader_subgroup_partitioned"]
                }
            },
            Self::KernelEnqueueFlags(v) => match v {
                s::KernelEnqueueFlags::NoWait
                | s::KernelEnqueueFlags::WaitKernel
                | s::KernelEnqueueFlags::WaitWorkGroup => vec![],
            },
            Self::Capability(v) => match v {
                s::Capability::Matrix
                | s::Capability::Shader
                | s::Capability::Geometry
                | s::Capability::Tessellation
                | s::Capability::Addresses
                | s::Capability::Linkage
                | s::Capability::Kernel
                | s::Capability::Vector16
                | s::Capability::Float16Buffer
                | s::Capability::Float16
                | s::Capability::Float64
                | s::Capability::Int64
                | s::Capability::Int64Atomics
                | s::Capability::ImageBasic
                | s::Capability::ImageReadWrite
                | s::Capability::ImageMipmap
                | s::Capability::Pipes
                | s::Capability::DeviceEnqueue
                | s::Capability::LiteralSampler
                | s::Capability::AtomicStorage
                | s::Capability::Int16
                | s::Capability::TessellationPointSize
                | s::Capability::GeometryPointSize
                | s::Capability::ImageGatherExtended
                | s::Capability::StorageImageMultisample
                | s::Capability::UniformBufferArrayDynamicIndexing
                | s::Capability::SampledImageArrayDynamicIndexing
                | s::Capability::StorageBufferArrayDynamicIndexing
                | s::Capability::StorageImageArrayDynamicIndexing
                | s::Capability::ClipDistance
                | s::Capability::CullDistance
                | s::Capability::ImageCubeArray
                | s::Capability::SampleRateShading
                | s::Capability::ImageRect
                | s::Capability::SampledRect
                | s::Capability::GenericPointer
                | s::Capability::Int8
                | s::Capability::InputAttachment
                | s::Capability::SparseResidency
                | s::Capability::MinLod
                | s::Capability::Sampled1D
                | s::Capability::Image1D
                | s::Capability::SampledCubeArray
                | s::Capability::SampledBuffer
                | s::Capability::ImageBuffer
                | s::Capability::ImageMSArray
                | s::Capability::StorageImageExtendedFormats
                | s::Capability::ImageQuery
                | s::Capability::DerivativeControl
                | s::Capability::InterpolationFunction
                | s::Capability::TransformFeedback
                | s::Capability::GeometryStreams
                | s::Capability::StorageImageReadWithoutFormat
                | s::Capability::StorageImageWriteWithoutFormat
                | s::Capability::MultiViewport
                | s::Capability::SubgroupDispatch
                | s::Capability::NamedBarrier
                | s::Capability::PipeStorage
                | s::Capability::GroupNonUniform
                | s::Capability::GroupNonUniformVote
                | s::Capability::GroupNonUniformArithmetic
                | s::Capability::GroupNonUniformBallot
                | s::Capability::GroupNonUniformShuffle
                | s::Capability::GroupNonUniformShuffleRelative
                | s::Capability::GroupNonUniformClustered
                | s::Capability::GroupNonUniformQuad
                | s::Capability::ShaderLayer
                | s::Capability::ShaderViewportIndex
                | s::Capability::ShaderNonUniform
                | s::Capability::RuntimeDescriptorArray
                | s::Capability::InputAttachmentArrayDynamicIndexing
                | s::Capability::UniformTexelBufferArrayDynamicIndexing
                | s::Capability::StorageTexelBufferArrayDynamicIndexing
                | s::Capability::UniformBufferArrayNonUniformIndexing
                | s::Capability::SampledImageArrayNonUniformIndexing
                | s::Capability::StorageBufferArrayNonUniformIndexing
                | s::Capability::StorageImageArrayNonUniformIndexing
                | s::Capability::InputAttachmentArrayNonUniformIndexing
                | s::Capability::UniformTexelBufferArrayNonUniformIndexing
                | s::Capability::StorageTexelBufferArrayNonUniformIndexing
                | s::Capability::VulkanMemoryModel
                | s::Capability::VulkanMemoryModelDeviceScope => vec![],
                s::Capability::Float16ImageAMD => vec!["SPV_AMD_gpu_shader_half_float_fetch"],
                s::Capability::Groups => vec!["SPV_AMD_shader_ballot"],
                s::Capability::FragmentMaskAMD => vec!["SPV_AMD_shader_fragment_mask"],
                s::Capability::ImageReadWriteLodAMD => vec!["SPV_AMD_shader_image_load_store_lod"],
                s::Capability::ImageGatherBiasLodAMD => vec!["SPV_AMD_texture_gather_bias_lod"],
                s::Capability::DemoteToHelperInvocationEXT => {
                    vec!["SPV_EXT_demote_to_helper_invocation"]
                }
                s::Capability::FragmentFullyCoveredEXT => vec!["SPV_EXT_fragment_fully_covered"],
                s::Capability::FragmentDensityEXT => {
                    vec!["SPV_EXT_fragment_invocation_density", "SPV_NV_shading_rate"]
                }
                s::Capability::FragmentShaderSampleInterlockEXT
                | s::Capability::FragmentShaderShadingRateInterlockEXT
                | s::Capability::FragmentShaderPixelInterlockEXT => {
                    vec!["SPV_EXT_fragment_shader_interlock"]
                }
                s::Capability::PhysicalStorageBufferAddresses => vec![
                    "SPV_EXT_physical_storage_buffer",
                    "SPV_KHR_physical_storage_buffer",
                ],
                s::Capability::AtomicFloat32AddEXT | s::Capability::AtomicFloat64AddEXT => {
                    vec!["SPV_EXT_shader_atomic_float_add"]
                }
                s::Capability::Int64ImageEXT => vec!["SPV_EXT_shader_image_int64"],
                s::Capability::StencilExportEXT => vec!["SPV_EXT_shader_stencil_export"],
                s::Capability::ShaderViewportIndexLayerEXT => {
                    vec!["SPV_EXT_shader_viewport_index_layer"]
                }
                s::Capability::BlockingPipesINTEL => vec!["SPV_INTEL_blocking_pipes"],
                s::Capability::SubgroupAvcMotionEstimationINTEL
                | s::Capability::SubgroupAvcMotionEstimationIntraINTEL
                | s::Capability::SubgroupAvcMotionEstimationChromaINTEL => {
                    vec!["SPV_INTEL_device_side_avc_motion_estimation"]
                }
                s::Capability::FPGALoopControlsINTEL => vec!["SPV_INTEL_fpga_loop_controls"],
                s::Capability::FPGAMemoryAttributesINTEL => {
                    vec!["SPV_INTEL_fpga_memory_attributes"]
                }
                s::Capability::FPGARegINTEL => vec!["SPV_INTEL_fpga_reg"],
                s::Capability::FunctionPointersINTEL | s::Capability::IndirectReferencesINTEL => {
                    vec!["SPV_INTEL_function_pointers"]
                }
                s::Capability::KernelAttributesINTEL | s::Capability::FPGAKernelAttributesINTEL => {
                    vec!["SPV_INTEL_kernel_attributes"]
                }
                s::Capability::SubgroupImageMediaBlockIOINTEL => vec!["SPV_INTEL_media_block_io"],
                s::Capability::IntegerFunctions2INTEL => {
                    vec!["SPV_INTEL_shader_integer_functions2"]
                }
                s::Capability::SubgroupShuffleINTEL
                | s::Capability::SubgroupBufferBlockIOINTEL
                | s::Capability::SubgroupImageBlockIOINTEL => vec!["SPV_INTEL_subgroups"],
                s::Capability::UnstructuredLoopControlsINTEL => {
                    vec!["SPV_INTEL_unstructured_loop_controls"]
                }
                s::Capability::StorageBuffer16BitAccess
                | s::Capability::UniformAndStorageBuffer16BitAccess
                | s::Capability::StoragePushConstant16
                | s::Capability::StorageInputOutput16 => vec!["SPV_KHR_16bit_storage"],
                s::Capability::StorageBuffer8BitAccess
                | s::Capability::UniformAndStorageBuffer8BitAccess
                | s::Capability::StoragePushConstant8 => vec!["SPV_KHR_8bit_storage"],
                s::Capability::DeviceGroup => vec!["SPV_KHR_device_group"],
                s::Capability::DenormPreserve
                | s::Capability::DenormFlushToZero
                | s::Capability::SignedZeroInfNanPreserve
                | s::Capability::RoundingModeRTE
                | s::Capability::RoundingModeRTZ => vec!["SPV_KHR_float_controls"],
                s::Capability::FragmentShadingRateKHR => vec!["SPV_KHR_fragment_shading_rate"],
                s::Capability::MultiView => vec!["SPV_KHR_multiview"],
                s::Capability::SampleMaskPostDepthCoverage => vec!["SPV_KHR_post_depth_coverage"],
                s::Capability::RayQueryProvisionalKHR | s::Capability::RayQueryKHR => {
                    vec!["SPV_KHR_ray_query"]
                }
                s::Capability::RayTraversalPrimitiveCullingKHR => {
                    vec!["SPV_KHR_ray_query", "SPV_KHR_ray_tracing"]
                }
                s::Capability::RayTracingKHR | s::Capability::RayTracingProvisionalKHR => {
                    vec!["SPV_KHR_ray_tracing"]
                }
                s::Capability::AtomicStorageOps => vec!["SPV_KHR_shader_atomic_counter_ops"],
                s::Capability::SubgroupBallotKHR => vec!["SPV_KHR_shader_ballot"],
                s::Capability::ShaderClockKHR => vec!["SPV_KHR_shader_clock"],
                s::Capability::DrawParameters => vec!["SPV_KHR_shader_draw_parameters"],
                s::Capability::SubgroupVoteKHR => vec!["SPV_KHR_subgroup_vote"],
                s::Capability::VariablePointersStorageBuffer | s::Capability::VariablePointers => {
                    vec!["SPV_KHR_variable_pointers"]
                }
                s::Capability::PerViewAttributesNV => vec!["SPV_NVX_multiview_per_view_attributes"],
                s::Capability::ComputeDerivativeGroupQuadsNV
                | s::Capability::ComputeDerivativeGroupLinearNV => {
                    vec!["SPV_NV_compute_shader_derivatives"]
                }
                s::Capability::CooperativeMatrixNV => vec!["SPV_NV_cooperative_matrix"],
                s::Capability::FragmentBarycentricNV => vec!["SPV_NV_fragment_shader_barycentric"],
                s::Capability::GeometryShaderPassthroughNV => {
                    vec!["SPV_NV_geometry_shader_passthrough"]
                }
                s::Capability::MeshShadingNV => vec!["SPV_NV_mesh_shader"],
                s::Capability::RayTracingNV => vec!["SPV_NV_ray_tracing"],
                s::Capability::SampleMaskOverrideCoverageNV => {
                    vec!["SPV_NV_sample_mask_override_coverage"]
                }
                s::Capability::ImageFootprintNV => vec!["SPV_NV_shader_image_footprint"],
                s::Capability::ShaderSMBuiltinsNV => vec!["SPV_NV_shader_sm_builtins"],
                s::Capability::GroupNonUniformPartitionedNV => {
                    vec!["SPV_NV_shader_subgroup_partitioned"]
                }
                s::Capability::ShaderStereoViewNV => vec!["SPV_NV_stereo_view_rendering"],
                s::Capability::ShaderViewportMaskNV => vec!["SPV_NV_viewport_array2"],
            },
            Self::RayQueryIntersection(v) => match v {
                s::RayQueryIntersection::RayQueryCandidateIntersectionKHR
                | s::RayQueryIntersection::RayQueryCommittedIntersectionKHR => vec![],
            },
            Self::RayQueryCommittedIntersectionType(v) => match v {
                s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionNoneKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionTriangleKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionGeneratedKHR => {
                    vec![]
                }
            },
            Self::RayQueryCandidateIntersectionType(v) => match v {
                s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionTriangleKHR
                | s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionAABBKHR => {
                    vec![]
                }
            },
            _ => vec![],
        }
    }
    pub fn additional_operands(&self) -> Vec<crate::grammar::LogicalOperand> {
        use spirv as s;
        match self {
            Self::ImageOperands(v) => {
                let mut result = vec![];
                result.extend(
                    [
                        s::ImageOperands::BIAS,
                        s::ImageOperands::LOD,
                        s::ImageOperands::CONST_OFFSET,
                        s::ImageOperands::OFFSET,
                        s::ImageOperands::CONST_OFFSETS,
                        s::ImageOperands::SAMPLE,
                        s::ImageOperands::MIN_LOD,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdRef,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result.extend(
                    [s::ImageOperands::GRAD]
                        .iter()
                        .filter(|arg| v.contains(**arg))
                        .flat_map(|_| {
                            [
                                crate::grammar::LogicalOperand {
                                    kind: crate::grammar::OperandKind::IdRef,
                                    quantifier: crate::grammar::OperandQuantifier::One,
                                },
                                crate::grammar::LogicalOperand {
                                    kind: crate::grammar::OperandKind::IdRef,
                                    quantifier: crate::grammar::OperandQuantifier::One,
                                },
                            ]
                            .iter()
                            .cloned()
                        }),
                );
                result.extend(
                    [
                        s::ImageOperands::MAKE_TEXEL_AVAILABLE,
                        s::ImageOperands::MAKE_TEXEL_VISIBLE,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdScope,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result
            }
            Self::LoopControl(v) => {
                let mut result = vec![];
                result.extend(
                    [
                        s::LoopControl::DEPENDENCY_LENGTH,
                        s::LoopControl::MIN_ITERATIONS,
                        s::LoopControl::MAX_ITERATIONS,
                        s::LoopControl::ITERATION_MULTIPLE,
                        s::LoopControl::PEEL_COUNT,
                        s::LoopControl::PARTIAL_COUNT,
                        s::LoopControl::INITIATION_INTERVAL_INTEL,
                        s::LoopControl::MAX_CONCURRENCY_INTEL,
                        s::LoopControl::DEPENDENCY_ARRAY_INTEL,
                        s::LoopControl::PIPELINE_ENABLE_INTEL,
                        s::LoopControl::LOOP_COALESCE_INTEL,
                        s::LoopControl::MAX_INTERLEAVING_INTEL,
                        s::LoopControl::SPECULATED_ITERATIONS_INTEL,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::LiteralInteger,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result
            }
            Self::MemoryAccess(v) => {
                let mut result = vec![];
                result.extend(
                    [
                        s::MemoryAccess::MAKE_POINTER_AVAILABLE,
                        s::MemoryAccess::MAKE_POINTER_VISIBLE,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdScope,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result.extend(
                    [s::MemoryAccess::ALIGNED]
                        .iter()
                        .filter(|arg| v.contains(**arg))
                        .flat_map(|_| {
                            [crate::grammar::LogicalOperand {
                                kind: crate::grammar::OperandKind::LiteralInteger,
                                quantifier: crate::grammar::OperandQuantifier::One,
                            }]
                            .iter()
                            .cloned()
                        }),
                );
                result
            }
            Self::ExecutionMode(v) => match v {
                s::ExecutionMode::LocalSizeHintId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::SubgroupsPerWorkgroupId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::LocalSizeId => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::Invocations => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::OutputPrimitivesNV => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::SubgroupSize => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::SubgroupsPerWorkgroup => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::DenormPreserve
                | s::ExecutionMode::DenormFlushToZero
                | s::ExecutionMode::SignedZeroInfNanPreserve
                | s::ExecutionMode::RoundingModeRTE
                | s::ExecutionMode::RoundingModeRTZ => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::VecTypeHint => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::OutputVertices => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::MaxWorkDimINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::MaxWorkgroupSizeINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::NumSIMDWorkitemsINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::LocalSize | s::ExecutionMode::LocalSizeHint => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                _ => vec![],
            },
            Self::Decoration(v) => match v {
                s::Decoration::BuiltIn => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::BuiltIn,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FPFastMathMode => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::FPFastMathMode,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FPRoundingMode => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::FPRoundingMode,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FuncParamAttr => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::FunctionParameterAttribute,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::AlignmentId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::CounterBuffer => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxByteOffsetId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::UniformId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdScope,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Alignment => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::ArrayStride => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::InputAttachmentIndex => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::BankBitsINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::ZeroOrMore,
                }],
                s::Decoration::BankwidthINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::NumbanksINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Binding => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Offset => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Component => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::DescriptorSet => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::ForcePow2DepthINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Index => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Location => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MatrixStride => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxByteOffset => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxPrivateCopiesINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxReplicatesINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::SecondaryViewportRelativeNV => {
                    vec![crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    }]
                }
                s::Decoration::SpecId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Stream => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::XfbBuffer => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::XfbStride => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MemoryINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralString,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MergeINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralString,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralString,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::LinkageAttributes => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralString,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LinkageType,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::UserSemantic => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralString,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::UserTypeGOOGLE => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralString,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                _ => vec![],
            },
            _ => vec![],
        }
    }
}
