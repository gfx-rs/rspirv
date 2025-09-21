// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    #[doc = "Appends an OpTypeVoid instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_void(&mut self) -> spirv::Word {
        self.type_void_id(None)
    }
    #[doc = "Appends an OpTypeVoid instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_void_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeVoid, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeBool instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_bool(&mut self) -> spirv::Word {
        self.type_bool_id(None)
    }
    #[doc = "Appends an OpTypeBool instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_bool_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeBool, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeInt instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_int(&mut self, width: u32, signedness: u32) -> spirv::Word {
        self.type_int_id(None, width, signedness)
    }
    #[doc = "Appends an OpTypeInt instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_int_id(
        &mut self,
        result_id: Option<spirv::Word>,
        width: u32,
        signedness: u32,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeInt,
            None,
            result_id,
            vec![
                dr::Operand::LiteralBit32(width),
                dr::Operand::LiteralBit32(signedness),
            ],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeFloat instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_float(
        &mut self,
        width: u32,
        floating_point_encoding: Option<spirv::FPEncoding>,
    ) -> spirv::Word {
        self.type_float_id(None, width, floating_point_encoding)
    }
    #[doc = "Appends an OpTypeFloat instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_float_id(
        &mut self,
        result_id: Option<spirv::Word>,
        width: u32,
        floating_point_encoding: Option<spirv::FPEncoding>,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeFloat,
            None,
            result_id,
            vec![dr::Operand::LiteralBit32(width)],
        );
        if let Some(v) = floating_point_encoding {
            inst.operands.push(dr::Operand::FPEncoding(v));
        }
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeVector instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_vector(
        &mut self,
        component_type: spirv::Word,
        component_count: u32,
    ) -> spirv::Word {
        self.type_vector_id(None, component_type, component_count)
    }
    #[doc = "Appends an OpTypeVector instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_vector_id(
        &mut self,
        result_id: Option<spirv::Word>,
        component_type: spirv::Word,
        component_count: u32,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeVector,
            None,
            result_id,
            vec![
                dr::Operand::IdRef(component_type),
                dr::Operand::LiteralBit32(component_count),
            ],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeMatrix instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_matrix(&mut self, column_type: spirv::Word, column_count: u32) -> spirv::Word {
        self.type_matrix_id(None, column_type, column_count)
    }
    #[doc = "Appends an OpTypeMatrix instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_matrix_id(
        &mut self,
        result_id: Option<spirv::Word>,
        column_type: spirv::Word,
        column_count: u32,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeMatrix,
            None,
            result_id,
            vec![
                dr::Operand::IdRef(column_type),
                dr::Operand::LiteralBit32(column_count),
            ],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
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
        self.type_image_id(
            None,
            sampled_type,
            dim,
            depth,
            arrayed,
            ms,
            sampled,
            image_format,
            access_qualifier,
        )
    }
    #[doc = "Appends an OpTypeImage instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_image_id(
        &mut self,
        result_id: Option<spirv::Word>,
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
            result_id,
            vec![
                dr::Operand::IdRef(sampled_type),
                dr::Operand::Dim(dim),
                dr::Operand::LiteralBit32(depth),
                dr::Operand::LiteralBit32(arrayed),
                dr::Operand::LiteralBit32(ms),
                dr::Operand::LiteralBit32(sampled),
                dr::Operand::ImageFormat(image_format),
            ],
        );
        if let Some(v) = access_qualifier {
            inst.operands.push(dr::Operand::AccessQualifier(v));
        }
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeSampler instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_sampler(&mut self) -> spirv::Word {
        self.type_sampler_id(None)
    }
    #[doc = "Appends an OpTypeSampler instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_sampler_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeSampler, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeSampledImage instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_sampled_image(&mut self, image_type: spirv::Word) -> spirv::Word {
        self.type_sampled_image_id(None, image_type)
    }
    #[doc = "Appends an OpTypeSampledImage instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_sampled_image_id(
        &mut self,
        result_id: Option<spirv::Word>,
        image_type: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeSampledImage,
            None,
            result_id,
            vec![dr::Operand::IdRef(image_type)],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeArray instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_array(&mut self, element_type: spirv::Word, length: spirv::Word) -> spirv::Word {
        self.type_array_id(None, element_type, length)
    }
    #[doc = "Appends an OpTypeArray instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_array_id(
        &mut self,
        result_id: Option<spirv::Word>,
        element_type: spirv::Word,
        length: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeArray,
            None,
            result_id,
            vec![dr::Operand::IdRef(element_type), dr::Operand::IdRef(length)],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeRuntimeArray instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_runtime_array(&mut self, element_type: spirv::Word) -> spirv::Word {
        self.type_runtime_array_id(None, element_type)
    }
    #[doc = "Appends an OpTypeRuntimeArray instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_runtime_array_id(
        &mut self,
        result_id: Option<spirv::Word>,
        element_type: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeRuntimeArray,
            None,
            result_id,
            vec![dr::Operand::IdRef(element_type)],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeStruct instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_struct(
        &mut self,
        member_0_type_member_1_type: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.type_struct_id(None, member_0_type_member_1_type)
    }
    #[doc = "Appends an OpTypeStruct instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_struct_id(
        &mut self,
        result_id: Option<spirv::Word>,
        member_0_type_member_1_type: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeStruct, None, result_id, vec![]);
        inst.operands.extend(
            member_0_type_member_1_type
                .into_iter()
                .map(dr::Operand::IdRef),
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeFunction instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_function(
        &mut self,
        return_type: spirv::Word,
        parameter_0_type_parameter_1_type: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.type_function_id(None, return_type, parameter_0_type_parameter_1_type)
    }
    #[doc = "Appends an OpTypeFunction instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_function_id(
        &mut self,
        result_id: Option<spirv::Word>,
        return_type: spirv::Word,
        parameter_0_type_parameter_1_type: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeFunction,
            None,
            result_id,
            vec![dr::Operand::IdRef(return_type)],
        );
        inst.operands.extend(
            parameter_0_type_parameter_1_type
                .into_iter()
                .map(dr::Operand::IdRef),
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeEvent instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_event(&mut self) -> spirv::Word {
        self.type_event_id(None)
    }
    #[doc = "Appends an OpTypeEvent instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_event_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeEvent, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeDeviceEvent instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_device_event(&mut self) -> spirv::Word {
        self.type_device_event_id(None)
    }
    #[doc = "Appends an OpTypeDeviceEvent instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_device_event_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeDeviceEvent, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeReserveId instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_reserve_id(&mut self) -> spirv::Word {
        self.type_reserve_id_id(None)
    }
    #[doc = "Appends an OpTypeReserveId instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_reserve_id_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeReserveId, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeQueue instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_queue(&mut self) -> spirv::Word {
        self.type_queue_id(None)
    }
    #[doc = "Appends an OpTypeQueue instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_queue_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeQueue, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypePipe instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_pipe(&mut self, qualifier: spirv::AccessQualifier) -> spirv::Word {
        self.type_pipe_id(None, qualifier)
    }
    #[doc = "Appends an OpTypePipe instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_pipe_id(
        &mut self,
        result_id: Option<spirv::Word>,
        qualifier: spirv::AccessQualifier,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypePipe,
            None,
            result_id,
            vec![dr::Operand::AccessQualifier(qualifier)],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypePipeStorage instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_pipe_storage(&mut self) -> spirv::Word {
        self.type_pipe_storage_id(None)
    }
    #[doc = "Appends an OpTypePipeStorage instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_pipe_storage_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypePipeStorage, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeNamedBarrier instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_named_barrier(&mut self) -> spirv::Word {
        self.type_named_barrier_id(None)
    }
    #[doc = "Appends an OpTypeNamedBarrier instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_named_barrier_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeNamedBarrier, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeTensorARM instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_tensor_arm(
        &mut self,
        element_type: spirv::Word,
        rank: Option<spirv::Word>,
        shape: Option<spirv::Word>,
    ) -> spirv::Word {
        self.type_tensor_arm_id(None, element_type, rank, shape)
    }
    #[doc = "Appends an OpTypeTensorARM instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_tensor_arm_id(
        &mut self,
        result_id: Option<spirv::Word>,
        element_type: spirv::Word,
        rank: Option<spirv::Word>,
        shape: Option<spirv::Word>,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeTensorARM,
            None,
            result_id,
            vec![dr::Operand::IdRef(element_type)],
        );
        if let Some(v) = rank {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = shape {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeGraphARM instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_graph_arm(
        &mut self,
        num_inputs: u32,
        in_out_types: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.type_graph_arm_id(None, num_inputs, in_out_types)
    }
    #[doc = "Appends an OpTypeGraphARM instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_graph_arm_id(
        &mut self,
        result_id: Option<spirv::Word>,
        num_inputs: u32,
        in_out_types: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeGraphARM,
            None,
            result_id,
            vec![dr::Operand::LiteralBit32(num_inputs)],
        );
        inst.operands
            .extend(in_out_types.into_iter().map(dr::Operand::IdRef));
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeUntypedPointerKHR instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_untyped_pointer_khr(&mut self, storage_class: spirv::StorageClass) -> spirv::Word {
        self.type_untyped_pointer_khr_id(None, storage_class)
    }
    #[doc = "Appends an OpTypeUntypedPointerKHR instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_untyped_pointer_khr_id(
        &mut self,
        result_id: Option<spirv::Word>,
        storage_class: spirv::StorageClass,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeUntypedPointerKHR,
            None,
            result_id,
            vec![dr::Operand::StorageClass(storage_class)],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeCooperativeMatrixKHR instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_cooperative_matrix_khr(
        &mut self,
        component_type: spirv::Word,
        scope: spirv::Word,
        rows: spirv::Word,
        columns: spirv::Word,
        usage: spirv::Word,
    ) -> spirv::Word {
        self.type_cooperative_matrix_khr_id(None, component_type, scope, rows, columns, usage)
    }
    #[doc = "Appends an OpTypeCooperativeMatrixKHR instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_cooperative_matrix_khr_id(
        &mut self,
        result_id: Option<spirv::Word>,
        component_type: spirv::Word,
        scope: spirv::Word,
        rows: spirv::Word,
        columns: spirv::Word,
        usage: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeCooperativeMatrixKHR,
            None,
            result_id,
            vec![
                dr::Operand::IdRef(component_type),
                dr::Operand::IdScope(scope),
                dr::Operand::IdRef(rows),
                dr::Operand::IdRef(columns),
                dr::Operand::IdRef(usage),
            ],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeRayQueryKHR instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_ray_query_khr(&mut self) -> spirv::Word {
        self.type_ray_query_khr_id(None)
    }
    #[doc = "Appends an OpTypeRayQueryKHR instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_ray_query_khr_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeRayQueryKHR, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeNodePayloadArrayAMDX instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_node_payload_array_amdx(&mut self, payload_type: spirv::Word) -> spirv::Word {
        self.type_node_payload_array_amdx_id(None, payload_type)
    }
    #[doc = "Appends an OpTypeNodePayloadArrayAMDX instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_node_payload_array_amdx_id(
        &mut self,
        result_id: Option<spirv::Word>,
        payload_type: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeNodePayloadArrayAMDX,
            None,
            result_id,
            vec![dr::Operand::IdRef(payload_type)],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeHitObjectNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_hit_object_nv(&mut self) -> spirv::Word {
        self.type_hit_object_nv_id(None)
    }
    #[doc = "Appends an OpTypeHitObjectNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_hit_object_nv_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst = dr::Instruction::new(spirv::Op::TypeHitObjectNV, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeCooperativeVectorNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_cooperative_vector_nv(
        &mut self,
        component_type: spirv::Word,
        component_count: spirv::Word,
    ) -> spirv::Word {
        self.type_cooperative_vector_nv_id(None, component_type, component_count)
    }
    #[doc = "Appends an OpTypeCooperativeVectorNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_cooperative_vector_nv_id(
        &mut self,
        result_id: Option<spirv::Word>,
        component_type: spirv::Word,
        component_count: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeCooperativeVectorNV,
            None,
            result_id,
            vec![
                dr::Operand::IdRef(component_type),
                dr::Operand::IdRef(component_count),
            ],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeAccelerationStructureKHR instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_acceleration_structure_khr(&mut self) -> spirv::Word {
        self.type_acceleration_structure_khr_id(None)
    }
    #[doc = "Appends an OpTypeAccelerationStructureKHR instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_acceleration_structure_khr_id(
        &mut self,
        result_id: Option<spirv::Word>,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeAccelerationStructureKHR,
            None,
            result_id,
            vec![],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeCooperativeMatrixNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_cooperative_matrix_nv(
        &mut self,
        component_type: spirv::Word,
        execution: spirv::Word,
        rows: spirv::Word,
        columns: spirv::Word,
    ) -> spirv::Word {
        self.type_cooperative_matrix_nv_id(None, component_type, execution, rows, columns)
    }
    #[doc = "Appends an OpTypeCooperativeMatrixNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_cooperative_matrix_nv_id(
        &mut self,
        result_id: Option<spirv::Word>,
        component_type: spirv::Word,
        execution: spirv::Word,
        rows: spirv::Word,
        columns: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeCooperativeMatrixNV,
            None,
            result_id,
            vec![
                dr::Operand::IdRef(component_type),
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(rows),
                dr::Operand::IdRef(columns),
            ],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeTensorLayoutNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_tensor_layout_nv(
        &mut self,
        dim: spirv::Word,
        clamp_mode: spirv::Word,
    ) -> spirv::Word {
        self.type_tensor_layout_nv_id(None, dim, clamp_mode)
    }
    #[doc = "Appends an OpTypeTensorLayoutNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_tensor_layout_nv_id(
        &mut self,
        result_id: Option<spirv::Word>,
        dim: spirv::Word,
        clamp_mode: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeTensorLayoutNV,
            None,
            result_id,
            vec![dr::Operand::IdRef(dim), dr::Operand::IdRef(clamp_mode)],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeTensorViewNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_tensor_view_nv(
        &mut self,
        dim: spirv::Word,
        has_dimensions: spirv::Word,
        p: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.type_tensor_view_nv_id(None, dim, has_dimensions, p)
    }
    #[doc = "Appends an OpTypeTensorViewNV instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_tensor_view_nv_id(
        &mut self,
        result_id: Option<spirv::Word>,
        dim: spirv::Word,
        has_dimensions: spirv::Word,
        p: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeTensorViewNV,
            None,
            result_id,
            vec![dr::Operand::IdRef(dim), dr::Operand::IdRef(has_dimensions)],
        );
        inst.operands.extend(p.into_iter().map(dr::Operand::IdRef));
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeBufferSurfaceINTEL instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_buffer_surface_intel(
        &mut self,
        access_qualifier: spirv::AccessQualifier,
    ) -> spirv::Word {
        self.type_buffer_surface_intel_id(None, access_qualifier)
    }
    #[doc = "Appends an OpTypeBufferSurfaceINTEL instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_buffer_surface_intel_id(
        &mut self,
        result_id: Option<spirv::Word>,
        access_qualifier: spirv::AccessQualifier,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypeBufferSurfaceINTEL,
            None,
            result_id,
            vec![dr::Operand::AccessQualifier(access_qualifier)],
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeStructContinuedINTEL instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_struct_continued_intel(
        &mut self,
        member_0_type_member_1_type: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.type_struct_continued_intel_id(None, member_0_type_member_1_type)
    }
    #[doc = "Appends an OpTypeStructContinuedINTEL instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_struct_continued_intel_id(
        &mut self,
        result_id: Option<spirv::Word>,
        member_0_type_member_1_type: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let mut inst =
            dr::Instruction::new(spirv::Op::TypeStructContinuedINTEL, None, result_id, vec![]);
        inst.operands.extend(
            member_0_type_member_1_type
                .into_iter()
                .map(dr::Operand::IdRef),
        );
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
    #[doc = "Appends an OpTypeTaskSequenceINTEL instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_task_sequence_intel(&mut self) -> spirv::Word {
        self.type_task_sequence_intel_id(None)
    }
    #[doc = "Appends an OpTypeTaskSequenceINTEL instruction and returns the result id, or return the existing id if the instruction was already present."]
    pub fn type_task_sequence_intel_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let mut inst =
            dr::Instruction::new(spirv::Op::TypeTaskSequenceINTEL, None, result_id, vec![]);
        if let Some(result_id) = result_id {
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }
}
