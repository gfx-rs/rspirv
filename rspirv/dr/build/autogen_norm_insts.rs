// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[allow(clippy::useless_conversion, clippy::too_many_arguments)]
impl Builder {
    #[doc = "Appends an OpNop instruction to the current block."]
    pub fn nop(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Nop, None, None, vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpNop instruction to the current block."]
    pub fn insert_nop(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Nop, None, None, vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpFunctionCall instruction to the current block."]
    pub fn function_call(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        function: spirv::Word,
        argument_0_argument_1: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FunctionCall,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(function)],
        );
        inst.operands
            .extend(argument_0_argument_1.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFunctionCall instruction to the current block."]
    pub fn insert_function_call(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        function: spirv::Word,
        argument_0_argument_1: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FunctionCall,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(function)],
        );
        inst.operands
            .extend(argument_0_argument_1.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageTexelPointer instruction to the current block."]
    pub fn image_texel_pointer(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        sample: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageTexelPointer,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(sample),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageTexelPointer instruction to the current block."]
    pub fn insert_image_texel_pointer(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        sample: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageTexelPointer,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(sample),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLoad instruction to the current block."]
    pub fn load(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Load,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLoad instruction to the current block."]
    pub fn insert_load(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Load,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpStore instruction to the current block."]
    pub fn store(
        &mut self,
        pointer: spirv::Word,
        object: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Store,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::IdRef(object)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpStore instruction to the current block."]
    pub fn insert_store(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        object: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Store,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::IdRef(object)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCopyMemory instruction to the current block."]
    pub fn copy_memory(
        &mut self,
        target: spirv::Word,
        source: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        memory_access_2: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyMemory,
            None,
            None,
            vec![dr::Operand::IdRef(target), dr::Operand::IdRef(source)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        if let Some(v) = memory_access_2 {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCopyMemory instruction to the current block."]
    pub fn insert_copy_memory(
        &mut self,
        insert_point: InsertPoint,
        target: spirv::Word,
        source: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        memory_access_2: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyMemory,
            None,
            None,
            vec![dr::Operand::IdRef(target), dr::Operand::IdRef(source)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        if let Some(v) = memory_access_2 {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCopyMemorySized instruction to the current block."]
    pub fn copy_memory_sized(
        &mut self,
        target: spirv::Word,
        source: spirv::Word,
        size: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        memory_access_2: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyMemorySized,
            None,
            None,
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(size),
            ],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        if let Some(v) = memory_access_2 {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCopyMemorySized instruction to the current block."]
    pub fn insert_copy_memory_sized(
        &mut self,
        insert_point: InsertPoint,
        target: spirv::Word,
        source: spirv::Word,
        size: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        memory_access_2: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyMemorySized,
            None,
            None,
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(size),
            ],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        if let Some(v) = memory_access_2 {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpAccessChain instruction to the current block."]
    pub fn access_chain(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAccessChain instruction to the current block."]
    pub fn insert_access_chain(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpInBoundsAccessChain instruction to the current block."]
    pub fn in_bounds_access_chain(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::InBoundsAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpInBoundsAccessChain instruction to the current block."]
    pub fn insert_in_bounds_access_chain(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::InBoundsAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrAccessChain instruction to the current block."]
    pub fn ptr_access_chain(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        element: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(element)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrAccessChain instruction to the current block."]
    pub fn insert_ptr_access_chain(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        element: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(element)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpArrayLength instruction to the current block."]
    pub fn array_length(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        structure: spirv::Word,
        array_member: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ArrayLength,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(structure),
                dr::Operand::LiteralBit32(array_member),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpArrayLength instruction to the current block."]
    pub fn insert_array_length(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        structure: spirv::Word,
        array_member: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ArrayLength,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(structure),
                dr::Operand::LiteralBit32(array_member),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGenericPtrMemSemantics instruction to the current block."]
    pub fn generic_ptr_mem_semantics(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GenericPtrMemSemantics,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGenericPtrMemSemantics instruction to the current block."]
    pub fn insert_generic_ptr_mem_semantics(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GenericPtrMemSemantics,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpInBoundsPtrAccessChain instruction to the current block."]
    pub fn in_bounds_ptr_access_chain(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        element: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::InBoundsPtrAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(element)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpInBoundsPtrAccessChain instruction to the current block."]
    pub fn insert_in_bounds_ptr_access_chain(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        element: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::InBoundsPtrAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(element)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorExtractDynamic instruction to the current block."]
    pub fn vector_extract_dynamic(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorExtractDynamic,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(index)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorExtractDynamic instruction to the current block."]
    pub fn insert_vector_extract_dynamic(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorExtractDynamic,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(index)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorInsertDynamic instruction to the current block."]
    pub fn vector_insert_dynamic(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        component: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorInsertDynamic,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(vector),
                dr::Operand::IdRef(component),
                dr::Operand::IdRef(index),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorInsertDynamic instruction to the current block."]
    pub fn insert_vector_insert_dynamic(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        component: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorInsertDynamic,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(vector),
                dr::Operand::IdRef(component),
                dr::Operand::IdRef(index),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorShuffle instruction to the current block."]
    pub fn vector_shuffle(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        components: impl IntoIterator<Item = u32>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorShuffle,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        inst.operands
            .extend(components.into_iter().map(dr::Operand::LiteralBit32));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorShuffle instruction to the current block."]
    pub fn insert_vector_shuffle(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        components: impl IntoIterator<Item = u32>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorShuffle,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        inst.operands
            .extend(components.into_iter().map(dr::Operand::LiteralBit32));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeConstruct instruction to the current block."]
    pub fn composite_construct(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        constituents: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeConstruct,
            Some(result_type),
            Some(_id),
            vec![],
        );
        inst.operands
            .extend(constituents.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeConstruct instruction to the current block."]
    pub fn insert_composite_construct(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        constituents: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeConstruct,
            Some(result_type),
            Some(_id),
            vec![],
        );
        inst.operands
            .extend(constituents.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeExtract instruction to the current block."]
    pub fn composite_extract(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        composite: spirv::Word,
        indexes: impl IntoIterator<Item = u32>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeExtract,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(composite)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::LiteralBit32));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeExtract instruction to the current block."]
    pub fn insert_composite_extract(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        composite: spirv::Word,
        indexes: impl IntoIterator<Item = u32>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeExtract,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(composite)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::LiteralBit32));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeInsert instruction to the current block."]
    pub fn composite_insert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        object: spirv::Word,
        composite: spirv::Word,
        indexes: impl IntoIterator<Item = u32>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeInsert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(object), dr::Operand::IdRef(composite)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::LiteralBit32));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeInsert instruction to the current block."]
    pub fn insert_composite_insert(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        object: spirv::Word,
        composite: spirv::Word,
        indexes: impl IntoIterator<Item = u32>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeInsert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(object), dr::Operand::IdRef(composite)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::LiteralBit32));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCopyObject instruction to the current block."]
    pub fn copy_object(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyObject,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCopyObject instruction to the current block."]
    pub fn insert_copy_object(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyObject,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTranspose instruction to the current block."]
    pub fn transpose(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Transpose,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTranspose instruction to the current block."]
    pub fn insert_transpose(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Transpose,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSampledImage instruction to the current block."]
    pub fn sampled_image(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        sampler: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SampledImage,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(sampler)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSampledImage instruction to the current block."]
    pub fn insert_sampled_image(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        sampler: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SampledImage,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(sampler)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleImplicitLod instruction to the current block."]
    pub fn image_sample_implicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleImplicitLod instruction to the current block."]
    pub fn insert_image_sample_implicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleExplicitLod instruction to the current block."]
    pub fn image_sample_explicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleExplicitLod instruction to the current block."]
    pub fn insert_image_sample_explicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleDrefImplicitLod instruction to the current block."]
    pub fn image_sample_dref_implicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleDrefImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleDrefImplicitLod instruction to the current block."]
    pub fn insert_image_sample_dref_implicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleDrefImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleDrefExplicitLod instruction to the current block."]
    pub fn image_sample_dref_explicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleDrefExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleDrefExplicitLod instruction to the current block."]
    pub fn insert_image_sample_dref_explicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleDrefExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjImplicitLod instruction to the current block."]
    pub fn image_sample_proj_implicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleProjImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjImplicitLod instruction to the current block."]
    pub fn insert_image_sample_proj_implicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleProjImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjExplicitLod instruction to the current block."]
    pub fn image_sample_proj_explicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleProjExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjExplicitLod instruction to the current block."]
    pub fn insert_image_sample_proj_explicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleProjExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjDrefImplicitLod instruction to the current block."]
    pub fn image_sample_proj_dref_implicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleProjDrefImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjDrefImplicitLod instruction to the current block."]
    pub fn insert_image_sample_proj_dref_implicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleProjDrefImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjDrefExplicitLod instruction to the current block."]
    pub fn image_sample_proj_dref_explicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleProjDrefExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjDrefExplicitLod instruction to the current block."]
    pub fn insert_image_sample_proj_dref_explicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleProjDrefExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageFetch instruction to the current block."]
    pub fn image_fetch(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageFetch,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageFetch instruction to the current block."]
    pub fn insert_image_fetch(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageFetch,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageGather instruction to the current block."]
    pub fn image_gather(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageGather,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(component),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageGather instruction to the current block."]
    pub fn insert_image_gather(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageGather,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(component),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageDrefGather instruction to the current block."]
    pub fn image_dref_gather(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageDrefGather,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageDrefGather instruction to the current block."]
    pub fn insert_image_dref_gather(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageDrefGather,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageRead instruction to the current block."]
    pub fn image_read(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageRead,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageRead instruction to the current block."]
    pub fn insert_image_read(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageRead,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageWrite instruction to the current block."]
    pub fn image_write(
        &mut self,
        image: spirv::Word,
        coordinate: spirv::Word,
        texel: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageWrite,
            None,
            None,
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(texel),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpImageWrite instruction to the current block."]
    pub fn insert_image_write(
        &mut self,
        insert_point: InsertPoint,
        image: spirv::Word,
        coordinate: spirv::Word,
        texel: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageWrite,
            None,
            None,
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(texel),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpImage instruction to the current block."]
    pub fn image(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Image,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(sampled_image)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImage instruction to the current block."]
    pub fn insert_image(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Image,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(sampled_image)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryFormat instruction to the current block."]
    pub fn image_query_format(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryFormat,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryFormat instruction to the current block."]
    pub fn insert_image_query_format(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryFormat,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryOrder instruction to the current block."]
    pub fn image_query_order(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryOrder,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryOrder instruction to the current block."]
    pub fn insert_image_query_order(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryOrder,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySizeLod instruction to the current block."]
    pub fn image_query_size_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        level_of_detail: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQuerySizeLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(level_of_detail),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySizeLod instruction to the current block."]
    pub fn insert_image_query_size_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        level_of_detail: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQuerySizeLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(level_of_detail),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySize instruction to the current block."]
    pub fn image_query_size(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQuerySize,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySize instruction to the current block."]
    pub fn insert_image_query_size(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQuerySize,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryLod instruction to the current block."]
    pub fn image_query_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryLod instruction to the current block."]
    pub fn insert_image_query_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryLevels instruction to the current block."]
    pub fn image_query_levels(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryLevels,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryLevels instruction to the current block."]
    pub fn insert_image_query_levels(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryLevels,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySamples instruction to the current block."]
    pub fn image_query_samples(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQuerySamples,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySamples instruction to the current block."]
    pub fn insert_image_query_samples(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQuerySamples,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertFToU instruction to the current block."]
    pub fn convert_f_to_u(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertFToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertFToU instruction to the current block."]
    pub fn insert_convert_f_to_u(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertFToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertFToS instruction to the current block."]
    pub fn convert_f_to_s(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertFToS,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertFToS instruction to the current block."]
    pub fn insert_convert_f_to_s(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertFToS,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertSToF instruction to the current block."]
    pub fn convert_s_to_f(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertSToF,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertSToF instruction to the current block."]
    pub fn insert_convert_s_to_f(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertSToF,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToF instruction to the current block."]
    pub fn convert_u_to_f(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToF,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToF instruction to the current block."]
    pub fn insert_convert_u_to_f(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToF,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUConvert instruction to the current block."]
    pub fn u_convert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUConvert instruction to the current block."]
    pub fn insert_u_convert(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSConvert instruction to the current block."]
    pub fn s_convert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSConvert instruction to the current block."]
    pub fn insert_s_convert(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFConvert instruction to the current block."]
    pub fn f_convert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFConvert instruction to the current block."]
    pub fn insert_f_convert(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpQuantizeToF16 instruction to the current block."]
    pub fn quantize_to_f16(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::QuantizeToF16,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpQuantizeToF16 instruction to the current block."]
    pub fn insert_quantize_to_f16(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::QuantizeToF16,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertPtrToU instruction to the current block."]
    pub fn convert_ptr_to_u(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertPtrToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertPtrToU instruction to the current block."]
    pub fn insert_convert_ptr_to_u(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertPtrToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSatConvertSToU instruction to the current block."]
    pub fn sat_convert_s_to_u(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SatConvertSToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSatConvertSToU instruction to the current block."]
    pub fn insert_sat_convert_s_to_u(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SatConvertSToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSatConvertUToS instruction to the current block."]
    pub fn sat_convert_u_to_s(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SatConvertUToS,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSatConvertUToS instruction to the current block."]
    pub fn insert_sat_convert_u_to_s(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SatConvertUToS,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToPtr instruction to the current block."]
    pub fn convert_u_to_ptr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        integer_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToPtr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(integer_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToPtr instruction to the current block."]
    pub fn insert_convert_u_to_ptr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        integer_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToPtr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(integer_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrCastToGeneric instruction to the current block."]
    pub fn ptr_cast_to_generic(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrCastToGeneric,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrCastToGeneric instruction to the current block."]
    pub fn insert_ptr_cast_to_generic(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrCastToGeneric,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGenericCastToPtr instruction to the current block."]
    pub fn generic_cast_to_ptr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GenericCastToPtr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGenericCastToPtr instruction to the current block."]
    pub fn insert_generic_cast_to_ptr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GenericCastToPtr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGenericCastToPtrExplicit instruction to the current block."]
    pub fn generic_cast_to_ptr_explicit(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        storage: spirv::StorageClass,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GenericCastToPtrExplicit,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::StorageClass(storage),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGenericCastToPtrExplicit instruction to the current block."]
    pub fn insert_generic_cast_to_ptr_explicit(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        storage: spirv::StorageClass,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GenericCastToPtrExplicit,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::StorageClass(storage),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitcast instruction to the current block."]
    pub fn bitcast(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Bitcast,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitcast instruction to the current block."]
    pub fn insert_bitcast(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Bitcast,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSNegate instruction to the current block."]
    pub fn s_negate(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SNegate,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSNegate instruction to the current block."]
    pub fn insert_s_negate(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SNegate,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFNegate instruction to the current block."]
    pub fn f_negate(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FNegate,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFNegate instruction to the current block."]
    pub fn insert_f_negate(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FNegate,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAdd instruction to the current block."]
    pub fn i_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAdd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAdd instruction to the current block."]
    pub fn insert_i_add(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAdd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFAdd instruction to the current block."]
    pub fn f_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FAdd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFAdd instruction to the current block."]
    pub fn insert_f_add(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FAdd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpISub instruction to the current block."]
    pub fn i_sub(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ISub,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpISub instruction to the current block."]
    pub fn insert_i_sub(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ISub,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFSub instruction to the current block."]
    pub fn f_sub(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FSub,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFSub instruction to the current block."]
    pub fn insert_f_sub(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FSub,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIMul instruction to the current block."]
    pub fn i_mul(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IMul,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIMul instruction to the current block."]
    pub fn insert_i_mul(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IMul,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFMul instruction to the current block."]
    pub fn f_mul(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FMul,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFMul instruction to the current block."]
    pub fn insert_f_mul(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FMul,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUDiv instruction to the current block."]
    pub fn u_div(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUDiv instruction to the current block."]
    pub fn insert_u_div(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSDiv instruction to the current block."]
    pub fn s_div(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSDiv instruction to the current block."]
    pub fn insert_s_div(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFDiv instruction to the current block."]
    pub fn f_div(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFDiv instruction to the current block."]
    pub fn insert_f_div(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUMod instruction to the current block."]
    pub fn u_mod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUMod instruction to the current block."]
    pub fn insert_u_mod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSRem instruction to the current block."]
    pub fn s_rem(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SRem,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSRem instruction to the current block."]
    pub fn insert_s_rem(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SRem,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSMod instruction to the current block."]
    pub fn s_mod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSMod instruction to the current block."]
    pub fn insert_s_mod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFRem instruction to the current block."]
    pub fn f_rem(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FRem,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFRem instruction to the current block."]
    pub fn insert_f_rem(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FRem,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFMod instruction to the current block."]
    pub fn f_mod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFMod instruction to the current block."]
    pub fn insert_f_mod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorTimesScalar instruction to the current block."]
    pub fn vector_times_scalar(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        scalar: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorTimesScalar,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(scalar)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorTimesScalar instruction to the current block."]
    pub fn insert_vector_times_scalar(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        scalar: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorTimesScalar,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(scalar)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesScalar instruction to the current block."]
    pub fn matrix_times_scalar(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
        scalar: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MatrixTimesScalar,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix), dr::Operand::IdRef(scalar)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesScalar instruction to the current block."]
    pub fn insert_matrix_times_scalar(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
        scalar: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MatrixTimesScalar,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix), dr::Operand::IdRef(scalar)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorTimesMatrix instruction to the current block."]
    pub fn vector_times_matrix(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorTimesMatrix,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(matrix)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpVectorTimesMatrix instruction to the current block."]
    pub fn insert_vector_times_matrix(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorTimesMatrix,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(matrix)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesVector instruction to the current block."]
    pub fn matrix_times_vector(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MatrixTimesVector,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix), dr::Operand::IdRef(vector)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesVector instruction to the current block."]
    pub fn insert_matrix_times_vector(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MatrixTimesVector,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix), dr::Operand::IdRef(vector)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesMatrix instruction to the current block."]
    pub fn matrix_times_matrix(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        left_matrix: spirv::Word,
        right_matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MatrixTimesMatrix,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(left_matrix),
                dr::Operand::IdRef(right_matrix),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesMatrix instruction to the current block."]
    pub fn insert_matrix_times_matrix(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        left_matrix: spirv::Word,
        right_matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MatrixTimesMatrix,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(left_matrix),
                dr::Operand::IdRef(right_matrix),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpOuterProduct instruction to the current block."]
    pub fn outer_product(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::OuterProduct,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpOuterProduct instruction to the current block."]
    pub fn insert_outer_product(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::OuterProduct,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDot instruction to the current block."]
    pub fn dot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Dot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDot instruction to the current block."]
    pub fn insert_dot(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Dot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAddCarry instruction to the current block."]
    pub fn i_add_carry(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAddCarry,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAddCarry instruction to the current block."]
    pub fn insert_i_add_carry(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAddCarry,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpISubBorrow instruction to the current block."]
    pub fn i_sub_borrow(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ISubBorrow,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpISubBorrow instruction to the current block."]
    pub fn insert_i_sub_borrow(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ISubBorrow,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUMulExtended instruction to the current block."]
    pub fn u_mul_extended(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UMulExtended,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUMulExtended instruction to the current block."]
    pub fn insert_u_mul_extended(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UMulExtended,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSMulExtended instruction to the current block."]
    pub fn s_mul_extended(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SMulExtended,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSMulExtended instruction to the current block."]
    pub fn insert_s_mul_extended(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SMulExtended,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAny instruction to the current block."]
    pub fn any(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Any,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAny instruction to the current block."]
    pub fn insert_any(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Any,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAll instruction to the current block."]
    pub fn all(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::All,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAll instruction to the current block."]
    pub fn insert_all(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::All,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsNan instruction to the current block."]
    pub fn is_nan(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsNan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsNan instruction to the current block."]
    pub fn insert_is_nan(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsNan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsInf instruction to the current block."]
    pub fn is_inf(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsInf,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsInf instruction to the current block."]
    pub fn insert_is_inf(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsInf,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsFinite instruction to the current block."]
    pub fn is_finite(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsFinite,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsFinite instruction to the current block."]
    pub fn insert_is_finite(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsFinite,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsNormal instruction to the current block."]
    pub fn is_normal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsNormal,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsNormal instruction to the current block."]
    pub fn insert_is_normal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsNormal,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSignBitSet instruction to the current block."]
    pub fn sign_bit_set(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SignBitSet,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSignBitSet instruction to the current block."]
    pub fn insert_sign_bit_set(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SignBitSet,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLessOrGreater instruction to the current block."]
    pub fn less_or_greater(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LessOrGreater,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLessOrGreater instruction to the current block."]
    pub fn insert_less_or_greater(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LessOrGreater,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpOrdered instruction to the current block."]
    pub fn ordered(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Ordered,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpOrdered instruction to the current block."]
    pub fn insert_ordered(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Ordered,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUnordered instruction to the current block."]
    pub fn unordered(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Unordered,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUnordered instruction to the current block."]
    pub fn insert_unordered(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Unordered,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalEqual instruction to the current block."]
    pub fn logical_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalEqual instruction to the current block."]
    pub fn insert_logical_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalNotEqual instruction to the current block."]
    pub fn logical_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalNotEqual instruction to the current block."]
    pub fn insert_logical_not_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalOr instruction to the current block."]
    pub fn logical_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalOr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalOr instruction to the current block."]
    pub fn insert_logical_or(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalOr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalAnd instruction to the current block."]
    pub fn logical_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalAnd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalAnd instruction to the current block."]
    pub fn insert_logical_and(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalAnd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalNot instruction to the current block."]
    pub fn logical_not(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalNot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalNot instruction to the current block."]
    pub fn insert_logical_not(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalNot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSelect instruction to the current block."]
    pub fn select(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        condition: spirv::Word,
        object_1: spirv::Word,
        object_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Select,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(condition),
                dr::Operand::IdRef(object_1),
                dr::Operand::IdRef(object_2),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSelect instruction to the current block."]
    pub fn insert_select(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        condition: spirv::Word,
        object_1: spirv::Word,
        object_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Select,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(condition),
                dr::Operand::IdRef(object_1),
                dr::Operand::IdRef(object_2),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIEqual instruction to the current block."]
    pub fn i_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIEqual instruction to the current block."]
    pub fn insert_i_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpINotEqual instruction to the current block."]
    pub fn i_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::INotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpINotEqual instruction to the current block."]
    pub fn insert_i_not_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::INotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUGreaterThan instruction to the current block."]
    pub fn u_greater_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUGreaterThan instruction to the current block."]
    pub fn insert_u_greater_than(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSGreaterThan instruction to the current block."]
    pub fn s_greater_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSGreaterThan instruction to the current block."]
    pub fn insert_s_greater_than(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUGreaterThanEqual instruction to the current block."]
    pub fn u_greater_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUGreaterThanEqual instruction to the current block."]
    pub fn insert_u_greater_than_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSGreaterThanEqual instruction to the current block."]
    pub fn s_greater_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSGreaterThanEqual instruction to the current block."]
    pub fn insert_s_greater_than_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpULessThan instruction to the current block."]
    pub fn u_less_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ULessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpULessThan instruction to the current block."]
    pub fn insert_u_less_than(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ULessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSLessThan instruction to the current block."]
    pub fn s_less_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSLessThan instruction to the current block."]
    pub fn insert_s_less_than(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpULessThanEqual instruction to the current block."]
    pub fn u_less_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ULessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpULessThanEqual instruction to the current block."]
    pub fn insert_u_less_than_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ULessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSLessThanEqual instruction to the current block."]
    pub fn s_less_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSLessThanEqual instruction to the current block."]
    pub fn insert_s_less_than_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdEqual instruction to the current block."]
    pub fn f_ord_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdEqual instruction to the current block."]
    pub fn insert_f_ord_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordEqual instruction to the current block."]
    pub fn f_unord_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordEqual instruction to the current block."]
    pub fn insert_f_unord_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdNotEqual instruction to the current block."]
    pub fn f_ord_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdNotEqual instruction to the current block."]
    pub fn insert_f_ord_not_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordNotEqual instruction to the current block."]
    pub fn f_unord_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordNotEqual instruction to the current block."]
    pub fn insert_f_unord_not_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdLessThan instruction to the current block."]
    pub fn f_ord_less_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdLessThan instruction to the current block."]
    pub fn insert_f_ord_less_than(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordLessThan instruction to the current block."]
    pub fn f_unord_less_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordLessThan instruction to the current block."]
    pub fn insert_f_unord_less_than(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdGreaterThan instruction to the current block."]
    pub fn f_ord_greater_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdGreaterThan instruction to the current block."]
    pub fn insert_f_ord_greater_than(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordGreaterThan instruction to the current block."]
    pub fn f_unord_greater_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordGreaterThan instruction to the current block."]
    pub fn insert_f_unord_greater_than(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdLessThanEqual instruction to the current block."]
    pub fn f_ord_less_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdLessThanEqual instruction to the current block."]
    pub fn insert_f_ord_less_than_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordLessThanEqual instruction to the current block."]
    pub fn f_unord_less_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordLessThanEqual instruction to the current block."]
    pub fn insert_f_unord_less_than_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdGreaterThanEqual instruction to the current block."]
    pub fn f_ord_greater_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdGreaterThanEqual instruction to the current block."]
    pub fn insert_f_ord_greater_than_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordGreaterThanEqual instruction to the current block."]
    pub fn f_unord_greater_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordGreaterThanEqual instruction to the current block."]
    pub fn insert_f_unord_greater_than_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpShiftRightLogical instruction to the current block."]
    pub fn shift_right_logical(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftRightLogical,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpShiftRightLogical instruction to the current block."]
    pub fn insert_shift_right_logical(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftRightLogical,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpShiftRightArithmetic instruction to the current block."]
    pub fn shift_right_arithmetic(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftRightArithmetic,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpShiftRightArithmetic instruction to the current block."]
    pub fn insert_shift_right_arithmetic(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftRightArithmetic,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpShiftLeftLogical instruction to the current block."]
    pub fn shift_left_logical(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftLeftLogical,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpShiftLeftLogical instruction to the current block."]
    pub fn insert_shift_left_logical(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftLeftLogical,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseOr instruction to the current block."]
    pub fn bitwise_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseOr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseOr instruction to the current block."]
    pub fn insert_bitwise_or(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseOr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseXor instruction to the current block."]
    pub fn bitwise_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseXor,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseXor instruction to the current block."]
    pub fn insert_bitwise_xor(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseXor,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseAnd instruction to the current block."]
    pub fn bitwise_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseAnd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseAnd instruction to the current block."]
    pub fn insert_bitwise_and(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseAnd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpNot instruction to the current block."]
    pub fn not(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Not,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpNot instruction to the current block."]
    pub fn insert_not(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Not,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldInsert instruction to the current block."]
    pub fn bit_field_insert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        insert: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitFieldInsert,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(insert),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(count),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldInsert instruction to the current block."]
    pub fn insert_bit_field_insert(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        insert: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitFieldInsert,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(insert),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(count),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldSExtract instruction to the current block."]
    pub fn bit_field_s_extract(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitFieldSExtract,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(count),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldSExtract instruction to the current block."]
    pub fn insert_bit_field_s_extract(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitFieldSExtract,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(count),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldUExtract instruction to the current block."]
    pub fn bit_field_u_extract(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitFieldUExtract,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(count),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldUExtract instruction to the current block."]
    pub fn insert_bit_field_u_extract(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitFieldUExtract,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(count),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitReverse instruction to the current block."]
    pub fn bit_reverse(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitReverse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitReverse instruction to the current block."]
    pub fn insert_bit_reverse(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitReverse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitCount instruction to the current block."]
    pub fn bit_count(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitCount,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitCount instruction to the current block."]
    pub fn insert_bit_count(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitCount,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdx instruction to the current block."]
    pub fn d_pdx(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdx,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdx instruction to the current block."]
    pub fn insert_d_pdx(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdx,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdy instruction to the current block."]
    pub fn d_pdy(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdy,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdy instruction to the current block."]
    pub fn insert_d_pdy(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdy,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFwidth instruction to the current block."]
    pub fn fwidth(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Fwidth,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFwidth instruction to the current block."]
    pub fn insert_fwidth(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Fwidth,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdxFine instruction to the current block."]
    pub fn d_pdx_fine(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdxFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdxFine instruction to the current block."]
    pub fn insert_d_pdx_fine(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdxFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdyFine instruction to the current block."]
    pub fn d_pdy_fine(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdyFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdyFine instruction to the current block."]
    pub fn insert_d_pdy_fine(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdyFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFwidthFine instruction to the current block."]
    pub fn fwidth_fine(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FwidthFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFwidthFine instruction to the current block."]
    pub fn insert_fwidth_fine(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FwidthFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdxCoarse instruction to the current block."]
    pub fn d_pdx_coarse(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdxCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdxCoarse instruction to the current block."]
    pub fn insert_d_pdx_coarse(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdxCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdyCoarse instruction to the current block."]
    pub fn d_pdy_coarse(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdyCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDPdyCoarse instruction to the current block."]
    pub fn insert_d_pdy_coarse(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdyCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFwidthCoarse instruction to the current block."]
    pub fn fwidth_coarse(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FwidthCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFwidthCoarse instruction to the current block."]
    pub fn insert_fwidth_coarse(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FwidthCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpEmitVertex instruction to the current block."]
    pub fn emit_vertex(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::EmitVertex, None, None, vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEmitVertex instruction to the current block."]
    pub fn insert_emit_vertex(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::EmitVertex, None, None, vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEndPrimitive instruction to the current block."]
    pub fn end_primitive(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::EndPrimitive, None, None, vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEndPrimitive instruction to the current block."]
    pub fn insert_end_primitive(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::EndPrimitive, None, None, vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEmitStreamVertex instruction to the current block."]
    pub fn emit_stream_vertex(&mut self, stream: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EmitStreamVertex,
            None,
            None,
            vec![dr::Operand::IdRef(stream)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEmitStreamVertex instruction to the current block."]
    pub fn insert_emit_stream_vertex(
        &mut self,
        insert_point: InsertPoint,
        stream: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EmitStreamVertex,
            None,
            None,
            vec![dr::Operand::IdRef(stream)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEndStreamPrimitive instruction to the current block."]
    pub fn end_stream_primitive(&mut self, stream: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EndStreamPrimitive,
            None,
            None,
            vec![dr::Operand::IdRef(stream)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEndStreamPrimitive instruction to the current block."]
    pub fn insert_end_stream_primitive(
        &mut self,
        insert_point: InsertPoint,
        stream: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EndStreamPrimitive,
            None,
            None,
            vec![dr::Operand::IdRef(stream)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpControlBarrier instruction to the current block."]
    pub fn control_barrier(
        &mut self,
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ControlBarrier,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpControlBarrier instruction to the current block."]
    pub fn insert_control_barrier(
        &mut self,
        insert_point: InsertPoint,
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ControlBarrier,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpMemoryBarrier instruction to the current block."]
    pub fn memory_barrier(
        &mut self,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemoryBarrier,
            None,
            None,
            vec![
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpMemoryBarrier instruction to the current block."]
    pub fn insert_memory_barrier(
        &mut self,
        insert_point: InsertPoint,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemoryBarrier,
            None,
            None,
            vec![
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpAtomicLoad instruction to the current block."]
    pub fn atomic_load(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicLoad,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicLoad instruction to the current block."]
    pub fn insert_atomic_load(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicLoad,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicStore instruction to the current block."]
    pub fn atomic_store(
        &mut self,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicStore,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpAtomicStore instruction to the current block."]
    pub fn insert_atomic_store(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicStore,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpAtomicExchange instruction to the current block."]
    pub fn atomic_exchange(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicExchange,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicExchange instruction to the current block."]
    pub fn insert_atomic_exchange(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicExchange,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicCompareExchange instruction to the current block."]
    pub fn atomic_compare_exchange(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicCompareExchange,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(equal),
                dr::Operand::IdMemorySemantics(unequal),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(comparator),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicCompareExchange instruction to the current block."]
    pub fn insert_atomic_compare_exchange(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicCompareExchange,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(equal),
                dr::Operand::IdMemorySemantics(unequal),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(comparator),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicCompareExchangeWeak instruction to the current block."]
    pub fn atomic_compare_exchange_weak(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicCompareExchangeWeak,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(equal),
                dr::Operand::IdMemorySemantics(unequal),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(comparator),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicCompareExchangeWeak instruction to the current block."]
    pub fn insert_atomic_compare_exchange_weak(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicCompareExchangeWeak,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(equal),
                dr::Operand::IdMemorySemantics(unequal),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(comparator),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIIncrement instruction to the current block."]
    pub fn atomic_i_increment(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIIncrement,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIIncrement instruction to the current block."]
    pub fn insert_atomic_i_increment(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIIncrement,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIDecrement instruction to the current block."]
    pub fn atomic_i_decrement(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIDecrement,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIDecrement instruction to the current block."]
    pub fn insert_atomic_i_decrement(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIDecrement,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIAdd instruction to the current block."]
    pub fn atomic_i_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIAdd instruction to the current block."]
    pub fn insert_atomic_i_add(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicISub instruction to the current block."]
    pub fn atomic_i_sub(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicISub,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicISub instruction to the current block."]
    pub fn insert_atomic_i_sub(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicISub,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicSMin instruction to the current block."]
    pub fn atomic_s_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicSMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicSMin instruction to the current block."]
    pub fn insert_atomic_s_min(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicSMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicUMin instruction to the current block."]
    pub fn atomic_u_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicUMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicUMin instruction to the current block."]
    pub fn insert_atomic_u_min(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicUMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicSMax instruction to the current block."]
    pub fn atomic_s_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicSMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicSMax instruction to the current block."]
    pub fn insert_atomic_s_max(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicSMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicUMax instruction to the current block."]
    pub fn atomic_u_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicUMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicUMax instruction to the current block."]
    pub fn insert_atomic_u_max(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicUMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicAnd instruction to the current block."]
    pub fn atomic_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicAnd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicAnd instruction to the current block."]
    pub fn insert_atomic_and(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicAnd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicOr instruction to the current block."]
    pub fn atomic_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicOr,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicOr instruction to the current block."]
    pub fn insert_atomic_or(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicOr,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicXor instruction to the current block."]
    pub fn atomic_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicXor instruction to the current block."]
    pub fn insert_atomic_xor(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPhi instruction to the current block."]
    pub fn phi(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        variable_parent: impl IntoIterator<Item = (spirv::Word, spirv::Word)>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Phi, Some(result_type), Some(_id), vec![]);
        for v in variable_parent {
            inst.operands.push(dr::Operand::IdRef(v.0));
            inst.operands.push(dr::Operand::IdRef(v.1));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPhi instruction to the current block."]
    pub fn insert_phi(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        variable_parent: impl IntoIterator<Item = (spirv::Word, spirv::Word)>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Phi, Some(result_type), Some(_id), vec![]);
        for v in variable_parent {
            inst.operands.push(dr::Operand::IdRef(v.0));
            inst.operands.push(dr::Operand::IdRef(v.1));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLoopMerge instruction to the current block."]
    pub fn loop_merge(
        &mut self,
        merge_block: spirv::Word,
        continue_target: spirv::Word,
        loop_control: spirv::LoopControl,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LoopMerge,
            None,
            None,
            vec![
                dr::Operand::IdRef(merge_block),
                dr::Operand::IdRef(continue_target),
                dr::Operand::LoopControl(loop_control),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpLoopMerge instruction to the current block."]
    pub fn insert_loop_merge(
        &mut self,
        insert_point: InsertPoint,
        merge_block: spirv::Word,
        continue_target: spirv::Word,
        loop_control: spirv::LoopControl,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LoopMerge,
            None,
            None,
            vec![
                dr::Operand::IdRef(merge_block),
                dr::Operand::IdRef(continue_target),
                dr::Operand::LoopControl(loop_control),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSelectionMerge instruction to the current block."]
    pub fn selection_merge(
        &mut self,
        merge_block: spirv::Word,
        selection_control: spirv::SelectionControl,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SelectionMerge,
            None,
            None,
            vec![
                dr::Operand::IdRef(merge_block),
                dr::Operand::SelectionControl(selection_control),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSelectionMerge instruction to the current block."]
    pub fn insert_selection_merge(
        &mut self,
        insert_point: InsertPoint,
        merge_block: spirv::Word,
        selection_control: spirv::SelectionControl,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SelectionMerge,
            None,
            None,
            vec![
                dr::Operand::IdRef(merge_block),
                dr::Operand::SelectionControl(selection_control),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGroupAsyncCopy instruction to the current block."]
    pub fn group_async_copy(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        destination: spirv::Word,
        source: spirv::Word,
        num_elements: spirv::Word,
        stride: spirv::Word,
        event: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupAsyncCopy,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(destination),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(num_elements),
                dr::Operand::IdRef(stride),
                dr::Operand::IdRef(event),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupAsyncCopy instruction to the current block."]
    pub fn insert_group_async_copy(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        destination: spirv::Word,
        source: spirv::Word,
        num_elements: spirv::Word,
        stride: spirv::Word,
        event: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupAsyncCopy,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(destination),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(num_elements),
                dr::Operand::IdRef(stride),
                dr::Operand::IdRef(event),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupWaitEvents instruction to the current block."]
    pub fn group_wait_events(
        &mut self,
        execution: spirv::Word,
        num_events: spirv::Word,
        events_list: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupWaitEvents,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(num_events),
                dr::Operand::IdRef(events_list),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGroupWaitEvents instruction to the current block."]
    pub fn insert_group_wait_events(
        &mut self,
        insert_point: InsertPoint,
        execution: spirv::Word,
        num_events: spirv::Word,
        events_list: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupWaitEvents,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(num_events),
                dr::Operand::IdRef(events_list),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGroupAll instruction to the current block."]
    pub fn group_all(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupAll,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupAll instruction to the current block."]
    pub fn insert_group_all(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupAll,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupAny instruction to the current block."]
    pub fn group_any(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupAny,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupAny instruction to the current block."]
    pub fn insert_group_any(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupAny,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBroadcast instruction to the current block."]
    pub fn group_broadcast(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        local_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupBroadcast,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(local_id),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBroadcast instruction to the current block."]
    pub fn insert_group_broadcast(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        local_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupBroadcast,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(local_id),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupIAdd instruction to the current block."]
    pub fn group_i_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupIAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupIAdd instruction to the current block."]
    pub fn insert_group_i_add(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupIAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFAdd instruction to the current block."]
    pub fn group_f_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFAdd instruction to the current block."]
    pub fn insert_group_f_add(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMin instruction to the current block."]
    pub fn group_f_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMin instruction to the current block."]
    pub fn insert_group_f_min(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMin instruction to the current block."]
    pub fn group_u_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupUMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMin instruction to the current block."]
    pub fn insert_group_u_min(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupUMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMin instruction to the current block."]
    pub fn group_s_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupSMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMin instruction to the current block."]
    pub fn insert_group_s_min(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupSMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMax instruction to the current block."]
    pub fn group_f_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMax instruction to the current block."]
    pub fn insert_group_f_max(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMax instruction to the current block."]
    pub fn group_u_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupUMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMax instruction to the current block."]
    pub fn insert_group_u_max(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupUMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMax instruction to the current block."]
    pub fn group_s_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupSMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMax instruction to the current block."]
    pub fn insert_group_s_max(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupSMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReadPipe instruction to the current block."]
    pub fn read_pipe(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReadPipe,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReadPipe instruction to the current block."]
    pub fn insert_read_pipe(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReadPipe,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpWritePipe instruction to the current block."]
    pub fn write_pipe(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::WritePipe,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpWritePipe instruction to the current block."]
    pub fn insert_write_pipe(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::WritePipe,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReservedReadPipe instruction to the current block."]
    pub fn reserved_read_pipe(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        index: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReservedReadPipe,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(index),
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReservedReadPipe instruction to the current block."]
    pub fn insert_reserved_read_pipe(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        index: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReservedReadPipe,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(index),
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReservedWritePipe instruction to the current block."]
    pub fn reserved_write_pipe(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        index: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReservedWritePipe,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(index),
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReservedWritePipe instruction to the current block."]
    pub fn insert_reserved_write_pipe(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        index: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReservedWritePipe,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(index),
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReserveReadPipePackets instruction to the current block."]
    pub fn reserve_read_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReserveReadPipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(num_packets),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReserveReadPipePackets instruction to the current block."]
    pub fn insert_reserve_read_pipe_packets(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReserveReadPipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(num_packets),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReserveWritePipePackets instruction to the current block."]
    pub fn reserve_write_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReserveWritePipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(num_packets),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReserveWritePipePackets instruction to the current block."]
    pub fn insert_reserve_write_pipe_packets(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReserveWritePipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(num_packets),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCommitReadPipe instruction to the current block."]
    pub fn commit_read_pipe(
        &mut self,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CommitReadPipe,
            None,
            None,
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCommitReadPipe instruction to the current block."]
    pub fn insert_commit_read_pipe(
        &mut self,
        insert_point: InsertPoint,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CommitReadPipe,
            None,
            None,
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCommitWritePipe instruction to the current block."]
    pub fn commit_write_pipe(
        &mut self,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CommitWritePipe,
            None,
            None,
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCommitWritePipe instruction to the current block."]
    pub fn insert_commit_write_pipe(
        &mut self,
        insert_point: InsertPoint,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CommitWritePipe,
            None,
            None,
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpIsValidReserveId instruction to the current block."]
    pub fn is_valid_reserve_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        reserve_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsValidReserveId,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(reserve_id)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsValidReserveId instruction to the current block."]
    pub fn insert_is_valid_reserve_id(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        reserve_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsValidReserveId,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(reserve_id)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetNumPipePackets instruction to the current block."]
    pub fn get_num_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetNumPipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetNumPipePackets instruction to the current block."]
    pub fn insert_get_num_pipe_packets(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetNumPipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetMaxPipePackets instruction to the current block."]
    pub fn get_max_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetMaxPipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetMaxPipePackets instruction to the current block."]
    pub fn insert_get_max_pipe_packets(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetMaxPipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupReserveReadPipePackets instruction to the current block."]
    pub fn group_reserve_read_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupReserveReadPipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(num_packets),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupReserveReadPipePackets instruction to the current block."]
    pub fn insert_group_reserve_read_pipe_packets(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupReserveReadPipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(num_packets),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupReserveWritePipePackets instruction to the current block."]
    pub fn group_reserve_write_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupReserveWritePipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(num_packets),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupReserveWritePipePackets instruction to the current block."]
    pub fn insert_group_reserve_write_pipe_packets(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupReserveWritePipePackets,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(num_packets),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupCommitReadPipe instruction to the current block."]
    pub fn group_commit_read_pipe(
        &mut self,
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupCommitReadPipe,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGroupCommitReadPipe instruction to the current block."]
    pub fn insert_group_commit_read_pipe(
        &mut self,
        insert_point: InsertPoint,
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupCommitReadPipe,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGroupCommitWritePipe instruction to the current block."]
    pub fn group_commit_write_pipe(
        &mut self,
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupCommitWritePipe,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGroupCommitWritePipe instruction to the current block."]
    pub fn insert_group_commit_write_pipe(
        &mut self,
        insert_point: InsertPoint,
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupCommitWritePipe,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(pipe),
                dr::Operand::IdRef(reserve_id),
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEnqueueMarker instruction to the current block."]
    pub fn enqueue_marker(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        queue: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EnqueueMarker,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(queue),
                dr::Operand::IdRef(num_events),
                dr::Operand::IdRef(wait_events),
                dr::Operand::IdRef(ret_event),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpEnqueueMarker instruction to the current block."]
    pub fn insert_enqueue_marker(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        queue: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EnqueueMarker,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(queue),
                dr::Operand::IdRef(num_events),
                dr::Operand::IdRef(wait_events),
                dr::Operand::IdRef(ret_event),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpEnqueueKernel instruction to the current block."]
    pub fn enqueue_kernel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        queue: spirv::Word,
        flags: spirv::Word,
        nd_range: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
        local_size: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EnqueueKernel,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(queue),
                dr::Operand::IdRef(flags),
                dr::Operand::IdRef(nd_range),
                dr::Operand::IdRef(num_events),
                dr::Operand::IdRef(wait_events),
                dr::Operand::IdRef(ret_event),
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        inst.operands
            .extend(local_size.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpEnqueueKernel instruction to the current block."]
    pub fn insert_enqueue_kernel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        queue: spirv::Word,
        flags: spirv::Word,
        nd_range: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
        local_size: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EnqueueKernel,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(queue),
                dr::Operand::IdRef(flags),
                dr::Operand::IdRef(nd_range),
                dr::Operand::IdRef(num_events),
                dr::Operand::IdRef(wait_events),
                dr::Operand::IdRef(ret_event),
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        inst.operands
            .extend(local_size.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelNDrangeSubGroupCount instruction to the current block."]
    pub fn get_kernel_n_drange_sub_group_count(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        nd_range: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelNDrangeSubGroupCount,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(nd_range),
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelNDrangeSubGroupCount instruction to the current block."]
    pub fn insert_get_kernel_n_drange_sub_group_count(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        nd_range: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelNDrangeSubGroupCount,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(nd_range),
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelNDrangeMaxSubGroupSize instruction to the current block."]
    pub fn get_kernel_n_drange_max_sub_group_size(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        nd_range: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelNDrangeMaxSubGroupSize,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(nd_range),
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelNDrangeMaxSubGroupSize instruction to the current block."]
    pub fn insert_get_kernel_n_drange_max_sub_group_size(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        nd_range: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelNDrangeMaxSubGroupSize,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(nd_range),
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelWorkGroupSize instruction to the current block."]
    pub fn get_kernel_work_group_size(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelWorkGroupSize,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelWorkGroupSize instruction to the current block."]
    pub fn insert_get_kernel_work_group_size(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelWorkGroupSize,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelPreferredWorkGroupSizeMultiple instruction to the current block."]
    pub fn get_kernel_preferred_work_group_size_multiple(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelPreferredWorkGroupSizeMultiple,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelPreferredWorkGroupSizeMultiple instruction to the current block."]
    pub fn insert_get_kernel_preferred_work_group_size_multiple(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelPreferredWorkGroupSizeMultiple,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRetainEvent instruction to the current block."]
    pub fn retain_event(&mut self, event: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RetainEvent,
            None,
            None,
            vec![dr::Operand::IdRef(event)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRetainEvent instruction to the current block."]
    pub fn insert_retain_event(
        &mut self,
        insert_point: InsertPoint,
        event: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RetainEvent,
            None,
            None,
            vec![dr::Operand::IdRef(event)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpReleaseEvent instruction to the current block."]
    pub fn release_event(&mut self, event: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReleaseEvent,
            None,
            None,
            vec![dr::Operand::IdRef(event)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpReleaseEvent instruction to the current block."]
    pub fn insert_release_event(
        &mut self,
        insert_point: InsertPoint,
        event: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReleaseEvent,
            None,
            None,
            vec![dr::Operand::IdRef(event)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCreateUserEvent instruction to the current block."]
    pub fn create_user_event(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreateUserEvent,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCreateUserEvent instruction to the current block."]
    pub fn insert_create_user_event(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreateUserEvent,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsValidEvent instruction to the current block."]
    pub fn is_valid_event(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        event: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsValidEvent,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(event)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsValidEvent instruction to the current block."]
    pub fn insert_is_valid_event(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        event: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsValidEvent,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(event)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSetUserEventStatus instruction to the current block."]
    pub fn set_user_event_status(
        &mut self,
        event: spirv::Word,
        status: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SetUserEventStatus,
            None,
            None,
            vec![dr::Operand::IdRef(event), dr::Operand::IdRef(status)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSetUserEventStatus instruction to the current block."]
    pub fn insert_set_user_event_status(
        &mut self,
        insert_point: InsertPoint,
        event: spirv::Word,
        status: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SetUserEventStatus,
            None,
            None,
            vec![dr::Operand::IdRef(event), dr::Operand::IdRef(status)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCaptureEventProfilingInfo instruction to the current block."]
    pub fn capture_event_profiling_info(
        &mut self,
        event: spirv::Word,
        profiling_info: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CaptureEventProfilingInfo,
            None,
            None,
            vec![
                dr::Operand::IdRef(event),
                dr::Operand::IdRef(profiling_info),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCaptureEventProfilingInfo instruction to the current block."]
    pub fn insert_capture_event_profiling_info(
        &mut self,
        insert_point: InsertPoint,
        event: spirv::Word,
        profiling_info: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CaptureEventProfilingInfo,
            None,
            None,
            vec![
                dr::Operand::IdRef(event),
                dr::Operand::IdRef(profiling_info),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGetDefaultQueue instruction to the current block."]
    pub fn get_default_queue(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetDefaultQueue,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetDefaultQueue instruction to the current block."]
    pub fn insert_get_default_queue(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetDefaultQueue,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBuildNDRange instruction to the current block."]
    pub fn build_nd_range(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        global_work_size: spirv::Word,
        local_work_size: spirv::Word,
        global_work_offset: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BuildNDRange,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(global_work_size),
                dr::Operand::IdRef(local_work_size),
                dr::Operand::IdRef(global_work_offset),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBuildNDRange instruction to the current block."]
    pub fn insert_build_nd_range(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        global_work_size: spirv::Word,
        local_work_size: spirv::Word,
        global_work_offset: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BuildNDRange,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(global_work_size),
                dr::Operand::IdRef(local_work_size),
                dr::Operand::IdRef(global_work_offset),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleImplicitLod instruction to the current block."]
    pub fn image_sparse_sample_implicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleImplicitLod instruction to the current block."]
    pub fn insert_image_sparse_sample_implicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleExplicitLod instruction to the current block."]
    pub fn image_sparse_sample_explicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleExplicitLod instruction to the current block."]
    pub fn insert_image_sparse_sample_explicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleDrefImplicitLod instruction to the current block."]
    pub fn image_sparse_sample_dref_implicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleDrefImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleDrefImplicitLod instruction to the current block."]
    pub fn insert_image_sparse_sample_dref_implicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleDrefImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleDrefExplicitLod instruction to the current block."]
    pub fn image_sparse_sample_dref_explicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleDrefExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleDrefExplicitLod instruction to the current block."]
    pub fn insert_image_sparse_sample_dref_explicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleDrefExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjImplicitLod instruction to the current block."]
    pub fn image_sparse_sample_proj_implicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleProjImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjImplicitLod instruction to the current block."]
    pub fn insert_image_sparse_sample_proj_implicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleProjImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjExplicitLod instruction to the current block."]
    pub fn image_sparse_sample_proj_explicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleProjExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjExplicitLod instruction to the current block."]
    pub fn insert_image_sparse_sample_proj_explicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleProjExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjDrefImplicitLod instruction to the current block."]
    pub fn image_sparse_sample_proj_dref_implicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleProjDrefImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjDrefImplicitLod instruction to the current block."]
    pub fn insert_image_sparse_sample_proj_dref_implicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleProjDrefImplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjDrefExplicitLod instruction to the current block."]
    pub fn image_sparse_sample_proj_dref_explicit_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleProjDrefExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjDrefExplicitLod instruction to the current block."]
    pub fn insert_image_sparse_sample_proj_dref_explicit_lod(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseSampleProjDrefExplicitLod,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
                dr::Operand::ImageOperands(image_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseFetch instruction to the current block."]
    pub fn image_sparse_fetch(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseFetch,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseFetch instruction to the current block."]
    pub fn insert_image_sparse_fetch(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseFetch,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseGather instruction to the current block."]
    pub fn image_sparse_gather(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseGather,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(component),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseGather instruction to the current block."]
    pub fn insert_image_sparse_gather(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseGather,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(component),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseDrefGather instruction to the current block."]
    pub fn image_sparse_dref_gather(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseDrefGather,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseDrefGather instruction to the current block."]
    pub fn insert_image_sparse_dref_gather(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseDrefGather,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(d_ref),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseTexelsResident instruction to the current block."]
    pub fn image_sparse_texels_resident(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        resident_code: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseTexelsResident,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(resident_code)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseTexelsResident instruction to the current block."]
    pub fn insert_image_sparse_texels_resident(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        resident_code: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseTexelsResident,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(resident_code)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFlagTestAndSet instruction to the current block."]
    pub fn atomic_flag_test_and_set(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFlagTestAndSet,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFlagTestAndSet instruction to the current block."]
    pub fn insert_atomic_flag_test_and_set(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFlagTestAndSet,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFlagClear instruction to the current block."]
    pub fn atomic_flag_clear(
        &mut self,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFlagClear,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpAtomicFlagClear instruction to the current block."]
    pub fn insert_atomic_flag_clear(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFlagClear,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpImageSparseRead instruction to the current block."]
    pub fn image_sparse_read(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseRead,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseRead instruction to the current block."]
    pub fn insert_image_sparse_read(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseRead,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSizeOf instruction to the current block."]
    pub fn size_of(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SizeOf,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSizeOf instruction to the current block."]
    pub fn insert_size_of(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SizeOf,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConstantPipeStorage instruction to the current block."]
    pub fn constant_pipe_storage(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        packet_size: u32,
        packet_alignment: u32,
        capacity: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantPipeStorage,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::LiteralBit32(packet_size),
                dr::Operand::LiteralBit32(packet_alignment),
                dr::Operand::LiteralBit32(capacity),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConstantPipeStorage instruction to the current block."]
    pub fn insert_constant_pipe_storage(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        packet_size: u32,
        packet_alignment: u32,
        capacity: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantPipeStorage,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::LiteralBit32(packet_size),
                dr::Operand::LiteralBit32(packet_alignment),
                dr::Operand::LiteralBit32(capacity),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCreatePipeFromPipeStorage instruction to the current block."]
    pub fn create_pipe_from_pipe_storage(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe_storage: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreatePipeFromPipeStorage,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pipe_storage)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCreatePipeFromPipeStorage instruction to the current block."]
    pub fn insert_create_pipe_from_pipe_storage(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe_storage: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreatePipeFromPipeStorage,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pipe_storage)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelLocalSizeForSubgroupCount instruction to the current block."]
    pub fn get_kernel_local_size_for_subgroup_count(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        subgroup_count: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelLocalSizeForSubgroupCount,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(subgroup_count),
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelLocalSizeForSubgroupCount instruction to the current block."]
    pub fn insert_get_kernel_local_size_for_subgroup_count(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        subgroup_count: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelLocalSizeForSubgroupCount,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(subgroup_count),
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelMaxNumSubgroups instruction to the current block."]
    pub fn get_kernel_max_num_subgroups(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelMaxNumSubgroups,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelMaxNumSubgroups instruction to the current block."]
    pub fn insert_get_kernel_max_num_subgroups(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetKernelMaxNumSubgroups,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(invoke),
                dr::Operand::IdRef(param),
                dr::Operand::IdRef(param_size),
                dr::Operand::IdRef(param_align),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpNamedBarrierInitialize instruction to the current block."]
    pub fn named_barrier_initialize(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        subgroup_count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::NamedBarrierInitialize,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(subgroup_count)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpNamedBarrierInitialize instruction to the current block."]
    pub fn insert_named_barrier_initialize(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        subgroup_count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::NamedBarrierInitialize,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(subgroup_count)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMemoryNamedBarrier instruction to the current block."]
    pub fn memory_named_barrier(
        &mut self,
        named_barrier: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemoryNamedBarrier,
            None,
            None,
            vec![
                dr::Operand::IdRef(named_barrier),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpMemoryNamedBarrier instruction to the current block."]
    pub fn insert_memory_named_barrier(
        &mut self,
        insert_point: InsertPoint,
        named_barrier: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MemoryNamedBarrier,
            None,
            None,
            vec![
                dr::Operand::IdRef(named_barrier),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGroupNonUniformElect instruction to the current block."]
    pub fn group_non_uniform_elect(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformElect,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformElect instruction to the current block."]
    pub fn insert_group_non_uniform_elect(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformElect,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAll instruction to the current block."]
    pub fn group_non_uniform_all(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformAll,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAll instruction to the current block."]
    pub fn insert_group_non_uniform_all(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformAll,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAny instruction to the current block."]
    pub fn group_non_uniform_any(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformAny,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAny instruction to the current block."]
    pub fn insert_group_non_uniform_any(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformAny,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAllEqual instruction to the current block."]
    pub fn group_non_uniform_all_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformAllEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAllEqual instruction to the current block."]
    pub fn insert_group_non_uniform_all_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformAllEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBroadcast instruction to the current block."]
    pub fn group_non_uniform_broadcast(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBroadcast,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(id),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBroadcast instruction to the current block."]
    pub fn insert_group_non_uniform_broadcast(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBroadcast,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(id),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBroadcastFirst instruction to the current block."]
    pub fn group_non_uniform_broadcast_first(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBroadcastFirst,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBroadcastFirst instruction to the current block."]
    pub fn insert_group_non_uniform_broadcast_first(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBroadcastFirst,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallot instruction to the current block."]
    pub fn group_non_uniform_ballot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallot,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallot instruction to the current block."]
    pub fn insert_group_non_uniform_ballot(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallot,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(predicate),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformInverseBallot instruction to the current block."]
    pub fn group_non_uniform_inverse_ballot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformInverseBallot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformInverseBallot instruction to the current block."]
    pub fn insert_group_non_uniform_inverse_ballot(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformInverseBallot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotBitExtract instruction to the current block."]
    pub fn group_non_uniform_ballot_bit_extract(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotBitExtract,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(index),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotBitExtract instruction to the current block."]
    pub fn insert_group_non_uniform_ballot_bit_extract(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotBitExtract,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(index),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotBitCount instruction to the current block."]
    pub fn group_non_uniform_ballot_bit_count(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotBitCount,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotBitCount instruction to the current block."]
    pub fn insert_group_non_uniform_ballot_bit_count(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotBitCount,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotFindLSB instruction to the current block."]
    pub fn group_non_uniform_ballot_find_lsb(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotFindLSB,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotFindLSB instruction to the current block."]
    pub fn insert_group_non_uniform_ballot_find_lsb(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotFindLSB,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotFindMSB instruction to the current block."]
    pub fn group_non_uniform_ballot_find_msb(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotFindMSB,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotFindMSB instruction to the current block."]
    pub fn insert_group_non_uniform_ballot_find_msb(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotFindMSB,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffle instruction to the current block."]
    pub fn group_non_uniform_shuffle(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformShuffle,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(id),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffle instruction to the current block."]
    pub fn insert_group_non_uniform_shuffle(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformShuffle,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(id),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleXor instruction to the current block."]
    pub fn group_non_uniform_shuffle_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        mask: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformShuffleXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(mask),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleXor instruction to the current block."]
    pub fn insert_group_non_uniform_shuffle_xor(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        mask: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformShuffleXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(mask),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleUp instruction to the current block."]
    pub fn group_non_uniform_shuffle_up(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformShuffleUp,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(delta),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleUp instruction to the current block."]
    pub fn insert_group_non_uniform_shuffle_up(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformShuffleUp,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(delta),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleDown instruction to the current block."]
    pub fn group_non_uniform_shuffle_down(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformShuffleDown,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(delta),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleDown instruction to the current block."]
    pub fn insert_group_non_uniform_shuffle_down(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformShuffleDown,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(delta),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformIAdd instruction to the current block."]
    pub fn group_non_uniform_i_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformIAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformIAdd instruction to the current block."]
    pub fn insert_group_non_uniform_i_add(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformIAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFAdd instruction to the current block."]
    pub fn group_non_uniform_f_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformFAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFAdd instruction to the current block."]
    pub fn insert_group_non_uniform_f_add(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformFAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformIMul instruction to the current block."]
    pub fn group_non_uniform_i_mul(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformIMul,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformIMul instruction to the current block."]
    pub fn insert_group_non_uniform_i_mul(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformIMul,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMul instruction to the current block."]
    pub fn group_non_uniform_f_mul(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformFMul,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMul instruction to the current block."]
    pub fn insert_group_non_uniform_f_mul(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformFMul,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformSMin instruction to the current block."]
    pub fn group_non_uniform_s_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformSMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformSMin instruction to the current block."]
    pub fn insert_group_non_uniform_s_min(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformSMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformUMin instruction to the current block."]
    pub fn group_non_uniform_u_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformUMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformUMin instruction to the current block."]
    pub fn insert_group_non_uniform_u_min(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformUMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMin instruction to the current block."]
    pub fn group_non_uniform_f_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformFMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMin instruction to the current block."]
    pub fn insert_group_non_uniform_f_min(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformFMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformSMax instruction to the current block."]
    pub fn group_non_uniform_s_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformSMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformSMax instruction to the current block."]
    pub fn insert_group_non_uniform_s_max(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformSMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformUMax instruction to the current block."]
    pub fn group_non_uniform_u_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformUMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformUMax instruction to the current block."]
    pub fn insert_group_non_uniform_u_max(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformUMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMax instruction to the current block."]
    pub fn group_non_uniform_f_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformFMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMax instruction to the current block."]
    pub fn insert_group_non_uniform_f_max(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformFMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseAnd instruction to the current block."]
    pub fn group_non_uniform_bitwise_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBitwiseAnd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseAnd instruction to the current block."]
    pub fn insert_group_non_uniform_bitwise_and(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBitwiseAnd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseOr instruction to the current block."]
    pub fn group_non_uniform_bitwise_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBitwiseOr,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseOr instruction to the current block."]
    pub fn insert_group_non_uniform_bitwise_or(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBitwiseOr,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseXor instruction to the current block."]
    pub fn group_non_uniform_bitwise_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBitwiseXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseXor instruction to the current block."]
    pub fn insert_group_non_uniform_bitwise_xor(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBitwiseXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalAnd instruction to the current block."]
    pub fn group_non_uniform_logical_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformLogicalAnd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalAnd instruction to the current block."]
    pub fn insert_group_non_uniform_logical_and(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformLogicalAnd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalOr instruction to the current block."]
    pub fn group_non_uniform_logical_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformLogicalOr,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalOr instruction to the current block."]
    pub fn insert_group_non_uniform_logical_or(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformLogicalOr,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalXor instruction to the current block."]
    pub fn group_non_uniform_logical_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformLogicalXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalXor instruction to the current block."]
    pub fn insert_group_non_uniform_logical_xor(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformLogicalXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(value),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadBroadcast instruction to the current block."]
    pub fn group_non_uniform_quad_broadcast(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformQuadBroadcast,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(index),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadBroadcast instruction to the current block."]
    pub fn insert_group_non_uniform_quad_broadcast(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformQuadBroadcast,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(index),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadSwap instruction to the current block."]
    pub fn group_non_uniform_quad_swap(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        direction: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformQuadSwap,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(direction),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadSwap instruction to the current block."]
    pub fn insert_group_non_uniform_quad_swap(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        direction: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformQuadSwap,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(direction),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCopyLogical instruction to the current block."]
    pub fn copy_logical(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyLogical,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCopyLogical instruction to the current block."]
    pub fn insert_copy_logical(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyLogical,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrEqual instruction to the current block."]
    pub fn ptr_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrEqual instruction to the current block."]
    pub fn insert_ptr_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrNotEqual instruction to the current block."]
    pub fn ptr_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrNotEqual instruction to the current block."]
    pub fn insert_ptr_not_equal(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrDiff instruction to the current block."]
    pub fn ptr_diff(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrDiff,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpPtrDiff instruction to the current block."]
    pub fn insert_ptr_diff(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrDiff,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpColorAttachmentReadEXT instruction to the current block."]
    pub fn color_attachment_read_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        attachment: spirv::Word,
        sample: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ColorAttachmentReadEXT,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(attachment)],
        );
        if let Some(v) = sample {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpColorAttachmentReadEXT instruction to the current block."]
    pub fn insert_color_attachment_read_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        attachment: spirv::Word,
        sample: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ColorAttachmentReadEXT,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(attachment)],
        );
        if let Some(v) = sample {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDepthAttachmentReadEXT instruction to the current block."]
    pub fn depth_attachment_read_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sample: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DepthAttachmentReadEXT,
            Some(result_type),
            Some(_id),
            vec![],
        );
        if let Some(v) = sample {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpDepthAttachmentReadEXT instruction to the current block."]
    pub fn insert_depth_attachment_read_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sample: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DepthAttachmentReadEXT,
            Some(result_type),
            Some(_id),
            vec![],
        );
        if let Some(v) = sample {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpStencilAttachmentReadEXT instruction to the current block."]
    pub fn stencil_attachment_read_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sample: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::StencilAttachmentReadEXT,
            Some(result_type),
            Some(_id),
            vec![],
        );
        if let Some(v) = sample {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpStencilAttachmentReadEXT instruction to the current block."]
    pub fn insert_stencil_attachment_read_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sample: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::StencilAttachmentReadEXT,
            Some(result_type),
            Some(_id),
            vec![],
        );
        if let Some(v) = sample {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorReadARM instruction to the current block."]
    pub fn tensor_read_arm(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor: spirv::Word,
        coordinates: spirv::Word,
        tensor_operands: Option<spirv::TensorOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorReadARM,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor), dr::Operand::IdRef(coordinates)],
        );
        if let Some(v) = tensor_operands {
            inst.operands.push(dr::Operand::TensorOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorReadARM instruction to the current block."]
    pub fn insert_tensor_read_arm(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor: spirv::Word,
        coordinates: spirv::Word,
        tensor_operands: Option<spirv::TensorOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorReadARM,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor), dr::Operand::IdRef(coordinates)],
        );
        if let Some(v) = tensor_operands {
            inst.operands.push(dr::Operand::TensorOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorWriteARM instruction to the current block."]
    pub fn tensor_write_arm(
        &mut self,
        tensor: spirv::Word,
        coordinates: spirv::Word,
        object: spirv::Word,
        tensor_operands: Option<spirv::TensorOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorWriteARM,
            None,
            None,
            vec![
                dr::Operand::IdRef(tensor),
                dr::Operand::IdRef(coordinates),
                dr::Operand::IdRef(object),
            ],
        );
        if let Some(v) = tensor_operands {
            inst.operands.push(dr::Operand::TensorOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTensorWriteARM instruction to the current block."]
    pub fn insert_tensor_write_arm(
        &mut self,
        insert_point: InsertPoint,
        tensor: spirv::Word,
        coordinates: spirv::Word,
        object: spirv::Word,
        tensor_operands: Option<spirv::TensorOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorWriteARM,
            None,
            None,
            vec![
                dr::Operand::IdRef(tensor),
                dr::Operand::IdRef(coordinates),
                dr::Operand::IdRef(object),
            ],
        );
        if let Some(v) = tensor_operands {
            inst.operands.push(dr::Operand::TensorOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTensorQuerySizeARM instruction to the current block."]
    pub fn tensor_query_size_arm(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor: spirv::Word,
        dimension: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorQuerySizeARM,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor), dr::Operand::IdRef(dimension)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorQuerySizeARM instruction to the current block."]
    pub fn insert_tensor_query_size_arm(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor: spirv::Word,
        dimension: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorQuerySizeARM,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor), dr::Operand::IdRef(dimension)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGraphConstantARM instruction to the current block."]
    pub fn graph_constant_arm(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        graph_constant_id: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GraphConstantARM,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::LiteralBit32(graph_constant_id)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGraphConstantARM instruction to the current block."]
    pub fn insert_graph_constant_arm(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        graph_constant_id: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GraphConstantARM,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::LiteralBit32(graph_constant_id)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGraphEntryPointARM instruction to the current block."]
    pub fn graph_entry_point_arm(
        &mut self,
        graph: spirv::Word,
        name: impl Into<String>,
        interface: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GraphEntryPointARM,
            None,
            None,
            vec![
                dr::Operand::IdRef(graph),
                dr::Operand::LiteralString(name.into()),
            ],
        );
        inst.operands
            .extend(interface.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGraphEntryPointARM instruction to the current block."]
    pub fn insert_graph_entry_point_arm(
        &mut self,
        insert_point: InsertPoint,
        graph: spirv::Word,
        name: impl Into<String>,
        interface: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GraphEntryPointARM,
            None,
            None,
            vec![
                dr::Operand::IdRef(graph),
                dr::Operand::LiteralString(name.into()),
            ],
        );
        inst.operands
            .extend(interface.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGraphARM instruction to the current block."]
    pub fn graph_arm(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst =
            dr::Instruction::new(spirv::Op::GraphARM, Some(result_type), Some(_id), vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGraphARM instruction to the current block."]
    pub fn insert_graph_arm(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst =
            dr::Instruction::new(spirv::Op::GraphARM, Some(result_type), Some(_id), vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGraphInputARM instruction to the current block."]
    pub fn graph_input_arm(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        input_index: spirv::Word,
        element_index: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GraphInputARM,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(input_index)],
        );
        inst.operands
            .extend(element_index.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGraphInputARM instruction to the current block."]
    pub fn insert_graph_input_arm(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        input_index: spirv::Word,
        element_index: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GraphInputARM,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(input_index)],
        );
        inst.operands
            .extend(element_index.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGraphSetOutputARM instruction to the current block."]
    pub fn graph_set_output_arm(
        &mut self,
        value: spirv::Word,
        output_index: spirv::Word,
        element_index: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GraphSetOutputARM,
            None,
            None,
            vec![dr::Operand::IdRef(value), dr::Operand::IdRef(output_index)],
        );
        inst.operands
            .extend(element_index.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGraphSetOutputARM instruction to the current block."]
    pub fn insert_graph_set_output_arm(
        &mut self,
        insert_point: InsertPoint,
        value: spirv::Word,
        output_index: spirv::Word,
        element_index: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GraphSetOutputARM,
            None,
            None,
            vec![dr::Operand::IdRef(value), dr::Operand::IdRef(output_index)],
        );
        inst.operands
            .extend(element_index.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGraphEndARM instruction to the current block."]
    pub fn graph_end_arm(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::GraphEndARM, None, None, vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGraphEndARM instruction to the current block."]
    pub fn insert_graph_end_arm(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::GraphEndARM, None, None, vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpUntypedVariableKHR instruction to the current block."]
    pub fn untyped_variable_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        storage_class: spirv::StorageClass,
        data_type: Option<spirv::Word>,
        initializer: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedVariableKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::StorageClass(storage_class)],
        );
        if let Some(v) = data_type {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = initializer {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedVariableKHR instruction to the current block."]
    pub fn insert_untyped_variable_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        storage_class: spirv::StorageClass,
        data_type: Option<spirv::Word>,
        initializer: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedVariableKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::StorageClass(storage_class)],
        );
        if let Some(v) = data_type {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = initializer {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedAccessChainKHR instruction to the current block."]
    pub fn untyped_access_chain_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        base: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedAccessChainKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base_type), dr::Operand::IdRef(base)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedAccessChainKHR instruction to the current block."]
    pub fn insert_untyped_access_chain_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        base: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedAccessChainKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base_type), dr::Operand::IdRef(base)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedInBoundsAccessChainKHR instruction to the current block."]
    pub fn untyped_in_bounds_access_chain_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        base: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedInBoundsAccessChainKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base_type), dr::Operand::IdRef(base)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedInBoundsAccessChainKHR instruction to the current block."]
    pub fn insert_untyped_in_bounds_access_chain_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        base: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedInBoundsAccessChainKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base_type), dr::Operand::IdRef(base)],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupBallotKHR instruction to the current block."]
    pub fn subgroup_ballot_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBallotKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupBallotKHR instruction to the current block."]
    pub fn insert_subgroup_ballot_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBallotKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupFirstInvocationKHR instruction to the current block."]
    pub fn subgroup_first_invocation_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupFirstInvocationKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupFirstInvocationKHR instruction to the current block."]
    pub fn insert_subgroup_first_invocation_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupFirstInvocationKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedPtrAccessChainKHR instruction to the current block."]
    pub fn untyped_ptr_access_chain_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        base: spirv::Word,
        element: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedPtrAccessChainKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base_type),
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(element),
            ],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedPtrAccessChainKHR instruction to the current block."]
    pub fn insert_untyped_ptr_access_chain_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        base: spirv::Word,
        element: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedPtrAccessChainKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base_type),
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(element),
            ],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedInBoundsPtrAccessChainKHR instruction to the current block."]
    pub fn untyped_in_bounds_ptr_access_chain_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        base: spirv::Word,
        element: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedInBoundsPtrAccessChainKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base_type),
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(element),
            ],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedInBoundsPtrAccessChainKHR instruction to the current block."]
    pub fn insert_untyped_in_bounds_ptr_access_chain_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        base: spirv::Word,
        element: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedInBoundsPtrAccessChainKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base_type),
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(element),
            ],
        );
        inst.operands
            .extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedArrayLengthKHR instruction to the current block."]
    pub fn untyped_array_length_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        structure: spirv::Word,
        pointer: spirv::Word,
        array_member: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedArrayLengthKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(structure),
                dr::Operand::IdRef(pointer),
                dr::Operand::LiteralBit32(array_member),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedArrayLengthKHR instruction to the current block."]
    pub fn insert_untyped_array_length_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        structure: spirv::Word,
        pointer: spirv::Word,
        array_member: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedArrayLengthKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(structure),
                dr::Operand::IdRef(pointer),
                dr::Operand::LiteralBit32(array_member),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUntypedPrefetchKHR instruction to the current block."]
    pub fn untyped_prefetch_khr(
        &mut self,
        pointer_type: spirv::Word,
        num_bytes: spirv::Word,
        rw: Option<spirv::Word>,
        locality: Option<spirv::Word>,
        cache_type: Option<spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedPrefetchKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer_type),
                dr::Operand::IdRef(num_bytes),
            ],
        );
        if let Some(v) = rw {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = locality {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = cache_type {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpUntypedPrefetchKHR instruction to the current block."]
    pub fn insert_untyped_prefetch_khr(
        &mut self,
        insert_point: InsertPoint,
        pointer_type: spirv::Word,
        num_bytes: spirv::Word,
        rw: Option<spirv::Word>,
        locality: Option<spirv::Word>,
        cache_type: Option<spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UntypedPrefetchKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer_type),
                dr::Operand::IdRef(num_bytes),
            ],
        );
        if let Some(v) = rw {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = locality {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = cache_type {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupAllKHR instruction to the current block."]
    pub fn subgroup_all_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAllKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupAllKHR instruction to the current block."]
    pub fn insert_subgroup_all_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAllKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupAnyKHR instruction to the current block."]
    pub fn subgroup_any_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAnyKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupAnyKHR instruction to the current block."]
    pub fn insert_subgroup_any_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAnyKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupAllEqualKHR instruction to the current block."]
    pub fn subgroup_all_equal_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAllEqualKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupAllEqualKHR instruction to the current block."]
    pub fn insert_subgroup_all_equal_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAllEqualKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformRotateKHR instruction to the current block."]
    pub fn group_non_uniform_rotate_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformRotateKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(delta),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformRotateKHR instruction to the current block."]
    pub fn insert_group_non_uniform_rotate_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformRotateKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(delta),
            ],
        );
        if let Some(v) = cluster_size {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupReadInvocationKHR instruction to the current block."]
    pub fn subgroup_read_invocation_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupReadInvocationKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value), dr::Operand::IdRef(index)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupReadInvocationKHR instruction to the current block."]
    pub fn insert_subgroup_read_invocation_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupReadInvocationKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value), dr::Operand::IdRef(index)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTraceRayKHR instruction to the current block."]
    pub fn trace_ray_khr(
        &mut self,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TraceRayKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(sbt_offset),
                dr::Operand::IdRef(sbt_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_tmin),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_tmax),
                dr::Operand::IdRef(payload),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTraceRayKHR instruction to the current block."]
    pub fn insert_trace_ray_khr(
        &mut self,
        insert_point: InsertPoint,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TraceRayKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(sbt_offset),
                dr::Operand::IdRef(sbt_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_tmin),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_tmax),
                dr::Operand::IdRef(payload),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpExecuteCallableKHR instruction to the current block."]
    pub fn execute_callable_khr(
        &mut self,
        sbt_index: spirv::Word,
        callable_data: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ExecuteCallableKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(sbt_index),
                dr::Operand::IdRef(callable_data),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpExecuteCallableKHR instruction to the current block."]
    pub fn insert_execute_callable_khr(
        &mut self,
        insert_point: InsertPoint,
        sbt_index: spirv::Word,
        callable_data: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ExecuteCallableKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(sbt_index),
                dr::Operand::IdRef(callable_data),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpConvertUToAccelerationStructureKHR instruction to the current block."]
    pub fn convert_u_to_acceleration_structure_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        accel: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToAccelerationStructureKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(accel)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToAccelerationStructureKHR instruction to the current block."]
    pub fn insert_convert_u_to_acceleration_structure_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        accel: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToAccelerationStructureKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(accel)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSDot instruction to the current block."]
    pub fn s_dot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SDot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSDot instruction to the current block."]
    pub fn insert_s_dot(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SDot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUDot instruction to the current block."]
    pub fn u_dot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UDot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUDot instruction to the current block."]
    pub fn insert_u_dot(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UDot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSUDot instruction to the current block."]
    pub fn su_dot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SUDot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSUDot instruction to the current block."]
    pub fn insert_su_dot(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SUDot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSDotAccSat instruction to the current block."]
    pub fn s_dot_acc_sat(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SDotAccSat,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(vector_1),
                dr::Operand::IdRef(vector_2),
                dr::Operand::IdRef(accumulator),
            ],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSDotAccSat instruction to the current block."]
    pub fn insert_s_dot_acc_sat(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SDotAccSat,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(vector_1),
                dr::Operand::IdRef(vector_2),
                dr::Operand::IdRef(accumulator),
            ],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUDotAccSat instruction to the current block."]
    pub fn u_dot_acc_sat(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UDotAccSat,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(vector_1),
                dr::Operand::IdRef(vector_2),
                dr::Operand::IdRef(accumulator),
            ],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUDotAccSat instruction to the current block."]
    pub fn insert_u_dot_acc_sat(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UDotAccSat,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(vector_1),
                dr::Operand::IdRef(vector_2),
                dr::Operand::IdRef(accumulator),
            ],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSUDotAccSat instruction to the current block."]
    pub fn su_dot_acc_sat(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SUDotAccSat,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(vector_1),
                dr::Operand::IdRef(vector_2),
                dr::Operand::IdRef(accumulator),
            ],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSUDotAccSat instruction to the current block."]
    pub fn insert_su_dot_acc_sat(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        accumulator: spirv::Word,
        packed_vector_format: Option<spirv::PackedVectorFormat>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SUDotAccSat,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(vector_1),
                dr::Operand::IdRef(vector_2),
                dr::Operand::IdRef(accumulator),
            ],
        );
        if let Some(v) = packed_vector_format {
            inst.operands.push(dr::Operand::PackedVectorFormat(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLoadKHR instruction to the current block."]
    pub fn cooperative_matrix_load_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory_layout: spirv::Word,
        stride: Option<spirv::Word>,
        memory_operand: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLoadKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(memory_layout),
            ],
        );
        if let Some(v) = stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = memory_operand {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLoadKHR instruction to the current block."]
    pub fn insert_cooperative_matrix_load_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory_layout: spirv::Word,
        stride: Option<spirv::Word>,
        memory_operand: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLoadKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(memory_layout),
            ],
        );
        if let Some(v) = stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = memory_operand {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixStoreKHR instruction to the current block."]
    pub fn cooperative_matrix_store_khr(
        &mut self,
        pointer: spirv::Word,
        object: spirv::Word,
        memory_layout: spirv::Word,
        stride: Option<spirv::Word>,
        memory_operand: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixStoreKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(object),
                dr::Operand::IdRef(memory_layout),
            ],
        );
        if let Some(v) = stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = memory_operand {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeMatrixStoreKHR instruction to the current block."]
    pub fn insert_cooperative_matrix_store_khr(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        object: spirv::Word,
        memory_layout: spirv::Word,
        stride: Option<spirv::Word>,
        memory_operand: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixStoreKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(object),
                dr::Operand::IdRef(memory_layout),
            ],
        );
        if let Some(v) = stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = memory_operand {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeMatrixMulAddKHR instruction to the current block."]
    pub fn cooperative_matrix_mul_add_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
        cooperative_matrix_operands: Option<spirv::CooperativeMatrixOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixMulAddKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(a),
                dr::Operand::IdRef(b),
                dr::Operand::IdRef(c),
            ],
        );
        if let Some(v) = cooperative_matrix_operands {
            inst.operands
                .push(dr::Operand::CooperativeMatrixOperands(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixMulAddKHR instruction to the current block."]
    pub fn insert_cooperative_matrix_mul_add_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
        cooperative_matrix_operands: Option<spirv::CooperativeMatrixOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixMulAddKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(a),
                dr::Operand::IdRef(b),
                dr::Operand::IdRef(c),
            ],
        );
        if let Some(v) = cooperative_matrix_operands {
            inst.operands
                .push(dr::Operand::CooperativeMatrixOperands(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLengthKHR instruction to the current block."]
    pub fn cooperative_matrix_length_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ty: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLengthKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ty)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLengthKHR instruction to the current block."]
    pub fn insert_cooperative_matrix_length_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ty: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLengthKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ty)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeConstructReplicateEXT instruction to the current block."]
    pub fn composite_construct_replicate_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeConstructReplicateEXT,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeConstructReplicateEXT instruction to the current block."]
    pub fn insert_composite_construct_replicate_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeConstructReplicateEXT,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryInitializeKHR instruction to the current block."]
    pub fn ray_query_initialize_khr(
        &mut self,
        ray_query: spirv::Word,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        ray_origin: spirv::Word,
        ray_t_min: spirv::Word,
        ray_direction: spirv::Word,
        ray_t_max: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryInitializeKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_t_min),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_t_max),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryInitializeKHR instruction to the current block."]
    pub fn insert_ray_query_initialize_khr(
        &mut self,
        insert_point: InsertPoint,
        ray_query: spirv::Word,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        ray_origin: spirv::Word,
        ray_t_min: spirv::Word,
        ray_direction: spirv::Word,
        ray_t_max: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryInitializeKHR,
            None,
            None,
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_t_min),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_t_max),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryTerminateKHR instruction to the current block."]
    pub fn ray_query_terminate_khr(&mut self, ray_query: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryTerminateKHR,
            None,
            None,
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryTerminateKHR instruction to the current block."]
    pub fn insert_ray_query_terminate_khr(
        &mut self,
        insert_point: InsertPoint,
        ray_query: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryTerminateKHR,
            None,
            None,
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryGenerateIntersectionKHR instruction to the current block."]
    pub fn ray_query_generate_intersection_khr(
        &mut self,
        ray_query: spirv::Word,
        hit_t: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGenerateIntersectionKHR,
            None,
            None,
            vec![dr::Operand::IdRef(ray_query), dr::Operand::IdRef(hit_t)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryGenerateIntersectionKHR instruction to the current block."]
    pub fn insert_ray_query_generate_intersection_khr(
        &mut self,
        insert_point: InsertPoint,
        ray_query: spirv::Word,
        hit_t: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGenerateIntersectionKHR,
            None,
            None,
            vec![dr::Operand::IdRef(ray_query), dr::Operand::IdRef(hit_t)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryConfirmIntersectionKHR instruction to the current block."]
    pub fn ray_query_confirm_intersection_khr(
        &mut self,
        ray_query: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryConfirmIntersectionKHR,
            None,
            None,
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryConfirmIntersectionKHR instruction to the current block."]
    pub fn insert_ray_query_confirm_intersection_khr(
        &mut self,
        insert_point: InsertPoint,
        ray_query: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryConfirmIntersectionKHR,
            None,
            None,
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryProceedKHR instruction to the current block."]
    pub fn ray_query_proceed_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryProceedKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryProceedKHR instruction to the current block."]
    pub fn insert_ray_query_proceed_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryProceedKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionTypeKHR instruction to the current block."]
    pub fn ray_query_get_intersection_type_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionTypeKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionTypeKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_type_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionTypeKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleWeightedQCOM instruction to the current block."]
    pub fn image_sample_weighted_qcom(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        texture: spirv::Word,
        coordinates: spirv::Word,
        weights: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleWeightedQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(texture),
                dr::Operand::IdRef(coordinates),
                dr::Operand::IdRef(weights),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleWeightedQCOM instruction to the current block."]
    pub fn insert_image_sample_weighted_qcom(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        texture: spirv::Word,
        coordinates: spirv::Word,
        weights: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleWeightedQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(texture),
                dr::Operand::IdRef(coordinates),
                dr::Operand::IdRef(weights),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBoxFilterQCOM instruction to the current block."]
    pub fn image_box_filter_qcom(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        texture: spirv::Word,
        coordinates: spirv::Word,
        box_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBoxFilterQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(texture),
                dr::Operand::IdRef(coordinates),
                dr::Operand::IdRef(box_size),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBoxFilterQCOM instruction to the current block."]
    pub fn insert_image_box_filter_qcom(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        texture: spirv::Word,
        coordinates: spirv::Word,
        box_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBoxFilterQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(texture),
                dr::Operand::IdRef(coordinates),
                dr::Operand::IdRef(box_size),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchSSDQCOM instruction to the current block."]
    pub fn image_block_match_ssdqcom(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target: spirv::Word,
        target_coordinates: spirv::Word,
        reference: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchSSDQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchSSDQCOM instruction to the current block."]
    pub fn insert_image_block_match_ssdqcom(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target: spirv::Word,
        target_coordinates: spirv::Word,
        reference: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchSSDQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchSADQCOM instruction to the current block."]
    pub fn image_block_match_sadqcom(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target: spirv::Word,
        target_coordinates: spirv::Word,
        reference: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchSADQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchSADQCOM instruction to the current block."]
    pub fn insert_image_block_match_sadqcom(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target: spirv::Word,
        target_coordinates: spirv::Word,
        reference: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchSADQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchWindowSSDQCOM instruction to the current block."]
    pub fn image_block_match_window_ssdqcom(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target_sampled_image: spirv::Word,
        target_coordinates: spirv::Word,
        reference_sampled_image: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchWindowSSDQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target_sampled_image),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference_sampled_image),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchWindowSSDQCOM instruction to the current block."]
    pub fn insert_image_block_match_window_ssdqcom(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target_sampled_image: spirv::Word,
        target_coordinates: spirv::Word,
        reference_sampled_image: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchWindowSSDQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target_sampled_image),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference_sampled_image),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchWindowSADQCOM instruction to the current block."]
    pub fn image_block_match_window_sadqcom(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target_sampled_image: spirv::Word,
        target_coordinates: spirv::Word,
        reference_sampled_image: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchWindowSADQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target_sampled_image),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference_sampled_image),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchWindowSADQCOM instruction to the current block."]
    pub fn insert_image_block_match_window_sadqcom(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target_sampled_image: spirv::Word,
        target_coordinates: spirv::Word,
        reference_sampled_image: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchWindowSADQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target_sampled_image),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference_sampled_image),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchGatherSSDQCOM instruction to the current block."]
    pub fn image_block_match_gather_ssdqcom(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target_sampled_image: spirv::Word,
        target_coordinates: spirv::Word,
        reference_sampled_image: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchGatherSSDQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target_sampled_image),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference_sampled_image),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchGatherSSDQCOM instruction to the current block."]
    pub fn insert_image_block_match_gather_ssdqcom(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target_sampled_image: spirv::Word,
        target_coordinates: spirv::Word,
        reference_sampled_image: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchGatherSSDQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target_sampled_image),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference_sampled_image),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchGatherSADQCOM instruction to the current block."]
    pub fn image_block_match_gather_sadqcom(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target_sampled_image: spirv::Word,
        target_coordinates: spirv::Word,
        reference_sampled_image: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchGatherSADQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target_sampled_image),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference_sampled_image),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageBlockMatchGatherSADQCOM instruction to the current block."]
    pub fn insert_image_block_match_gather_sadqcom(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target_sampled_image: spirv::Word,
        target_coordinates: spirv::Word,
        reference_sampled_image: spirv::Word,
        reference_coordinates: spirv::Word,
        block_size: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageBlockMatchGatherSADQCOM,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(target_sampled_image),
                dr::Operand::IdRef(target_coordinates),
                dr::Operand::IdRef(reference_sampled_image),
                dr::Operand::IdRef(reference_coordinates),
                dr::Operand::IdRef(block_size),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupIAddNonUniformAMD instruction to the current block."]
    pub fn group_i_add_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupIAddNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupIAddNonUniformAMD instruction to the current block."]
    pub fn insert_group_i_add_non_uniform_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupIAddNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFAddNonUniformAMD instruction to the current block."]
    pub fn group_f_add_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFAddNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFAddNonUniformAMD instruction to the current block."]
    pub fn insert_group_f_add_non_uniform_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFAddNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMinNonUniformAMD instruction to the current block."]
    pub fn group_f_min_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMinNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMinNonUniformAMD instruction to the current block."]
    pub fn insert_group_f_min_non_uniform_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMinNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMinNonUniformAMD instruction to the current block."]
    pub fn group_u_min_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupUMinNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMinNonUniformAMD instruction to the current block."]
    pub fn insert_group_u_min_non_uniform_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupUMinNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMinNonUniformAMD instruction to the current block."]
    pub fn group_s_min_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupSMinNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMinNonUniformAMD instruction to the current block."]
    pub fn insert_group_s_min_non_uniform_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupSMinNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMaxNonUniformAMD instruction to the current block."]
    pub fn group_f_max_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMaxNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMaxNonUniformAMD instruction to the current block."]
    pub fn insert_group_f_max_non_uniform_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMaxNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMaxNonUniformAMD instruction to the current block."]
    pub fn group_u_max_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupUMaxNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMaxNonUniformAMD instruction to the current block."]
    pub fn insert_group_u_max_non_uniform_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupUMaxNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMaxNonUniformAMD instruction to the current block."]
    pub fn group_s_max_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupSMaxNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMaxNonUniformAMD instruction to the current block."]
    pub fn insert_group_s_max_non_uniform_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupSMaxNonUniformAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFragmentMaskFetchAMD instruction to the current block."]
    pub fn fragment_mask_fetch_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FragmentMaskFetchAMD,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFragmentMaskFetchAMD instruction to the current block."]
    pub fn insert_fragment_mask_fetch_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FragmentMaskFetchAMD,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFragmentFetchAMD instruction to the current block."]
    pub fn fragment_fetch_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        fragment_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FragmentFetchAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(fragment_index),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFragmentFetchAMD instruction to the current block."]
    pub fn insert_fragment_fetch_amd(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        fragment_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FragmentFetchAMD,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(fragment_index),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReadClockKHR instruction to the current block."]
    pub fn read_clock_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        scope: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReadClockKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(scope)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReadClockKHR instruction to the current block."]
    pub fn insert_read_clock_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        scope: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReadClockKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(scope)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAllocateNodePayloadsAMDX instruction to the current block."]
    pub fn allocate_node_payloads_amdx(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        visibility: spirv::Word,
        payload_count: spirv::Word,
        node_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AllocateNodePayloadsAMDX,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(visibility),
                dr::Operand::IdRef(payload_count),
                dr::Operand::IdRef(node_index),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAllocateNodePayloadsAMDX instruction to the current block."]
    pub fn insert_allocate_node_payloads_amdx(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        visibility: spirv::Word,
        payload_count: spirv::Word,
        node_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AllocateNodePayloadsAMDX,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(visibility),
                dr::Operand::IdRef(payload_count),
                dr::Operand::IdRef(node_index),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpEnqueueNodePayloadsAMDX instruction to the current block."]
    pub fn enqueue_node_payloads_amdx(&mut self, payload_array: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EnqueueNodePayloadsAMDX,
            None,
            None,
            vec![dr::Operand::IdRef(payload_array)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEnqueueNodePayloadsAMDX instruction to the current block."]
    pub fn insert_enqueue_node_payloads_amdx(
        &mut self,
        insert_point: InsertPoint,
        payload_array: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EnqueueNodePayloadsAMDX,
            None,
            None,
            vec![dr::Operand::IdRef(payload_array)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpFinishWritingNodePayloadAMDX instruction to the current block."]
    pub fn finish_writing_node_payload_amdx(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        payload: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FinishWritingNodePayloadAMDX,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(payload)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFinishWritingNodePayloadAMDX instruction to the current block."]
    pub fn insert_finish_writing_node_payload_amdx(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        payload: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FinishWritingNodePayloadAMDX,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(payload)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpNodePayloadArrayLengthAMDX instruction to the current block."]
    pub fn node_payload_array_length_amdx(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        payload_array: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::NodePayloadArrayLengthAMDX,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(payload_array)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpNodePayloadArrayLengthAMDX instruction to the current block."]
    pub fn insert_node_payload_array_length_amdx(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        payload_array: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::NodePayloadArrayLengthAMDX,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(payload_array)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsNodePayloadValidAMDX instruction to the current block."]
    pub fn is_node_payload_valid_amdx(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        payload_type: spirv::Word,
        node_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsNodePayloadValidAMDX,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(payload_type),
                dr::Operand::IdRef(node_index),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsNodePayloadValidAMDX instruction to the current block."]
    pub fn insert_is_node_payload_valid_amdx(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        payload_type: spirv::Word,
        node_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsNodePayloadValidAMDX,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(payload_type),
                dr::Operand::IdRef(node_index),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConstantStringAMDX instruction to the current block."]
    pub fn constant_string_amdx(
        &mut self,
        result_id: Option<spirv::Word>,
        literal_string: impl Into<String>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantStringAMDX,
            None,
            Some(_id),
            vec![dr::Operand::LiteralString(literal_string.into())],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConstantStringAMDX instruction to the current block."]
    pub fn insert_constant_string_amdx(
        &mut self,
        insert_point: InsertPoint,
        result_id: Option<spirv::Word>,
        literal_string: impl Into<String>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConstantStringAMDX,
            None,
            Some(_id),
            vec![dr::Operand::LiteralString(literal_string.into())],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSpecConstantStringAMDX instruction to the current block."]
    pub fn spec_constant_string_amdx(
        &mut self,
        result_id: Option<spirv::Word>,
        literal_string: impl Into<String>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantStringAMDX,
            None,
            Some(_id),
            vec![dr::Operand::LiteralString(literal_string.into())],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSpecConstantStringAMDX instruction to the current block."]
    pub fn insert_spec_constant_string_amdx(
        &mut self,
        insert_point: InsertPoint,
        result_id: Option<spirv::Word>,
        literal_string: impl Into<String>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SpecConstantStringAMDX,
            None,
            Some(_id),
            vec![dr::Operand::LiteralString(literal_string.into())],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadAllKHR instruction to the current block."]
    pub fn group_non_uniform_quad_all_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformQuadAllKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadAllKHR instruction to the current block."]
    pub fn insert_group_non_uniform_quad_all_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformQuadAllKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadAnyKHR instruction to the current block."]
    pub fn group_non_uniform_quad_any_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformQuadAnyKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadAnyKHR instruction to the current block."]
    pub fn insert_group_non_uniform_quad_any_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformQuadAnyKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectRecordHitMotionNV instruction to the current block."]
    pub fn hit_object_record_hit_motion_nv(
        &mut self,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
        hit_object_attributes: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordHitMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(primitive_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(hit_kind),
                dr::Operand::IdRef(sbt_record_offset),
                dr::Operand::IdRef(sbt_record_stride),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(current_time),
                dr::Operand::IdRef(hit_object_attributes),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordHitMotionNV instruction to the current block."]
    pub fn insert_hit_object_record_hit_motion_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
        hit_object_attributes: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordHitMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(primitive_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(hit_kind),
                dr::Operand::IdRef(sbt_record_offset),
                dr::Operand::IdRef(sbt_record_stride),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(current_time),
                dr::Operand::IdRef(hit_object_attributes),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordHitWithIndexMotionNV instruction to the current block."]
    pub fn hit_object_record_hit_with_index_motion_nv(
        &mut self,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
        hit_object_attributes: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordHitWithIndexMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(primitive_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(hit_kind),
                dr::Operand::IdRef(sbt_record_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(current_time),
                dr::Operand::IdRef(hit_object_attributes),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordHitWithIndexMotionNV instruction to the current block."]
    pub fn insert_hit_object_record_hit_with_index_motion_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
        hit_object_attributes: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordHitWithIndexMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(primitive_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(hit_kind),
                dr::Operand::IdRef(sbt_record_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(current_time),
                dr::Operand::IdRef(hit_object_attributes),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordMissMotionNV instruction to the current block."]
    pub fn hit_object_record_miss_motion_nv(
        &mut self,
        hit_object: spirv::Word,
        sbt_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordMissMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(sbt_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(current_time),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordMissMotionNV instruction to the current block."]
    pub fn insert_hit_object_record_miss_motion_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        sbt_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        current_time: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordMissMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(sbt_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(current_time),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectGetWorldToObjectNV instruction to the current block."]
    pub fn hit_object_get_world_to_object_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetWorldToObjectNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetWorldToObjectNV instruction to the current block."]
    pub fn insert_hit_object_get_world_to_object_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetWorldToObjectNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetObjectToWorldNV instruction to the current block."]
    pub fn hit_object_get_object_to_world_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetObjectToWorldNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetObjectToWorldNV instruction to the current block."]
    pub fn insert_hit_object_get_object_to_world_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetObjectToWorldNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetObjectRayDirectionNV instruction to the current block."]
    pub fn hit_object_get_object_ray_direction_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetObjectRayDirectionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetObjectRayDirectionNV instruction to the current block."]
    pub fn insert_hit_object_get_object_ray_direction_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetObjectRayDirectionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetObjectRayOriginNV instruction to the current block."]
    pub fn hit_object_get_object_ray_origin_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetObjectRayOriginNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetObjectRayOriginNV instruction to the current block."]
    pub fn insert_hit_object_get_object_ray_origin_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetObjectRayOriginNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectTraceRayMotionNV instruction to the current block."]
    pub fn hit_object_trace_ray_motion_nv(
        &mut self,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        ray_flags: spirv::Word,
        cullmask: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        miss_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        time: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectTraceRayMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cullmask),
                dr::Operand::IdRef(sbt_record_offset),
                dr::Operand::IdRef(sbt_record_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(time),
                dr::Operand::IdRef(payload),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectTraceRayMotionNV instruction to the current block."]
    pub fn insert_hit_object_trace_ray_motion_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        ray_flags: spirv::Word,
        cullmask: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        miss_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        time: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectTraceRayMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cullmask),
                dr::Operand::IdRef(sbt_record_offset),
                dr::Operand::IdRef(sbt_record_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(time),
                dr::Operand::IdRef(payload),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectGetShaderRecordBufferHandleNV instruction to the current block."]
    pub fn hit_object_get_shader_record_buffer_handle_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetShaderRecordBufferHandleNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetShaderRecordBufferHandleNV instruction to the current block."]
    pub fn insert_hit_object_get_shader_record_buffer_handle_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetShaderRecordBufferHandleNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetShaderBindingTableRecordIndexNV instruction to the current block."]
    pub fn hit_object_get_shader_binding_table_record_index_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetShaderBindingTableRecordIndexNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetShaderBindingTableRecordIndexNV instruction to the current block."]
    pub fn insert_hit_object_get_shader_binding_table_record_index_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetShaderBindingTableRecordIndexNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectRecordEmptyNV instruction to the current block."]
    pub fn hit_object_record_empty_nv(&mut self, hit_object: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordEmptyNV,
            None,
            None,
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordEmptyNV instruction to the current block."]
    pub fn insert_hit_object_record_empty_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordEmptyNV,
            None,
            None,
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectTraceRayNV instruction to the current block."]
    pub fn hit_object_trace_ray_nv(
        &mut self,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        ray_flags: spirv::Word,
        cullmask: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        miss_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectTraceRayNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cullmask),
                dr::Operand::IdRef(sbt_record_offset),
                dr::Operand::IdRef(sbt_record_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(payload),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectTraceRayNV instruction to the current block."]
    pub fn insert_hit_object_trace_ray_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        ray_flags: spirv::Word,
        cullmask: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        miss_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectTraceRayNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cullmask),
                dr::Operand::IdRef(sbt_record_offset),
                dr::Operand::IdRef(sbt_record_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(payload),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordHitNV instruction to the current block."]
    pub fn hit_object_record_hit_nv(
        &mut self,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        hit_object_attributes: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordHitNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(primitive_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(hit_kind),
                dr::Operand::IdRef(sbt_record_offset),
                dr::Operand::IdRef(sbt_record_stride),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(hit_object_attributes),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordHitNV instruction to the current block."]
    pub fn insert_hit_object_record_hit_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_offset: spirv::Word,
        sbt_record_stride: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        hit_object_attributes: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordHitNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(primitive_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(hit_kind),
                dr::Operand::IdRef(sbt_record_offset),
                dr::Operand::IdRef(sbt_record_stride),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(hit_object_attributes),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordHitWithIndexNV instruction to the current block."]
    pub fn hit_object_record_hit_with_index_nv(
        &mut self,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        hit_object_attributes: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordHitWithIndexNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(primitive_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(hit_kind),
                dr::Operand::IdRef(sbt_record_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(hit_object_attributes),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordHitWithIndexNV instruction to the current block."]
    pub fn insert_hit_object_record_hit_with_index_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        acceleration_structure: spirv::Word,
        instance_id: spirv::Word,
        primitive_id: spirv::Word,
        geometry_index: spirv::Word,
        hit_kind: spirv::Word,
        sbt_record_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
        hit_object_attributes: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordHitWithIndexNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(acceleration_structure),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(primitive_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(hit_kind),
                dr::Operand::IdRef(sbt_record_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
                dr::Operand::IdRef(hit_object_attributes),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordMissNV instruction to the current block."]
    pub fn hit_object_record_miss_nv(
        &mut self,
        hit_object: spirv::Word,
        sbt_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordMissNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(sbt_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectRecordMissNV instruction to the current block."]
    pub fn insert_hit_object_record_miss_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        sbt_index: spirv::Word,
        origin: spirv::Word,
        t_min: spirv::Word,
        direction: spirv::Word,
        t_max: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectRecordMissNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(sbt_index),
                dr::Operand::IdRef(origin),
                dr::Operand::IdRef(t_min),
                dr::Operand::IdRef(direction),
                dr::Operand::IdRef(t_max),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectExecuteShaderNV instruction to the current block."]
    pub fn hit_object_execute_shader_nv(
        &mut self,
        hit_object: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectExecuteShaderNV,
            None,
            None,
            vec![dr::Operand::IdRef(hit_object), dr::Operand::IdRef(payload)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectExecuteShaderNV instruction to the current block."]
    pub fn insert_hit_object_execute_shader_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectExecuteShaderNV,
            None,
            None,
            vec![dr::Operand::IdRef(hit_object), dr::Operand::IdRef(payload)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectGetCurrentTimeNV instruction to the current block."]
    pub fn hit_object_get_current_time_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetCurrentTimeNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetCurrentTimeNV instruction to the current block."]
    pub fn insert_hit_object_get_current_time_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetCurrentTimeNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetAttributesNV instruction to the current block."]
    pub fn hit_object_get_attributes_nv(
        &mut self,
        hit_object: spirv::Word,
        hit_object_attribute: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetAttributesNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(hit_object_attribute),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectGetAttributesNV instruction to the current block."]
    pub fn insert_hit_object_get_attributes_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        hit_object_attribute: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetAttributesNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(hit_object),
                dr::Operand::IdRef(hit_object_attribute),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpHitObjectGetHitKindNV instruction to the current block."]
    pub fn hit_object_get_hit_kind_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetHitKindNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetHitKindNV instruction to the current block."]
    pub fn insert_hit_object_get_hit_kind_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetHitKindNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetPrimitiveIndexNV instruction to the current block."]
    pub fn hit_object_get_primitive_index_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetPrimitiveIndexNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetPrimitiveIndexNV instruction to the current block."]
    pub fn insert_hit_object_get_primitive_index_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetPrimitiveIndexNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetGeometryIndexNV instruction to the current block."]
    pub fn hit_object_get_geometry_index_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetGeometryIndexNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetGeometryIndexNV instruction to the current block."]
    pub fn insert_hit_object_get_geometry_index_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetGeometryIndexNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetInstanceIdNV instruction to the current block."]
    pub fn hit_object_get_instance_id_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetInstanceIdNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetInstanceIdNV instruction to the current block."]
    pub fn insert_hit_object_get_instance_id_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetInstanceIdNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetInstanceCustomIndexNV instruction to the current block."]
    pub fn hit_object_get_instance_custom_index_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetInstanceCustomIndexNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetInstanceCustomIndexNV instruction to the current block."]
    pub fn insert_hit_object_get_instance_custom_index_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetInstanceCustomIndexNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetWorldRayDirectionNV instruction to the current block."]
    pub fn hit_object_get_world_ray_direction_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetWorldRayDirectionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetWorldRayDirectionNV instruction to the current block."]
    pub fn insert_hit_object_get_world_ray_direction_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetWorldRayDirectionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetWorldRayOriginNV instruction to the current block."]
    pub fn hit_object_get_world_ray_origin_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetWorldRayOriginNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetWorldRayOriginNV instruction to the current block."]
    pub fn insert_hit_object_get_world_ray_origin_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetWorldRayOriginNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetRayTMaxNV instruction to the current block."]
    pub fn hit_object_get_ray_t_max_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetRayTMaxNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetRayTMaxNV instruction to the current block."]
    pub fn insert_hit_object_get_ray_t_max_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetRayTMaxNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetRayTMinNV instruction to the current block."]
    pub fn hit_object_get_ray_t_min_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetRayTMinNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetRayTMinNV instruction to the current block."]
    pub fn insert_hit_object_get_ray_t_min_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetRayTMinNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsEmptyNV instruction to the current block."]
    pub fn hit_object_is_empty_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsEmptyNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsEmptyNV instruction to the current block."]
    pub fn insert_hit_object_is_empty_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsEmptyNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsHitNV instruction to the current block."]
    pub fn hit_object_is_hit_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsHitNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsHitNV instruction to the current block."]
    pub fn insert_hit_object_is_hit_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsHitNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsMissNV instruction to the current block."]
    pub fn hit_object_is_miss_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsMissNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsMissNV instruction to the current block."]
    pub fn insert_hit_object_is_miss_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsMissNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReorderThreadWithHitObjectNV instruction to the current block."]
    pub fn reorder_thread_with_hit_object_nv(
        &mut self,
        hit_object: spirv::Word,
        hint: Option<spirv::Word>,
        bits: Option<spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReorderThreadWithHitObjectNV,
            None,
            None,
            vec![dr::Operand::IdRef(hit_object)],
        );
        if let Some(v) = hint {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = bits {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpReorderThreadWithHitObjectNV instruction to the current block."]
    pub fn insert_reorder_thread_with_hit_object_nv(
        &mut self,
        insert_point: InsertPoint,
        hit_object: spirv::Word,
        hint: Option<spirv::Word>,
        bits: Option<spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReorderThreadWithHitObjectNV,
            None,
            None,
            vec![dr::Operand::IdRef(hit_object)],
        );
        if let Some(v) = hint {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = bits {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpReorderThreadWithHintNV instruction to the current block."]
    pub fn reorder_thread_with_hint_nv(
        &mut self,
        hint: spirv::Word,
        bits: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReorderThreadWithHintNV,
            None,
            None,
            vec![dr::Operand::IdRef(hint), dr::Operand::IdRef(bits)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpReorderThreadWithHintNV instruction to the current block."]
    pub fn insert_reorder_thread_with_hint_nv(
        &mut self,
        insert_point: InsertPoint,
        hint: spirv::Word,
        bits: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReorderThreadWithHintNV,
            None,
            None,
            vec![dr::Operand::IdRef(hint), dr::Operand::IdRef(bits)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpImageSampleFootprintNV instruction to the current block."]
    pub fn image_sample_footprint_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        granularity: spirv::Word,
        coarse: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleFootprintNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(granularity),
                dr::Operand::IdRef(coarse),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleFootprintNV instruction to the current block."]
    pub fn insert_image_sample_footprint_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        granularity: spirv::Word,
        coarse: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSampleFootprintNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(sampled_image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(granularity),
                dr::Operand::IdRef(coarse),
            ],
        );
        if let Some(v) = image_operands {
            inst.operands.push(dr::Operand::ImageOperands(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeVectorMatrixMulNV instruction to the current block."]
    pub fn cooperative_vector_matrix_mul_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        input: spirv::Word,
        input_interpretation: spirv::Word,
        matrix: spirv::Word,
        matrix_offset: spirv::Word,
        matrix_interpretation: spirv::Word,
        m: spirv::Word,
        k: spirv::Word,
        memory_layout: spirv::Word,
        transpose: spirv::Word,
        matrix_stride: Option<spirv::Word>,
        cooperative_matrix_operands: Option<spirv::CooperativeMatrixOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorMatrixMulNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(input),
                dr::Operand::IdRef(input_interpretation),
                dr::Operand::IdRef(matrix),
                dr::Operand::IdRef(matrix_offset),
                dr::Operand::IdRef(matrix_interpretation),
                dr::Operand::IdRef(m),
                dr::Operand::IdRef(k),
                dr::Operand::IdRef(memory_layout),
                dr::Operand::IdRef(transpose),
            ],
        );
        if let Some(v) = matrix_stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = cooperative_matrix_operands {
            inst.operands
                .push(dr::Operand::CooperativeMatrixOperands(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeVectorMatrixMulNV instruction to the current block."]
    pub fn insert_cooperative_vector_matrix_mul_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        input: spirv::Word,
        input_interpretation: spirv::Word,
        matrix: spirv::Word,
        matrix_offset: spirv::Word,
        matrix_interpretation: spirv::Word,
        m: spirv::Word,
        k: spirv::Word,
        memory_layout: spirv::Word,
        transpose: spirv::Word,
        matrix_stride: Option<spirv::Word>,
        cooperative_matrix_operands: Option<spirv::CooperativeMatrixOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorMatrixMulNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(input),
                dr::Operand::IdRef(input_interpretation),
                dr::Operand::IdRef(matrix),
                dr::Operand::IdRef(matrix_offset),
                dr::Operand::IdRef(matrix_interpretation),
                dr::Operand::IdRef(m),
                dr::Operand::IdRef(k),
                dr::Operand::IdRef(memory_layout),
                dr::Operand::IdRef(transpose),
            ],
        );
        if let Some(v) = matrix_stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = cooperative_matrix_operands {
            inst.operands
                .push(dr::Operand::CooperativeMatrixOperands(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeVectorOuterProductAccumulateNV instruction to the current block."]
    pub fn cooperative_vector_outer_product_accumulate_nv(
        &mut self,
        pointer: spirv::Word,
        offset: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        memory_layout: spirv::Word,
        matrix_interpretation: spirv::Word,
        matrix_stride: Option<spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorOuterProductAccumulateNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(a),
                dr::Operand::IdRef(b),
                dr::Operand::IdRef(memory_layout),
                dr::Operand::IdRef(matrix_interpretation),
            ],
        );
        if let Some(v) = matrix_stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeVectorOuterProductAccumulateNV instruction to the current block."]
    pub fn insert_cooperative_vector_outer_product_accumulate_nv(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        offset: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        memory_layout: spirv::Word,
        matrix_interpretation: spirv::Word,
        matrix_stride: Option<spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorOuterProductAccumulateNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(a),
                dr::Operand::IdRef(b),
                dr::Operand::IdRef(memory_layout),
                dr::Operand::IdRef(matrix_interpretation),
            ],
        );
        if let Some(v) = matrix_stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeVectorReduceSumAccumulateNV instruction to the current block."]
    pub fn cooperative_vector_reduce_sum_accumulate_nv(
        &mut self,
        pointer: spirv::Word,
        offset: spirv::Word,
        v: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorReduceSumAccumulateNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(v),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeVectorReduceSumAccumulateNV instruction to the current block."]
    pub fn insert_cooperative_vector_reduce_sum_accumulate_nv(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        offset: spirv::Word,
        v: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorReduceSumAccumulateNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(v),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeVectorMatrixMulAddNV instruction to the current block."]
    pub fn cooperative_vector_matrix_mul_add_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        input: spirv::Word,
        input_interpretation: spirv::Word,
        matrix: spirv::Word,
        matrix_offset: spirv::Word,
        matrix_interpretation: spirv::Word,
        bias: spirv::Word,
        bias_offset: spirv::Word,
        bias_interpretation: spirv::Word,
        m: spirv::Word,
        k: spirv::Word,
        memory_layout: spirv::Word,
        transpose: spirv::Word,
        matrix_stride: Option<spirv::Word>,
        cooperative_matrix_operands: Option<spirv::CooperativeMatrixOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorMatrixMulAddNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(input),
                dr::Operand::IdRef(input_interpretation),
                dr::Operand::IdRef(matrix),
                dr::Operand::IdRef(matrix_offset),
                dr::Operand::IdRef(matrix_interpretation),
                dr::Operand::IdRef(bias),
                dr::Operand::IdRef(bias_offset),
                dr::Operand::IdRef(bias_interpretation),
                dr::Operand::IdRef(m),
                dr::Operand::IdRef(k),
                dr::Operand::IdRef(memory_layout),
                dr::Operand::IdRef(transpose),
            ],
        );
        if let Some(v) = matrix_stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = cooperative_matrix_operands {
            inst.operands
                .push(dr::Operand::CooperativeMatrixOperands(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeVectorMatrixMulAddNV instruction to the current block."]
    pub fn insert_cooperative_vector_matrix_mul_add_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        input: spirv::Word,
        input_interpretation: spirv::Word,
        matrix: spirv::Word,
        matrix_offset: spirv::Word,
        matrix_interpretation: spirv::Word,
        bias: spirv::Word,
        bias_offset: spirv::Word,
        bias_interpretation: spirv::Word,
        m: spirv::Word,
        k: spirv::Word,
        memory_layout: spirv::Word,
        transpose: spirv::Word,
        matrix_stride: Option<spirv::Word>,
        cooperative_matrix_operands: Option<spirv::CooperativeMatrixOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorMatrixMulAddNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(input),
                dr::Operand::IdRef(input_interpretation),
                dr::Operand::IdRef(matrix),
                dr::Operand::IdRef(matrix_offset),
                dr::Operand::IdRef(matrix_interpretation),
                dr::Operand::IdRef(bias),
                dr::Operand::IdRef(bias_offset),
                dr::Operand::IdRef(bias_interpretation),
                dr::Operand::IdRef(m),
                dr::Operand::IdRef(k),
                dr::Operand::IdRef(memory_layout),
                dr::Operand::IdRef(transpose),
            ],
        );
        if let Some(v) = matrix_stride {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(v) = cooperative_matrix_operands {
            inst.operands
                .push(dr::Operand::CooperativeMatrixOperands(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixConvertNV instruction to the current block."]
    pub fn cooperative_matrix_convert_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixConvertNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixConvertNV instruction to the current block."]
    pub fn insert_cooperative_matrix_convert_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixConvertNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSetMeshOutputsEXT instruction to the current block."]
    pub fn set_mesh_outputs_ext(
        &mut self,
        vertex_count: spirv::Word,
        primitive_count: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SetMeshOutputsEXT,
            None,
            None,
            vec![
                dr::Operand::IdRef(vertex_count),
                dr::Operand::IdRef(primitive_count),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSetMeshOutputsEXT instruction to the current block."]
    pub fn insert_set_mesh_outputs_ext(
        &mut self,
        insert_point: InsertPoint,
        vertex_count: spirv::Word,
        primitive_count: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SetMeshOutputsEXT,
            None,
            None,
            vec![
                dr::Operand::IdRef(vertex_count),
                dr::Operand::IdRef(primitive_count),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpGroupNonUniformPartitionNV instruction to the current block."]
    pub fn group_non_uniform_partition_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformPartitionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformPartitionNV instruction to the current block."]
    pub fn insert_group_non_uniform_partition_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformPartitionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpWritePackedPrimitiveIndices4x8NV instruction to the current block."]
    pub fn write_packed_primitive_indices4x8_nv(
        &mut self,
        index_offset: spirv::Word,
        packed_indices: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::WritePackedPrimitiveIndices4x8NV,
            None,
            None,
            vec![
                dr::Operand::IdRef(index_offset),
                dr::Operand::IdRef(packed_indices),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpWritePackedPrimitiveIndices4x8NV instruction to the current block."]
    pub fn insert_write_packed_primitive_indices4x8_nv(
        &mut self,
        insert_point: InsertPoint,
        index_offset: spirv::Word,
        packed_indices: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::WritePackedPrimitiveIndices4x8NV,
            None,
            None,
            vec![
                dr::Operand::IdRef(index_offset),
                dr::Operand::IdRef(packed_indices),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpFetchMicroTriangleVertexPositionNV instruction to the current block."]
    pub fn fetch_micro_triangle_vertex_position_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        accel: spirv::Word,
        instance_id: spirv::Word,
        geometry_index: spirv::Word,
        primitive_index: spirv::Word,
        barycentric: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FetchMicroTriangleVertexPositionNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(primitive_index),
                dr::Operand::IdRef(barycentric),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFetchMicroTriangleVertexPositionNV instruction to the current block."]
    pub fn insert_fetch_micro_triangle_vertex_position_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        accel: spirv::Word,
        instance_id: spirv::Word,
        geometry_index: spirv::Word,
        primitive_index: spirv::Word,
        barycentric: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FetchMicroTriangleVertexPositionNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(primitive_index),
                dr::Operand::IdRef(barycentric),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFetchMicroTriangleVertexBarycentricNV instruction to the current block."]
    pub fn fetch_micro_triangle_vertex_barycentric_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        accel: spirv::Word,
        instance_id: spirv::Word,
        geometry_index: spirv::Word,
        primitive_index: spirv::Word,
        barycentric: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FetchMicroTriangleVertexBarycentricNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(primitive_index),
                dr::Operand::IdRef(barycentric),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFetchMicroTriangleVertexBarycentricNV instruction to the current block."]
    pub fn insert_fetch_micro_triangle_vertex_barycentric_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        accel: spirv::Word,
        instance_id: spirv::Word,
        geometry_index: spirv::Word,
        primitive_index: spirv::Word,
        barycentric: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FetchMicroTriangleVertexBarycentricNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(instance_id),
                dr::Operand::IdRef(geometry_index),
                dr::Operand::IdRef(primitive_index),
                dr::Operand::IdRef(barycentric),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeVectorLoadNV instruction to the current block."]
    pub fn cooperative_vector_load_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        offset: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorLoadNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer), dr::Operand::IdRef(offset)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeVectorLoadNV instruction to the current block."]
    pub fn insert_cooperative_vector_load_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        offset: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorLoadNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer), dr::Operand::IdRef(offset)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeVectorStoreNV instruction to the current block."]
    pub fn cooperative_vector_store_nv(
        &mut self,
        pointer: spirv::Word,
        offset: spirv::Word,
        object: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorStoreNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(object),
            ],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeVectorStoreNV instruction to the current block."]
    pub fn insert_cooperative_vector_store_nv(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        offset: spirv::Word,
        object: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeVectorStoreNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(object),
            ],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpReportIntersectionKHR instruction to the current block."]
    pub fn report_intersection_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit: spirv::Word,
        hit_kind: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReportIntersectionKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit), dr::Operand::IdRef(hit_kind)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReportIntersectionKHR instruction to the current block."]
    pub fn insert_report_intersection_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit: spirv::Word,
        hit_kind: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReportIntersectionKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit), dr::Operand::IdRef(hit_kind)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIgnoreIntersectionNV instruction to the current block."]
    pub fn ignore_intersection_nv(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::IgnoreIntersectionNV, None, None, vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpIgnoreIntersectionNV instruction to the current block."]
    pub fn insert_ignore_intersection_nv(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::IgnoreIntersectionNV, None, None, vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTerminateRayNV instruction to the current block."]
    pub fn terminate_ray_nv(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::TerminateRayNV, None, None, vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTerminateRayNV instruction to the current block."]
    pub fn insert_terminate_ray_nv(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::TerminateRayNV, None, None, vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTraceNV instruction to the current block."]
    pub fn trace_nv(
        &mut self,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        payload_id: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TraceNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(sbt_offset),
                dr::Operand::IdRef(sbt_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_tmin),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_tmax),
                dr::Operand::IdRef(payload_id),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTraceNV instruction to the current block."]
    pub fn insert_trace_nv(
        &mut self,
        insert_point: InsertPoint,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        payload_id: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TraceNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(sbt_offset),
                dr::Operand::IdRef(sbt_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_tmin),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_tmax),
                dr::Operand::IdRef(payload_id),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTraceMotionNV instruction to the current block."]
    pub fn trace_motion_nv(
        &mut self,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        time: spirv::Word,
        payload_id: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TraceMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(sbt_offset),
                dr::Operand::IdRef(sbt_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_tmin),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_tmax),
                dr::Operand::IdRef(time),
                dr::Operand::IdRef(payload_id),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTraceMotionNV instruction to the current block."]
    pub fn insert_trace_motion_nv(
        &mut self,
        insert_point: InsertPoint,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        time: spirv::Word,
        payload_id: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TraceMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(sbt_offset),
                dr::Operand::IdRef(sbt_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_tmin),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_tmax),
                dr::Operand::IdRef(time),
                dr::Operand::IdRef(payload_id),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTraceRayMotionNV instruction to the current block."]
    pub fn trace_ray_motion_nv(
        &mut self,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        time: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TraceRayMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(sbt_offset),
                dr::Operand::IdRef(sbt_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_tmin),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_tmax),
                dr::Operand::IdRef(time),
                dr::Operand::IdRef(payload),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTraceRayMotionNV instruction to the current block."]
    pub fn insert_trace_ray_motion_nv(
        &mut self,
        insert_point: InsertPoint,
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        time: spirv::Word,
        payload: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TraceRayMotionNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(accel),
                dr::Operand::IdRef(ray_flags),
                dr::Operand::IdRef(cull_mask),
                dr::Operand::IdRef(sbt_offset),
                dr::Operand::IdRef(sbt_stride),
                dr::Operand::IdRef(miss_index),
                dr::Operand::IdRef(ray_origin),
                dr::Operand::IdRef(ray_tmin),
                dr::Operand::IdRef(ray_direction),
                dr::Operand::IdRef(ray_tmax),
                dr::Operand::IdRef(time),
                dr::Operand::IdRef(payload),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryGetIntersectionTriangleVertexPositionsKHR instruction to the current block."]
    pub fn ray_query_get_intersection_triangle_vertex_positions_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionTriangleVertexPositionsKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionTriangleVertexPositionsKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_triangle_vertex_positions_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionTriangleVertexPositionsKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpExecuteCallableNV instruction to the current block."]
    pub fn execute_callable_nv(
        &mut self,
        sbt_index: spirv::Word,
        callable_data_id: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ExecuteCallableNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(sbt_index),
                dr::Operand::IdRef(callable_data_id),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpExecuteCallableNV instruction to the current block."]
    pub fn insert_execute_callable_nv(
        &mut self,
        insert_point: InsertPoint,
        sbt_index: spirv::Word,
        callable_data_id: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ExecuteCallableNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(sbt_index),
                dr::Operand::IdRef(callable_data_id),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpRayQueryGetClusterIdNV instruction to the current block."]
    pub fn ray_query_get_cluster_id_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetClusterIdNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetClusterIdNV instruction to the current block."]
    pub fn insert_ray_query_get_cluster_id_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetClusterIdNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetClusterIdNV instruction to the current block."]
    pub fn hit_object_get_cluster_id_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetClusterIdNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetClusterIdNV instruction to the current block."]
    pub fn insert_hit_object_get_cluster_id_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetClusterIdNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLoadNV instruction to the current block."]
    pub fn cooperative_matrix_load_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        stride: spirv::Word,
        column_major: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLoadNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(stride),
                dr::Operand::IdRef(column_major),
            ],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLoadNV instruction to the current block."]
    pub fn insert_cooperative_matrix_load_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        stride: spirv::Word,
        column_major: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLoadNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(stride),
                dr::Operand::IdRef(column_major),
            ],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixStoreNV instruction to the current block."]
    pub fn cooperative_matrix_store_nv(
        &mut self,
        pointer: spirv::Word,
        object: spirv::Word,
        stride: spirv::Word,
        column_major: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixStoreNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(object),
                dr::Operand::IdRef(stride),
                dr::Operand::IdRef(column_major),
            ],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeMatrixStoreNV instruction to the current block."]
    pub fn insert_cooperative_matrix_store_nv(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        object: spirv::Word,
        stride: spirv::Word,
        column_major: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixStoreNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(object),
                dr::Operand::IdRef(stride),
                dr::Operand::IdRef(column_major),
            ],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeMatrixMulAddNV instruction to the current block."]
    pub fn cooperative_matrix_mul_add_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixMulAddNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(a),
                dr::Operand::IdRef(b),
                dr::Operand::IdRef(c),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixMulAddNV instruction to the current block."]
    pub fn insert_cooperative_matrix_mul_add_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixMulAddNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(a),
                dr::Operand::IdRef(b),
                dr::Operand::IdRef(c),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLengthNV instruction to the current block."]
    pub fn cooperative_matrix_length_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ty: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLengthNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ty)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLengthNV instruction to the current block."]
    pub fn insert_cooperative_matrix_length_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ty: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLengthNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ty)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBeginInvocationInterlockEXT instruction to the current block."]
    pub fn begin_invocation_interlock_ext(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst =
            dr::Instruction::new(spirv::Op::BeginInvocationInterlockEXT, None, None, vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpBeginInvocationInterlockEXT instruction to the current block."]
    pub fn insert_begin_invocation_interlock_ext(
        &mut self,
        insert_point: InsertPoint,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst =
            dr::Instruction::new(spirv::Op::BeginInvocationInterlockEXT, None, None, vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEndInvocationInterlockEXT instruction to the current block."]
    pub fn end_invocation_interlock_ext(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst =
            dr::Instruction::new(spirv::Op::EndInvocationInterlockEXT, None, None, vec![]);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpEndInvocationInterlockEXT instruction to the current block."]
    pub fn insert_end_invocation_interlock_ext(
        &mut self,
        insert_point: InsertPoint,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst =
            dr::Instruction::new(spirv::Op::EndInvocationInterlockEXT, None, None, vec![]);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeMatrixReduceNV instruction to the current block."]
    pub fn cooperative_matrix_reduce_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
        reduce: spirv::CooperativeMatrixReduce,
        combine_func: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixReduceNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(matrix),
                dr::Operand::CooperativeMatrixReduce(reduce),
                dr::Operand::IdRef(combine_func),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixReduceNV instruction to the current block."]
    pub fn insert_cooperative_matrix_reduce_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
        reduce: spirv::CooperativeMatrixReduce,
        combine_func: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixReduceNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(matrix),
                dr::Operand::CooperativeMatrixReduce(reduce),
                dr::Operand::IdRef(combine_func),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLoadTensorNV instruction to the current block."]
    pub fn cooperative_matrix_load_tensor_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        object: spirv::Word,
        tensor_layout: spirv::Word,
        memory_operand: spirv::MemoryAccess,
        tensor_addressing_operands: spirv::TensorAddressingOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLoadTensorNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(object),
                dr::Operand::IdRef(tensor_layout),
                dr::Operand::MemoryAccess(memory_operand),
                dr::Operand::TensorAddressingOperands(tensor_addressing_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixLoadTensorNV instruction to the current block."]
    pub fn insert_cooperative_matrix_load_tensor_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        object: spirv::Word,
        tensor_layout: spirv::Word,
        memory_operand: spirv::MemoryAccess,
        tensor_addressing_operands: spirv::TensorAddressingOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixLoadTensorNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(object),
                dr::Operand::IdRef(tensor_layout),
                dr::Operand::MemoryAccess(memory_operand),
                dr::Operand::TensorAddressingOperands(tensor_addressing_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixStoreTensorNV instruction to the current block."]
    pub fn cooperative_matrix_store_tensor_nv(
        &mut self,
        pointer: spirv::Word,
        object: spirv::Word,
        tensor_layout: spirv::Word,
        memory_operand: spirv::MemoryAccess,
        tensor_addressing_operands: spirv::TensorAddressingOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixStoreTensorNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(object),
                dr::Operand::IdRef(tensor_layout),
                dr::Operand::MemoryAccess(memory_operand),
                dr::Operand::TensorAddressingOperands(tensor_addressing_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCooperativeMatrixStoreTensorNV instruction to the current block."]
    pub fn insert_cooperative_matrix_store_tensor_nv(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        object: spirv::Word,
        tensor_layout: spirv::Word,
        memory_operand: spirv::MemoryAccess,
        tensor_addressing_operands: spirv::TensorAddressingOperands,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixStoreTensorNV,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdRef(object),
                dr::Operand::IdRef(tensor_layout),
                dr::Operand::MemoryAccess(memory_operand),
                dr::Operand::TensorAddressingOperands(tensor_addressing_operands),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpCreateTensorLayoutNV instruction to the current block."]
    pub fn create_tensor_layout_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreateTensorLayoutNV,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCreateTensorLayoutNV instruction to the current block."]
    pub fn insert_create_tensor_layout_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreateTensorLayoutNV,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSetDimensionNV instruction to the current block."]
    pub fn tensor_layout_set_dimension_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        dim: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSetDimensionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout)],
        );
        inst.operands
            .extend(dim.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSetDimensionNV instruction to the current block."]
    pub fn insert_tensor_layout_set_dimension_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        dim: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSetDimensionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout)],
        );
        inst.operands
            .extend(dim.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSetStrideNV instruction to the current block."]
    pub fn tensor_layout_set_stride_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        stride: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSetStrideNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout)],
        );
        inst.operands
            .extend(stride.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSetStrideNV instruction to the current block."]
    pub fn insert_tensor_layout_set_stride_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        stride: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSetStrideNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout)],
        );
        inst.operands
            .extend(stride.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSliceNV instruction to the current block."]
    pub fn tensor_layout_slice_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        operands: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSliceNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout)],
        );
        inst.operands
            .extend(operands.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSliceNV instruction to the current block."]
    pub fn insert_tensor_layout_slice_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        operands: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSliceNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout)],
        );
        inst.operands
            .extend(operands.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSetClampValueNV instruction to the current block."]
    pub fn tensor_layout_set_clamp_value_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSetClampValueNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSetClampValueNV instruction to the current block."]
    pub fn insert_tensor_layout_set_clamp_value_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSetClampValueNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCreateTensorViewNV instruction to the current block."]
    pub fn create_tensor_view_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreateTensorViewNV,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCreateTensorViewNV instruction to the current block."]
    pub fn insert_create_tensor_view_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreateTensorViewNV,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorViewSetDimensionNV instruction to the current block."]
    pub fn tensor_view_set_dimension_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_view: spirv::Word,
        dim: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorViewSetDimensionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_view)],
        );
        inst.operands
            .extend(dim.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorViewSetDimensionNV instruction to the current block."]
    pub fn insert_tensor_view_set_dimension_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_view: spirv::Word,
        dim: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorViewSetDimensionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_view)],
        );
        inst.operands
            .extend(dim.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorViewSetStrideNV instruction to the current block."]
    pub fn tensor_view_set_stride_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_view: spirv::Word,
        stride: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorViewSetStrideNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_view)],
        );
        inst.operands
            .extend(stride.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorViewSetStrideNV instruction to the current block."]
    pub fn insert_tensor_view_set_stride_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_view: spirv::Word,
        stride: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorViewSetStrideNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_view)],
        );
        inst.operands
            .extend(stride.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsHelperInvocationEXT instruction to the current block."]
    pub fn is_helper_invocation_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsHelperInvocationEXT,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIsHelperInvocationEXT instruction to the current block."]
    pub fn insert_is_helper_invocation_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsHelperInvocationEXT,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorViewSetClipNV instruction to the current block."]
    pub fn tensor_view_set_clip_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_view: spirv::Word,
        clip_row_offset: spirv::Word,
        clip_row_span: spirv::Word,
        clip_col_offset: spirv::Word,
        clip_col_span: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorViewSetClipNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(tensor_view),
                dr::Operand::IdRef(clip_row_offset),
                dr::Operand::IdRef(clip_row_span),
                dr::Operand::IdRef(clip_col_offset),
                dr::Operand::IdRef(clip_col_span),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorViewSetClipNV instruction to the current block."]
    pub fn insert_tensor_view_set_clip_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_view: spirv::Word,
        clip_row_offset: spirv::Word,
        clip_row_span: spirv::Word,
        clip_col_offset: spirv::Word,
        clip_col_span: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorViewSetClipNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(tensor_view),
                dr::Operand::IdRef(clip_row_offset),
                dr::Operand::IdRef(clip_row_span),
                dr::Operand::IdRef(clip_col_offset),
                dr::Operand::IdRef(clip_col_span),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSetBlockSizeNV instruction to the current block."]
    pub fn tensor_layout_set_block_size_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        block_size: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSetBlockSizeNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout)],
        );
        inst.operands
            .extend(block_size.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTensorLayoutSetBlockSizeNV instruction to the current block."]
    pub fn insert_tensor_layout_set_block_size_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        tensor_layout: spirv::Word,
        block_size: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TensorLayoutSetBlockSizeNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(tensor_layout)],
        );
        inst.operands
            .extend(block_size.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixTransposeNV instruction to the current block."]
    pub fn cooperative_matrix_transpose_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixTransposeNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCooperativeMatrixTransposeNV instruction to the current block."]
    pub fn insert_cooperative_matrix_transpose_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CooperativeMatrixTransposeNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToImageNV instruction to the current block."]
    pub fn convert_u_to_image_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToImageNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToImageNV instruction to the current block."]
    pub fn insert_convert_u_to_image_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToImageNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToSamplerNV instruction to the current block."]
    pub fn convert_u_to_sampler_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToSamplerNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToSamplerNV instruction to the current block."]
    pub fn insert_convert_u_to_sampler_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToSamplerNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertImageToUNV instruction to the current block."]
    pub fn convert_image_to_unv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertImageToUNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertImageToUNV instruction to the current block."]
    pub fn insert_convert_image_to_unv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertImageToUNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertSamplerToUNV instruction to the current block."]
    pub fn convert_sampler_to_unv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertSamplerToUNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertSamplerToUNV instruction to the current block."]
    pub fn insert_convert_sampler_to_unv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertSamplerToUNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToSampledImageNV instruction to the current block."]
    pub fn convert_u_to_sampled_image_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToSampledImageNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToSampledImageNV instruction to the current block."]
    pub fn insert_convert_u_to_sampled_image_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToSampledImageNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertSampledImageToUNV instruction to the current block."]
    pub fn convert_sampled_image_to_unv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertSampledImageToUNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertSampledImageToUNV instruction to the current block."]
    pub fn insert_convert_sampled_image_to_unv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertSampledImageToUNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRawAccessChainNV instruction to the current block."]
    pub fn raw_access_chain_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        byte_stride: spirv::Word,
        element_index: spirv::Word,
        byte_offset: spirv::Word,
        raw_access_chain_operands: Option<spirv::RawAccessChainOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RawAccessChainNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(byte_stride),
                dr::Operand::IdRef(element_index),
                dr::Operand::IdRef(byte_offset),
            ],
        );
        if let Some(v) = raw_access_chain_operands {
            inst.operands.push(dr::Operand::RawAccessChainOperands(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRawAccessChainNV instruction to the current block."]
    pub fn insert_raw_access_chain_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        byte_stride: spirv::Word,
        element_index: spirv::Word,
        byte_offset: spirv::Word,
        raw_access_chain_operands: Option<spirv::RawAccessChainOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RawAccessChainNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(base),
                dr::Operand::IdRef(byte_stride),
                dr::Operand::IdRef(element_index),
                dr::Operand::IdRef(byte_offset),
            ],
        );
        if let Some(v) = raw_access_chain_operands {
            inst.operands.push(dr::Operand::RawAccessChainOperands(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionSpherePositionNV instruction to the current block."]
    pub fn ray_query_get_intersection_sphere_position_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionSpherePositionNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionSpherePositionNV instruction to the current block."]
    pub fn insert_ray_query_get_intersection_sphere_position_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionSpherePositionNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionSphereRadiusNV instruction to the current block."]
    pub fn ray_query_get_intersection_sphere_radius_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionSphereRadiusNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionSphereRadiusNV instruction to the current block."]
    pub fn insert_ray_query_get_intersection_sphere_radius_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionSphereRadiusNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionLSSPositionsNV instruction to the current block."]
    pub fn ray_query_get_intersection_lss_positions_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionLSSPositionsNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionLSSPositionsNV instruction to the current block."]
    pub fn insert_ray_query_get_intersection_lss_positions_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionLSSPositionsNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionLSSRadiiNV instruction to the current block."]
    pub fn ray_query_get_intersection_lss_radii_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionLSSRadiiNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionLSSRadiiNV instruction to the current block."]
    pub fn insert_ray_query_get_intersection_lss_radii_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionLSSRadiiNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionLSSHitValueNV instruction to the current block."]
    pub fn ray_query_get_intersection_lss_hit_value_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionLSSHitValueNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionLSSHitValueNV instruction to the current block."]
    pub fn insert_ray_query_get_intersection_lss_hit_value_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionLSSHitValueNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetSpherePositionNV instruction to the current block."]
    pub fn hit_object_get_sphere_position_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetSpherePositionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetSpherePositionNV instruction to the current block."]
    pub fn insert_hit_object_get_sphere_position_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetSpherePositionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetSphereRadiusNV instruction to the current block."]
    pub fn hit_object_get_sphere_radius_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetSphereRadiusNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetSphereRadiusNV instruction to the current block."]
    pub fn insert_hit_object_get_sphere_radius_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetSphereRadiusNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetLSSPositionsNV instruction to the current block."]
    pub fn hit_object_get_lss_positions_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetLSSPositionsNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetLSSPositionsNV instruction to the current block."]
    pub fn insert_hit_object_get_lss_positions_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetLSSPositionsNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetLSSRadiiNV instruction to the current block."]
    pub fn hit_object_get_lss_radii_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetLSSRadiiNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectGetLSSRadiiNV instruction to the current block."]
    pub fn insert_hit_object_get_lss_radii_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectGetLSSRadiiNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsSphereHitNV instruction to the current block."]
    pub fn hit_object_is_sphere_hit_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsSphereHitNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsSphereHitNV instruction to the current block."]
    pub fn insert_hit_object_is_sphere_hit_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsSphereHitNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsLSSHitNV instruction to the current block."]
    pub fn hit_object_is_lss_hit_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsLSSHitNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpHitObjectIsLSSHitNV instruction to the current block."]
    pub fn insert_hit_object_is_lss_hit_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hit_object: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::HitObjectIsLSSHitNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(hit_object)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryIsSphereHitNV instruction to the current block."]
    pub fn ray_query_is_sphere_hit_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryIsSphereHitNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryIsSphereHitNV instruction to the current block."]
    pub fn insert_ray_query_is_sphere_hit_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryIsSphereHitNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryIsLSSHitNV instruction to the current block."]
    pub fn ray_query_is_lss_hit_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryIsLSSHitNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryIsLSSHitNV instruction to the current block."]
    pub fn insert_ray_query_is_lss_hit_nv(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryIsLSSHitNV,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleINTEL instruction to the current block."]
    pub fn subgroup_shuffle_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        invocation_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(data), dr::Operand::IdRef(invocation_id)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleINTEL instruction to the current block."]
    pub fn insert_subgroup_shuffle_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        invocation_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(data), dr::Operand::IdRef(invocation_id)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleDownINTEL instruction to the current block."]
    pub fn subgroup_shuffle_down_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        current: spirv::Word,
        next: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleDownINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(current),
                dr::Operand::IdRef(next),
                dr::Operand::IdRef(delta),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleDownINTEL instruction to the current block."]
    pub fn insert_subgroup_shuffle_down_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        current: spirv::Word,
        next: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleDownINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(current),
                dr::Operand::IdRef(next),
                dr::Operand::IdRef(delta),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleUpINTEL instruction to the current block."]
    pub fn subgroup_shuffle_up_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        previous: spirv::Word,
        current: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleUpINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(previous),
                dr::Operand::IdRef(current),
                dr::Operand::IdRef(delta),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleUpINTEL instruction to the current block."]
    pub fn insert_subgroup_shuffle_up_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        previous: spirv::Word,
        current: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleUpINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(previous),
                dr::Operand::IdRef(current),
                dr::Operand::IdRef(delta),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleXorINTEL instruction to the current block."]
    pub fn subgroup_shuffle_xor_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleXorINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(data), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleXorINTEL instruction to the current block."]
    pub fn insert_subgroup_shuffle_xor_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleXorINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(data), dr::Operand::IdRef(value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupBlockReadINTEL instruction to the current block."]
    pub fn subgroup_block_read_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ptr: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBlockReadINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ptr)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupBlockReadINTEL instruction to the current block."]
    pub fn insert_subgroup_block_read_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ptr: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBlockReadINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ptr)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupBlockWriteINTEL instruction to the current block."]
    pub fn subgroup_block_write_intel(
        &mut self,
        ptr: spirv::Word,
        data: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBlockWriteINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(ptr), dr::Operand::IdRef(data)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupBlockWriteINTEL instruction to the current block."]
    pub fn insert_subgroup_block_write_intel(
        &mut self,
        insert_point: InsertPoint,
        ptr: spirv::Word,
        data: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBlockWriteINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(ptr), dr::Operand::IdRef(data)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupImageBlockReadINTEL instruction to the current block."]
    pub fn subgroup_image_block_read_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageBlockReadINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupImageBlockReadINTEL instruction to the current block."]
    pub fn insert_subgroup_image_block_read_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageBlockReadINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupImageBlockWriteINTEL instruction to the current block."]
    pub fn subgroup_image_block_write_intel(
        &mut self,
        image: spirv::Word,
        coordinate: spirv::Word,
        data: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageBlockWriteINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(data),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupImageBlockWriteINTEL instruction to the current block."]
    pub fn insert_subgroup_image_block_write_intel(
        &mut self,
        insert_point: InsertPoint,
        image: spirv::Word,
        coordinate: spirv::Word,
        data: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageBlockWriteINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(data),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupImageMediaBlockReadINTEL instruction to the current block."]
    pub fn subgroup_image_media_block_read_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        width: spirv::Word,
        height: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageMediaBlockReadINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(width),
                dr::Operand::IdRef(height),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupImageMediaBlockReadINTEL instruction to the current block."]
    pub fn insert_subgroup_image_media_block_read_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        width: spirv::Word,
        height: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageMediaBlockReadINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(width),
                dr::Operand::IdRef(height),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupImageMediaBlockWriteINTEL instruction to the current block."]
    pub fn subgroup_image_media_block_write_intel(
        &mut self,
        image: spirv::Word,
        coordinate: spirv::Word,
        width: spirv::Word,
        height: spirv::Word,
        data: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageMediaBlockWriteINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(width),
                dr::Operand::IdRef(height),
                dr::Operand::IdRef(data),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupImageMediaBlockWriteINTEL instruction to the current block."]
    pub fn insert_subgroup_image_media_block_write_intel(
        &mut self,
        insert_point: InsertPoint,
        image: spirv::Word,
        coordinate: spirv::Word,
        width: spirv::Word,
        height: spirv::Word,
        data: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageMediaBlockWriteINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(image),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(width),
                dr::Operand::IdRef(height),
                dr::Operand::IdRef(data),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpUCountLeadingZerosINTEL instruction to the current block."]
    pub fn u_count_leading_zeros_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UCountLeadingZerosINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUCountLeadingZerosINTEL instruction to the current block."]
    pub fn insert_u_count_leading_zeros_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UCountLeadingZerosINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUCountTrailingZerosINTEL instruction to the current block."]
    pub fn u_count_trailing_zeros_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UCountTrailingZerosINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUCountTrailingZerosINTEL instruction to the current block."]
    pub fn insert_u_count_trailing_zeros_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UCountTrailingZerosINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAbsISubINTEL instruction to the current block."]
    pub fn abs_i_sub_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AbsISubINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAbsISubINTEL instruction to the current block."]
    pub fn insert_abs_i_sub_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AbsISubINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAbsUSubINTEL instruction to the current block."]
    pub fn abs_u_sub_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AbsUSubINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAbsUSubINTEL instruction to the current block."]
    pub fn insert_abs_u_sub_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AbsUSubINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAddSatINTEL instruction to the current block."]
    pub fn i_add_sat_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAddSatINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAddSatINTEL instruction to the current block."]
    pub fn insert_i_add_sat_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAddSatINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUAddSatINTEL instruction to the current block."]
    pub fn u_add_sat_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UAddSatINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUAddSatINTEL instruction to the current block."]
    pub fn insert_u_add_sat_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UAddSatINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAverageINTEL instruction to the current block."]
    pub fn i_average_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAverageINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAverageINTEL instruction to the current block."]
    pub fn insert_i_average_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAverageINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUAverageINTEL instruction to the current block."]
    pub fn u_average_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UAverageINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUAverageINTEL instruction to the current block."]
    pub fn insert_u_average_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UAverageINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAverageRoundedINTEL instruction to the current block."]
    pub fn i_average_rounded_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAverageRoundedINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIAverageRoundedINTEL instruction to the current block."]
    pub fn insert_i_average_rounded_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAverageRoundedINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUAverageRoundedINTEL instruction to the current block."]
    pub fn u_average_rounded_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UAverageRoundedINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUAverageRoundedINTEL instruction to the current block."]
    pub fn insert_u_average_rounded_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UAverageRoundedINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpISubSatINTEL instruction to the current block."]
    pub fn i_sub_sat_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ISubSatINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpISubSatINTEL instruction to the current block."]
    pub fn insert_i_sub_sat_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ISubSatINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUSubSatINTEL instruction to the current block."]
    pub fn u_sub_sat_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::USubSatINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUSubSatINTEL instruction to the current block."]
    pub fn insert_u_sub_sat_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::USubSatINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIMul32x16INTEL instruction to the current block."]
    pub fn i_mul32x16_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IMul32x16INTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpIMul32x16INTEL instruction to the current block."]
    pub fn insert_i_mul32x16_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IMul32x16INTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUMul32x16INTEL instruction to the current block."]
    pub fn u_mul32x16_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UMul32x16INTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpUMul32x16INTEL instruction to the current block."]
    pub fn insert_u_mul32x16_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UMul32x16INTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFMinEXT instruction to the current block."]
    pub fn atomic_f_min_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFMinEXT,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFMinEXT instruction to the current block."]
    pub fn insert_atomic_f_min_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFMinEXT,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFMaxEXT instruction to the current block."]
    pub fn atomic_f_max_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFMaxEXT,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFMaxEXT instruction to the current block."]
    pub fn insert_atomic_f_max_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFMaxEXT,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAssumeTrueKHR instruction to the current block."]
    pub fn assume_true_khr(&mut self, condition: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AssumeTrueKHR,
            None,
            None,
            vec![dr::Operand::IdRef(condition)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpAssumeTrueKHR instruction to the current block."]
    pub fn insert_assume_true_khr(
        &mut self,
        insert_point: InsertPoint,
        condition: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AssumeTrueKHR,
            None,
            None,
            vec![dr::Operand::IdRef(condition)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpExpectKHR instruction to the current block."]
    pub fn expect_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
        expected_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ExpectKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(expected_value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpExpectKHR instruction to the current block."]
    pub fn insert_expect_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
        expected_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ExpectKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(expected_value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpLoopControlINTEL instruction to the current block."]
    pub fn loop_control_intel(
        &mut self,
        loop_control_parameters: impl IntoIterator<Item = u32>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::LoopControlINTEL, None, None, vec![]);
        inst.operands.extend(
            loop_control_parameters
                .into_iter()
                .map(dr::Operand::LiteralBit32),
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpLoopControlINTEL instruction to the current block."]
    pub fn insert_loop_control_intel(
        &mut self,
        insert_point: InsertPoint,
        loop_control_parameters: impl IntoIterator<Item = u32>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::LoopControlINTEL, None, None, vec![]);
        inst.operands.extend(
            loop_control_parameters
                .into_iter()
                .map(dr::Operand::LiteralBit32),
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpReadPipeBlockingINTEL instruction to the current block."]
    pub fn read_pipe_blocking_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReadPipeBlockingINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpReadPipeBlockingINTEL instruction to the current block."]
    pub fn insert_read_pipe_blocking_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReadPipeBlockingINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpWritePipeBlockingINTEL instruction to the current block."]
    pub fn write_pipe_blocking_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::WritePipeBlockingINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpWritePipeBlockingINTEL instruction to the current block."]
    pub fn insert_write_pipe_blocking_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::WritePipeBlockingINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(packet_size),
                dr::Operand::IdRef(packet_alignment),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFPGARegINTEL instruction to the current block."]
    pub fn fpga_reg_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        input: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FPGARegINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(input)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpFPGARegINTEL instruction to the current block."]
    pub fn insert_fpga_reg_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        input: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FPGARegINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(input)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetRayTMinKHR instruction to the current block."]
    pub fn ray_query_get_ray_t_min_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetRayTMinKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetRayTMinKHR instruction to the current block."]
    pub fn insert_ray_query_get_ray_t_min_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetRayTMinKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetRayFlagsKHR instruction to the current block."]
    pub fn ray_query_get_ray_flags_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetRayFlagsKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetRayFlagsKHR instruction to the current block."]
    pub fn insert_ray_query_get_ray_flags_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetRayFlagsKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionTKHR instruction to the current block."]
    pub fn ray_query_get_intersection_tkhr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionTKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionTKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_tkhr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionTKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionInstanceCustomIndexKHR instruction to the current block."]
    pub fn ray_query_get_intersection_instance_custom_index_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionInstanceCustomIndexKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionInstanceCustomIndexKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_instance_custom_index_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionInstanceCustomIndexKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionInstanceIdKHR instruction to the current block."]
    pub fn ray_query_get_intersection_instance_id_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionInstanceIdKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionInstanceIdKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_instance_id_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionInstanceIdKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR instruction to the current block."]
    pub fn ray_query_get_intersection_instance_shader_binding_table_record_offset_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_instance_shader_binding_table_record_offset_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionGeometryIndexKHR instruction to the current block."]
    pub fn ray_query_get_intersection_geometry_index_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionGeometryIndexKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionGeometryIndexKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_geometry_index_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionGeometryIndexKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionPrimitiveIndexKHR instruction to the current block."]
    pub fn ray_query_get_intersection_primitive_index_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionPrimitiveIndexKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionPrimitiveIndexKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_primitive_index_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionPrimitiveIndexKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionBarycentricsKHR instruction to the current block."]
    pub fn ray_query_get_intersection_barycentrics_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionBarycentricsKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionBarycentricsKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_barycentrics_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionBarycentricsKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionFrontFaceKHR instruction to the current block."]
    pub fn ray_query_get_intersection_front_face_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionFrontFaceKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionFrontFaceKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_front_face_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionFrontFaceKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionCandidateAABBOpaqueKHR instruction to the current block."]
    pub fn ray_query_get_intersection_candidate_aabb_opaque_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionCandidateAABBOpaqueKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionCandidateAABBOpaqueKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_candidate_aabb_opaque_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionCandidateAABBOpaqueKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionObjectRayDirectionKHR instruction to the current block."]
    pub fn ray_query_get_intersection_object_ray_direction_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionObjectRayDirectionKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionObjectRayDirectionKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_object_ray_direction_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionObjectRayDirectionKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionObjectRayOriginKHR instruction to the current block."]
    pub fn ray_query_get_intersection_object_ray_origin_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionObjectRayOriginKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionObjectRayOriginKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_object_ray_origin_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionObjectRayOriginKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetWorldRayDirectionKHR instruction to the current block."]
    pub fn ray_query_get_world_ray_direction_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetWorldRayDirectionKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetWorldRayDirectionKHR instruction to the current block."]
    pub fn insert_ray_query_get_world_ray_direction_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetWorldRayDirectionKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetWorldRayOriginKHR instruction to the current block."]
    pub fn ray_query_get_world_ray_origin_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetWorldRayOriginKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetWorldRayOriginKHR instruction to the current block."]
    pub fn insert_ray_query_get_world_ray_origin_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetWorldRayOriginKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ray_query)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionObjectToWorldKHR instruction to the current block."]
    pub fn ray_query_get_intersection_object_to_world_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionObjectToWorldKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionObjectToWorldKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_object_to_world_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionObjectToWorldKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionWorldToObjectKHR instruction to the current block."]
    pub fn ray_query_get_intersection_world_to_object_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionWorldToObjectKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRayQueryGetIntersectionWorldToObjectKHR instruction to the current block."]
    pub fn insert_ray_query_get_intersection_world_to_object_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ray_query: spirv::Word,
        intersection: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RayQueryGetIntersectionWorldToObjectKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ray_query),
                dr::Operand::IdRef(intersection),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFAddEXT instruction to the current block."]
    pub fn atomic_f_add_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFAddEXT,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFAddEXT instruction to the current block."]
    pub fn insert_atomic_f_add_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFAddEXT,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeConstructContinuedINTEL instruction to the current block."]
    pub fn composite_construct_continued_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        constituents: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeConstructContinuedINTEL,
            Some(result_type),
            Some(_id),
            vec![],
        );
        inst.operands
            .extend(constituents.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeConstructContinuedINTEL instruction to the current block."]
    pub fn insert_composite_construct_continued_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        constituents: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeConstructContinuedINTEL,
            Some(result_type),
            Some(_id),
            vec![],
        );
        inst.operands
            .extend(constituents.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertFToBF16INTEL instruction to the current block."]
    pub fn convert_f_to_bf16intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertFToBF16INTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertFToBF16INTEL instruction to the current block."]
    pub fn insert_convert_f_to_bf16intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertFToBF16INTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertBF16ToFINTEL instruction to the current block."]
    pub fn convert_bf16_to_fintel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        b_float16_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertBF16ToFINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(b_float16_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertBF16ToFINTEL instruction to the current block."]
    pub fn insert_convert_bf16_to_fintel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        b_float16_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertBF16ToFINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(b_float16_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpControlBarrierArriveINTEL instruction to the current block."]
    pub fn control_barrier_arrive_intel(
        &mut self,
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ControlBarrierArriveINTEL,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpControlBarrierArriveINTEL instruction to the current block."]
    pub fn insert_control_barrier_arrive_intel(
        &mut self,
        insert_point: InsertPoint,
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ControlBarrierArriveINTEL,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpControlBarrierWaitINTEL instruction to the current block."]
    pub fn control_barrier_wait_intel(
        &mut self,
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ControlBarrierWaitINTEL,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpControlBarrierWaitINTEL instruction to the current block."]
    pub fn insert_control_barrier_wait_intel(
        &mut self,
        insert_point: InsertPoint,
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ControlBarrierWaitINTEL,
            None,
            None,
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::IdScope(memory),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpArithmeticFenceEXT instruction to the current block."]
    pub fn arithmetic_fence_ext(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ArithmeticFenceEXT,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(target)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpArithmeticFenceEXT instruction to the current block."]
    pub fn insert_arithmetic_fence_ext(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        target: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ArithmeticFenceEXT,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(target)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTaskSequenceCreateINTEL instruction to the current block."]
    pub fn task_sequence_create_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        function: spirv::Word,
        pipelined: u32,
        use_stall_enable_clusters: u32,
        get_capacity: u32,
        async_capacity: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TaskSequenceCreateINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(function),
                dr::Operand::LiteralBit32(pipelined),
                dr::Operand::LiteralBit32(use_stall_enable_clusters),
                dr::Operand::LiteralBit32(get_capacity),
                dr::Operand::LiteralBit32(async_capacity),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTaskSequenceCreateINTEL instruction to the current block."]
    pub fn insert_task_sequence_create_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        function: spirv::Word,
        pipelined: u32,
        use_stall_enable_clusters: u32,
        get_capacity: u32,
        async_capacity: u32,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TaskSequenceCreateINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(function),
                dr::Operand::LiteralBit32(pipelined),
                dr::Operand::LiteralBit32(use_stall_enable_clusters),
                dr::Operand::LiteralBit32(get_capacity),
                dr::Operand::LiteralBit32(async_capacity),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTaskSequenceAsyncINTEL instruction to the current block."]
    pub fn task_sequence_async_intel(
        &mut self,
        sequence: spirv::Word,
        arguments: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TaskSequenceAsyncINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(sequence)],
        );
        inst.operands
            .extend(arguments.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTaskSequenceAsyncINTEL instruction to the current block."]
    pub fn insert_task_sequence_async_intel(
        &mut self,
        insert_point: InsertPoint,
        sequence: spirv::Word,
        arguments: impl IntoIterator<Item = spirv::Word>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TaskSequenceAsyncINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(sequence)],
        );
        inst.operands
            .extend(arguments.into_iter().map(dr::Operand::IdRef));
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTaskSequenceGetINTEL instruction to the current block."]
    pub fn task_sequence_get_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sequence: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TaskSequenceGetINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(sequence)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTaskSequenceGetINTEL instruction to the current block."]
    pub fn insert_task_sequence_get_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sequence: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TaskSequenceGetINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(sequence)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpTaskSequenceReleaseINTEL instruction to the current block."]
    pub fn task_sequence_release_intel(&mut self, sequence: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TaskSequenceReleaseINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(sequence)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpTaskSequenceReleaseINTEL instruction to the current block."]
    pub fn insert_task_sequence_release_intel(
        &mut self,
        insert_point: InsertPoint,
        sequence: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::TaskSequenceReleaseINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(sequence)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupBlockPrefetchINTEL instruction to the current block."]
    pub fn subgroup_block_prefetch_intel(
        &mut self,
        ptr: spirv::Word,
        num_bytes: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBlockPrefetchINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(ptr), dr::Operand::IdRef(num_bytes)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupBlockPrefetchINTEL instruction to the current block."]
    pub fn insert_subgroup_block_prefetch_intel(
        &mut self,
        insert_point: InsertPoint,
        ptr: spirv::Word,
        num_bytes: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBlockPrefetchINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(ptr), dr::Operand::IdRef(num_bytes)],
        );
        if let Some(v) = memory_access {
            inst.operands.push(dr::Operand::MemoryAccess(v));
        }
        inst.operands.extend(additional_params);
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockLoadINTEL instruction to the current block."]
    pub fn subgroup2_d_block_load_intel(
        &mut self,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
        dst_pointer: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockLoadINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(dst_pointer),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockLoadINTEL instruction to the current block."]
    pub fn insert_subgroup2_d_block_load_intel(
        &mut self,
        insert_point: InsertPoint,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
        dst_pointer: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockLoadINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(dst_pointer),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockLoadTransformINTEL instruction to the current block."]
    pub fn subgroup2_d_block_load_transform_intel(
        &mut self,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
        dst_pointer: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockLoadTransformINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(dst_pointer),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockLoadTransformINTEL instruction to the current block."]
    pub fn insert_subgroup2_d_block_load_transform_intel(
        &mut self,
        insert_point: InsertPoint,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
        dst_pointer: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockLoadTransformINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(dst_pointer),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockLoadTransposeINTEL instruction to the current block."]
    pub fn subgroup2_d_block_load_transpose_intel(
        &mut self,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
        dst_pointer: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockLoadTransposeINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(dst_pointer),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockLoadTransposeINTEL instruction to the current block."]
    pub fn insert_subgroup2_d_block_load_transpose_intel(
        &mut self,
        insert_point: InsertPoint,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
        dst_pointer: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockLoadTransposeINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
                dr::Operand::IdRef(dst_pointer),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockPrefetchINTEL instruction to the current block."]
    pub fn subgroup2_d_block_prefetch_intel(
        &mut self,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockPrefetchINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockPrefetchINTEL instruction to the current block."]
    pub fn insert_subgroup2_d_block_prefetch_intel(
        &mut self,
        insert_point: InsertPoint,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockPrefetchINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockStoreINTEL instruction to the current block."]
    pub fn subgroup2_d_block_store_intel(
        &mut self,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_pointer: spirv::Word,
        dst_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockStoreINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_pointer),
                dr::Operand::IdRef(dst_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroup2DBlockStoreINTEL instruction to the current block."]
    pub fn insert_subgroup2_d_block_store_intel(
        &mut self,
        insert_point: InsertPoint,
        element_size: spirv::Word,
        block_width: spirv::Word,
        block_height: spirv::Word,
        block_count: spirv::Word,
        src_pointer: spirv::Word,
        dst_base_pointer: spirv::Word,
        memory_width: spirv::Word,
        memory_height: spirv::Word,
        memory_pitch: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Subgroup2DBlockStoreINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(element_size),
                dr::Operand::IdRef(block_width),
                dr::Operand::IdRef(block_height),
                dr::Operand::IdRef(block_count),
                dr::Operand::IdRef(src_pointer),
                dr::Operand::IdRef(dst_base_pointer),
                dr::Operand::IdRef(memory_width),
                dr::Operand::IdRef(memory_height),
                dr::Operand::IdRef(memory_pitch),
                dr::Operand::IdRef(coordinate),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpSubgroupMatrixMultiplyAccumulateINTEL instruction to the current block."]
    pub fn subgroup_matrix_multiply_accumulate_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        k_dim: spirv::Word,
        matrix_a: spirv::Word,
        matrix_b: spirv::Word,
        matrix_c: spirv::Word,
        matrix_multiply_accumulate_operands: Option<spirv::MatrixMultiplyAccumulateOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupMatrixMultiplyAccumulateINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(k_dim),
                dr::Operand::IdRef(matrix_a),
                dr::Operand::IdRef(matrix_b),
                dr::Operand::IdRef(matrix_c),
            ],
        );
        if let Some(v) = matrix_multiply_accumulate_operands {
            inst.operands
                .push(dr::Operand::MatrixMultiplyAccumulateOperands(v));
        }
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupMatrixMultiplyAccumulateINTEL instruction to the current block."]
    pub fn insert_subgroup_matrix_multiply_accumulate_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        k_dim: spirv::Word,
        matrix_a: spirv::Word,
        matrix_b: spirv::Word,
        matrix_c: spirv::Word,
        matrix_multiply_accumulate_operands: Option<spirv::MatrixMultiplyAccumulateOperands>,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupMatrixMultiplyAccumulateINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(k_dim),
                dr::Operand::IdRef(matrix_a),
                dr::Operand::IdRef(matrix_b),
                dr::Operand::IdRef(matrix_c),
            ],
        );
        if let Some(v) = matrix_multiply_accumulate_operands {
            inst.operands
                .push(dr::Operand::MatrixMultiplyAccumulateOperands(v));
        }
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseFunctionINTEL instruction to the current block."]
    pub fn bitwise_function_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
        lut_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseFunctionINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(a),
                dr::Operand::IdRef(b),
                dr::Operand::IdRef(c),
                dr::Operand::IdRef(lut_index),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseFunctionINTEL instruction to the current block."]
    pub fn insert_bitwise_function_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
        lut_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseFunctionINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(a),
                dr::Operand::IdRef(b),
                dr::Operand::IdRef(c),
                dr::Operand::IdRef(lut_index),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupIMulKHR instruction to the current block."]
    pub fn group_i_mul_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupIMulKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupIMulKHR instruction to the current block."]
    pub fn insert_group_i_mul_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupIMulKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMulKHR instruction to the current block."]
    pub fn group_f_mul_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMulKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMulKHR instruction to the current block."]
    pub fn insert_group_f_mul_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupFMulKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBitwiseAndKHR instruction to the current block."]
    pub fn group_bitwise_and_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupBitwiseAndKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBitwiseAndKHR instruction to the current block."]
    pub fn insert_group_bitwise_and_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupBitwiseAndKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBitwiseOrKHR instruction to the current block."]
    pub fn group_bitwise_or_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupBitwiseOrKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBitwiseOrKHR instruction to the current block."]
    pub fn insert_group_bitwise_or_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupBitwiseOrKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBitwiseXorKHR instruction to the current block."]
    pub fn group_bitwise_xor_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupBitwiseXorKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBitwiseXorKHR instruction to the current block."]
    pub fn insert_group_bitwise_xor_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupBitwiseXorKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupLogicalAndKHR instruction to the current block."]
    pub fn group_logical_and_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupLogicalAndKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupLogicalAndKHR instruction to the current block."]
    pub fn insert_group_logical_and_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupLogicalAndKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupLogicalOrKHR instruction to the current block."]
    pub fn group_logical_or_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupLogicalOrKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupLogicalOrKHR instruction to the current block."]
    pub fn insert_group_logical_or_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupLogicalOrKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupLogicalXorKHR instruction to the current block."]
    pub fn group_logical_xor_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupLogicalXorKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpGroupLogicalXorKHR instruction to the current block."]
    pub fn insert_group_logical_xor_khr(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupLogicalXorKHR,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdScope(execution),
                dr::Operand::GroupOperation(operation),
                dr::Operand::IdRef(x),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRoundFToTF32INTEL instruction to the current block."]
    pub fn round_f_to_tf32intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RoundFToTF32INTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpRoundFToTF32INTEL instruction to the current block."]
    pub fn insert_round_f_to_tf32intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RoundFToTF32INTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMaskedGatherINTEL instruction to the current block."]
    pub fn masked_gather_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ptr_vector: spirv::Word,
        alignment: u32,
        mask: spirv::Word,
        fill_empty: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MaskedGatherINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ptr_vector),
                dr::Operand::LiteralBit32(alignment),
                dr::Operand::IdRef(mask),
                dr::Operand::IdRef(fill_empty),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMaskedGatherINTEL instruction to the current block."]
    pub fn insert_masked_gather_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ptr_vector: spirv::Word,
        alignment: u32,
        mask: spirv::Word,
        fill_empty: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MaskedGatherINTEL,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(ptr_vector),
                dr::Operand::LiteralBit32(alignment),
                dr::Operand::IdRef(mask),
                dr::Operand::IdRef(fill_empty),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpMaskedScatterINTEL instruction to the current block."]
    pub fn masked_scatter_intel(
        &mut self,
        input_vector: spirv::Word,
        ptr_vector: spirv::Word,
        alignment: u32,
        mask: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MaskedScatterINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(input_vector),
                dr::Operand::IdRef(ptr_vector),
                dr::Operand::LiteralBit32(alignment),
                dr::Operand::IdRef(mask),
            ],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpMaskedScatterINTEL instruction to the current block."]
    pub fn insert_masked_scatter_intel(
        &mut self,
        insert_point: InsertPoint,
        input_vector: spirv::Word,
        ptr_vector: spirv::Word,
        alignment: u32,
        mask: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MaskedScatterINTEL,
            None,
            None,
            vec![
                dr::Operand::IdRef(input_vector),
                dr::Operand::IdRef(ptr_vector),
                dr::Operand::LiteralBit32(alignment),
                dr::Operand::IdRef(mask),
            ],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(())
    }
    #[doc = "Appends an OpConvertHandleToImageINTEL instruction to the current block."]
    pub fn convert_handle_to_image_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertHandleToImageINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertHandleToImageINTEL instruction to the current block."]
    pub fn insert_convert_handle_to_image_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertHandleToImageINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertHandleToSamplerINTEL instruction to the current block."]
    pub fn convert_handle_to_sampler_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertHandleToSamplerINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertHandleToSamplerINTEL instruction to the current block."]
    pub fn insert_convert_handle_to_sampler_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertHandleToSamplerINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertHandleToSampledImageINTEL instruction to the current block."]
    pub fn convert_handle_to_sampled_image_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertHandleToSampledImageINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }
    #[doc = "Appends an OpConvertHandleToSampledImageINTEL instruction to the current block."]
    pub fn insert_convert_handle_to_sampled_image_intel(
        &mut self,
        insert_point: InsertPoint,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        let _id = result_id.unwrap_or_else(|| self.id());
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertHandleToSampledImageINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.insert_into_block(insert_point, inst)?;
        Ok(_id)
    }
}
