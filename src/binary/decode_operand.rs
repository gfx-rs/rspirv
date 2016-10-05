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

// This rust module is automatically generated from the SPIR-V JSON grammar:
//   https://github.com/KhronosGroup/SPIRV-Headers/
//           blob/master/include/spirv/1.1/spirv.core.grammar.json

use num::FromPrimitive;

impl Decoder {
    pub fn image_operands(&mut self) -> Result<spirv::ImageOperands> {
        if let Ok(word) = self.word() {
            spirv::ImageOperands::from_bits(word).ok_or(Error::ImageOperandsUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn fpfast_math_mode(&mut self) -> Result<spirv::FPFastMathMode> {
        if let Ok(word) = self.word() {
            spirv::FPFastMathMode::from_bits(word).ok_or(Error::FPFastMathModeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn selection_control(&mut self) -> Result<spirv::SelectionControl> {
        if let Ok(word) = self.word() {
            spirv::SelectionControl::from_bits(word).ok_or(Error::SelectionControlUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn loop_control(&mut self) -> Result<spirv::LoopControl> {
        if let Ok(word) = self.word() {
            spirv::LoopControl::from_bits(word).ok_or(Error::LoopControlUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn function_control(&mut self) -> Result<spirv::FunctionControl> {
        if let Ok(word) = self.word() {
            spirv::FunctionControl::from_bits(word).ok_or(Error::FunctionControlUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn memory_semantics(&mut self) -> Result<spirv::MemorySemantics> {
        if let Ok(word) = self.word() {
            spirv::MemorySemantics::from_bits(word).ok_or(Error::MemorySemanticsUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn memory_access(&mut self) -> Result<spirv::MemoryAccess> {
        if let Ok(word) = self.word() {
            spirv::MemoryAccess::from_bits(word).ok_or(Error::MemoryAccessUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn kernel_profiling_info(&mut self) -> Result<spirv::KernelProfilingInfo> {
        if let Ok(word) = self.word() {
            spirv::KernelProfilingInfo::from_bits(word).ok_or(Error::KernelProfilingInfoUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn source_language(&mut self) -> Result<spirv::SourceLanguage> {
        if let Ok(word) = self.word() {
            spirv::SourceLanguage::from_u32(word).ok_or(Error::SourceLanguageUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn execution_model(&mut self) -> Result<spirv::ExecutionModel> {
        if let Ok(word) = self.word() {
            spirv::ExecutionModel::from_u32(word).ok_or(Error::ExecutionModelUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn addressing_model(&mut self) -> Result<spirv::AddressingModel> {
        if let Ok(word) = self.word() {
            spirv::AddressingModel::from_u32(word).ok_or(Error::AddressingModelUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn memory_model(&mut self) -> Result<spirv::MemoryModel> {
        if let Ok(word) = self.word() {
            spirv::MemoryModel::from_u32(word).ok_or(Error::MemoryModelUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn execution_mode(&mut self) -> Result<spirv::ExecutionMode> {
        if let Ok(word) = self.word() {
            spirv::ExecutionMode::from_u32(word).ok_or(Error::ExecutionModeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn storage_class(&mut self) -> Result<spirv::StorageClass> {
        if let Ok(word) = self.word() {
            spirv::StorageClass::from_u32(word).ok_or(Error::StorageClassUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn dim(&mut self) -> Result<spirv::Dim> {
        if let Ok(word) = self.word() {
            spirv::Dim::from_u32(word).ok_or(Error::DimUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn sampler_addressing_mode(&mut self) -> Result<spirv::SamplerAddressingMode> {
        if let Ok(word) = self.word() {
            spirv::SamplerAddressingMode::from_u32(word).ok_or(Error::SamplerAddressingModeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn sampler_filter_mode(&mut self) -> Result<spirv::SamplerFilterMode> {
        if let Ok(word) = self.word() {
            spirv::SamplerFilterMode::from_u32(word).ok_or(Error::SamplerFilterModeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn image_format(&mut self) -> Result<spirv::ImageFormat> {
        if let Ok(word) = self.word() {
            spirv::ImageFormat::from_u32(word).ok_or(Error::ImageFormatUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn image_channel_order(&mut self) -> Result<spirv::ImageChannelOrder> {
        if let Ok(word) = self.word() {
            spirv::ImageChannelOrder::from_u32(word).ok_or(Error::ImageChannelOrderUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn image_channel_data_type(&mut self) -> Result<spirv::ImageChannelDataType> {
        if let Ok(word) = self.word() {
            spirv::ImageChannelDataType::from_u32(word).ok_or(Error::ImageChannelDataTypeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn fprounding_mode(&mut self) -> Result<spirv::FPRoundingMode> {
        if let Ok(word) = self.word() {
            spirv::FPRoundingMode::from_u32(word).ok_or(Error::FPRoundingModeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn linkage_type(&mut self) -> Result<spirv::LinkageType> {
        if let Ok(word) = self.word() {
            spirv::LinkageType::from_u32(word).ok_or(Error::LinkageTypeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn access_qualifier(&mut self) -> Result<spirv::AccessQualifier> {
        if let Ok(word) = self.word() {
            spirv::AccessQualifier::from_u32(word).ok_or(Error::AccessQualifierUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn function_parameter_attribute(&mut self) -> Result<spirv::FunctionParameterAttribute> {
        if let Ok(word) = self.word() {
            spirv::FunctionParameterAttribute::from_u32(word).ok_or(Error::FunctionParameterAttributeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn decoration(&mut self) -> Result<spirv::Decoration> {
        if let Ok(word) = self.word() {
            spirv::Decoration::from_u32(word).ok_or(Error::DecorationUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn built_in(&mut self) -> Result<spirv::BuiltIn> {
        if let Ok(word) = self.word() {
            spirv::BuiltIn::from_u32(word).ok_or(Error::BuiltInUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn scope(&mut self) -> Result<spirv::Scope> {
        if let Ok(word) = self.word() {
            spirv::Scope::from_u32(word).ok_or(Error::ScopeUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn group_operation(&mut self) -> Result<spirv::GroupOperation> {
        if let Ok(word) = self.word() {
            spirv::GroupOperation::from_u32(word).ok_or(Error::GroupOperationUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn kernel_enqueue_flags(&mut self) -> Result<spirv::KernelEnqueueFlags> {
        if let Ok(word) = self.word() {
            spirv::KernelEnqueueFlags::from_u32(word).ok_or(Error::KernelEnqueueFlagsUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }

    pub fn capability(&mut self) -> Result<spirv::Capability> {
        if let Ok(word) = self.word() {
            spirv::Capability::from_u32(word).ok_or(Error::CapabilityUnknown(self.offset, word))
        } else {
            Err(Error::StreamExpected(self.offset))
        }
    }
}
