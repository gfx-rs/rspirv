//! Reflect functions for SPIR-V instructions.

use crate::spirv;

/// Returns true if the given opcode is for a location debug instruction.
pub fn is_location_debug(opcode: spirv::Op) -> bool {
    matches!(opcode, spirv::Op::Line | spirv::Op::NoLine)
}

/// Returns true if the given opcode is for a non-location debug instruction.
pub fn is_nonlocation_debug(opcode: spirv::Op) -> bool {
    matches!(
        opcode,
        spirv::Op::SourceContinued
            | spirv::Op::Source
            | spirv::Op::SourceExtension
            | spirv::Op::Name
            | spirv::Op::MemberName
            | spirv::Op::String
    )
}

/// Returns true if the given opcode is for a debug instruction.
pub fn is_debug(opcode: spirv::Op) -> bool {
    is_location_debug(opcode) || is_nonlocation_debug(opcode)
}

/// Returns true if the given opcode is for an annotation instruction.
pub fn is_annotation(opcode: spirv::Op) -> bool {
    matches!(
        opcode,
        spirv::Op::Decorate
            | spirv::Op::MemberDecorate
            | spirv::Op::DecorationGroup
            | spirv::Op::GroupDecorate
            | spirv::Op::GroupMemberDecorate
            | spirv::Op::DecorateString
            | spirv::Op::MemberDecorateStringGOOGLE
    )
}

/// Returns true if the given opcode is for a type-declaring instruction.
pub fn is_type(opcode: spirv::Op) -> bool {
    matches!(
        opcode,
        spirv::Op::TypeVoid
            | spirv::Op::TypeBool
            | spirv::Op::TypeInt
            | spirv::Op::TypeFloat
            | spirv::Op::TypeVector
            | spirv::Op::TypeMatrix
            | spirv::Op::TypeImage
            | spirv::Op::TypeSampler
            | spirv::Op::TypeSampledImage
            | spirv::Op::TypeArray
            | spirv::Op::TypeRuntimeArray
            | spirv::Op::TypeStruct
            | spirv::Op::TypeOpaque
            | spirv::Op::TypePointer
            | spirv::Op::TypeFunction
            | spirv::Op::TypeEvent
            | spirv::Op::TypeDeviceEvent
            | spirv::Op::TypeReserveId
            | spirv::Op::TypeQueue
            | spirv::Op::TypePipe
            | spirv::Op::TypeAccelerationStructureKHR
            | spirv::Op::TypeRayQueryKHR
            | spirv::Op::TypeForwardPointer
    )
}

/// Returns true if the given opcode is for a constant-defining instruction.
pub fn is_constant(opcode: spirv::Op) -> bool {
    matches!(
        opcode,
        spirv::Op::ConstantTrue
            | spirv::Op::ConstantFalse
            | spirv::Op::Constant
            | spirv::Op::ConstantComposite
            | spirv::Op::ConstantSampler
            | spirv::Op::ConstantNull
            | spirv::Op::SpecConstantTrue
            | spirv::Op::SpecConstantFalse
            | spirv::Op::SpecConstant
            | spirv::Op::SpecConstantComposite
            | spirv::Op::SpecConstantOp
            | spirv::Op::ConstantCompositeContinuedINTEL
            | spirv::Op::SpecConstantCompositeContinuedINTEL
    )
}

/// Returns true if the given opcode is for a variable-defining instruction.
pub fn is_variable(opcode: spirv::Op) -> bool {
    opcode == spirv::Op::Variable
}

/// Returns true if the given opcode is a return instruction.
pub fn is_return(opcode: spirv::Op) -> bool {
    matches!(opcode, spirv::Op::Return | spirv::Op::ReturnValue)
}

/// Returns true if the given opcode aborts execution.
pub fn is_abort(opcode: spirv::Op) -> bool {
    matches!(
        opcode,
        spirv::Op::Kill
            | spirv::Op::TerminateInvocation
            | spirv::Op::TerminateRayKHR
            | spirv::Op::IgnoreIntersectionKHR
            | spirv::Op::EmitMeshTasksEXT
            | spirv::Op::Unreachable
    )
}

/// Returns true if the given opcode is a return instruction or it aborts
/// execution.
pub fn is_return_or_abort(opcode: spirv::Op) -> bool {
    is_return(opcode) || is_abort(opcode)
}

/// Returns true if the given opcode is a branch instruction.
pub fn is_branch(opcode: spirv::Op) -> bool {
    matches!(
        opcode,
        spirv::Op::Branch | spirv::Op::BranchConditional | spirv::Op::Switch
    )
}

/// Returns true if the given opcode is for a terminator instruction.
pub fn is_block_terminator(opcode: spirv::Op) -> bool {
    is_branch(opcode) || is_return_or_abort(opcode)
}
