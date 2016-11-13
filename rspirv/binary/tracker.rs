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

use mr;
use grammar;
use spirv;

use std::collections;

use grammar::GlslStd450InstructionTable as GGlInstTable;

type GExtInstRef = &'static grammar::ExtendedInstruction<'static>;

enum ExtInstSet {
    GlslStd450,
}

/// Struct for tracking extended instruction sets.
///
/// If a given extended instruction set is not supported, it will just be
/// silently ignored.
pub struct ExtInstSetTracker {
    sets: collections::HashMap<spirv::Word, ExtInstSet>,
}

impl ExtInstSetTracker {
    pub fn new() -> ExtInstSetTracker {
        ExtInstSetTracker { sets: collections::HashMap::new() }
    }

    /// Tracks the extended instruction set declared by the given `inst`.
    ///
    /// If the given extended instruction set is not recognized, it will
    /// be silently ignored.
    pub fn track(&mut self, inst: &mr::Instruction) {
        if inst.class.opcode != spirv::Op::ExtInstImport {
            return;
        }
        if let mr::Operand::LiteralString(ref s) = inst.operands[0] {
            if s == "GLSL.std.450" {
                self.sets.insert(inst.result_id
                                     .expect("internal error: parser should \
                                              have already checked the \
                                              syntax"),
                                 ExtInstSet::GlslStd450);
            }
        } else {
            // The parser should already checked the well-formedness
            // of the OpExtInstImport instruction.
            unreachable!()
        }
    }

    /// Returns true if the given extended instruction `set` has been
    /// recognized thus tracked.
    pub fn have(&self, set: spirv::Word) -> bool {
        self.sets.get(&set).is_some()
    }

    /// Resolves the extended instruction with `opcode` in set `set`.
    ///
    /// This method will return `None` for both untracked instruction
    /// sets and unknown opcode in tracked instruction sets.
    pub fn resolve(&self,
                   set: spirv::Word,
                   opcode: spirv::Word)
                   -> Option<GExtInstRef> {
        if let Some(ext_inst_set) = self.sets.get(&set) {
            match *ext_inst_set {
                ExtInstSet::GlslStd450 => GGlInstTable::lookup_opcode(opcode),
            }
        } else {
            None
        }
    }
}
