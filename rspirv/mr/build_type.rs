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
    /// Creates TypeVoid and returns the result id.
    pub fn type_void(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeVoid, None, Some(id), vec![]));
        id
    }

    /// Creates TypeBool and returns the result id.
    pub fn type_bool(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeBool, None, Some(id), vec![]));
        id
    }

    /// Creates TypeInt and returns the result id.
    pub fn type_int(&mut self, width: spirv::Word, signedness: spirv::Word) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeInt, None, Some(id), vec![mr::Operand::LiteralInt32(width), mr::Operand::LiteralInt32(signedness)]));
        id
    }

    /// Creates TypeFloat and returns the result id.
    pub fn type_float(&mut self, width: spirv::Word) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeFloat, None, Some(id), vec![mr::Operand::LiteralInt32(width)]));
        id
    }

    /// Creates TypeVector and returns the result id.
    pub fn type_vector(&mut self, component_type: spirv::Word, component_count: spirv::Word) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeVector, None, Some(id), vec![mr::Operand::IdRef(component_type), mr::Operand::LiteralInt32(component_count)]));
        id
    }

    /// Creates TypeMatrix and returns the result id.
    pub fn type_matrix(&mut self, column_type: spirv::Word, column_count: spirv::Word) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeMatrix, None, Some(id), vec![mr::Operand::IdRef(column_type), mr::Operand::LiteralInt32(column_count)]));
        id
    }

    /// Creates TypeImage and returns the result id.
    pub fn type_image(&mut self, sampled_type: spirv::Word, dim: spirv::Dim, depth: spirv::Word, arrayed: spirv::Word, ms: spirv::Word, sampled: spirv::Word, image_format: spirv::ImageFormat, access_qualifier: Option<spirv::AccessQualifier>) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeImage, None, Some(id), vec![mr::Operand::IdRef(sampled_type), mr::Operand::Dim(dim), mr::Operand::LiteralInt32(depth), mr::Operand::LiteralInt32(arrayed), mr::Operand::LiteralInt32(ms), mr::Operand::LiteralInt32(sampled), mr::Operand::ImageFormat(image_format)]));
        if access_qualifier.is_some() {
            self.module.types_global_values.last_mut().expect("internal error").operands.push(mr::Operand::AccessQualifier(access_qualifier.unwrap()))
        };
        id
    }

    /// Creates TypeSampler and returns the result id.
    pub fn type_sampler(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeSampler, None, Some(id), vec![]));
        id
    }

    /// Creates TypeSampledImage and returns the result id.
    pub fn type_sampled_image(&mut self, image_type: spirv::Word) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeSampledImage, None, Some(id), vec![mr::Operand::IdRef(image_type)]));
        id
    }

    /// Creates TypeArray and returns the result id.
    pub fn type_array(&mut self, element_type: spirv::Word, length: spirv::Word) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeArray, None, Some(id), vec![mr::Operand::IdRef(element_type), mr::Operand::IdRef(length)]));
        id
    }

    /// Creates TypeRuntimeArray and returns the result id.
    pub fn type_runtime_array(&mut self, element_type: spirv::Word) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeRuntimeArray, None, Some(id), vec![mr::Operand::IdRef(element_type)]));
        id
    }

    /// Creates TypeStruct and returns the result id.
    pub fn type_struct(&mut self, field_types: Vec<spirv::Word>) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeStruct, None, Some(id), vec![]));
        for v in field_types {
            self.module.types_global_values.last_mut().expect("internal error").operands.push(mr::Operand::IdRef(v))
        };
        id
    }

    /// Creates TypeOpaque and returns the result id.
    pub fn type_opaque(&mut self, type_name: String) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeOpaque, None, Some(id), vec![mr::Operand::LiteralString(type_name)]));
        id
    }

    /// Creates TypePointer and returns the result id.
    pub fn type_pointer(&mut self, storage_class: spirv::StorageClass, pointee_type: spirv::Word) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypePointer, None, Some(id), vec![mr::Operand::StorageClass(storage_class), mr::Operand::IdRef(pointee_type)]));
        id
    }

    /// Creates TypeFunction and returns the result id.
    pub fn type_function(&mut self, return_type: spirv::Word, parameter_types: Vec<spirv::Word>) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeFunction, None, Some(id), vec![mr::Operand::IdRef(return_type)]));
        for v in parameter_types {
            self.module.types_global_values.last_mut().expect("internal error").operands.push(mr::Operand::IdRef(v))
        };
        id
    }

    /// Creates TypeEvent and returns the result id.
    pub fn type_event(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeEvent, None, Some(id), vec![]));
        id
    }

    /// Creates TypeDeviceEvent and returns the result id.
    pub fn type_device_event(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeDeviceEvent, None, Some(id), vec![]));
        id
    }

    /// Creates TypeReserveId and returns the result id.
    pub fn type_reserve_id(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeReserveId, None, Some(id), vec![]));
        id
    }

    /// Creates TypeQueue and returns the result id.
    pub fn type_queue(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeQueue, None, Some(id), vec![]));
        id
    }

    /// Creates TypePipe and returns the result id.
    pub fn type_pipe(&mut self, qualifier: spirv::AccessQualifier) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypePipe, None, Some(id), vec![mr::Operand::AccessQualifier(qualifier)]));
        id
    }

    /// Creates TypeForwardPointer and returns the result id.
    pub fn type_forward_pointer(&mut self, storage_class: spirv::StorageClass) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeForwardPointer, None, Some(id), vec![mr::Operand::StorageClass(storage_class)]));
        id
    }

    /// Creates TypePipeStorage and returns the result id.
    pub fn type_pipe_storage(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypePipeStorage, None, Some(id), vec![]));
        id
    }

    /// Creates TypeNamedBarrier and returns the result id.
    pub fn type_named_barrier(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        self.module.types_global_values.push(mr::Instruction::new(spirv::Op::TypeNamedBarrier, None, Some(id), vec![]));
        id
    }
}