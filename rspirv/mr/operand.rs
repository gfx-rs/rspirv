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

/// Memory representation of a SPIR-V operand.
#[derive(Debug, PartialEq)]
pub enum Operand {
    ImageOperands(spirv::ImageOperands),
    FPFastMathMode(spirv::FPFastMathMode),
    SelectionControl(spirv::SelectionControl),
    LoopControl(spirv::LoopControl),
    FunctionControl(spirv::FunctionControl),
    MemorySemantics(spirv::MemorySemantics),
    MemoryAccess(spirv::MemoryAccess),
    KernelProfilingInfo(spirv::KernelProfilingInfo),
    SourceLanguage(spirv::SourceLanguage),
    ExecutionModel(spirv::ExecutionModel),
    AddressingModel(spirv::AddressingModel),
    MemoryModel(spirv::MemoryModel),
    ExecutionMode(spirv::ExecutionMode),
    StorageClass(spirv::StorageClass),
    Dim(spirv::Dim),
    SamplerAddressingMode(spirv::SamplerAddressingMode),
    SamplerFilterMode(spirv::SamplerFilterMode),
    ImageFormat(spirv::ImageFormat),
    ImageChannelOrder(spirv::ImageChannelOrder),
    ImageChannelDataType(spirv::ImageChannelDataType),
    FPRoundingMode(spirv::FPRoundingMode),
    LinkageType(spirv::LinkageType),
    AccessQualifier(spirv::AccessQualifier),
    FunctionParameterAttribute(spirv::FunctionParameterAttribute),
    Decoration(spirv::Decoration),
    BuiltIn(spirv::BuiltIn),
    Scope(spirv::Scope),
    GroupOperation(spirv::GroupOperation),
    KernelEnqueueFlags(spirv::KernelEnqueueFlags),
    Capability(spirv::Capability),
    IdMemorySemantics(spirv::Word),
    IdScope(spirv::Word),
    IdRef(spirv::Word),
    LiteralInt32(u32),
    LiteralInt64(u64),
    LiteralFloat32(f32),
    LiteralFloat64(f64),
    LiteralExtInstInteger(u32),
    LiteralSpecConstantOpInteger(spirv::Op),
    LiteralString(String),
}

impl convert::From<spirv::ImageOperands> for Operand {
    fn from(val: spirv::ImageOperands) -> Self {
        Operand::ImageOperands(val)
    }
}

impl convert::From<spirv::FPFastMathMode> for Operand {
    fn from(val: spirv::FPFastMathMode) -> Self {
        Operand::FPFastMathMode(val)
    }
}

impl convert::From<spirv::SelectionControl> for Operand {
    fn from(val: spirv::SelectionControl) -> Self {
        Operand::SelectionControl(val)
    }
}

impl convert::From<spirv::LoopControl> for Operand {
    fn from(val: spirv::LoopControl) -> Self {
        Operand::LoopControl(val)
    }
}

impl convert::From<spirv::FunctionControl> for Operand {
    fn from(val: spirv::FunctionControl) -> Self {
        Operand::FunctionControl(val)
    }
}

impl convert::From<spirv::MemorySemantics> for Operand {
    fn from(val: spirv::MemorySemantics) -> Self {
        Operand::MemorySemantics(val)
    }
}

impl convert::From<spirv::MemoryAccess> for Operand {
    fn from(val: spirv::MemoryAccess) -> Self {
        Operand::MemoryAccess(val)
    }
}

impl convert::From<spirv::KernelProfilingInfo> for Operand {
    fn from(val: spirv::KernelProfilingInfo) -> Self {
        Operand::KernelProfilingInfo(val)
    }
}

impl convert::From<spirv::SourceLanguage> for Operand {
    fn from(val: spirv::SourceLanguage) -> Self {
        Operand::SourceLanguage(val)
    }
}

impl convert::From<spirv::ExecutionModel> for Operand {
    fn from(val: spirv::ExecutionModel) -> Self {
        Operand::ExecutionModel(val)
    }
}

impl convert::From<spirv::AddressingModel> for Operand {
    fn from(val: spirv::AddressingModel) -> Self {
        Operand::AddressingModel(val)
    }
}

impl convert::From<spirv::MemoryModel> for Operand {
    fn from(val: spirv::MemoryModel) -> Self {
        Operand::MemoryModel(val)
    }
}

impl convert::From<spirv::ExecutionMode> for Operand {
    fn from(val: spirv::ExecutionMode) -> Self {
        Operand::ExecutionMode(val)
    }
}

impl convert::From<spirv::StorageClass> for Operand {
    fn from(val: spirv::StorageClass) -> Self {
        Operand::StorageClass(val)
    }
}

impl convert::From<spirv::Dim> for Operand {
    fn from(val: spirv::Dim) -> Self {
        Operand::Dim(val)
    }
}

impl convert::From<spirv::SamplerAddressingMode> for Operand {
    fn from(val: spirv::SamplerAddressingMode) -> Self {
        Operand::SamplerAddressingMode(val)
    }
}

impl convert::From<spirv::SamplerFilterMode> for Operand {
    fn from(val: spirv::SamplerFilterMode) -> Self {
        Operand::SamplerFilterMode(val)
    }
}

impl convert::From<spirv::ImageFormat> for Operand {
    fn from(val: spirv::ImageFormat) -> Self {
        Operand::ImageFormat(val)
    }
}

