// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use num_traits::FromPrimitive;
impl<'a> Decoder<'a> {
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V ImageOperands value."]
    pub fn image_operands(&mut self) -> Result<spirv::ImageOperands> {
        if let Ok(word) = self.word() {
            spirv::ImageOperands::from_bits(word).ok_or(Error::ImageOperandsUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V FPFastMathMode value."]
    pub fn fp_fast_math_mode(&mut self) -> Result<spirv::FPFastMathMode> {
        if let Ok(word) = self.word() {
            spirv::FPFastMathMode::from_bits(word).ok_or(Error::FPFastMathModeUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V SelectionControl value."]
    pub fn selection_control(&mut self) -> Result<spirv::SelectionControl> {
        if let Ok(word) = self.word() {
            spirv::SelectionControl::from_bits(word).ok_or(Error::SelectionControlUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V LoopControl value."]
    pub fn loop_control(&mut self) -> Result<spirv::LoopControl> {
        if let Ok(word) = self.word() {
            spirv::LoopControl::from_bits(word).ok_or(Error::LoopControlUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V FunctionControl value."]
    pub fn function_control(&mut self) -> Result<spirv::FunctionControl> {
        if let Ok(word) = self.word() {
            spirv::FunctionControl::from_bits(word).ok_or(Error::FunctionControlUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V MemorySemantics value."]
    pub fn memory_semantics(&mut self) -> Result<spirv::MemorySemantics> {
        if let Ok(word) = self.word() {
            spirv::MemorySemantics::from_bits(word).ok_or(Error::MemorySemanticsUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V MemoryAccess value."]
    pub fn memory_access(&mut self) -> Result<spirv::MemoryAccess> {
        if let Ok(word) = self.word() {
            spirv::MemoryAccess::from_bits(word).ok_or(Error::MemoryAccessUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V KernelProfilingInfo value."]
    pub fn kernel_profiling_info(&mut self) -> Result<spirv::KernelProfilingInfo> {
        if let Ok(word) = self.word() {
            spirv::KernelProfilingInfo::from_bits(word).ok_or(Error::KernelProfilingInfoUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V SourceLanguage value."]
    pub fn source_language(&mut self) -> Result<spirv::SourceLanguage> {
        if let Ok(word) = self.word() {
            spirv::SourceLanguage::from_u32(word).ok_or(Error::SourceLanguageUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V ExecutionModel value."]
    pub fn execution_model(&mut self) -> Result<spirv::ExecutionModel> {
        if let Ok(word) = self.word() {
            spirv::ExecutionModel::from_u32(word).ok_or(Error::ExecutionModelUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V AddressingModel value."]
    pub fn addressing_model(&mut self) -> Result<spirv::AddressingModel> {
        if let Ok(word) = self.word() {
            spirv::AddressingModel::from_u32(word).ok_or(Error::AddressingModelUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V MemoryModel value."]
    pub fn memory_model(&mut self) -> Result<spirv::MemoryModel> {
        if let Ok(word) = self.word() {
            spirv::MemoryModel::from_u32(word).ok_or(Error::MemoryModelUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V ExecutionMode value."]
    pub fn execution_mode(&mut self) -> Result<spirv::ExecutionMode> {
        if let Ok(word) = self.word() {
            spirv::ExecutionMode::from_u32(word).ok_or(Error::ExecutionModeUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V StorageClass value."]
    pub fn storage_class(&mut self) -> Result<spirv::StorageClass> {
        if let Ok(word) = self.word() {
            spirv::StorageClass::from_u32(word).ok_or(Error::StorageClassUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V Dim value."]
    pub fn dim(&mut self) -> Result<spirv::Dim> {
        if let Ok(word) = self.word() {
            spirv::Dim::from_u32(word).ok_or(Error::DimUnknown(self.offset - WORD_NUM_BYTES, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V SamplerAddressingMode value."]
    pub fn sampler_addressing_mode(&mut self) -> Result<spirv::SamplerAddressingMode> {
        if let Ok(word) = self.word() {
            spirv::SamplerAddressingMode::from_u32(word).ok_or(Error::SamplerAddressingModeUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V SamplerFilterMode value."]
    pub fn sampler_filter_mode(&mut self) -> Result<spirv::SamplerFilterMode> {
        if let Ok(word) = self.word() {
            spirv::SamplerFilterMode::from_u32(word).ok_or(Error::SamplerFilterModeUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V ImageFormat value."]
    pub fn image_format(&mut self) -> Result<spirv::ImageFormat> {
        if let Ok(word) = self.word() {
            spirv::ImageFormat::from_u32(word).ok_or(Error::ImageFormatUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V ImageChannelOrder value."]
    pub fn image_channel_order(&mut self) -> Result<spirv::ImageChannelOrder> {
        if let Ok(word) = self.word() {
            spirv::ImageChannelOrder::from_u32(word).ok_or(Error::ImageChannelOrderUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V ImageChannelDataType value."]
    pub fn image_channel_data_type(&mut self) -> Result<spirv::ImageChannelDataType> {
        if let Ok(word) = self.word() {
            spirv::ImageChannelDataType::from_u32(word).ok_or(Error::ImageChannelDataTypeUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V FPRoundingMode value."]
    pub fn fp_rounding_mode(&mut self) -> Result<spirv::FPRoundingMode> {
        if let Ok(word) = self.word() {
            spirv::FPRoundingMode::from_u32(word).ok_or(Error::FPRoundingModeUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V LinkageType value."]
    pub fn linkage_type(&mut self) -> Result<spirv::LinkageType> {
        if let Ok(word) = self.word() {
            spirv::LinkageType::from_u32(word).ok_or(Error::LinkageTypeUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V AccessQualifier value."]
    pub fn access_qualifier(&mut self) -> Result<spirv::AccessQualifier> {
        if let Ok(word) = self.word() {
            spirv::AccessQualifier::from_u32(word).ok_or(Error::AccessQualifierUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V FunctionParameterAttribute value."]
    pub fn function_parameter_attribute(&mut self) -> Result<spirv::FunctionParameterAttribute> {
        if let Ok(word) = self.word() {
            spirv::FunctionParameterAttribute::from_u32(word).ok_or(
                Error::FunctionParameterAttributeUnknown(self.offset - WORD_NUM_BYTES, word),
            )
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V Decoration value."]
    pub fn decoration(&mut self) -> Result<spirv::Decoration> {
        if let Ok(word) = self.word() {
            spirv::Decoration::from_u32(word)
                .ok_or(Error::DecorationUnknown(self.offset - WORD_NUM_BYTES, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V BuiltIn value."]
    pub fn built_in(&mut self) -> Result<spirv::BuiltIn> {
        if let Ok(word) = self.word() {
            spirv::BuiltIn::from_u32(word)
                .ok_or(Error::BuiltInUnknown(self.offset - WORD_NUM_BYTES, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V Scope value."]
    pub fn scope(&mut self) -> Result<spirv::Scope> {
        if let Ok(word) = self.word() {
            spirv::Scope::from_u32(word)
                .ok_or(Error::ScopeUnknown(self.offset - WORD_NUM_BYTES, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V GroupOperation value."]
    pub fn group_operation(&mut self) -> Result<spirv::GroupOperation> {
        if let Ok(word) = self.word() {
            spirv::GroupOperation::from_u32(word).ok_or(Error::GroupOperationUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V KernelEnqueueFlags value."]
    pub fn kernel_enqueue_flags(&mut self) -> Result<spirv::KernelEnqueueFlags> {
        if let Ok(word) = self.word() {
            spirv::KernelEnqueueFlags::from_u32(word).ok_or(Error::KernelEnqueueFlagsUnknown(
                self.offset - WORD_NUM_BYTES,
                word,
            ))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
    #[doc = "Decodes and returns the next SPIR-V word as\na SPIR-V Capability value."]
    pub fn capability(&mut self) -> Result<spirv::Capability> {
        if let Ok(word) = self.word() {
            spirv::Capability::from_u32(word)
                .ok_or(Error::CapabilityUnknown(self.offset - WORD_NUM_BYTES, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
}
