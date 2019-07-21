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
    pub fn type_void(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Void,
            decorations: Vec::new(),
        })
    }
    pub fn type_bool(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Bool,
            decorations: Vec::new(),
        })
    }
    pub fn type_int(&mut self, width: u32, signedness: u32) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Int { width, signedness },
            decorations: Vec::new(),
        })
    }
    pub fn type_float(&mut self, width: u32) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Float { width },
            decorations: Vec::new(),
        })
    }
    pub fn type_vector(
        &mut self,
        component_type: Token<Type>,
        component_count: u32,
    ) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Vector {
                component_type,
                component_count,
            },
            decorations: Vec::new(),
        })
    }
    pub fn type_matrix(&mut self, column_type: Token<Type>, column_count: u32) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Matrix {
                column_type,
                column_count,
            },
            decorations: Vec::new(),
        })
    }
    pub fn type_image(
        &mut self,
        sampled_type: Token<Type>,
        dim: spirv::Dim,
        depth: u32,
        arrayed: u32,
        ms: u32,
        sampled: u32,
        image_format: spirv::ImageFormat,
        access_qualifier: Option<spirv::AccessQualifier>,
    ) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Image {
                sampled_type,
                dim,
                depth,
                arrayed,
                ms,
                sampled,
                image_format,
                access_qualifier,
            },
            decorations: Vec::new(),
        })
    }
    pub fn type_sampler(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Sampler,
            decorations: Vec::new(),
        })
    }
    pub fn type_sampled_image(&mut self, image_type: Token<Type>) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::SampledImage { image_type },
            decorations: Vec::new(),
        })
    }
    pub fn type_array(
        &mut self,
        element_type: Token<Type>,
        length: Token<Constant>,
    ) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Array {
                element_type,
                length,
            },
            decorations: Vec::new(),
        })
    }
    pub fn type_runtime_array(&mut self, element_type: Token<Type>) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::RuntimeArray { element_type },
            decorations: Vec::new(),
        })
    }
    pub fn type_opaque(&mut self, type_name: String) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Opaque { type_name },
            decorations: Vec::new(),
        })
    }
    pub fn type_pointer(
        &mut self,
        storage_class: spirv::StorageClass,
        pointee_type: Token<Type>,
    ) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Pointer {
                storage_class,
                pointee_type,
            },
            decorations: Vec::new(),
        })
    }
    pub fn type_function(
        &mut self,
        return_type: Token<Type>,
        parameter_types: Vec<Token<Type>>,
    ) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Function {
                return_type,
                parameter_types,
            },
            decorations: Vec::new(),
        })
    }
    pub fn type_event(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Event,
            decorations: Vec::new(),
        })
    }
    pub fn type_device_event(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::DeviceEvent,
            decorations: Vec::new(),
        })
    }
    pub fn type_reserve_id(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::ReserveId,
            decorations: Vec::new(),
        })
    }
    pub fn type_queue(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Queue,
            decorations: Vec::new(),
        })
    }
    pub fn type_pipe(&mut self, qualifier: spirv::AccessQualifier) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::Pipe { qualifier },
            decorations: Vec::new(),
        })
    }
    pub fn type_forward_pointer(&mut self, storage_class: spirv::StorageClass) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::ForwardPointer { storage_class },
            decorations: Vec::new(),
        })
    }
    pub fn type_pipe_storage(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::PipeStorage,
            decorations: Vec::new(),
        })
    }
    pub fn type_named_barrier(&mut self) -> Token<Type> {
        self.types.fetch_or_append(Type {
            ty: TypeEnum::NamedBarrier,
            decorations: Vec::new(),
        })
    }
}
