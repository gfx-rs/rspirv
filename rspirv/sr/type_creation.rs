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

impl Context {
    pub fn type_void(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Void,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_bool(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Bool,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_int(&mut self, width: u32, signedness: u32) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Int {
                width: width,
                signedness: signedness,
            },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_float(&mut self, width: u32) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Float { width: width },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_vector(&mut self, component_type: TypeToken, component_count: u32) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Vector {
                component_type: component_type,
                component_count: component_count,
            },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_matrix(&mut self, column_type: TypeToken, column_count: u32) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Matrix {
                column_type: column_type,
                column_count: column_count,
            },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_image(
        &mut self,
        sampled_type: TypeToken,
        dim: spirv::Dim,
        depth: u32,
        arrayed: u32,
        ms: u32,
        sampled: u32,
        image_format: spirv::ImageFormat,
        access_qualifier: Option<spirv::AccessQualifier>,
    ) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Image {
                sampled_type: sampled_type,
                dim: dim,
                depth: depth,
                arrayed: arrayed,
                ms: ms,
                sampled: sampled,
                image_format: image_format,
                access_qualifier: access_qualifier,
            },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_sampler(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Sampler,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_sampled_image(&mut self, image_type: TypeToken) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::SampledImage { image_type: image_type },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_array(&mut self, element_type: TypeToken, length: TypeToken) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Array {
                element_type: element_type,
                length: length,
            },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_runtime_array(&mut self, element_type: TypeToken) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::RuntimeArray { element_type: element_type },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_structure(&mut self, field_types: Vec<TypeToken>) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Struct { field_types: field_types },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_opaque(&mut self, type_name: String) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Opaque { type_name: type_name },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_pointer(
        &mut self,
        storage_class: spirv::StorageClass,
        pointee_type: TypeToken,
    ) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Pointer {
                storage_class: storage_class,
                pointee_type: pointee_type,
            },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_function(
        &mut self,
        return_type: TypeToken,
        parameter_types: Vec<TypeToken>,
    ) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Function {
                return_type: return_type,
                parameter_types: parameter_types,
            },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_event(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Event,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_device_event(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::DeviceEvent,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_reserve_id(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::ReserveId,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_queue(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Queue,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_pipe(&mut self, qualifier: spirv::AccessQualifier) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Pipe { qualifier: qualifier },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_forward_pointer(&mut self, storage_class: spirv::StorageClass) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::ForwardPointer { storage_class: storage_class },
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_pipe_storage(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::PipeStorage,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
    pub fn type_named_barrier(&mut self) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::NamedBarrier,
            decorations: BTreeSet::new(),
        });
        return TypeToken::new(self.types.len() as u32);
    }
}
