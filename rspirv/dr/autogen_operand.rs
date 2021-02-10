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
}
