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
    pub fn lift_terminator(&mut self, raw: &mr::Instruction) -> Result<Terminator, LiftError> {
        Ok(match raw.class.opcode as u32 {
            252u32 => Terminator::Kill {},
            255u32 => Terminator::Unreachable {},
            _ => Terminator::Branch(self.lift_branch(raw)?),
        })
    }
    pub fn lift_instruction(&mut self, raw: &mr::Instruction) -> Result<Instruction, LiftError> {
        let mut operands = raw.operands.iter();
        Ok(match raw.class.opcode as u32 {
            0u32 => Instruction::Nop {},
            1u32 => Instruction::Undef {},
            2u32 => Instruction::SourceContinued {
                continued_source: (match operands.next() {
                    Some(&mr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            3u32 => Instruction::Source {
                source_language: (match operands.next() {
                    Some(&mr::Operand::SourceLanguage(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                version: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                file: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
                source: match operands.next() {
                    Some(&mr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            4u32 => Instruction::SourceExtension {
                extension: (match operands.next() {
                    Some(&mr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5u32 => Instruction::Name {
                target: (match operands.next() {
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
            },
            6u32 => Instruction::MemberName {
                target_type: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                member: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
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
            },
            7u32 => Instruction::String {
                string: (match operands.next() {
                    Some(&mr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            8u32 => Instruction::Line {
                file: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                line: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                column: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            12u32 => Instruction::ExtInst {
                set: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                instruction: (match operands.next() {
                    Some(&mr::Operand::LiteralExtInstInteger(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operands: {
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
            },
            57u32 => Instruction::FunctionCall {
                function: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                arguments: {
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
            },
            59u32 => Instruction::Variable {
                storage_class: (match operands.next() {
                    Some(&mr::Operand::StorageClass(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                initializer: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            60u32 => Instruction::ImageTexelPointer {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                sample: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            61u32 => Instruction::Load {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&mr::Operand::MemoryAccess(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            62u32 => Instruction::Store {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                object: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&mr::Operand::MemoryAccess(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            63u32 => Instruction::CopyMemory {
                target: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&mr::Operand::MemoryAccess(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            64u32 => Instruction::CopyMemorySized {
                target: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&mr::Operand::MemoryAccess(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            65u32 => Instruction::AccessChain {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
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
            },
            66u32 => Instruction::InBoundsAccessChain {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
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
            },
            67u32 => Instruction::PtrAccessChain {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                element: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
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
            },
            68u32 => Instruction::ArrayLength {
                structure: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                array_member: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            69u32 => Instruction::GenericPtrMemSemantics {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            70u32 => Instruction::InBoundsPtrAccessChain {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                element: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
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
            },
            71u32 => Instruction::Decorate {
                target: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&mr::Operand::Decoration(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            72u32 => Instruction::MemberDecorate {
                structure_type: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                member: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&mr::Operand::Decoration(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            73u32 => Instruction::DecorationGroup {},
            74u32 => Instruction::GroupDecorate {
                decoration_group: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                targets: {
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
            },
            75u32 => Instruction::GroupMemberDecorate {
                decoration_group: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                targets: {
                    let mut vec = Vec::new();
                    while let Some(value) = match (operands.next(), operands.next()) {
                        (
                            Some(&mr::Operand::IdRef(id)),
                            Some(&mr::Operand::LiteralInt32(value)),
                        ) => Some((Token::new(id), value)),
                        (None, None) => None,
                        _ => Err(OperandError::Wrong)?,
                    } {
                        vec.push(value);
                    }
                    vec
                },
            },
            77u32 => Instruction::VectorExtractDynamic {
                vector: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            78u32 => Instruction::VectorInsertDynamic {
                vector: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            79u32 => Instruction::VectorShuffle {
                vector_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                components: {
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
            80u32 => Instruction::CompositeConstruct {
                constituents: {
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
            },
            81u32 => Instruction::CompositeExtract {
                composite: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
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
            82u32 => Instruction::CompositeInsert {
                object: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                composite: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
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
            83u32 => Instruction::CopyObject {
                operand: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            84u32 => Instruction::Transpose {
                matrix: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            86u32 => Instruction::SampledImage {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                sampler: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            87u32 => Instruction::ImageSampleImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            88u32 => Instruction::ImageSampleExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            89u32 => Instruction::ImageSampleDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            90u32 => Instruction::ImageSampleDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            91u32 => Instruction::ImageSampleProjImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            92u32 => Instruction::ImageSampleProjExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            93u32 => Instruction::ImageSampleProjDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            94u32 => Instruction::ImageSampleProjDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            95u32 => Instruction::ImageFetch {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            96u32 => Instruction::ImageGather {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            97u32 => Instruction::ImageDrefGather {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            98u32 => Instruction::ImageRead {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            99u32 => Instruction::ImageWrite {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                texel: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            100u32 => Instruction::Image {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            101u32 => Instruction::ImageQueryFormat {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            102u32 => Instruction::ImageQueryOrder {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            103u32 => Instruction::ImageQuerySizeLod {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                level_of_detail: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            104u32 => Instruction::ImageQuerySize {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            105u32 => Instruction::ImageQueryLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            106u32 => Instruction::ImageQueryLevels {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            107u32 => Instruction::ImageQuerySamples {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            109u32 => Instruction::ConvertFToU {
                float_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            110u32 => Instruction::ConvertFToS {
                float_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            111u32 => Instruction::ConvertSToF {
                signed_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            112u32 => Instruction::ConvertUToF {
                unsigned_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            113u32 => Instruction::UConvert {
                unsigned_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            114u32 => Instruction::SConvert {
                signed_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            115u32 => Instruction::FConvert {
                float_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            116u32 => Instruction::QuantizeToF16 {
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            117u32 => Instruction::ConvertPtrToU {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            118u32 => Instruction::SatConvertSToU {
                signed_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            119u32 => Instruction::SatConvertUToS {
                unsigned_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            120u32 => Instruction::ConvertUToPtr {
                integer_value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            121u32 => Instruction::PtrCastToGeneric {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            122u32 => Instruction::GenericCastToPtr {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            123u32 => Instruction::GenericCastToPtrExplicit {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                storage: (match operands.next() {
                    Some(&mr::Operand::StorageClass(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            124u32 => Instruction::Bitcast {
                operand: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            126u32 => Instruction::SNegate {
                operand: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            127u32 => Instruction::FNegate {
                operand: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            128u32 => Instruction::IAdd {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            129u32 => Instruction::FAdd {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            130u32 => Instruction::ISub {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            131u32 => Instruction::FSub {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            132u32 => Instruction::IMul {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            133u32 => Instruction::FMul {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            134u32 => Instruction::UDiv {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            135u32 => Instruction::SDiv {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            136u32 => Instruction::FDiv {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            137u32 => Instruction::UMod {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            138u32 => Instruction::SRem {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            139u32 => Instruction::SMod {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            140u32 => Instruction::FRem {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            141u32 => Instruction::FMod {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            142u32 => Instruction::VectorTimesScalar {
                vector: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scalar: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            143u32 => Instruction::MatrixTimesScalar {
                matrix: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scalar: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            144u32 => Instruction::VectorTimesMatrix {
                vector: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                matrix: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            145u32 => Instruction::MatrixTimesVector {
                matrix: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                vector: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            146u32 => Instruction::MatrixTimesMatrix {
                left_matrix: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                right_matrix: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            147u32 => Instruction::OuterProduct {
                vector_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            148u32 => Instruction::Dot {
                vector_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            149u32 => Instruction::IAddCarry {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            150u32 => Instruction::ISubBorrow {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            151u32 => Instruction::UMulExtended {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            152u32 => Instruction::SMulExtended {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            154u32 => Instruction::Any {
                vector: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            155u32 => Instruction::All {
                vector: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            156u32 => Instruction::IsNan {
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            157u32 => Instruction::IsInf {
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            158u32 => Instruction::IsFinite {
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            159u32 => Instruction::IsNormal {
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            160u32 => Instruction::SignBitSet {
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            161u32 => Instruction::LessOrGreater {
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            162u32 => Instruction::Ordered {
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            163u32 => Instruction::Unordered {
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            164u32 => Instruction::LogicalEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            165u32 => Instruction::LogicalNotEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            166u32 => Instruction::LogicalOr {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            167u32 => Instruction::LogicalAnd {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            168u32 => Instruction::LogicalNot {
                operand: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            169u32 => Instruction::Select {
                condition: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                object_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                object_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            170u32 => Instruction::IEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            171u32 => Instruction::INotEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            172u32 => Instruction::UGreaterThan {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            173u32 => Instruction::SGreaterThan {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            174u32 => Instruction::UGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            175u32 => Instruction::SGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            176u32 => Instruction::ULessThan {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            177u32 => Instruction::SLessThan {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            178u32 => Instruction::ULessThanEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            179u32 => Instruction::SLessThanEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            180u32 => Instruction::FOrdEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            181u32 => Instruction::FUnordEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            182u32 => Instruction::FOrdNotEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            183u32 => Instruction::FUnordNotEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            184u32 => Instruction::FOrdLessThan {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            185u32 => Instruction::FUnordLessThan {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            186u32 => Instruction::FOrdGreaterThan {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            187u32 => Instruction::FUnordGreaterThan {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            188u32 => Instruction::FOrdLessThanEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            189u32 => Instruction::FUnordLessThanEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            190u32 => Instruction::FOrdGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            191u32 => Instruction::FUnordGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            194u32 => Instruction::ShiftRightLogical {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            195u32 => Instruction::ShiftRightArithmetic {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            196u32 => Instruction::ShiftLeftLogical {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            197u32 => Instruction::BitwiseOr {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            198u32 => Instruction::BitwiseXor {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            199u32 => Instruction::BitwiseAnd {
                operand_1: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            200u32 => Instruction::Not {
                operand: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            201u32 => Instruction::BitFieldInsert {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                insert: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            202u32 => Instruction::BitFieldSExtract {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            203u32 => Instruction::BitFieldUExtract {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            204u32 => Instruction::BitReverse {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            205u32 => Instruction::BitCount {
                base: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            207u32 => Instruction::DPdx {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            208u32 => Instruction::DPdy {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            209u32 => Instruction::Fwidth {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            210u32 => Instruction::DPdxFine {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            211u32 => Instruction::DPdyFine {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            212u32 => Instruction::FwidthFine {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            213u32 => Instruction::DPdxCoarse {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            214u32 => Instruction::DPdyCoarse {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            215u32 => Instruction::FwidthCoarse {
                p: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            218u32 => Instruction::EmitVertex {},
            219u32 => Instruction::EndPrimitive {},
            220u32 => Instruction::EmitStreamVertex {
                stream: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            221u32 => Instruction::EndStreamPrimitive {
                stream: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            224u32 => Instruction::ControlBarrier {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            225u32 => Instruction::MemoryBarrier {
                memory: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            227u32 => Instruction::AtomicLoad {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            228u32 => Instruction::AtomicStore {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            229u32 => Instruction::AtomicExchange {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            230u32 => Instruction::AtomicCompareExchange {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                equal: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                unequal: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                comparator: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            231u32 => Instruction::AtomicCompareExchangeWeak {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                equal: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                unequal: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                comparator: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            232u32 => Instruction::AtomicIIncrement {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            233u32 => Instruction::AtomicIDecrement {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            234u32 => Instruction::AtomicIAdd {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            235u32 => Instruction::AtomicISub {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            236u32 => Instruction::AtomicSMin {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            237u32 => Instruction::AtomicUMin {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            238u32 => Instruction::AtomicSMax {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            239u32 => Instruction::AtomicUMax {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            240u32 => Instruction::AtomicAnd {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            241u32 => Instruction::AtomicOr {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            242u32 => Instruction::AtomicXor {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            246u32 => Instruction::LoopMerge {
                merge_block: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                continue_target: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                loop_control: (match operands.next() {
                    Some(&mr::Operand::LoopControl(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            247u32 => Instruction::SelectionMerge {
                merge_block: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                selection_control: (match operands.next() {
                    Some(&mr::Operand::SelectionControl(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            256u32 => Instruction::LifetimeStart {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            257u32 => Instruction::LifetimeStop {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            259u32 => Instruction::GroupAsyncCopy {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                destination: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                num_elements: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                stride: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                event: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            260u32 => Instruction::GroupWaitEvents {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                events_list: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            261u32 => Instruction::GroupAll {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            262u32 => Instruction::GroupAny {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            263u32 => Instruction::GroupBroadcast {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                local_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            264u32 => Instruction::GroupIAdd {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            265u32 => Instruction::GroupFAdd {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            266u32 => Instruction::GroupFMin {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            267u32 => Instruction::GroupUMin {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            268u32 => Instruction::GroupSMin {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            269u32 => Instruction::GroupFMax {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            270u32 => Instruction::GroupUMax {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            271u32 => Instruction::GroupSMax {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            274u32 => Instruction::ReadPipe {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            275u32 => Instruction::WritePipe {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            276u32 => Instruction::ReservedReadPipe {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            277u32 => Instruction::ReservedWritePipe {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            278u32 => Instruction::ReserveReadPipePackets {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            279u32 => Instruction::ReserveWritePipePackets {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            280u32 => Instruction::CommitReadPipe {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            281u32 => Instruction::CommitWritePipe {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            282u32 => Instruction::IsValidReserveId {
                reserve_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            283u32 => Instruction::GetNumPipePackets {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            284u32 => Instruction::GetMaxPipePackets {
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            285u32 => Instruction::GroupReserveReadPipePackets {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            286u32 => Instruction::GroupReserveWritePipePackets {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            287u32 => Instruction::GroupCommitReadPipe {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            288u32 => Instruction::GroupCommitWritePipe {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            291u32 => Instruction::EnqueueMarker {
                queue: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                wait_events: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                ret_event: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            292u32 => Instruction::EnqueueKernel {
                queue: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                flags: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                nd_range: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                wait_events: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                ret_event: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                local_size: {
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
            },
            293u32 => Instruction::GetKernelNDrangeSubGroupCount {
                nd_range: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            294u32 => Instruction::GetKernelNDrangeMaxSubGroupSize {
                nd_range: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            295u32 => Instruction::GetKernelWorkGroupSize {
                invoke: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            296u32 => Instruction::GetKernelPreferredWorkGroupSizeMultiple {
                invoke: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            297u32 => Instruction::RetainEvent {
                event: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            298u32 => Instruction::ReleaseEvent {
                event: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            299u32 => Instruction::CreateUserEvent {},
            300u32 => Instruction::IsValidEvent {
                event: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            301u32 => Instruction::SetUserEventStatus {
                event: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                status: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            302u32 => Instruction::CaptureEventProfilingInfo {
                event: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                profiling_info: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            303u32 => Instruction::GetDefaultQueue {},
            304u32 => Instruction::BuildNDRange {
                global_work_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                local_work_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                global_work_offset: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            305u32 => Instruction::ImageSparseSampleImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            306u32 => Instruction::ImageSparseSampleExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            307u32 => Instruction::ImageSparseSampleDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            308u32 => Instruction::ImageSparseSampleDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            309u32 => Instruction::ImageSparseSampleProjImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            310u32 => Instruction::ImageSparseSampleProjExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            311u32 => Instruction::ImageSparseSampleProjDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            312u32 => Instruction::ImageSparseSampleProjDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            313u32 => Instruction::ImageSparseFetch {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            314u32 => Instruction::ImageSparseGather {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            315u32 => Instruction::ImageSparseDrefGather {
                sampled_image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                dref: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            316u32 => Instruction::ImageSparseTexelsResident {
                resident_code: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            317u32 => Instruction::NoLine {},
            318u32 => Instruction::AtomicFlagTestAndSet {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            319u32 => Instruction::AtomicFlagClear {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            320u32 => Instruction::ImageSparseRead {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&mr::Operand::ImageOperands(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            321u32 => Instruction::SizeOf {
                pointer: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            324u32 => Instruction::CreatePipeFromPipeStorage {
                pipe_storage: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            325u32 => Instruction::GetKernelLocalSizeForSubgroupCount {
                subgroup_count: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            326u32 => Instruction::GetKernelMaxNumSubgroups {
                invoke: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            328u32 => Instruction::NamedBarrierInitialize {
                subgroup_count: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            329u32 => Instruction::MemoryNamedBarrier {
                named_barrier: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&mr::Operand::IdMemorySemantics(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            330u32 => Instruction::ModuleProcessed {
                process: (match operands.next() {
                    Some(&mr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            332u32 => Instruction::DecorateId {
                target: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&mr::Operand::Decoration(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            333u32 => Instruction::GroupNonUniformElect {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            334u32 => Instruction::GroupNonUniformAll {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            335u32 => Instruction::GroupNonUniformAny {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            336u32 => Instruction::GroupNonUniformAllEqual {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            337u32 => Instruction::GroupNonUniformBroadcast {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            338u32 => Instruction::GroupNonUniformBroadcastFirst {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            339u32 => Instruction::GroupNonUniformBallot {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            340u32 => Instruction::GroupNonUniformInverseBallot {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            341u32 => Instruction::GroupNonUniformBallotBitExtract {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            342u32 => Instruction::GroupNonUniformBallotBitCount {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            343u32 => Instruction::GroupNonUniformBallotFindLSB {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            344u32 => Instruction::GroupNonUniformBallotFindMSB {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            345u32 => Instruction::GroupNonUniformShuffle {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            346u32 => Instruction::GroupNonUniformShuffleXor {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                mask: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            347u32 => Instruction::GroupNonUniformShuffleUp {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            348u32 => Instruction::GroupNonUniformShuffleDown {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            349u32 => Instruction::GroupNonUniformIAdd {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            350u32 => Instruction::GroupNonUniformFAdd {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            351u32 => Instruction::GroupNonUniformIMul {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            352u32 => Instruction::GroupNonUniformFMul {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            353u32 => Instruction::GroupNonUniformSMin {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            354u32 => Instruction::GroupNonUniformUMin {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            355u32 => Instruction::GroupNonUniformFMin {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            356u32 => Instruction::GroupNonUniformSMax {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            357u32 => Instruction::GroupNonUniformUMax {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            358u32 => Instruction::GroupNonUniformFMax {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            359u32 => Instruction::GroupNonUniformBitwiseAnd {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            360u32 => Instruction::GroupNonUniformBitwiseOr {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            361u32 => Instruction::GroupNonUniformBitwiseXor {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            362u32 => Instruction::GroupNonUniformLogicalAnd {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            363u32 => Instruction::GroupNonUniformLogicalOr {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            364u32 => Instruction::GroupNonUniformLogicalXor {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                },
            },
            365u32 => Instruction::GroupNonUniformQuadBroadcast {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            366u32 => Instruction::GroupNonUniformQuadSwap {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            4421u32 => Instruction::SubgroupBallotKHR {
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            4422u32 => Instruction::SubgroupFirstInvocationKHR {
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            4428u32 => Instruction::SubgroupAllKHR {
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            4429u32 => Instruction::SubgroupAnyKHR {
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            4430u32 => Instruction::SubgroupAllEqualKHR {
                predicate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            4432u32 => Instruction::SubgroupReadInvocationKHR {
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5000u32 => Instruction::GroupIAddNonUniformAMD {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5001u32 => Instruction::GroupFAddNonUniformAMD {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5002u32 => Instruction::GroupFMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5003u32 => Instruction::GroupUMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5004u32 => Instruction::GroupSMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5005u32 => Instruction::GroupFMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5006u32 => Instruction::GroupUMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5007u32 => Instruction::GroupSMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(&mr::Operand::IdScope(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&mr::Operand::GroupOperation(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5011u32 => Instruction::FragmentMaskFetchAMD {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5012u32 => Instruction::FragmentFetchAMD {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                fragment_index: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5571u32 => Instruction::SubgroupShuffleINTEL {
                data: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                invocation_id: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5572u32 => Instruction::SubgroupShuffleDownINTEL {
                current: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                next: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5573u32 => Instruction::SubgroupShuffleUpINTEL {
                previous: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                current: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5574u32 => Instruction::SubgroupShuffleXorINTEL {
                data: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5575u32 => Instruction::SubgroupBlockReadINTEL {
                ptr: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5576u32 => Instruction::SubgroupBlockWriteINTEL {
                ptr: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                data: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5577u32 => Instruction::SubgroupImageBlockReadINTEL {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5578u32 => Instruction::SubgroupImageBlockWriteINTEL {
                image: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                data: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5632u32 => Instruction::DecorateStringGOOGLE {
                target: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&mr::Operand::Decoration(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5633u32 => Instruction::MemberDecorateStringGOOGLE {
                struct_type: (match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                member: (match operands.next() {
                    Some(&mr::Operand::LiteralInt32(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&mr::Operand::Decoration(ref value)) => Some(value.clone()),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                })
                .ok_or(OperandError::Missing)?,
            },
            5296u32 => Instruction::GroupNonUniformPartitionNV {
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
    pub fn lift_phi(&mut self, raw: &mr::Instruction) -> Result<structs::Phi, LiftError> {
        if raw.class.opcode as u32 != 245u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(structs::Phi {
            value_label_pairs: {
                let mut vec = Vec::new();
                while let Some(value) = match (operands.next(), operands.next()) {
                    (Some(&mr::Operand::IdRef(id1)), Some(&mr::Operand::IdRef(id2))) => {
                        Some((Token::new(id1), Token::new(id2)))
                    }
                    (None, None) => None,
                    _ => Err(OperandError::Wrong)?,
                } {
                    vec.push(value);
                }
                vec
            },
        })
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
