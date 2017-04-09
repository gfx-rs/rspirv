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

# [ derive ( Debug , Eq , PartialEq ) ]
enum Ty {
    Void,
    Bool,
    Int { width: u32, signedness: u32 },
    Float { width: u32 },
    Vector {
        component_type: spirv::Word,
        component_count: u32,
    },
    Matrix {
        column_type: spirv::Word,
        column_count: u32,
    },
    Image {
        sampled_type: spirv::Word,
        dim: spirv::Dim,
        depth: u32,
        arrayed: u32,
        ms: u32,
        sampled: u32,
        image_format: spirv::ImageFormat,
        access_qualifier: Option<spirv::AccessQualifier>,
    },
    Sampler,
    SampledImage { image_type: spirv::Word },
    Array {
        element_type: spirv::Word,
        length: spirv::Word,
    },
    RuntimeArray { element_type: spirv::Word },
    Struct { field_types: Vec<spirv::Word> },
    Opaque { type_name: String },
    Pointer {
        storage_class: spirv::StorageClass,
        pointee_type: spirv::Word,
    },
    Function {
        return_type: spirv::Word,
        parameter_types: Vec<spirv::Word>,
    },
    Event,
    DeviceEvent,
    ReserveId,
    Queue,
    Pipe { qualifier: spirv::AccessQualifier },
    ForwardPointer { storage_class: spirv::StorageClass },
    PipeStorage,
    NamedBarrier,
}
impl Type {
    pub fn void() -> Type {
        Type { ty: Ty::Void }
    }
    pub fn bool() -> Type {
        Type { ty: Ty::Bool }
    }
    pub fn int(width: u32, signedness: u32) -> Type {
        Type {
            ty: Ty::Int {
                width: width,
                signedness: signedness,
            },
        }
    }
    pub fn float(width: u32) -> Type {
        Type { ty: Ty::Float { width: width } }
    }
    pub fn vector(component_type: spirv::Word, component_count: u32) -> Type {
        Type {
            ty: Ty::Vector {
                component_type: component_type,
                component_count: component_count,
            },
        }
    }
    pub fn matrix(column_type: spirv::Word, column_count: u32) -> Type {
        Type {
            ty: Ty::Matrix {
                column_type: column_type,
                column_count: column_count,
            },
        }
    }
    pub fn image(sampled_type: spirv::Word,
                 dim: spirv::Dim,
                 depth: u32,
                 arrayed: u32,
                 ms: u32,
                 sampled: u32,
                 image_format: spirv::ImageFormat,
                 access_qualifier: Option<spirv::AccessQualifier>)
                 -> Type {
        Type {
            ty: Ty::Image {
                sampled_type: sampled_type,
                dim: dim,
                depth: depth,
                arrayed: arrayed,
                ms: ms,
                sampled: sampled,
                image_format: image_format,
                access_qualifier: access_qualifier,
            },
        }
    }
    pub fn sampler() -> Type {
        Type { ty: Ty::Sampler }
    }
    pub fn sampled_image(image_type: spirv::Word) -> Type {
        Type { ty: Ty::SampledImage { image_type: image_type } }
    }
    pub fn array(element_type: spirv::Word, length: spirv::Word) -> Type {
        Type {
            ty: Ty::Array {
                element_type: element_type,
                length: length,
            },
        }
    }
    pub fn runtime_array(element_type: spirv::Word) -> Type {
        Type { ty: Ty::RuntimeArray { element_type: element_type } }
    }
    pub fn structure(field_types: Vec<spirv::Word>) -> Type {
        Type { ty: Ty::Struct { field_types: field_types } }
    }
    pub fn opaque(type_name: String) -> Type {
        Type { ty: Ty::Opaque { type_name: type_name } }
    }
    pub fn pointer(storage_class: spirv::StorageClass, pointee_type: spirv::Word) -> Type {
        Type {
            ty: Ty::Pointer {
                storage_class: storage_class,
                pointee_type: pointee_type,
            },
        }
    }
    pub fn function(return_type: spirv::Word, parameter_types: Vec<spirv::Word>) -> Type {
        Type {
            ty: Ty::Function {
                return_type: return_type,
                parameter_types: parameter_types,
            },
        }
    }
    pub fn event() -> Type {
        Type { ty: Ty::Event }
    }
    pub fn device_event() -> Type {
        Type { ty: Ty::DeviceEvent }
    }
    pub fn reserve_id() -> Type {
        Type { ty: Ty::ReserveId }
    }
    pub fn queue() -> Type {
        Type { ty: Ty::Queue }
    }
    pub fn pipe(qualifier: spirv::AccessQualifier) -> Type {
        Type { ty: Ty::Pipe { qualifier: qualifier } }
    }
    pub fn forward_pointer(storage_class: spirv::StorageClass) -> Type {
        Type { ty: Ty::ForwardPointer { storage_class: storage_class } }
    }
    pub fn pipe_storage() -> Type {
        Type { ty: Ty::PipeStorage }
    }
    pub fn named_barrier() -> Type {
        Type { ty: Ty::NamedBarrier }
    }
}
