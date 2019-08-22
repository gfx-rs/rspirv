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
        field_types: Vec<StructMember>,
    },
    Opaque {
        type_name: String,
    },
    Pointer {
        storage_class: spirv::StorageClass,
        pointee_type: Token<Type>,
    },
    Function {
        return_type: Token<Type>,
        parameter_types: Vec<Token<Type>>,
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
}
