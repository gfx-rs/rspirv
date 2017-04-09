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

use spirv;
# [ derive ( Debug , Eq , PartialEq , From ) ]
pub enum Ty {
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
        access_qualifier: spirv::AccessQualifier,
    },
    Sampler,
    SampledImage { image_type: spirv::Word },
    Array {
        element_type: spirv::Word,
        length: spirv::Word,
    },
    RuntimeArray { element_type: spirv::Word },
    Struct { field_types: spirv::Word },
    Opaque { type_name: String },
    Pointer {
        storage_class: spirv::StorageClass,
        pointee_type: spirv::Word,
    },
    Function {
        return_type: spirv::Word,
        parameter_types: spirv::Word,
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
