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

//! Reflect functions for SPIR-V instructions.

use crate::spirv;

/// Returns true if the given opcode is for a location debug instruction.
pub fn is_location_debug(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::Line | spirv::Op::NoLine => true,
        _ => false,
    }
}

/// Returns true if the given opcode is for a non-location debug instruction.
pub fn is_nonlocation_debug(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::SourceContinued |
        spirv::Op::Source |
        spirv::Op::SourceExtension |
        spirv::Op::Name |
        spirv::Op::MemberName |
        spirv::Op::String => true,
        _ => false,
    }
}

/// Returns true if the given opcode is for a debug instruction.
pub fn is_debug(opcode: spirv::Op) -> bool {
    is_location_debug(opcode) || is_nonlocation_debug(opcode)
}

/// Returns true if the given opcode is for an annotation instruction.
pub fn is_annotation(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::Decorate |
        spirv::Op::MemberDecorate |
        spirv::Op::DecorationGroup |
        spirv::Op::GroupDecorate |
        spirv::Op::GroupMemberDecorate |
        spirv::Op::DecorateStringGOOGLE |
        spirv::Op::MemberDecorateStringGOOGLE => true,
        _ => false,
    }
}


/// Returns true if the given opcode is for a type-declaring instruction.
pub fn is_type(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::TypeVoid |
        spirv::Op::TypeBool |
        spirv::Op::TypeInt |
        spirv::Op::TypeFloat |
        spirv::Op::TypeVector |
        spirv::Op::TypeMatrix |
        spirv::Op::TypeImage |
        spirv::Op::TypeSampler |
        spirv::Op::TypeSampledImage |
        spirv::Op::TypeArray |
        spirv::Op::TypeRuntimeArray |
        spirv::Op::TypeStruct |
        spirv::Op::TypeOpaque |
        spirv::Op::TypePointer |
        spirv::Op::TypeFunction |
        spirv::Op::TypeEvent |
        spirv::Op::TypeDeviceEvent |
        spirv::Op::TypeReserveId |
        spirv::Op::TypeQueue |
        spirv::Op::TypePipe |
        spirv::Op::TypeForwardPointer => true,
        _ => false,
    }
}

/// Returns true if the given opcode is for a constant-defining instruction.
pub fn is_constant(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::ConstantTrue |
        spirv::Op::ConstantFalse |
        spirv::Op::Constant |
        spirv::Op::ConstantComposite |
        spirv::Op::ConstantSampler |
        spirv::Op::ConstantNull |
        spirv::Op::SpecConstantTrue |
        spirv::Op::SpecConstantFalse |
        spirv::Op::SpecConstant |
        spirv::Op::SpecConstantComposite |
        spirv::Op::SpecConstantOp => true,
        _ => false,
    }
}

/// Returns true if the given opcode is for a variable-defining instruction.
pub fn is_variable(opcode: spirv::Op) -> bool {
    opcode == spirv::Op::Variable
}

/// Returns true if the given opcode is for a terminator instruction.
pub fn is_terminator(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::Branch |
        spirv::Op::BranchConditional |
        spirv::Op::Switch |
        spirv::Op::Kill |
        spirv::Op::Return |
        spirv::Op::ReturnValue |
        spirv::Op::Unreachable => true,
        _ => false,
    }
}
