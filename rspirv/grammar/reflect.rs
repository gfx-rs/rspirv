//! Reflect functions for SPIR-V instructions.

use crate::spirv;

/// Returns [`true`] if the given opcode is for a location debug instruction.
pub fn is_location_debug(opcode: spirv::Op) -> bool {
    matches!(opcode, spirv::Op::Line | spirv::Op::NoLine)
}

/// Returns [`true`] if the given opcode is for a variable-defining instruction.
pub fn is_variable(opcode: spirv::Op) -> bool {
    opcode == spirv::Op::Variable
}

/// Returns [`true`] if the given opcode is a return instruction.
pub fn is_return(opcode: spirv::Op) -> bool {
    matches!(opcode, spirv::Op::Return | spirv::Op::ReturnValue)
}

/// Returns [`true`] if the given opcode aborts execution.
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

/// Returns [`true`] if the given opcode is a return instruction or it aborts execution.
///
/// <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#FunctionTermination>
pub fn is_return_or_abort(opcode: spirv::Op) -> bool {
    is_return(opcode) || is_abort(opcode)
}

/// Returns [`true`] if the given opcode is a branch instruction.
///
/// <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Branch> and <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#ConditionalBranch>
pub fn is_branch(opcode: spirv::Op) -> bool {
    matches!(
        opcode,
        spirv::Op::Branch | spirv::Op::BranchConditional | spirv::Op::Switch
    )
}

/// Returns [`true`] if the given opcode is for a terminator instruction.
///
/// <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Termination>
pub fn is_block_terminator(opcode: spirv::Op) -> bool {
    is_branch(opcode) || is_return_or_abort(opcode)
}
