// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    #[doc = "Appends an OpTypeVoid instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_void(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeVoid, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeBool instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_bool(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeBool, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeInt instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_int(&mut self, width: u32, signedness: u32) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeInt,
            None,
            None,
            vec![
                dr::Operand::LiteralInt32(width),
                dr::Operand::LiteralInt32(signedness),
            ],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeFloat instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_float(&mut self, width: u32) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeFloat,
            None,
            None,
            vec![dr::Operand::LiteralInt32(width)],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeVector instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_vector(
        &mut self,
        component_type: spirv::Word,
        component_count: u32,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeVector,
            None,
            None,
            vec![
                dr::Operand::IdRef(component_type),
                dr::Operand::LiteralInt32(component_count),
            ],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeMatrix instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_matrix(&mut self, column_type: spirv::Word, column_count: u32) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeMatrix,
            None,
            None,
            vec![
                dr::Operand::IdRef(column_type),
                dr::Operand::LiteralInt32(column_count),
            ],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeImage instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_image(
        &mut self,
        sampled_type: spirv::Word,
        dim: spirv::Dim,
        depth: u32,
        arrayed: u32,
        ms: u32,
        sampled: u32,
        image_format: spirv::ImageFormat,
        access_qualifier: Option<spirv::AccessQualifier>,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeImage,
            None,
            None,
            vec![
                dr::Operand::IdRef(sampled_type),
                dr::Operand::Dim(dim),
                dr::Operand::LiteralInt32(depth),
                dr::Operand::LiteralInt32(arrayed),
                dr::Operand::LiteralInt32(ms),
                dr::Operand::LiteralInt32(sampled),
                dr::Operand::ImageFormat(image_format),
            ],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        if let Some(v) = access_qualifier {
            #[allow(clippy::identity_conversion)]
            self.module
                .types_global_values
                .last_mut()
                .expect("interal error")
                .operands
                .push(dr::Operand::AccessQualifier(v.into()));
        }
        id
    }
    #[doc = "Appends an OpTypeSampler instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_sampler(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeSampler, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeSampledImage instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_sampled_image(&mut self, image_type: spirv::Word) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeSampledImage,
            None,
            None,
            vec![dr::Operand::IdRef(image_type)],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeArray instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_array(&mut self, element_type: spirv::Word, length: spirv::Word) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeArray,
            None,
            None,
            vec![dr::Operand::IdRef(element_type), dr::Operand::IdRef(length)],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeRuntimeArray instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_runtime_array(&mut self, element_type: spirv::Word) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeRuntimeArray,
            None,
            None,
            vec![dr::Operand::IdRef(element_type)],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeStruct instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_struct<T: AsRef<[spirv::Word]>>(
        &mut self,
        member_0_type_member_1_type: T,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeStruct, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        for v in member_0_type_member_1_type.as_ref() {
            self.module
                .types_global_values
                .last_mut()
                .expect("interal error")
                .operands
                .push(dr::Operand::IdRef(*v));
        }
        id
    }
    #[doc = "Appends an OpTypeFunction instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_function<T: AsRef<[spirv::Word]>>(
        &mut self,
        return_type: spirv::Word,
        parameter_0_type_parameter_1_type: T,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeFunction,
            None,
            None,
            vec![dr::Operand::IdRef(return_type)],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        for v in parameter_0_type_parameter_1_type.as_ref() {
            self.module
                .types_global_values
                .last_mut()
                .expect("interal error")
                .operands
                .push(dr::Operand::IdRef(*v));
        }
        id
    }
    #[doc = "Appends an OpTypeEvent instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_event(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeEvent, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeDeviceEvent instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_device_event(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeDeviceEvent, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeReserveId instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_reserve_id(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeReserveId, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeQueue instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_queue(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeQueue, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypePipe instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_pipe(&mut self, qualifier: spirv::AccessQualifier) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypePipe,
            None,
            None,
            vec![dr::Operand::AccessQualifier(qualifier)],
        );
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypePipeStorage instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_pipe_storage(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypePipeStorage, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
    #[doc = "Appends an OpTypeNamedBarrier instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_named_barrier(&mut self) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeNamedBarrier, None, None, vec![]);
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return id;
                }
            }
        }
        let id = self.id();
        inst.result_id = Some(id);
        self.module.types_global_values.push(inst);
        id
    }
}
