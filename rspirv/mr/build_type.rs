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

impl Builder {
    /// Appends an OpTypeVoid instruction and returns the result id.
    pub fn type_void(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeVoid, None, Some(id), vec![]));
        id
    }

    /// Appends an OpTypeBool instruction and returns the result id.
    pub fn type_bool(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeBool, None, Some(id), vec![]));
        id
    }

    /// Appends an OpTypeInt instruction and returns the result id.
    pub fn type_int(&mut self, width: u32, signedness: u32) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeInt, None, Some(id), vec![mr::Operand::LiteralInt32(width), mr::Operand::LiteralInt32(signedness)]));
        id
    }

    /// Appends an OpTypeFloat instruction and returns the result id.
    pub fn type_float(&mut self, width: u32) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeFloat, None, Some(id), vec![mr::Operand::LiteralInt32(width)]));
        id
    }

    /// Appends an OpTypeVector instruction and returns the result id.
    pub fn type_vector(&mut self, component_type: spirv::Word, component_count: u32) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeVector, None, Some(id), vec![mr::Operand::IdRef(component_type), mr::Operand::LiteralInt32(component_count)]));
        id
    }

    /// Appends an OpTypeMatrix instruction and returns the result id.
    pub fn type_matrix(&mut self, column_type: spirv::Word, column_count: u32) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeMatrix, None, Some(id), vec![mr::Operand::IdRef(column_type), mr::Operand::LiteralInt32(column_count)]));
        id
    }

    /// Appends an OpTypeImage instruction and returns the result id.
    pub fn type_image(&mut self, sampled_type: spirv::Word, dim: spirv::Dim, depth: u32, arrayed: u32, ms: u32, sampled: u32, image_format: spirv::ImageFormat, access_qualifier: Option<spirv::AccessQualifier>) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeImage, None, Some(id), vec![mr::Operand::IdRef(sampled_type), mr::Operand::Dim(dim), mr::Operand::LiteralInt32(depth), mr::Operand::LiteralInt32(arrayed), mr::Operand::LiteralInt32(ms), mr::Operand::LiteralInt32(sampled), mr::Operand::ImageFormat(image_format)]));
        if let Some(v) = access_qualifier {
            self.module.types_global_values.last_mut().expect("interal error").operands.push(mr::Operand::AccessQualifier(v));
        };
        id
    }

    /// Appends an OpTypeSampler instruction and returns the result id.
    pub fn type_sampler(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeSampler, None, Some(id), vec![]));
        id
    }

    /// Appends an OpTypeSampledImage instruction and returns the result id.
    pub fn type_sampled_image(&mut self, image_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeSampledImage, None, Some(id), vec![mr::Operand::IdRef(image_type)]));
        id
    }

    /// Appends an OpTypeArray instruction and returns the result id.
    pub fn type_array(&mut self, element_type: spirv::Word, length: spirv::Word) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeArray, None, Some(id), vec![mr::Operand::IdRef(element_type), mr::Operand::IdRef(length)]));
        id
    }

    /// Appends an OpTypeRuntimeArray instruction and returns the result id.
    pub fn type_runtime_array(&mut self, element_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeRuntimeArray, None, Some(id), vec![mr::Operand::IdRef(element_type)]));
        id
    }

    /// Appends an OpTypeStruct instruction and returns the result id.
    pub fn type_struct(&mut self, field_types: &[spirv::Word]) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeStruct, None, Some(id), vec![]));
        for v in field_types {
            self.module.types_global_values.last_mut().expect("interal error").operands.push(mr::Operand::IdRef(*v))
        };
        id
    }

    /// Appends an OpTypeFunction instruction and returns the result id.
    pub fn type_function(&mut self, return_type: spirv::Word, parameter_types: &[spirv::Word]) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeFunction, None, Some(id), vec![mr::Operand::IdRef(return_type)]));
        for v in parameter_types {
            self.module.types_global_values.last_mut().expect("interal error").operands.push(mr::Operand::IdRef(*v))
        };
        id
    }

    /// Appends an OpTypeEvent instruction and returns the result id.
    pub fn type_event(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeEvent, None, Some(id), vec![]));
        id
    }

    /// Appends an OpTypeDeviceEvent instruction and returns the result id.
    pub fn type_device_event(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeDeviceEvent, None, Some(id), vec![]));
        id
    }

    /// Appends an OpTypeReserveId instruction and returns the result id.
    pub fn type_reserve_id(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeReserveId, None, Some(id), vec![]));
        id
    }

    /// Appends an OpTypeQueue instruction and returns the result id.
    pub fn type_queue(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeQueue, None, Some(id), vec![]));
        id
    }

    /// Appends an OpTypePipe instruction and returns the result id.
    pub fn type_pipe(&mut self, qualifier: spirv::AccessQualifier) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypePipe, None, Some(id), vec![mr::Operand::AccessQualifier(qualifier)]));
        id
    }

    /// Appends an OpTypePipeStorage instruction and returns the result id.
    pub fn type_pipe_storage(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypePipeStorage, None, Some(id), vec![]));
        id
    }

    /// Appends an OpTypeNamedBarrier instruction and returns the result id.
    pub fn type_named_barrier(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeNamedBarrier, None, Some(id), vec![]));
        id
    }
}