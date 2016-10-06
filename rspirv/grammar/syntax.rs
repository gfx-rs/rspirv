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
#[derive(Debug)]
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

/// Grammar for a SPIR-V logical operand.
#[derive(Debug)]
pub struct LogicalOperand {
    /// The kind of this logical operand.
    pub kind: OperandKind,
    /// The repeat specification for this logical operand.
    pub quantifier: OperandQuantifier,
}

/// The repeat specification for a SPIR-V logical operand.
#[derive(Clone, Copy, Debug)]
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

/// The table for all SPIR-V instructions.
///
/// This table is staic data stored in the library.
pub struct InstructionTable;

impl InstructionTable {
    /// Looks up the given `opcode` in the instruction table and returns
    /// a reference to the instruction grammar entry if found.
    pub fn lookup_opcode(opcode: u16) -> Option<&'static Instruction<'static>> {
        INSTRUCTION_TABLE.iter().find(|&inst| (inst.opcode as u16) == opcode)
    }
}

include!("table.rs");
