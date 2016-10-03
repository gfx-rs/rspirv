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

#[derive(Debug)]
pub struct Instruction<'a> {
    pub opname: &'a str,
    pub opcode: spirv::Op,
    pub capabilities: &'a [spirv::Capability],
    pub operands: &'a [LogicalOperand],
}

#[derive(Debug)]
pub struct LogicalOperand {
    pub kind: OperandKind,
    pub quantifier: OperandQuantifier,
}

#[derive(Clone, Copy, Debug)]
pub enum OperandQuantifier {
    One,
    ZeroOrOne,
    ZeroOrMore,
}

macro_rules! inst {
    ($op:ident, [$( $cap:ident ),*], [$( ($kind:ident, $quantifier:ident) ),*]) => {
        Instruction {
            opname: stringify!($op),
            opcode: spirv::Op::$op,
            capabilities: &[
                $( spirv::Capability::$cap ),*
            ],
            operands: &[
                $( LogicalOperand {
                    kind: OperandKind::$kind,
                    quantifier: OperandQuantifier::$quantifier }
                ),*
            ],
        }
    }
}

pub struct InstructionTable;

impl InstructionTable {
    pub fn lookup_opcode(opcode: u16) -> Option<&'static Instruction<'static>> {
        INSTRUCTION_TABLE.iter().find(|&inst| (inst.opcode as u16) == opcode)
    }
}

include!("table.rs");
