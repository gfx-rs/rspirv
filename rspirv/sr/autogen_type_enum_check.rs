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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::sr) enum TypeEnum {
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
        component_type: Token<super::types::Type>,
        component_count: u32,
    },
    Matrix {
        column_type: Token<super::types::Type>,
        column_count: u32,
    },
    Image {
        sampled_type: Token<super::types::Type>,
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
        image_type: Token<super::types::Type>,
    },
    Array {
        element_type: Token<super::types::Type>,
        length: Token<super::Constant>,
    },
    RuntimeArray {
        element_type: Token<super::types::Type>,
    },
    Struct {
        field_types: Vec<super::types::StructMember>,
    },
    Opaque {
        type_name: String,
    },
    Pointer {
        storage_class: spirv::StorageClass,
        pointee_type: Token<super::types::Type>,
    },
    Event,
    DeviceEvent,
    ReserveId,
    Queue,
    Pipe {
        qualifier: spirv::AccessQualifier,
    },
    ForwardPointer {
        storage_class: spirv::StorageClass,
    },
    PipeStorage,
    NamedBarrier,
}
impl Type {
    pub fn is_void_type(&self) -> bool {
        match self.ty {
            TypeEnum::Void => true,
            _ => false,
        }
    }
    pub fn is_bool_type(&self) -> bool {
        match self.ty {
            TypeEnum::Bool => true,
            _ => false,
        }
    }
    pub fn is_int_type(&self) -> bool {
        match self.ty {
            TypeEnum::Int { .. } => true,
            _ => false,
        }
    }
    pub fn is_float_type(&self) -> bool {
        match self.ty {
            TypeEnum::Float { .. } => true,
            _ => false,
        }
    }
    pub fn is_vector_type(&self) -> bool {
        match self.ty {
            TypeEnum::Vector { .. } => true,
            _ => false,
        }
    }
    pub fn is_matrix_type(&self) -> bool {
        match self.ty {
            TypeEnum::Matrix { .. } => true,
            _ => false,
        }
    }
    pub fn is_image_type(&self) -> bool {
        match self.ty {
            TypeEnum::Image { .. } => true,
            _ => false,
        }
    }
    pub fn is_sampler_type(&self) -> bool {
        match self.ty {
            TypeEnum::Sampler => true,
            _ => false,
        }
    }
    pub fn is_sampled_image_type(&self) -> bool {
        match self.ty {
            TypeEnum::SampledImage { .. } => true,
            _ => false,
        }
    }
    pub fn is_array_type(&self) -> bool {
        match self.ty {
            TypeEnum::Array { .. } => true,
            _ => false,
        }
    }
    pub fn is_runtime_array_type(&self) -> bool {
        match self.ty {
            TypeEnum::RuntimeArray { .. } => true,
            _ => false,
        }
    }
    pub fn is_structure_type(&self) -> bool {
        match self.ty {
            TypeEnum::Struct { .. } => true,
            _ => false,
        }
    }
    pub fn is_opaque_type(&self) -> bool {
        match self.ty {
            TypeEnum::Opaque { .. } => true,
            _ => false,
        }
    }
    pub fn is_pointer_type(&self) -> bool {
        match self.ty {
            TypeEnum::Pointer { .. } => true,
            _ => false,
        }
    }
    pub fn is_event_type(&self) -> bool {
        match self.ty {
            TypeEnum::Event => true,
            _ => false,
        }
    }
    pub fn is_device_event_type(&self) -> bool {
        match self.ty {
            TypeEnum::DeviceEvent => true,
            _ => false,
        }
    }
    pub fn is_reserve_id_type(&self) -> bool {
        match self.ty {
            TypeEnum::ReserveId => true,
            _ => false,
        }
    }
    pub fn is_queue_type(&self) -> bool {
        match self.ty {
            TypeEnum::Queue => true,
            _ => false,
        }
    }
    pub fn is_pipe_type(&self) -> bool {
        match self.ty {
            TypeEnum::Pipe { .. } => true,
            _ => false,
        }
    }
    pub fn is_forward_pointer_type(&self) -> bool {
        match self.ty {
            TypeEnum::ForwardPointer { .. } => true,
            _ => false,
        }
    }
    pub fn is_pipe_storage_type(&self) -> bool {
        match self.ty {
            TypeEnum::PipeStorage => true,
            _ => false,
        }
    }
    pub fn is_named_barrier_type(&self) -> bool {
        match self.ty {
            TypeEnum::NamedBarrier => true,
            _ => false,
        }
    }
}
