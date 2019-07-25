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
    pub fn lift_branch(&mut self, raw: &mr::Instruction) -> Result<Branch, LiftError> {
        let mut operands = raw.operands.iter();
        Ok(match raw.class.opcode as u32 {
            249u32 => Branch::Branch {
                target_label: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            250u32 => Branch::BranchConditional {
                condition: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                true_label: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                false_label: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                branch_weights: {
                    let mut vec = Vec::new();
                    while let Some(value) = match operands.next() {
                        Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                        None => None,
                        Some(_) => Err(OperandError::Wrong)?,
                    } {
                        vec.push(value);
                    }
                    vec
                },
            },
            251u32 => Branch::Switch {
                selector: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                default: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                target: {
                    let mut vec = Vec::new();
                    while let Some(value) = match (operands.next(), operands.next()) {
                        (
                            Some(&mr::Operand::LiteralInt32(value)),
                            Some(&mr::Operand::IdRef(id)),
                        ) => Some((value, Token::new(id))),
                        (None, None) => None,
                        _ => Err(OperandError::Wrong)?,
                    } {
                        vec.push(value);
                    }
                    vec
                },
            },
            253u32 => Branch::Return {},
            254u32 => Branch::ReturnValue {
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            _ => return Err(LiftError::OpCode),
        })
    }
    pub fn lift_terminator(&mut self, _raw: &mr::Instruction) -> Result<Terminator, LiftError> {
        Ok(Terminator::Unreachable)
    }
    pub fn lift_extension(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::Extension, LiftError> {
        if raw.class.opcode as u32 != 10u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::Extension {
            name: (match operands.next() {
                Some(&mr::Operand::LiteralString(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    pub fn lift_ext_inst_import(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::ExtInstImport, LiftError> {
        if raw.class.opcode as u32 != 11u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::ExtInstImport {
            name: (match operands.next() {
                Some(&mr::Operand::LiteralString(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    pub fn lift_memory_model(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::MemoryModel, LiftError> {
        if raw.class.opcode as u32 != 14u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::MemoryModel {
            addressing_model: (match operands.next() {
                Some(&mr::Operand::AddressingModel(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
            memory_model: (match operands.next() {
                Some(&mr::Operand::MemoryModel(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    pub fn lift_entry_point(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::EntryPoint, LiftError> {
        if raw.class.opcode as u32 != 15u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::EntryPoint {
            execution_model: (match operands.next() {
                Some(&mr::Operand::ExecutionModel(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
            entry_point: (match operands.next() {
                Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
            name: (match operands.next() {
                Some(&mr::Operand::LiteralString(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
            interface: {
                let mut vec = Vec::new();
                while let Some(value) = match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                } {
                    vec.push(value);
                }
                vec
            },
        })
    }
    pub fn lift_execution_mode(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::ExecutionMode, LiftError> {
        if raw.class.opcode as u32 != 16u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::ExecutionMode {
            entry_point: (match operands.next() {
                Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
            mode: (match operands.next() {
                Some(&mr::Operand::ExecutionMode(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    pub fn lift_capability(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::Capability, LiftError> {
        if raw.class.opcode as u32 != 17u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::Capability {
            capability: (match operands.next() {
                Some(&mr::Operand::Capability(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    pub fn lift_function(&mut self, raw: &mr::Instruction) -> Result<structs::Function, LiftError> {
        if raw.class.opcode as u32 != 54u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::Function {
            function_control: (match operands.next() {
                Some(&mr::Operand::FunctionControl(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
            function_type: (match operands.next() {
                Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    pub fn lift_function_parameter(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::FunctionParameter, LiftError> {
        if raw.class.opcode as u32 != 55u32 {
            return Err(LiftError::OpCode);
        };
        Ok(structs::FunctionParameter {})
    }
    pub fn lift_function_end(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::FunctionEnd, LiftError> {
        if raw.class.opcode as u32 != 56u32 {
            return Err(LiftError::OpCode);
        };
        Ok(structs::FunctionEnd {})
    }
    pub fn lift_label(&mut self, raw: &mr::Instruction) -> Result<structs::Label, LiftError> {
        if raw.class.opcode as u32 != 248u32 {
            return Err(LiftError::OpCode);
        };
        Ok(structs::Label {})
    }
    pub fn lift_execution_mode_id(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<structs::ExecutionModeId, LiftError> {
        if raw.class.opcode as u32 != 331u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::ExecutionModeId {
            entry_point: (match operands.next() {
                Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
            mode: (match operands.next() {
                Some(&mr::Operand::ExecutionMode(ref value)) => Some(value.clone()),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
}
