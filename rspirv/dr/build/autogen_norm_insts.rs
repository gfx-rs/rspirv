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

#[allow(clippy::identity_conversion, clippy::too_many_arguments)]
impl Builder {
    #[doc = "Appends an OpNop instruction to the current basic block."]
    pub fn nop(&mut self) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Nop, None, None, vec![]);
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpExtInst instruction to the current basic block."]
    pub fn ext_inst<T: AsRef<[spirv::Word]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        set: spirv::Word,
        instruction: u32,
        operands: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(set),
                dr::Operand::LiteralExtInstInteger(instruction),
            ],
        );
        for v in operands.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFunctionCall instruction to the current basic block."]
    pub fn function_call<T: AsRef<[spirv::Word]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        function: spirv::Word,
        arguments: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FunctionCall,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(function)],
        );
        for v in arguments.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageTexelPointer instruction to the current basic block."]
    pub fn image_texel_pointer(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        sample: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpLoad instruction to the current basic block."]
    pub fn load<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Load,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        if let Some(v) = memory_access {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::MemoryAccess(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpStore instruction to the current basic block."]
    pub fn store<T: AsRef<[dr::Operand]>>(
        &mut self,
        pointer: spirv::Word,
        object: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: T,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Store,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::IdRef(object)],
        );
        if let Some(v) = memory_access {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::MemoryAccess(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpCopyMemory instruction to the current basic block."]
    pub fn copy_memory<T: AsRef<[dr::Operand]>>(
        &mut self,
        target: spirv::Word,
        source: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: T,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyMemory,
            None,
            None,
            vec![dr::Operand::IdRef(target), dr::Operand::IdRef(source)],
        );
        if let Some(v) = memory_access {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::MemoryAccess(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpCopyMemorySized instruction to the current basic block."]
    pub fn copy_memory_sized<T: AsRef<[dr::Operand]>>(
        &mut self,
        target: spirv::Word,
        source: spirv::Word,
        size: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
        additional_params: T,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::MemoryAccess(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpAccessChain instruction to the current basic block."]
    pub fn access_chain<T: AsRef<[spirv::Word]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        indexes: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        for v in indexes.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpInBoundsAccessChain instruction to the current basic block."]
    pub fn in_bounds_access_chain<T: AsRef<[spirv::Word]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        indexes: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::InBoundsAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        for v in indexes.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpPtrAccessChain instruction to the current basic block."]
    pub fn ptr_access_chain<T: AsRef<[spirv::Word]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        element: spirv::Word,
        indexes: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(element)],
        );
        for v in indexes.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpArrayLength instruction to the current basic block."]
    pub fn array_length(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        structure: spirv::Word,
        array_member: u32,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ArrayLength,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(structure),
                dr::Operand::LiteralInt32(array_member),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGenericPtrMemSemantics instruction to the current basic block."]
    pub fn generic_ptr_mem_semantics(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GenericPtrMemSemantics,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpInBoundsPtrAccessChain instruction to the current basic block."]
    pub fn in_bounds_ptr_access_chain<T: AsRef<[spirv::Word]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        element: spirv::Word,
        indexes: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::InBoundsPtrAccessChain,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(element)],
        );
        for v in indexes.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpVectorExtractDynamic instruction to the current basic block."]
    pub fn vector_extract_dynamic(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorExtractDynamic,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(index)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpVectorInsertDynamic instruction to the current basic block."]
    pub fn vector_insert_dynamic(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        component: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpVectorShuffle instruction to the current basic block."]
    pub fn vector_shuffle<T: AsRef<[u32]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        components: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorShuffle,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        for v in components.as_ref() {
            inst.operands.push(dr::Operand::LiteralInt32(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeConstruct instruction to the current basic block."]
    pub fn composite_construct<T: AsRef<[spirv::Word]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        constituents: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeConstruct,
            Some(result_type),
            Some(_id),
            vec![],
        );
        for v in constituents.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeExtract instruction to the current basic block."]
    pub fn composite_extract<T: AsRef<[u32]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        composite: spirv::Word,
        indexes: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeExtract,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(composite)],
        );
        for v in indexes.as_ref() {
            inst.operands.push(dr::Operand::LiteralInt32(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpCompositeInsert instruction to the current basic block."]
    pub fn composite_insert<T: AsRef<[u32]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        object: spirv::Word,
        composite: spirv::Word,
        indexes: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CompositeInsert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(object), dr::Operand::IdRef(composite)],
        );
        for v in indexes.as_ref() {
            inst.operands.push(dr::Operand::LiteralInt32(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpCopyObject instruction to the current basic block."]
    pub fn copy_object(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CopyObject,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpTranspose instruction to the current basic block."]
    pub fn transpose(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Transpose,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSampledImage instruction to the current basic block."]
    pub fn sampled_image(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        sampler: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SampledImage,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(sampler)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleImplicitLod instruction to the current basic block."]
    pub fn image_sample_implicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleExplicitLod instruction to the current basic block."]
    pub fn image_sample_explicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleDrefImplicitLod instruction to the current basic block."]
    pub fn image_sample_dref_implicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleDrefExplicitLod instruction to the current basic block."]
    pub fn image_sample_dref_explicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjImplicitLod instruction to the current basic block."]
    pub fn image_sample_proj_implicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjExplicitLod instruction to the current basic block."]
    pub fn image_sample_proj_explicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjDrefImplicitLod instruction to the current basic block."]
    pub fn image_sample_proj_dref_implicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSampleProjDrefExplicitLod instruction to the current basic block."]
    pub fn image_sample_proj_dref_explicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageFetch instruction to the current basic block."]
    pub fn image_fetch<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageFetch,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageGather instruction to the current basic block."]
    pub fn image_gather<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageDrefGather instruction to the current basic block."]
    pub fn image_dref_gather<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageRead instruction to the current basic block."]
    pub fn image_read<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageRead,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageWrite instruction to the current basic block."]
    pub fn image_write<T: AsRef<[dr::Operand]>>(
        &mut self,
        image: spirv::Word,
        coordinate: spirv::Word,
        texel: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpImage instruction to the current basic block."]
    pub fn image(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Image,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(sampled_image)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryFormat instruction to the current basic block."]
    pub fn image_query_format(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryFormat,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryOrder instruction to the current basic block."]
    pub fn image_query_order(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryOrder,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySizeLod instruction to the current basic block."]
    pub fn image_query_size_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        level_of_detail: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySize instruction to the current basic block."]
    pub fn image_query_size(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQuerySize,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryLod instruction to the current basic block."]
    pub fn image_query_lod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageQueryLevels instruction to the current basic block."]
    pub fn image_query_levels(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQueryLevels,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageQuerySamples instruction to the current basic block."]
    pub fn image_query_samples(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageQuerySamples,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpConvertFToU instruction to the current basic block."]
    pub fn convert_f_to_u(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertFToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpConvertFToS instruction to the current basic block."]
    pub fn convert_f_to_s(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertFToS,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpConvertSToF instruction to the current basic block."]
    pub fn convert_s_to_f(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertSToF,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToF instruction to the current basic block."]
    pub fn convert_u_to_f(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToF,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpUConvert instruction to the current basic block."]
    pub fn u_convert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSConvert instruction to the current basic block."]
    pub fn s_convert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFConvert instruction to the current basic block."]
    pub fn f_convert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        float_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FConvert,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(float_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpQuantizeToF16 instruction to the current basic block."]
    pub fn quantize_to_f16(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::QuantizeToF16,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpConvertPtrToU instruction to the current basic block."]
    pub fn convert_ptr_to_u(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertPtrToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSatConvertSToU instruction to the current basic block."]
    pub fn sat_convert_s_to_u(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        signed_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SatConvertSToU,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(signed_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSatConvertUToS instruction to the current basic block."]
    pub fn sat_convert_u_to_s(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        unsigned_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SatConvertUToS,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(unsigned_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpConvertUToPtr instruction to the current basic block."]
    pub fn convert_u_to_ptr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        integer_value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ConvertUToPtr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(integer_value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpPtrCastToGeneric instruction to the current basic block."]
    pub fn ptr_cast_to_generic(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::PtrCastToGeneric,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGenericCastToPtr instruction to the current basic block."]
    pub fn generic_cast_to_ptr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GenericCastToPtr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGenericCastToPtrExplicit instruction to the current basic block."]
    pub fn generic_cast_to_ptr_explicit(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        storage: spirv::StorageClass,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitcast instruction to the current basic block."]
    pub fn bitcast(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Bitcast,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSNegate instruction to the current basic block."]
    pub fn s_negate(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SNegate,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFNegate instruction to the current basic block."]
    pub fn f_negate(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FNegate,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIAdd instruction to the current basic block."]
    pub fn i_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAdd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFAdd instruction to the current basic block."]
    pub fn f_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FAdd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpISub instruction to the current basic block."]
    pub fn i_sub(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ISub,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFSub instruction to the current basic block."]
    pub fn f_sub(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FSub,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIMul instruction to the current basic block."]
    pub fn i_mul(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IMul,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFMul instruction to the current basic block."]
    pub fn f_mul(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FMul,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpUDiv instruction to the current basic block."]
    pub fn u_div(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSDiv instruction to the current basic block."]
    pub fn s_div(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFDiv instruction to the current basic block."]
    pub fn f_div(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FDiv,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpUMod instruction to the current basic block."]
    pub fn u_mod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSRem instruction to the current basic block."]
    pub fn s_rem(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SRem,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSMod instruction to the current basic block."]
    pub fn s_mod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFRem instruction to the current basic block."]
    pub fn f_rem(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FRem,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFMod instruction to the current basic block."]
    pub fn f_mod(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FMod,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpVectorTimesScalar instruction to the current basic block."]
    pub fn vector_times_scalar(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        scalar: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorTimesScalar,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(scalar)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesScalar instruction to the current basic block."]
    pub fn matrix_times_scalar(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
        scalar: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MatrixTimesScalar,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix), dr::Operand::IdRef(scalar)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpVectorTimesMatrix instruction to the current basic block."]
    pub fn vector_times_matrix(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
        matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::VectorTimesMatrix,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector), dr::Operand::IdRef(matrix)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesVector instruction to the current basic block."]
    pub fn matrix_times_vector(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        matrix: spirv::Word,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::MatrixTimesVector,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(matrix), dr::Operand::IdRef(vector)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpMatrixTimesMatrix instruction to the current basic block."]
    pub fn matrix_times_matrix(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        left_matrix: spirv::Word,
        right_matrix: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpOuterProduct instruction to the current basic block."]
    pub fn outer_product(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::OuterProduct,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpDot instruction to the current basic block."]
    pub fn dot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Dot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector_1), dr::Operand::IdRef(vector_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIAddCarry instruction to the current basic block."]
    pub fn i_add_carry(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IAddCarry,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpISubBorrow instruction to the current basic block."]
    pub fn i_sub_borrow(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ISubBorrow,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpUMulExtended instruction to the current basic block."]
    pub fn u_mul_extended(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UMulExtended,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSMulExtended instruction to the current basic block."]
    pub fn s_mul_extended(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SMulExtended,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAny instruction to the current basic block."]
    pub fn any(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Any,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAll instruction to the current basic block."]
    pub fn all(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        vector: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::All,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(vector)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIsNan instruction to the current basic block."]
    pub fn is_nan(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsNan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIsInf instruction to the current basic block."]
    pub fn is_inf(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsInf,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIsFinite instruction to the current basic block."]
    pub fn is_finite(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsFinite,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIsNormal instruction to the current basic block."]
    pub fn is_normal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsNormal,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSignBitSet instruction to the current basic block."]
    pub fn sign_bit_set(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SignBitSet,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpLessOrGreater instruction to the current basic block."]
    pub fn less_or_greater(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LessOrGreater,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpOrdered instruction to the current basic block."]
    pub fn ordered(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Ordered,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpUnordered instruction to the current basic block."]
    pub fn unordered(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Unordered,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalEqual instruction to the current basic block."]
    pub fn logical_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalNotEqual instruction to the current basic block."]
    pub fn logical_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalOr instruction to the current basic block."]
    pub fn logical_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalOr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalAnd instruction to the current basic block."]
    pub fn logical_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalAnd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpLogicalNot instruction to the current basic block."]
    pub fn logical_not(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LogicalNot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSelect instruction to the current basic block."]
    pub fn select(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        condition: spirv::Word,
        object_1: spirv::Word,
        object_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIEqual instruction to the current basic block."]
    pub fn i_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpINotEqual instruction to the current basic block."]
    pub fn i_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::INotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpUGreaterThan instruction to the current basic block."]
    pub fn u_greater_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSGreaterThan instruction to the current basic block."]
    pub fn s_greater_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpUGreaterThanEqual instruction to the current basic block."]
    pub fn u_greater_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::UGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSGreaterThanEqual instruction to the current basic block."]
    pub fn s_greater_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpULessThan instruction to the current basic block."]
    pub fn u_less_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ULessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSLessThan instruction to the current basic block."]
    pub fn s_less_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpULessThanEqual instruction to the current basic block."]
    pub fn u_less_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ULessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSLessThanEqual instruction to the current basic block."]
    pub fn s_less_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdEqual instruction to the current basic block."]
    pub fn f_ord_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordEqual instruction to the current basic block."]
    pub fn f_unord_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdNotEqual instruction to the current basic block."]
    pub fn f_ord_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordNotEqual instruction to the current basic block."]
    pub fn f_unord_not_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordNotEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdLessThan instruction to the current basic block."]
    pub fn f_ord_less_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordLessThan instruction to the current basic block."]
    pub fn f_unord_less_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordLessThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdGreaterThan instruction to the current basic block."]
    pub fn f_ord_greater_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordGreaterThan instruction to the current basic block."]
    pub fn f_unord_greater_than(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordGreaterThan,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdLessThanEqual instruction to the current basic block."]
    pub fn f_ord_less_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordLessThanEqual instruction to the current basic block."]
    pub fn f_unord_less_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordLessThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFOrdGreaterThanEqual instruction to the current basic block."]
    pub fn f_ord_greater_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FOrdGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFUnordGreaterThanEqual instruction to the current basic block."]
    pub fn f_unord_greater_than_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FUnordGreaterThanEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpShiftRightLogical instruction to the current basic block."]
    pub fn shift_right_logical(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftRightLogical,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpShiftRightArithmetic instruction to the current basic block."]
    pub fn shift_right_arithmetic(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftRightArithmetic,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpShiftLeftLogical instruction to the current basic block."]
    pub fn shift_left_logical(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        shift: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ShiftLeftLogical,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base), dr::Operand::IdRef(shift)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseOr instruction to the current basic block."]
    pub fn bitwise_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseOr,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseXor instruction to the current basic block."]
    pub fn bitwise_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseXor,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitwiseAnd instruction to the current basic block."]
    pub fn bitwise_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitwiseAnd,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand_1), dr::Operand::IdRef(operand_2)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpNot instruction to the current basic block."]
    pub fn not(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        operand: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Not,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(operand)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldInsert instruction to the current basic block."]
    pub fn bit_field_insert(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        insert: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldSExtract instruction to the current basic block."]
    pub fn bit_field_s_extract(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitFieldUExtract instruction to the current basic block."]
    pub fn bit_field_u_extract(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitReverse instruction to the current basic block."]
    pub fn bit_reverse(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitReverse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBitCount instruction to the current basic block."]
    pub fn bit_count(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        base: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BitCount,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(base)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpDPdx instruction to the current basic block."]
    pub fn d_pdx(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdx,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpDPdy instruction to the current basic block."]
    pub fn d_pdy(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdy,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFwidth instruction to the current basic block."]
    pub fn fwidth(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Fwidth,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpDPdxFine instruction to the current basic block."]
    pub fn d_pdx_fine(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdxFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpDPdyFine instruction to the current basic block."]
    pub fn d_pdy_fine(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdyFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFwidthFine instruction to the current basic block."]
    pub fn fwidth_fine(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FwidthFine,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpDPdxCoarse instruction to the current basic block."]
    pub fn d_pdx_coarse(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdxCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpDPdyCoarse instruction to the current basic block."]
    pub fn d_pdy_coarse(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::DPdyCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFwidthCoarse instruction to the current basic block."]
    pub fn fwidth_coarse(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FwidthCoarse,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(p)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpEmitVertex instruction to the current basic block."]
    pub fn emit_vertex(&mut self) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::EmitVertex, None, None, vec![]);
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpEndPrimitive instruction to the current basic block."]
    pub fn end_primitive(&mut self) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::EndPrimitive, None, None, vec![]);
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpEmitStreamVertex instruction to the current basic block."]
    pub fn emit_stream_vertex(&mut self, stream: spirv::Word) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EmitStreamVertex,
            None,
            None,
            vec![dr::Operand::IdRef(stream)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpEndStreamPrimitive instruction to the current basic block."]
    pub fn end_stream_primitive(&mut self, stream: spirv::Word) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::EndStreamPrimitive,
            None,
            None,
            vec![dr::Operand::IdRef(stream)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpControlBarrier instruction to the current basic block."]
    pub fn control_barrier(
        &mut self,
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpMemoryBarrier instruction to the current basic block."]
    pub fn memory_barrier(
        &mut self,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpAtomicLoad instruction to the current basic block."]
    pub fn atomic_load(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicLoad,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicStore instruction to the current basic block."]
    pub fn atomic_store(
        &mut self,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicStore,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpAtomicExchange instruction to the current basic block."]
    pub fn atomic_exchange(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicExchange,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicCompareExchange instruction to the current basic block."]
    pub fn atomic_compare_exchange(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicCompareExchange,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(equal),
                dr::Operand::IdMemorySemantics(unequal),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(comparator),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicCompareExchangeWeak instruction to the current basic block."]
    pub fn atomic_compare_exchange_weak(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicCompareExchangeWeak,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(equal),
                dr::Operand::IdMemorySemantics(unequal),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(comparator),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIIncrement instruction to the current basic block."]
    pub fn atomic_i_increment(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIIncrement,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIDecrement instruction to the current basic block."]
    pub fn atomic_i_decrement(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIDecrement,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicIAdd instruction to the current basic block."]
    pub fn atomic_i_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicIAdd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicISub instruction to the current basic block."]
    pub fn atomic_i_sub(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicISub,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicSMin instruction to the current basic block."]
    pub fn atomic_s_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicSMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicUMin instruction to the current basic block."]
    pub fn atomic_u_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicUMin,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicSMax instruction to the current basic block."]
    pub fn atomic_s_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicSMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicUMax instruction to the current basic block."]
    pub fn atomic_u_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicUMax,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicAnd instruction to the current basic block."]
    pub fn atomic_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicAnd,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicOr instruction to the current basic block."]
    pub fn atomic_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicOr,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicXor instruction to the current basic block."]
    pub fn atomic_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicXor,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
                dr::Operand::IdRef(value),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpPhi instruction to the current basic block."]
    pub fn phi<T: AsRef<[(spirv::Word, spirv::Word)]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value_label_pairs: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Phi, Some(result_type), Some(_id), vec![]);
        for v in value_label_pairs.as_ref() {
            inst.operands.push(dr::Operand::IdRef(v.0));
            inst.operands.push(dr::Operand::IdRef(v.1));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpLoopMerge instruction to the current basic block."]
    pub fn loop_merge<T: AsRef<[dr::Operand]>>(
        &mut self,
        merge_block: spirv::Word,
        continue_target: spirv::Word,
        loop_control: spirv::LoopControl,
        additional_params: T,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpSelectionMerge instruction to the current basic block."]
    pub fn selection_merge(
        &mut self,
        merge_block: spirv::Word,
        selection_control: spirv::SelectionControl,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpLifetimeStart instruction to the current basic block."]
    pub fn lifetime_start(&mut self, pointer: spirv::Word, size: u32) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LifetimeStart,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::LiteralInt32(size)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpLifetimeStop instruction to the current basic block."]
    pub fn lifetime_stop(&mut self, pointer: spirv::Word, size: u32) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LifetimeStop,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::LiteralInt32(size)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpGroupAsyncCopy instruction to the current basic block."]
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
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupWaitEvents instruction to the current basic block."]
    pub fn group_wait_events(
        &mut self,
        execution: spirv::Word,
        num_events: spirv::Word,
        events_list: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpGroupAll instruction to the current basic block."]
    pub fn group_all(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupAny instruction to the current basic block."]
    pub fn group_any(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupBroadcast instruction to the current basic block."]
    pub fn group_broadcast(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        local_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupIAdd instruction to the current basic block."]
    pub fn group_i_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFAdd instruction to the current basic block."]
    pub fn group_f_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMin instruction to the current basic block."]
    pub fn group_f_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMin instruction to the current basic block."]
    pub fn group_u_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMin instruction to the current basic block."]
    pub fn group_s_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMax instruction to the current basic block."]
    pub fn group_f_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMax instruction to the current basic block."]
    pub fn group_u_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMax instruction to the current basic block."]
    pub fn group_s_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpReadPipe instruction to the current basic block."]
    pub fn read_pipe(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpWritePipe instruction to the current basic block."]
    pub fn write_pipe(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpReservedReadPipe instruction to the current basic block."]
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
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpReservedWritePipe instruction to the current basic block."]
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
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpReserveReadPipePackets instruction to the current basic block."]
    pub fn reserve_read_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpReserveWritePipePackets instruction to the current basic block."]
    pub fn reserve_write_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpCommitReadPipe instruction to the current basic block."]
    pub fn commit_read_pipe(
        &mut self,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpCommitWritePipe instruction to the current basic block."]
    pub fn commit_write_pipe(
        &mut self,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpIsValidReserveId instruction to the current basic block."]
    pub fn is_valid_reserve_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        reserve_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsValidReserveId,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(reserve_id)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGetNumPipePackets instruction to the current basic block."]
    pub fn get_num_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGetMaxPipePackets instruction to the current basic block."]
    pub fn get_max_pipe_packets(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupReserveReadPipePackets instruction to the current basic block."]
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
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupReserveWritePipePackets instruction to the current basic block."]
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
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupCommitReadPipe instruction to the current basic block."]
    pub fn group_commit_read_pipe(
        &mut self,
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpGroupCommitWritePipe instruction to the current basic block."]
    pub fn group_commit_write_pipe(
        &mut self,
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpEnqueueMarker instruction to the current basic block."]
    pub fn enqueue_marker(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        queue: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpEnqueueKernel instruction to the current basic block."]
    pub fn enqueue_kernel<T: AsRef<[spirv::Word]>>(
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
        local_size: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        for v in local_size.as_ref() {
            inst.operands.push(dr::Operand::IdRef(*v));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelNDrangeSubGroupCount instruction to the current basic block."]
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
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelNDrangeMaxSubGroupSize instruction to the current basic block."]
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
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelWorkGroupSize instruction to the current basic block."]
    pub fn get_kernel_work_group_size(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelPreferredWorkGroupSizeMultiple instruction to the current basic block."]
    pub fn get_kernel_preferred_work_group_size_multiple(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpRetainEvent instruction to the current basic block."]
    pub fn retain_event(&mut self, event: spirv::Word) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::RetainEvent,
            None,
            None,
            vec![dr::Operand::IdRef(event)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpReleaseEvent instruction to the current basic block."]
    pub fn release_event(&mut self, event: spirv::Word) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReleaseEvent,
            None,
            None,
            vec![dr::Operand::IdRef(event)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpCreateUserEvent instruction to the current basic block."]
    pub fn create_user_event(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreateUserEvent,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpIsValidEvent instruction to the current basic block."]
    pub fn is_valid_event(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        event: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::IsValidEvent,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(event)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSetUserEventStatus instruction to the current basic block."]
    pub fn set_user_event_status(
        &mut self,
        event: spirv::Word,
        status: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SetUserEventStatus,
            None,
            None,
            vec![dr::Operand::IdRef(event), dr::Operand::IdRef(status)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpCaptureEventProfilingInfo instruction to the current basic block."]
    pub fn capture_event_profiling_info(
        &mut self,
        event: spirv::Word,
        profiling_info: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpGetDefaultQueue instruction to the current basic block."]
    pub fn get_default_queue(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GetDefaultQueue,
            Some(result_type),
            Some(_id),
            vec![],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpBuildNDRange instruction to the current basic block."]
    pub fn build_nd_range(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        global_work_size: spirv::Word,
        local_work_size: spirv::Word,
        global_work_offset: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleImplicitLod instruction to the current basic block."]
    pub fn image_sparse_sample_implicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleExplicitLod instruction to the current basic block."]
    pub fn image_sparse_sample_explicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleDrefImplicitLod instruction to the current basic block."]
    pub fn image_sparse_sample_dref_implicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleDrefExplicitLod instruction to the current basic block."]
    pub fn image_sparse_sample_dref_explicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjImplicitLod instruction to the current basic block."]
    pub fn image_sparse_sample_proj_implicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjExplicitLod instruction to the current basic block."]
    pub fn image_sparse_sample_proj_explicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjDrefImplicitLod instruction to the current basic block."]
    pub fn image_sparse_sample_proj_dref_implicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseSampleProjDrefExplicitLod instruction to the current basic block."]
    pub fn image_sparse_sample_proj_dref_explicit_lod<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseFetch instruction to the current basic block."]
    pub fn image_sparse_fetch<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseFetch,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseGather instruction to the current basic block."]
    pub fn image_sparse_gather<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseDrefGather instruction to the current basic block."]
    pub fn image_sparse_dref_gather<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpImageSparseTexelsResident instruction to the current basic block."]
    pub fn image_sparse_texels_resident(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        resident_code: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseTexelsResident,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(resident_code)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFlagTestAndSet instruction to the current basic block."]
    pub fn atomic_flag_test_and_set(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFlagTestAndSet,
            Some(result_type),
            Some(_id),
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpAtomicFlagClear instruction to the current basic block."]
    pub fn atomic_flag_clear(
        &mut self,
        pointer: spirv::Word,
        scope: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::AtomicFlagClear,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer),
                dr::Operand::IdScope(scope),
                dr::Operand::IdMemorySemantics(semantics),
            ],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpImageSparseRead instruction to the current basic block."]
    pub fn image_sparse_read<T: AsRef<[dr::Operand]>>(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
        additional_params: T,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ImageSparseRead,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        if let Some(v) = image_operands {
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::ImageOperands(v.into()));
        }
        inst.operands.extend_from_slice(additional_params.as_ref());
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSizeOf instruction to the current basic block."]
    pub fn size_of(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pointer: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SizeOf,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pointer)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpCreatePipeFromPipeStorage instruction to the current basic block."]
    pub fn create_pipe_from_pipe_storage(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        pipe_storage: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::CreatePipeFromPipeStorage,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(pipe_storage)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelLocalSizeForSubgroupCount instruction to the current basic block."]
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
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGetKernelMaxNumSubgroups instruction to the current basic block."]
    pub fn get_kernel_max_num_subgroups(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpNamedBarrierInitialize instruction to the current basic block."]
    pub fn named_barrier_initialize(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        subgroup_count: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::NamedBarrierInitialize,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(subgroup_count)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpMemoryNamedBarrier instruction to the current basic block."]
    pub fn memory_named_barrier(
        &mut self,
        named_barrier: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpGroupNonUniformElect instruction to the current basic block."]
    pub fn group_non_uniform_elect(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformElect,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAll instruction to the current basic block."]
    pub fn group_non_uniform_all(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAny instruction to the current basic block."]
    pub fn group_non_uniform_any(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformAllEqual instruction to the current basic block."]
    pub fn group_non_uniform_all_equal(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformAllEqual,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBroadcast instruction to the current basic block."]
    pub fn group_non_uniform_broadcast(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBroadcastFirst instruction to the current basic block."]
    pub fn group_non_uniform_broadcast_first(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBroadcastFirst,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallot instruction to the current basic block."]
    pub fn group_non_uniform_ballot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformInverseBallot instruction to the current basic block."]
    pub fn group_non_uniform_inverse_ballot(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformInverseBallot,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotBitExtract instruction to the current basic block."]
    pub fn group_non_uniform_ballot_bit_extract(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotBitCount instruction to the current basic block."]
    pub fn group_non_uniform_ballot_bit_count(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotFindLSB instruction to the current basic block."]
    pub fn group_non_uniform_ballot_find_lsb(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotFindLSB,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBallotFindMSB instruction to the current basic block."]
    pub fn group_non_uniform_ballot_find_msb(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformBallotFindMSB,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdScope(execution), dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffle instruction to the current basic block."]
    pub fn group_non_uniform_shuffle(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleXor instruction to the current basic block."]
    pub fn group_non_uniform_shuffle_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        mask: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleUp instruction to the current basic block."]
    pub fn group_non_uniform_shuffle_up(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformShuffleDown instruction to the current basic block."]
    pub fn group_non_uniform_shuffle_down(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformIAdd instruction to the current basic block."]
    pub fn group_non_uniform_i_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFAdd instruction to the current basic block."]
    pub fn group_non_uniform_f_add(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformIMul instruction to the current basic block."]
    pub fn group_non_uniform_i_mul(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMul instruction to the current basic block."]
    pub fn group_non_uniform_f_mul(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformSMin instruction to the current basic block."]
    pub fn group_non_uniform_s_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformUMin instruction to the current basic block."]
    pub fn group_non_uniform_u_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMin instruction to the current basic block."]
    pub fn group_non_uniform_f_min(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformSMax instruction to the current basic block."]
    pub fn group_non_uniform_s_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformUMax instruction to the current basic block."]
    pub fn group_non_uniform_u_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformFMax instruction to the current basic block."]
    pub fn group_non_uniform_f_max(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseAnd instruction to the current basic block."]
    pub fn group_non_uniform_bitwise_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseOr instruction to the current basic block."]
    pub fn group_non_uniform_bitwise_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformBitwiseXor instruction to the current basic block."]
    pub fn group_non_uniform_bitwise_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalAnd instruction to the current basic block."]
    pub fn group_non_uniform_logical_and(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalOr instruction to the current basic block."]
    pub fn group_non_uniform_logical_or(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformLogicalXor instruction to the current basic block."]
    pub fn group_non_uniform_logical_xor(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
            #[allow(clippy::identity_conversion)]
            inst.operands.push(dr::Operand::IdRef(v.into()));
        }
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadBroadcast instruction to the current basic block."]
    pub fn group_non_uniform_quad_broadcast(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupNonUniformQuadSwap instruction to the current basic block."]
    pub fn group_non_uniform_quad_swap(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        value: spirv::Word,
        direction: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupBallotKHR instruction to the current basic block."]
    pub fn subgroup_ballot_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBallotKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupFirstInvocationKHR instruction to the current basic block."]
    pub fn subgroup_first_invocation_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupFirstInvocationKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupAllKHR instruction to the current basic block."]
    pub fn subgroup_all_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAllKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupAnyKHR instruction to the current basic block."]
    pub fn subgroup_any_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAnyKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupAllEqualKHR instruction to the current basic block."]
    pub fn subgroup_all_equal_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        predicate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupAllEqualKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(predicate)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupReadInvocationKHR instruction to the current basic block."]
    pub fn subgroup_read_invocation_khr(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
        index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupReadInvocationKHR,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value), dr::Operand::IdRef(index)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupIAddNonUniformAMD instruction to the current basic block."]
    pub fn group_i_add_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFAddNonUniformAMD instruction to the current basic block."]
    pub fn group_f_add_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMinNonUniformAMD instruction to the current basic block."]
    pub fn group_f_min_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMinNonUniformAMD instruction to the current basic block."]
    pub fn group_u_min_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMinNonUniformAMD instruction to the current basic block."]
    pub fn group_s_min_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupFMaxNonUniformAMD instruction to the current basic block."]
    pub fn group_f_max_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupUMaxNonUniformAMD instruction to the current basic block."]
    pub fn group_u_max_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpGroupSMaxNonUniformAMD instruction to the current basic block."]
    pub fn group_s_max_non_uniform_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFragmentMaskFetchAMD instruction to the current basic block."]
    pub fn fragment_mask_fetch_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::FragmentMaskFetchAMD,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpFragmentFetchAMD instruction to the current basic block."]
    pub fn fragment_fetch_amd(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
        fragment_index: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleINTEL instruction to the current basic block."]
    pub fn subgroup_shuffle_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        invocation_id: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(data), dr::Operand::IdRef(invocation_id)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleDownINTEL instruction to the current basic block."]
    pub fn subgroup_shuffle_down_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        current: spirv::Word,
        next: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleUpINTEL instruction to the current basic block."]
    pub fn subgroup_shuffle_up_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        previous: spirv::Word,
        current: spirv::Word,
        delta: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupShuffleXorINTEL instruction to the current basic block."]
    pub fn subgroup_shuffle_xor_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupShuffleXorINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(data), dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupBlockReadINTEL instruction to the current basic block."]
    pub fn subgroup_block_read_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ptr: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBlockReadINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(ptr)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupBlockWriteINTEL instruction to the current basic block."]
    pub fn subgroup_block_write_intel(
        &mut self,
        ptr: spirv::Word,
        data: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupBlockWriteINTEL,
            None,
            None,
            vec![dr::Operand::IdRef(ptr), dr::Operand::IdRef(data)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpSubgroupImageBlockReadINTEL instruction to the current basic block."]
    pub fn subgroup_image_block_read_intel(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        image: spirv::Word,
        coordinate: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::SubgroupImageBlockReadINTEL,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(image), dr::Operand::IdRef(coordinate)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
    #[doc = "Appends an OpSubgroupImageBlockWriteINTEL instruction to the current basic block."]
    pub fn subgroup_image_block_write_intel(
        &mut self,
        image: spirv::Word,
        coordinate: spirv::Word,
        data: spirv::Word,
    ) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
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
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(())
    }
    #[doc = "Appends an OpGroupNonUniformPartitionNV instruction to the current basic block."]
    pub fn group_non_uniform_partition_nv(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.basic_block.is_none() {
            return Err(Error::DetachedInstruction);
        }
        let _id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::GroupNonUniformPartitionNV,
            Some(result_type),
            Some(_id),
            vec![dr::Operand::IdRef(value)],
        );
        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(_id)
    }
}