impl convert::From<spirv::ImageChannelOrder> for Operand {
    fn from(val: spirv::ImageChannelOrder) -> Self {
        Operand::ImageChannelOrder(val)
    }
}

impl convert::From<spirv::ImageChannelDataType> for Operand {
    fn from(val: spirv::ImageChannelDataType) -> Self {
        Operand::ImageChannelDataType(val)
    }
}

impl convert::From<spirv::FPRoundingMode> for Operand {
    fn from(val: spirv::FPRoundingMode) -> Self {
        Operand::FPRoundingMode(val)
    }
}

impl convert::From<spirv::LinkageType> for Operand {
    fn from(val: spirv::LinkageType) -> Self {
        Operand::LinkageType(val)
    }
}

impl convert::From<spirv::AccessQualifier> for Operand {
    fn from(val: spirv::AccessQualifier) -> Self {
        Operand::AccessQualifier(val)
    }
}

impl convert::From<spirv::FunctionParameterAttribute> for Operand {
    fn from(val: spirv::FunctionParameterAttribute) -> Self {
        Operand::FunctionParameterAttribute(val)
    }
}

impl convert::From<spirv::Decoration> for Operand {
    fn from(val: spirv::Decoration) -> Self {
        Operand::Decoration(val)
    }
}

impl convert::From<spirv::BuiltIn> for Operand {
    fn from(val: spirv::BuiltIn) -> Self {
        Operand::BuiltIn(val)
    }
}

impl convert::From<spirv::Scope> for Operand {
    fn from(val: spirv::Scope) -> Self {
        Operand::Scope(val)
    }
}

impl convert::From<spirv::GroupOperation> for Operand {
    fn from(val: spirv::GroupOperation) -> Self {
        Operand::GroupOperation(val)
    }
}

impl convert::From<spirv::KernelEnqueueFlags> for Operand {
    fn from(val: spirv::KernelEnqueueFlags) -> Self {
        Operand::KernelEnqueueFlags(val)
    }
}

impl convert::From<spirv::Capability> for Operand {
    fn from(val: spirv::Capability) -> Self {
        Operand::Capability(val)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::ImageOperands(ref v) => write!(f, "{:?}", v),
            Operand::FPFastMathMode(ref v) => write!(f, "{:?}", v),
            Operand::SelectionControl(ref v) => write!(f, "{:?}", v),
            Operand::LoopControl(ref v) => write!(f, "{:?}", v),
            Operand::FunctionControl(ref v) => write!(f, "{:?}", v),
            Operand::MemorySemantics(ref v) => write!(f, "{:?}", v),
            Operand::MemoryAccess(ref v) => write!(f, "{:?}", v),
            Operand::KernelProfilingInfo(ref v) => write!(f, "{:?}", v),
            Operand::SourceLanguage(ref v) => write!(f, "{:?}", v),
            Operand::ExecutionModel(ref v) => write!(f, "{:?}", v),
            Operand::AddressingModel(ref v) => write!(f, "{:?}", v),
            Operand::MemoryModel(ref v) => write!(f, "{:?}", v),
            Operand::ExecutionMode(ref v) => write!(f, "{:?}", v),
            Operand::StorageClass(ref v) => write!(f, "{:?}", v),
            Operand::Dim(ref v) => write!(f, "{:?}", v),
            Operand::SamplerAddressingMode(ref v) => write!(f, "{:?}", v),
            Operand::SamplerFilterMode(ref v) => write!(f, "{:?}", v),
            Operand::ImageFormat(ref v) => write!(f, "{:?}", v),
            Operand::ImageChannelOrder(ref v) => write!(f, "{:?}", v),
            Operand::ImageChannelDataType(ref v) => write!(f, "{:?}", v),
            Operand::FPRoundingMode(ref v) => write!(f, "{:?}", v),
            Operand::LinkageType(ref v) => write!(f, "{:?}", v),
            Operand::AccessQualifier(ref v) => write!(f, "{:?}", v),
            Operand::FunctionParameterAttribute(ref v) => write!(f, "{:?}", v),
            Operand::Decoration(ref v) => write!(f, "{:?}", v),
            Operand::BuiltIn(ref v) => write!(f, "{:?}", v),
            Operand::Scope(ref v) => write!(f, "{:?}", v),
            Operand::GroupOperation(ref v) => write!(f, "{:?}", v),
            Operand::KernelEnqueueFlags(ref v) => write!(f, "{:?}", v),
            Operand::Capability(ref v) => write!(f, "{:?}", v),
            Operand::IdMemorySemantics(ref v) => write!(f, "{:?}", v),
            Operand::IdScope(ref v) => write!(f, "{:?}", v),
            Operand::IdRef(ref v) => write!(f, "{:?}", v),
            Operand::LiteralString(ref v) => write!(f, "{:?}", v),
            Operand::LiteralExtInstInteger(ref v) => write!(f, "{:?}", v),
            Operand::LiteralSpecConstantOpInteger(ref v) => write!(f, "{:?}", v),
            Operand::LiteralInt32(ref v) => write!(f, "{:?}", v),
            Operand::LiteralInt64(ref v) => write!(f, "{:?}", v),
            Operand::LiteralFloat32(ref v) => write!(f, "{:?}", v),
            Operand::LiteralFloat64(ref v) => write!(f, "{:?}", v),
        }
    }
}
