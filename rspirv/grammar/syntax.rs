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

/// Grammar for a SPIR-V instruction.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Instruction<'a> {
    /// Opname.
    pub opname: &'a str,
    /// Opcode.
    pub opcode: spirv::Op,
    /// Capabilities required for this instruction.
    pub capabilities: &'a [spirv::Capability],
    /// Logical operands for this instruction.
    ///
    /// This includes result type id and result id.
    pub operands: &'a [LogicalOperand],
}

/// Grammar for an extended instruction.
pub struct ExtendedInstruction<'a> {
    /// OpName.
    pub opname: &'a str,
    /// Opcode.
    pub opcode: spirv::Word,
    /// Capabilities required for this instruction.
    pub capabilities: &'a [spirv::Capability],
    /// Logical operands for this instruction.
    pub operands: &'a [LogicalOperand],
}

/// Grammar for a SPIR-V logical operand.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LogicalOperand {
    /// The kind of this logical operand.
    pub kind: OperandKind,
    /// The repeat specification for this logical operand.
    pub quantifier: OperandQuantifier,
}

/// The repeat specification for a SPIR-V logical operand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperandQuantifier {
    /// This operand appears exactly one time.
    One,
    /// This operand can appear zero or one time.
    ZeroOrOne,
    /// This operand can appear zero or more times.
    ZeroOrMore,
}

/// Declares the grammar for an SPIR-V instruction.
macro_rules! inst {
    ($op:ident, [$( $cap:ident ),*], [$( ($kind:ident, $quant:ident) ),*]) => {
        Instruction {
            opname: stringify!($op),
            opcode: spirv::Op::$op,
            capabilities: &[
                $( spirv::Capability::$cap ),*
            ],
            operands: &[
                $( LogicalOperand {
                    kind: OperandKind::$kind,
                    quantifier: OperandQuantifier::$quant }
                ),*
            ],
        }
    }
}

/// Declares the grammar for an extended instruction instruction.
macro_rules! ext_inst {
    ($opname:ident, $opcode: expr, [$( $cap:ident ),*],
     [$( ($kind:ident, $quant:ident) ),*]) => {
        ExtendedInstruction {
            opname: stringify!($opname),
            opcode: $opcode,
            capabilities: &[
                $( spirv::Capability::$cap ),*
            ],
            operands: &[
                $( LogicalOperand {
                    kind: OperandKind::$kind,
                    quantifier: OperandQuantifier::$quant }
                ),*
            ],
        }
    }
}

/// The table for all SPIR-V core instructions.
///
/// This table is staic data stored in the library.
pub struct CoreInstructionTable;

impl CoreInstructionTable {
    /// Looks up the given `opcode` in the instruction table and returns
    /// a reference to the instruction grammar entry if found.
    pub fn lookup_opcode(opcode: u16) -> Option<&'static Instruction<'static>> {
        INSTRUCTION_TABLE.iter().find(|inst| {
            (inst.opcode as u16) == opcode
        })
    }

    /// Returns a reference to the instruction grammar entry with the given
    /// `opcode`.
    pub fn get(opcode: spirv::Op) -> &'static Instruction<'static> {
        INSTRUCTION_TABLE
            .iter()
            .find(|inst| (inst.opcode == opcode))
            .expect("internal error")
    }
}

include!("table.rs");

/// The table for all `GLSLstd450` extended instructions.
///
/// This table is staic data stored in the library.
pub struct GlslStd450InstructionTable;

impl GlslStd450InstructionTable {
    /// Looks up the given `opcode` in the instruction table and returns
    /// a reference to the instruction grammar entry if found.
    pub fn lookup_opcode(opcode: u32) -> Option<&'static ExtendedInstruction<'static>> {
        GLSL_STD_450_INSTRUCTION_TABLE.iter().find(|inst| {
            inst.opcode == opcode
        })
    }

    /// Returns a reference to the instruction grammar entry with the given
    /// `opcode`.
    pub fn get(opcode: spirv::GLOp) -> &'static ExtendedInstruction<'static> {
        GLSL_STD_450_INSTRUCTION_TABLE
            .iter()
            .find(|inst| (inst.opcode == opcode as spirv::Word))
            .expect("internal error")
    }
}

include!("glsl_std_450.rs");

/// The table for all `OpenCLstd100` extended instructions.
///
/// This table is staic data stored in the library.
pub struct OpenCLStd100InstructionTable;

impl OpenCLStd100InstructionTable {
    /// Looks up the given `opcode` in the instruction table and returns
    /// a reference to the instruction grammar entry if found.
    pub fn lookup_opcode(opcode: u32) -> Option<&'static ExtendedInstruction<'static>> {
        OPENCL_STD_100_INSTRUCTION_TABLE.iter().find(|inst| {
            inst.opcode == opcode
        })
    }

    /// Returns a reference to the instruction grammar entry with the given
    /// `opcode`.
    pub fn get(opcode: spirv::CLOp) -> &'static ExtendedInstruction<'static> {
        OPENCL_STD_100_INSTRUCTION_TABLE
            .iter()
            .find(|inst| (inst.opcode == opcode as spirv::Word))
            .expect("internal error")
    }
}

include!("opencl_std_100.rs");
