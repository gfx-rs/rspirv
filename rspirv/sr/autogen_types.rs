// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub enum Type {
    Void,
    Bool,
    Int {
        width: u32,
        signedness: u32,
    },
    Float {
        width: u32,
        floating_point_encoding: Option<spirv::FPEncoding>,
    },
    Vector {
        component_type: Token<Type>,
        component_count: u32,
    },
    Matrix {
        column_type: Token<Type>,
        column_count: u32,
    },
    Image {
        sampled_type: Token<Type>,
        dim: spirv::Dim,
        depth: u32,
        arrayed: u32,
        ms: u32,
        sampled: u32,
        image_format: spirv::ImageFormat,
        access_qualifier: Option<spirv::AccessQualifier>,
    },
    Sampler,
    SampledImage {
        image_type: Token<Type>,
    },
    Array {
        element_type: Token<Type>,
        length: Token<Constant>,
    },
    RuntimeArray {
        element_type: Token<Type>,
    },
    Struct {
        member_0_type_member_1_type: Vec<StructMember>,
    },
    Opaque {
        the_name_of_the_opaque_type: String,
    },
    Pointer {
        storage_class: spirv::StorageClass,
        ty: Token<Type>,
    },
    Function {
        return_type: Token<Type>,
        parameter_0_type_parameter_1_type: Vec<Token<Type>>,
    },
    Event,
    DeviceEvent,
    ReserveId,
    Queue,
    Pipe {
        qualifier: spirv::AccessQualifier,
    },
    ForwardPointer {
        pointer_type: Token<Type>,
        storage_class: spirv::StorageClass,
    },
    PipeStorage,
    NamedBarrier,
    TensorARM {
        element_type: Token<Type>,
        rank: Option<spirv::Word>,
        shape: Option<spirv::Word>,
    },
    GraphARM {
        num_inputs: u32,
        in_out_types: Vec<spirv::Word>,
    },
    UntypedPointerKHR {
        storage_class: spirv::StorageClass,
    },
    CooperativeMatrixKHR {
        component_type: Token<Type>,
        scope: spirv::Word,
        rows: spirv::Word,
        columns: spirv::Word,
        usage: spirv::Word,
    },
    RayQueryKHR,
    NodePayloadArrayAMDX {
        payload_type: spirv::Word,
    },
    HitObjectNV,
    CooperativeVectorNV {
        component_type: Token<Type>,
        component_count: spirv::Word,
    },
    AccelerationStructureKHR,
    CooperativeMatrixNV {
        component_type: Token<Type>,
        execution: spirv::Word,
        rows: spirv::Word,
        columns: spirv::Word,
    },
    TensorLayoutNV {
        dim: spirv::Word,
        clamp_mode: spirv::Word,
    },
    TensorViewNV {
        dim: spirv::Word,
        has_dimensions: spirv::Word,
        p: Vec<spirv::Word>,
    },
    BufferSurfaceINTEL {
        access_qualifier: spirv::AccessQualifier,
    },
    StructContinuedINTEL {
        member_0_type_member_1_type: Vec<spirv::Word>,
    },
    TaskSequenceINTEL,
}
