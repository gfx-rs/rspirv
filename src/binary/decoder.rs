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

use spirv;

use num::FromPrimitive;

const WORD_NUM_BYTES: usize = 4;

pub struct OperandDecoder {
    words: Vec<spirv::Word>,
    index: usize,
}

impl Iterator for OperandDecoder {
    type Item = spirv::Word;

    fn next(&mut self) -> Option<spirv::Word> {
        if self.empty() {
            None
        } else {
            self.index += 1;
            Some(self.words[self.index - 1])
        }
    }
}

impl OperandDecoder {
    pub fn new(words: Vec<spirv::Word>) -> OperandDecoder {
        OperandDecoder {
            words: words,
            index: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.index >= self.words.len()
    }

    fn split_word_to_bytes(&self, word: spirv::Word) -> Vec<u8> {
        (0..WORD_NUM_BYTES).map(|i| ((word >> (8 * i)) & 0xff) as u8).collect()
    }

    // TODO(antiagainst): in the following methods, we should distinguish None caused by no next
    // word and cannot decode the next word to the expected type.

    pub fn capability(&mut self) -> Option<spirv::Capability> {
        if let Some(word) = self.next() {
            spirv::Capability::from_u32(word)
        } else {
            None
        }
    }

    pub fn addressing_model(&mut self) -> Option<spirv::AddressingModel> {
        if let Some(word) = self.next() {
            spirv::AddressingModel::from_u32(word)
        } else {
            None
        }
    }

    pub fn memory_model(&mut self) -> Option<spirv::MemoryModel> {
        if let Some(word) = self.next() {
            spirv::MemoryModel::from_u32(word)
        } else {
            None
        }
    }

    pub fn execution_mode(&mut self) -> Option<spirv::ExecutionMode> {
        if let Some(word) = self.next() {
            spirv::ExecutionMode::from_u32(word)
        } else {
            None
        }
    }

    pub fn decode_execution_model(&mut self) -> Option<spirv::ExecutionModel> {
        if let Some(word) = self.next() {
            spirv::ExecutionModel::from_u32(word)
        } else {
            None
        }
    }

    pub fn source_language(&mut self) -> Option<spirv::SourceLanguage> {
        if let Some(word) = self.next() {
            spirv::SourceLanguage::from_u32(word)
        } else {
            None
        }
    }

    pub fn decoration(&mut self) -> Option<spirv::Decoration> {
        if let Some(word) = self.next() {
            spirv::Decoration::from_u32(word)
        } else {
            None
        }
    }

    pub fn storage_class(&mut self) -> Option<spirv::StorageClass> {
        if let Some(word) = self.next() {
            spirv::StorageClass::from_u32(word)
        } else {
            None
        }
    }

    pub fn dim(&mut self) -> Option<spirv::Dim> {
        if let Some(word) = self.next() {
            spirv::Dim::from_u32(word)
        } else {
            None
        }
    }

    pub fn sampler_addressing_mode(&mut self) -> Option<spirv::SamplerAddressingMode> {
        if let Some(word) = self.next() {
            spirv::SamplerAddressingMode::from_u32(word)
        } else {
            None
        }
    }

    pub fn sampler_filter_mode(&mut self) -> Option<spirv::SamplerFilterMode> {
        if let Some(word) = self.next() {
            spirv::SamplerFilterMode::from_u32(word)
        } else {
            None
        }
    }

    pub fn image_format(&mut self) -> Option<spirv::ImageFormat> {
        if let Some(word) = self.next() {
            spirv::ImageFormat::from_u32(word)
        } else {
            None
        }
    }

    pub fn image_channel_order(&mut self) -> Option<spirv::ImageChannelOrder> {
        if let Some(word) = self.next() {
            spirv::ImageChannelOrder::from_u32(word)
        } else {
            None
        }
    }

    pub fn image_channel_data_type(&mut self) -> Option<spirv::ImageChannelDataType> {
        if let Some(word) = self.next() {
            spirv::ImageChannelDataType::from_u32(word)
        } else {
            None
        }
    }

    pub fn fp_rounding_mode(&mut self) -> Option<spirv::FPRoundingMode> {
        if let Some(word) = self.next() {
            spirv::FPRoundingMode::from_u32(word)
        } else {
            None
        }
    }

    pub fn linkage_type(&mut self) -> Option<spirv::LinkageType> {
        if let Some(word) = self.next() {
            spirv::LinkageType::from_u32(word)
        } else {
            None
        }
    }

    pub fn access_qualifier(&mut self) -> Option<spirv::AccessQualifier> {
        if let Some(word) = self.next() {
            spirv::AccessQualifier::from_u32(word)
        } else {
            None
        }
    }

    pub fn function_parameter_attribute(&mut self) -> Option<spirv::FunctionParameterAttribute> {
        if let Some(word) = self.next() {
            spirv::FunctionParameterAttribute::from_u32(word)
        } else {
            None
        }
    }

    pub fn built_in(&mut self) -> Option<spirv::BuiltIn> {
        if let Some(word) = self.next() {
            spirv::BuiltIn::from_u32(word)
        } else {
            None
        }
    }

    pub fn group_operation(&mut self) -> Option<spirv::GroupOperation> {
        if let Some(word) = self.next() {
            spirv::GroupOperation::from_u32(word)
        } else {
            None
        }
    }

    pub fn kernel_enqueue_flags(&mut self) -> Option<spirv::KernelEnqueueFlags> {
        if let Some(word) = self.next() {
            spirv::KernelEnqueueFlags::from_u32(word)
        } else {
            None
        }
    }

    pub fn image_operands(&mut self) -> Option<spirv::ImageOperands> {
        if let Some(word) = self.next() {
            spirv::ImageOperands::from_bits(word)
        } else {
            None
        }
    }

    pub fn fp_fast_math_mode(&mut self) -> Option<spirv::FPFastMathMode> {
        if let Some(word) = self.next() {
            spirv::FPFastMathMode::from_bits(word)
        } else {
            None
        }
    }

    pub fn selection_control(&mut self) -> Option<spirv::SelectionControl> {
        if let Some(word) = self.next() {
            spirv::SelectionControl::from_bits(word)
        } else {
            None
        }
    }

    pub fn loop_control(&mut self) -> Option<spirv::LoopControl> {
        if let Some(word) = self.next() {
            spirv::LoopControl::from_bits(word)
        } else {
            None
        }
    }

    pub fn function_control(&mut self) -> Option<spirv::FunctionControl> {
        if let Some(word) = self.next() {
            spirv::FunctionControl::from_bits(word)
        } else {
            None
        }
    }

    pub fn memory_access(&mut self) -> Option<spirv::MemoryAccess> {
        if let Some(word) = self.next() {
            spirv::MemoryAccess::from_bits(word)
        } else {
            None
        }
    }

    pub fn kernel_profiling_info(&mut self) -> Option<spirv::KernelProfilingInfo> {
        if let Some(word) = self.next() {
            spirv::KernelProfilingInfo::from_bits(word)
        } else {
            None
        }
    }

    pub fn string(&mut self) -> Option<String> {
        let mut bytes = Vec::new();
        while let Some(word) = self.next() {
            bytes.append(&mut self.split_word_to_bytes(word));
            if bytes.last() == Some(&0) {
                break;
            }
        }
        while !bytes.is_empty() && bytes.last() == Some(&0) {
            bytes.pop();
        }
        String::from_utf8(bytes).ok()
    }

    pub fn id(&mut self) -> Option<spirv::Word> {
        self.next()
    }

    pub fn literal_integer(&mut self) -> Option<u32> {
        self.next()
    }

    pub fn context_dependent_number(&mut self) -> Option<u32> {
        // TODO(antiagainst): This should return the correct typed number.
        self.next()
    }
}
