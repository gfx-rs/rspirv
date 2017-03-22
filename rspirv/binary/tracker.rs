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
use grammar::OpenCLStd100InstructionTable as GClInstTable;

type GExtInstRef = &'static grammar::ExtendedInstruction<'static>;

// TODO: Add support for other types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// Integer type (size, signed).
    Integer(u32, bool),
    Float(u32),
}

/// Tracks ids to their types.
///
/// If the type of an id cannot be resolved due to some reason, this will
/// silently ignore that id instead of erroring out.
#[derive(Debug)]
pub struct TypeTracker {
    /// Mapping from an id to its type.
    ///
    /// Ids for both defining and using types are all kept here.
    types: collections::HashMap<spirv::Word, Type>,
}

impl TypeTracker {
    pub fn new() -> TypeTracker {
        TypeTracker { types: collections::HashMap::new() }
    }

    pub fn track(&mut self, inst: &mr::Instruction) {
        if let Some(rid) = inst.result_id {
            if grammar::reflect::is_type(inst.class.opcode) {
                match inst.class.opcode {
                    spirv::Op::TypeInt => {
                        if let (&mr::Operand::LiteralInt32(bits),
                                &mr::Operand::LiteralInt32(sign)) = (&inst.operands[0],
                                                                     &inst.operands[1]) {
                            self.types
                                .insert(rid, Type::Integer(bits, sign == 1));
                        }
                    }
                    spirv::Op::TypeFloat => {
                        if let mr::Operand::LiteralInt32(bits) = inst.operands[0] {
                            self.types.insert(rid, Type::Float(bits));
                        }
                    }
                    // TODO: handle the other types here.
                    _ => (),
                }
            } else {
                inst.result_type
                    .and_then(|t| self.resolve(t))
                    .map(|t| self.types.insert(rid, t));
            }
        }
    }

    pub fn resolve(&self, id: spirv::Word) -> Option<Type> {
        self.types.get(&id).cloned()
    }
}

enum ExtInstSet {
    GlslStd450,
    OpenCLStd100,
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
        if inst.class.opcode != spirv::Op::ExtInstImport || inst.result_id.is_none() ||
           inst.operands.is_empty() {
            return;
        }
        if let mr::Operand::LiteralString(ref s) = inst.operands[0] {
            if s == "GLSL.std.450" {
                self.sets
                    .insert(inst.result_id.unwrap(), ExtInstSet::GlslStd450);
            } else if s == "OpenCL.std" {
                self.sets
                    .insert(inst.result_id.unwrap(), ExtInstSet::OpenCLStd100);
            }
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
    pub fn resolve(&self, set: spirv::Word, opcode: spirv::Word) -> Option<GExtInstRef> {
        if let Some(ext_inst_set) = self.sets.get(&set) {
            match *ext_inst_set {
                ExtInstSet::GlslStd450 => GGlInstTable::lookup_opcode(opcode),
                ExtInstSet::OpenCLStd100 => GClInstTable::lookup_opcode(opcode),
            }
        } else {
            None
        }
    }
}
