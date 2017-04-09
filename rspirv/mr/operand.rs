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

/// Data representation of a SPIR-V operand.
#[derive(Debug, PartialEq, From)]
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
