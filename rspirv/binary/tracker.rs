use std::collections;

use crate::dr;
use crate::grammar;
use crate::spirv;

pub type GExtInstRef = &'static grammar::ExtendedInstruction<'static>;

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
        TypeTracker {
            types: collections::HashMap::new(),
        }
    }

    pub fn track(&mut self, inst: &dr::Instruction) {
        if let Some(rid) = inst.result_id {
            if inst.class.opcode.is_type() {
                match inst.class.opcode {
                    spirv::Op::TypeInt => {
                        if let (
                            &dr::Operand::LiteralBit32(bits),
                            &dr::Operand::LiteralBit32(sign),
                        ) = (&inst.operands[0], &inst.operands[1])
                        {
                            self.types.insert(rid, Type::Integer(bits, sign == 1));
                        }
                    }
                    spirv::Op::TypeFloat => {
                        if let dr::Operand::LiteralBit32(bits) = inst.operands[0] {
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

/// Struct for tracking extended instruction sets.
///
/// If a given extended instruction set is not supported, it will just be
/// silently ignored.
pub struct ExtInstSetTracker {
    sets: collections::HashMap<spirv::Word, &'static grammar::InstructionTable<grammar::ExtInstOp>>,
}

impl ExtInstSetTracker {
    pub fn new() -> ExtInstSetTracker {
        ExtInstSetTracker {
            sets: collections::HashMap::new(),
        }
    }

    /// Tracks the extended instruction set declared by the given `inst`.
    ///
    /// If the given extended instruction set is not recognized, it will
    /// be silently ignored.
    pub fn track(&mut self, inst: &dr::Instruction) {
        if inst.class.opcode != spirv::Op::ExtInstImport || inst.operands.is_empty() {
            return;
        }
        if let dr::Operand::LiteralString(ref s) = inst.operands[0] {
            if let Some(table) = grammar::ext_inst_table(s) {
                self.sets.insert(
                    inst.result_id
                        .expect("Importing extended instructions requires a result_id"),
                    table,
                );
            } else {
                // TODO: Bubble error up
                eprintln!("ERROR: Extended instruction set `{s}` not recognized");
            }
        }
    }

    /// Resolves the extended instruction with `opcode` in set `set`.
    ///
    /// This method will return `None` for both untracked instruction
    /// sets and unknown opcode in tracked instruction sets.
    pub fn resolve(&self, set: spirv::Word, opcode: spirv::Word) -> Option<GExtInstRef> {
        if let Some(ext_inst_set) = self.sets.get(&set) {
            ext_inst_set.lookup_opcode(opcode)
        } else {
            None
        }
    }
}
