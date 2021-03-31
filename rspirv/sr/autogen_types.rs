// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

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
    AccelerationStructureKHR,
}
