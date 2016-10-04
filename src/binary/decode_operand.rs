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

impl OperandDecoder {
    pub fn image_operands(&mut self) -> Result<spirv::ImageOperands> {
        if let Some(word) = self.next() {
            spirv::ImageOperands::from_bits(word).ok_or(Error::ImageOperandsUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn fpfast_math_mode(&mut self) -> Result<spirv::FPFastMathMode> {
        if let Some(word) = self.next() {
            spirv::FPFastMathMode::from_bits(word).ok_or(Error::FPFastMathModeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn selection_control(&mut self) -> Result<spirv::SelectionControl> {
        if let Some(word) = self.next() {
            spirv::SelectionControl::from_bits(word).ok_or(Error::SelectionControlUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn loop_control(&mut self) -> Result<spirv::LoopControl> {
        if let Some(word) = self.next() {
            spirv::LoopControl::from_bits(word).ok_or(Error::LoopControlUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn function_control(&mut self) -> Result<spirv::FunctionControl> {
        if let Some(word) = self.next() {
            spirv::FunctionControl::from_bits(word).ok_or(Error::FunctionControlUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn memory_semantics(&mut self) -> Result<spirv::MemorySemantics> {
        if let Some(word) = self.next() {
            spirv::MemorySemantics::from_bits(word).ok_or(Error::MemorySemanticsUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn memory_access(&mut self) -> Result<spirv::MemoryAccess> {
        if let Some(word) = self.next() {
            spirv::MemoryAccess::from_bits(word).ok_or(Error::MemoryAccessUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn kernel_profiling_info(&mut self) -> Result<spirv::KernelProfilingInfo> {
        if let Some(word) = self.next() {
            spirv::KernelProfilingInfo::from_bits(word).ok_or(Error::KernelProfilingInfoUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn source_language(&mut self) -> Result<spirv::SourceLanguage> {
        if let Some(word) = self.next() {
            spirv::SourceLanguage::from_u32(word).ok_or(Error::SourceLanguageUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn execution_model(&mut self) -> Result<spirv::ExecutionModel> {
        if let Some(word) = self.next() {
            spirv::ExecutionModel::from_u32(word).ok_or(Error::ExecutionModelUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn addressing_model(&mut self) -> Result<spirv::AddressingModel> {
        if let Some(word) = self.next() {
            spirv::AddressingModel::from_u32(word).ok_or(Error::AddressingModelUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn memory_model(&mut self) -> Result<spirv::MemoryModel> {
        if let Some(word) = self.next() {
            spirv::MemoryModel::from_u32(word).ok_or(Error::MemoryModelUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn execution_mode(&mut self) -> Result<spirv::ExecutionMode> {
        if let Some(word) = self.next() {
            spirv::ExecutionMode::from_u32(word).ok_or(Error::ExecutionModeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn storage_class(&mut self) -> Result<spirv::StorageClass> {
        if let Some(word) = self.next() {
            spirv::StorageClass::from_u32(word).ok_or(Error::StorageClassUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn dim(&mut self) -> Result<spirv::Dim> {
        if let Some(word) = self.next() {
            spirv::Dim::from_u32(word).ok_or(Error::DimUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn sampler_addressing_mode(&mut self) -> Result<spirv::SamplerAddressingMode> {
        if let Some(word) = self.next() {
            spirv::SamplerAddressingMode::from_u32(word).ok_or(Error::SamplerAddressingModeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn sampler_filter_mode(&mut self) -> Result<spirv::SamplerFilterMode> {
        if let Some(word) = self.next() {
            spirv::SamplerFilterMode::from_u32(word).ok_or(Error::SamplerFilterModeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn image_format(&mut self) -> Result<spirv::ImageFormat> {
        if let Some(word) = self.next() {
            spirv::ImageFormat::from_u32(word).ok_or(Error::ImageFormatUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn image_channel_order(&mut self) -> Result<spirv::ImageChannelOrder> {
        if let Some(word) = self.next() {
            spirv::ImageChannelOrder::from_u32(word).ok_or(Error::ImageChannelOrderUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn image_channel_data_type(&mut self) -> Result<spirv::ImageChannelDataType> {
        if let Some(word) = self.next() {
            spirv::ImageChannelDataType::from_u32(word).ok_or(Error::ImageChannelDataTypeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn fprounding_mode(&mut self) -> Result<spirv::FPRoundingMode> {
        if let Some(word) = self.next() {
            spirv::FPRoundingMode::from_u32(word).ok_or(Error::FPRoundingModeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn linkage_type(&mut self) -> Result<spirv::LinkageType> {
        if let Some(word) = self.next() {
            spirv::LinkageType::from_u32(word).ok_or(Error::LinkageTypeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn access_qualifier(&mut self) -> Result<spirv::AccessQualifier> {
        if let Some(word) = self.next() {
            spirv::AccessQualifier::from_u32(word).ok_or(Error::AccessQualifierUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn function_parameter_attribute(&mut self) -> Result<spirv::FunctionParameterAttribute> {
        if let Some(word) = self.next() {
            spirv::FunctionParameterAttribute::from_u32(word).ok_or(Error::FunctionParameterAttributeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn decoration(&mut self) -> Result<spirv::Decoration> {
        if let Some(word) = self.next() {
            spirv::Decoration::from_u32(word).ok_or(Error::DecorationUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn built_in(&mut self) -> Result<spirv::BuiltIn> {
        if let Some(word) = self.next() {
            spirv::BuiltIn::from_u32(word).ok_or(Error::BuiltInUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn scope(&mut self) -> Result<spirv::Scope> {
        if let Some(word) = self.next() {
            spirv::Scope::from_u32(word).ok_or(Error::ScopeUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn group_operation(&mut self) -> Result<spirv::GroupOperation> {
        if let Some(word) = self.next() {
            spirv::GroupOperation::from_u32(word).ok_or(Error::GroupOperationUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn kernel_enqueue_flags(&mut self) -> Result<spirv::KernelEnqueueFlags> {
        if let Some(word) = self.next() {
            spirv::KernelEnqueueFlags::from_u32(word).ok_or(Error::KernelEnqueueFlagsUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }

    pub fn capability(&mut self) -> Result<spirv::Capability> {
        if let Some(word) = self.next() {
            spirv::Capability::from_u32(word).ok_or(Error::CapabilityUnknown(self.index, word))
        } else {
            Err(Error::StreamExpected(self.index))
        }
    }
}
