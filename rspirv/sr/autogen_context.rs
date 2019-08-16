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

impl Context {
    #[allow(unused)]
    pub(in crate::sr) fn lift_extension(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Extension, InstructionError> {
        if raw.class.opcode as u32 != 10u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Extension {
            name: (match operands.next() {
                Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_ext_inst_import(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExtInstImport, InstructionError> {
        if raw.class.opcode as u32 != 11u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExtInstImport {
            name: (match operands.next() {
                Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_memory_model(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::MemoryModel, InstructionError> {
        if raw.class.opcode as u32 != 14u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::MemoryModel {
            addressing_model: (match operands.next() {
                Some(&dr::Operand::AddressingModel(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            memory_model: (match operands.next() {
                Some(&dr::Operand::MemoryModel(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_entry_point(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::EntryPoint, InstructionError> {
        if raw.class.opcode as u32 != 15u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::EntryPoint {
            execution_model: (match operands.next() {
                Some(&dr::Operand::ExecutionModel(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            entry_point: (match operands.next() {
                Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            name: (match operands.next() {
                Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            interface: {
                let mut vec = Vec::new();
                while let Some(value) = match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                } {
                    vec.push(value);
                }
                vec
            },
        })
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_execution_mode(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExecutionMode, InstructionError> {
        if raw.class.opcode as u32 != 16u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExecutionMode {
            entry_point: (match operands.next() {
                Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            mode: (match operands.next() {
                Some(&dr::Operand::ExecutionMode(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_capability(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Capability, InstructionError> {
        if raw.class.opcode as u32 != 17u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Capability {
            capability: (match operands.next() {
                Some(&dr::Operand::Capability(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_function(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Function, InstructionError> {
        if raw.class.opcode as u32 != 54u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Function {
            function_control: (match operands.next() {
                Some(&dr::Operand::FunctionControl(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            function_type: (match operands.next() {
                Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup(*value).unwrap()),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_function_parameter(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::FunctionParameter, InstructionError> {
        if raw.class.opcode as u32 != 55u32 {
            return Err(InstructionError::WrongOpcode);
        }
        Ok(instructions::FunctionParameter {})
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_function_end(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::FunctionEnd, InstructionError> {
        if raw.class.opcode as u32 != 56u32 {
            return Err(InstructionError::WrongOpcode);
        }
        Ok(instructions::FunctionEnd {})
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_label(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Label, InstructionError> {
        if raw.class.opcode as u32 != 248u32 {
            return Err(InstructionError::WrongOpcode);
        }
        Ok(instructions::Label {})
    }
    #[allow(unused)]
    pub(in crate::sr) fn lift_execution_mode_id(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExecutionModeId, InstructionError> {
        if raw.class.opcode as u32 != 331u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExecutionModeId {
            entry_point: (match operands.next() {
                Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            mode: (match operands.next() {
                Some(&dr::Operand::ExecutionMode(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
}
