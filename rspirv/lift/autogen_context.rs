// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl LiftContext {
    pub fn lift_branch(&mut self, raw: &dr::Instruction) -> Result<ops::Branch, InstructionError> {
        let mut operands = raw.operands.iter();
        match raw.class.opcode as u32 {
            245u32 => Ok(ops::Branch::Phi {
                variable_parent: {
                    let mut vec = Vec::new();
                    while let Some(item) = match (operands.next(), operands.next()) {
                        (Some(&dr::Operand::IdRef(first)), Some(&dr::Operand::IdRef(second))) => {
                            Some((first, second))
                        }
                        (None, None) => None,
                        _ => return Err(OperandError::WrongType.into()),
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            246u32 => Ok(ops::Branch::LoopMerge {
                merge_block: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                continue_target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                loop_control: (match operands.next() {
                    Some(dr::Operand::LoopControl(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            247u32 => Ok(ops::Branch::SelectionMerge {
                merge_block: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                selection_control: (match operands.next() {
                    Some(dr::Operand::SelectionControl(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            248u32 => Ok(ops::Branch::Label),
            249u32 => Ok(ops::Branch::Branch {
                target_label: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            250u32 => Ok(ops::Branch::BranchConditional {
                condition: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                true_label: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                false_label: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                branch_weights: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            251u32 => Ok(ops::Branch::Switch {
                selector: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                default: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                target: {
                    let mut vec = Vec::new();
                    while let Some(item) = match (operands.next(), operands.next()) {
                        (
                            Some(&dr::Operand::LiteralBit32(first)),
                            Some(&dr::Operand::IdRef(second)),
                        ) => Some((first, self.lookup_jump(second))),
                        (None, None) => None,
                        _ => return Err(OperandError::WrongType.into()),
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            252u32 => Ok(ops::Branch::Kill),
            253u32 => Ok(ops::Branch::Return),
            254u32 => Ok(ops::Branch::ReturnValue {
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            255u32 => Ok(ops::Branch::Unreachable),
            256u32 => Ok(ops::Branch::LifetimeStart {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            257u32 => Ok(ops::Branch::LifetimeStop {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4416u32 => Ok(ops::Branch::TerminateInvocation),
            5380u32 => Ok(ops::Branch::DemoteToHelperInvocation),
            _ => Err(InstructionError::WrongOpcode),
        }
    }
    pub fn lift_terminator(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<ops::Terminator, InstructionError> {
        let mut operands = raw.operands.iter();
        match raw.class.opcode as u32 {
            4448u32 => Ok(ops::Terminator::IgnoreIntersectionKHR),
            4449u32 => Ok(ops::Terminator::TerminateRayKHR),
            5294u32 => Ok(ops::Terminator::EmitMeshTasksEXT {
                group_count_x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                group_count_y: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                group_count_z: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            _ => self.lift_branch(raw).map(ops::Terminator::Branch),
        }
    }
    #[allow(unreachable_patterns)]
    pub fn lift_op(&mut self, raw: &dr::Instruction) -> Result<ops::Op, InstructionError> {
        let mut operands = raw.operands.iter();
        match raw.class.opcode as u32 {
            0u32 => Ok(ops::Op::Nop),
            1u32 => Ok(ops::Op::Undef),
            2u32 => Ok(ops::Op::SourceContinued {
                continued_source: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            3u32 => Ok(ops::Op::Source {
                source_language: (match operands.next() {
                    Some(dr::Operand::SourceLanguage(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                version: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                file: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
                source: match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4u32 => Ok(ops::Op::SourceExtension {
                extension: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5u32 => Ok(ops::Op::Name {
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                name: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6u32 => Ok(ops::Op::MemberName {
                ty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                member: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                name: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            7u32 => Ok(ops::Op::String {
                string: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            8u32 => Ok(ops::Op::Line {
                file: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                line: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                column: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            59u32 => Ok(ops::Op::Variable {
                storage_class: (match operands.next() {
                    Some(dr::Operand::StorageClass(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                initializer: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            60u32 => Ok(ops::Op::ImageTexelPointer {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sample: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            61u32 => Ok(ops::Op::Load {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            62u32 => Ok(ops::Op::Store {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            63u32 => Ok(ops::Op::CopyMemory {
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
                memory_access_2: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            64u32 => Ok(ops::Op::CopyMemorySized {
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
                memory_access_2: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            65u32 => Ok(ops::Op::AccessChain {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            66u32 => Ok(ops::Op::InBoundsAccessChain {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            67u32 => Ok(ops::Op::PtrAccessChain {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                element: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            68u32 => Ok(ops::Op::ArrayLength {
                structure: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                array_member: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            69u32 => Ok(ops::Op::GenericPtrMemSemantics {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            70u32 => Ok(ops::Op::InBoundsPtrAccessChain {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                element: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            71u32 => Ok(ops::Op::Decorate {
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(dr::Operand::Decoration(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            72u32 => Ok(ops::Op::MemberDecorate {
                structure_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                member: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(dr::Operand::Decoration(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            73u32 => Ok(ops::Op::DecorationGroup),
            74u32 => Ok(ops::Op::GroupDecorate {
                decoration_group: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                targets: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            75u32 => Ok(ops::Op::GroupMemberDecorate {
                decoration_group: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                targets: {
                    let mut vec = Vec::new();
                    while let Some(item) = match (operands.next(), operands.next()) {
                        (
                            Some(&dr::Operand::IdRef(first)),
                            Some(&dr::Operand::LiteralBit32(second)),
                        ) => Some((self.lookup_jump(first), second)),
                        (None, None) => None,
                        _ => return Err(OperandError::WrongType.into()),
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            77u32 => Ok(ops::Op::VectorExtractDynamic {
                vector: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            78u32 => Ok(ops::Op::VectorInsertDynamic {
                vector: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            79u32 => Ok(ops::Op::VectorShuffle {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                components: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            80u32 => Ok(ops::Op::CompositeConstruct {
                constituents: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            81u32 => Ok(ops::Op::CompositeExtract {
                composite: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            82u32 => Ok(ops::Op::CompositeInsert {
                object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                composite: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            83u32 => Ok(ops::Op::CopyObject {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            84u32 => Ok(ops::Op::Transpose {
                matrix: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            86u32 => Ok(ops::Op::SampledImage {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sampler: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            87u32 => Ok(ops::Op::ImageSampleImplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            88u32 => Ok(ops::Op::ImageSampleExplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            89u32 => Ok(ops::Op::ImageSampleDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            90u32 => Ok(ops::Op::ImageSampleDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            91u32 => Ok(ops::Op::ImageSampleProjImplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            92u32 => Ok(ops::Op::ImageSampleProjExplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            93u32 => Ok(ops::Op::ImageSampleProjDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            94u32 => Ok(ops::Op::ImageSampleProjDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            95u32 => Ok(ops::Op::ImageFetch {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            96u32 => Ok(ops::Op::ImageGather {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            97u32 => Ok(ops::Op::ImageDrefGather {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            98u32 => Ok(ops::Op::ImageRead {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            99u32 => Ok(ops::Op::ImageWrite {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                texel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            100u32 => Ok(ops::Op::Image {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            101u32 => Ok(ops::Op::ImageQueryFormat {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            102u32 => Ok(ops::Op::ImageQueryOrder {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            103u32 => Ok(ops::Op::ImageQuerySizeLod {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                level_of_detail: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            104u32 => Ok(ops::Op::ImageQuerySize {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            105u32 => Ok(ops::Op::ImageQueryLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            106u32 => Ok(ops::Op::ImageQueryLevels {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            107u32 => Ok(ops::Op::ImageQuerySamples {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            109u32 => Ok(ops::Op::ConvertFToU {
                float_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            110u32 => Ok(ops::Op::ConvertFToS {
                float_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            111u32 => Ok(ops::Op::ConvertSToF {
                signed_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            112u32 => Ok(ops::Op::ConvertUToF {
                unsigned_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            113u32 => Ok(ops::Op::UConvert {
                unsigned_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            114u32 => Ok(ops::Op::SConvert {
                signed_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            115u32 => Ok(ops::Op::FConvert {
                float_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            116u32 => Ok(ops::Op::QuantizeToF16 {
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            117u32 => Ok(ops::Op::ConvertPtrToU {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            118u32 => Ok(ops::Op::SatConvertSToU {
                signed_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            119u32 => Ok(ops::Op::SatConvertUToS {
                unsigned_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            120u32 => Ok(ops::Op::ConvertUToPtr {
                integer_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            121u32 => Ok(ops::Op::PtrCastToGeneric {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            122u32 => Ok(ops::Op::GenericCastToPtr {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            123u32 => Ok(ops::Op::GenericCastToPtrExplicit {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                storage: (match operands.next() {
                    Some(dr::Operand::StorageClass(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            124u32 => Ok(ops::Op::Bitcast {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            126u32 => Ok(ops::Op::SNegate {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            127u32 => Ok(ops::Op::FNegate {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            128u32 => Ok(ops::Op::IAdd {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            129u32 => Ok(ops::Op::FAdd {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            130u32 => Ok(ops::Op::ISub {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            131u32 => Ok(ops::Op::FSub {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            132u32 => Ok(ops::Op::IMul {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            133u32 => Ok(ops::Op::FMul {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            134u32 => Ok(ops::Op::UDiv {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            135u32 => Ok(ops::Op::SDiv {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            136u32 => Ok(ops::Op::FDiv {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            137u32 => Ok(ops::Op::UMod {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            138u32 => Ok(ops::Op::SRem {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            139u32 => Ok(ops::Op::SMod {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            140u32 => Ok(ops::Op::FRem {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            141u32 => Ok(ops::Op::FMod {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            142u32 => Ok(ops::Op::VectorTimesScalar {
                vector: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                scalar: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            143u32 => Ok(ops::Op::MatrixTimesScalar {
                matrix: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                scalar: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            144u32 => Ok(ops::Op::VectorTimesMatrix {
                vector: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                matrix: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            145u32 => Ok(ops::Op::MatrixTimesVector {
                matrix: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            146u32 => Ok(ops::Op::MatrixTimesMatrix {
                left_matrix: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                right_matrix: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            147u32 => Ok(ops::Op::OuterProduct {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            148u32 => Ok(ops::Op::Dot {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            149u32 => Ok(ops::Op::IAddCarry {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            150u32 => Ok(ops::Op::ISubBorrow {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            151u32 => Ok(ops::Op::UMulExtended {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            152u32 => Ok(ops::Op::SMulExtended {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            154u32 => Ok(ops::Op::Any {
                vector: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            155u32 => Ok(ops::Op::All {
                vector: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            156u32 => Ok(ops::Op::IsNan {
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            157u32 => Ok(ops::Op::IsInf {
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            158u32 => Ok(ops::Op::IsFinite {
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            159u32 => Ok(ops::Op::IsNormal {
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            160u32 => Ok(ops::Op::SignBitSet {
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            161u32 => Ok(ops::Op::LessOrGreater {
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            162u32 => Ok(ops::Op::Ordered {
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            163u32 => Ok(ops::Op::Unordered {
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            164u32 => Ok(ops::Op::LogicalEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            165u32 => Ok(ops::Op::LogicalNotEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            166u32 => Ok(ops::Op::LogicalOr {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            167u32 => Ok(ops::Op::LogicalAnd {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            168u32 => Ok(ops::Op::LogicalNot {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            169u32 => Ok(ops::Op::Select {
                condition: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            170u32 => Ok(ops::Op::IEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            171u32 => Ok(ops::Op::INotEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            172u32 => Ok(ops::Op::UGreaterThan {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            173u32 => Ok(ops::Op::SGreaterThan {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            174u32 => Ok(ops::Op::UGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            175u32 => Ok(ops::Op::SGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            176u32 => Ok(ops::Op::ULessThan {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            177u32 => Ok(ops::Op::SLessThan {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            178u32 => Ok(ops::Op::ULessThanEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            179u32 => Ok(ops::Op::SLessThanEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            180u32 => Ok(ops::Op::FOrdEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            181u32 => Ok(ops::Op::FUnordEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            182u32 => Ok(ops::Op::FOrdNotEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            183u32 => Ok(ops::Op::FUnordNotEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            184u32 => Ok(ops::Op::FOrdLessThan {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            185u32 => Ok(ops::Op::FUnordLessThan {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            186u32 => Ok(ops::Op::FOrdGreaterThan {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            187u32 => Ok(ops::Op::FUnordGreaterThan {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            188u32 => Ok(ops::Op::FOrdLessThanEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            189u32 => Ok(ops::Op::FUnordLessThanEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            190u32 => Ok(ops::Op::FOrdGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            191u32 => Ok(ops::Op::FUnordGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            194u32 => Ok(ops::Op::ShiftRightLogical {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            195u32 => Ok(ops::Op::ShiftRightArithmetic {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            196u32 => Ok(ops::Op::ShiftLeftLogical {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            197u32 => Ok(ops::Op::BitwiseOr {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            198u32 => Ok(ops::Op::BitwiseXor {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            199u32 => Ok(ops::Op::BitwiseAnd {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            200u32 => Ok(ops::Op::Not {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            201u32 => Ok(ops::Op::BitFieldInsert {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                insert: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            202u32 => Ok(ops::Op::BitFieldSExtract {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            203u32 => Ok(ops::Op::BitFieldUExtract {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            204u32 => Ok(ops::Op::BitReverse {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            205u32 => Ok(ops::Op::BitCount {
                base: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            207u32 => Ok(ops::Op::DPdx {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            208u32 => Ok(ops::Op::DPdy {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            209u32 => Ok(ops::Op::Fwidth {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            210u32 => Ok(ops::Op::DPdxFine {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            211u32 => Ok(ops::Op::DPdyFine {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            212u32 => Ok(ops::Op::FwidthFine {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            213u32 => Ok(ops::Op::DPdxCoarse {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            214u32 => Ok(ops::Op::DPdyCoarse {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            215u32 => Ok(ops::Op::FwidthCoarse {
                p: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            218u32 => Ok(ops::Op::EmitVertex),
            219u32 => Ok(ops::Op::EndPrimitive),
            220u32 => Ok(ops::Op::EmitStreamVertex {
                stream: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            221u32 => Ok(ops::Op::EndStreamPrimitive {
                stream: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            224u32 => Ok(ops::Op::ControlBarrier {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            225u32 => Ok(ops::Op::MemoryBarrier {
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            227u32 => Ok(ops::Op::AtomicLoad {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            228u32 => Ok(ops::Op::AtomicStore {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            229u32 => Ok(ops::Op::AtomicExchange {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            230u32 => Ok(ops::Op::AtomicCompareExchange {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                equal: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                unequal: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                comparator: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            231u32 => Ok(ops::Op::AtomicCompareExchangeWeak {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                equal: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                unequal: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                comparator: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            232u32 => Ok(ops::Op::AtomicIIncrement {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            233u32 => Ok(ops::Op::AtomicIDecrement {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            234u32 => Ok(ops::Op::AtomicIAdd {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            235u32 => Ok(ops::Op::AtomicISub {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            236u32 => Ok(ops::Op::AtomicSMin {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            237u32 => Ok(ops::Op::AtomicUMin {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            238u32 => Ok(ops::Op::AtomicSMax {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            239u32 => Ok(ops::Op::AtomicUMax {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            240u32 => Ok(ops::Op::AtomicAnd {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            241u32 => Ok(ops::Op::AtomicOr {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            242u32 => Ok(ops::Op::AtomicXor {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            259u32 => Ok(ops::Op::GroupAsyncCopy {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                destination: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_elements: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                event: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            260u32 => Ok(ops::Op::GroupWaitEvents {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                events_list: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            261u32 => Ok(ops::Op::GroupAll {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            262u32 => Ok(ops::Op::GroupAny {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            263u32 => Ok(ops::Op::GroupBroadcast {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                local_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            264u32 => Ok(ops::Op::GroupIAdd {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            265u32 => Ok(ops::Op::GroupFAdd {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            266u32 => Ok(ops::Op::GroupFMin {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            267u32 => Ok(ops::Op::GroupUMin {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            268u32 => Ok(ops::Op::GroupSMin {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            269u32 => Ok(ops::Op::GroupFMax {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            270u32 => Ok(ops::Op::GroupUMax {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            271u32 => Ok(ops::Op::GroupSMax {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            274u32 => Ok(ops::Op::ReadPipe {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            275u32 => Ok(ops::Op::WritePipe {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            276u32 => Ok(ops::Op::ReservedReadPipe {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            277u32 => Ok(ops::Op::ReservedWritePipe {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            278u32 => Ok(ops::Op::ReserveReadPipePackets {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            279u32 => Ok(ops::Op::ReserveWritePipePackets {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            280u32 => Ok(ops::Op::CommitReadPipe {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            281u32 => Ok(ops::Op::CommitWritePipe {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            282u32 => Ok(ops::Op::IsValidReserveId {
                reserve_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            283u32 => Ok(ops::Op::GetNumPipePackets {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            284u32 => Ok(ops::Op::GetMaxPipePackets {
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            285u32 => Ok(ops::Op::GroupReserveReadPipePackets {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            286u32 => Ok(ops::Op::GroupReserveWritePipePackets {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            287u32 => Ok(ops::Op::GroupCommitReadPipe {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            288u32 => Ok(ops::Op::GroupCommitWritePipe {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            291u32 => Ok(ops::Op::EnqueueMarker {
                queue: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                wait_events: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ret_event: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            292u32 => Ok(ops::Op::EnqueueKernel {
                queue: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                flags: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                nd_range: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                wait_events: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ret_event: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                local_size: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            293u32 => Ok(ops::Op::GetKernelNDrangeSubGroupCount {
                nd_range: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            294u32 => Ok(ops::Op::GetKernelNDrangeMaxSubGroupSize {
                nd_range: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            295u32 => Ok(ops::Op::GetKernelWorkGroupSize {
                invoke: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            296u32 => Ok(ops::Op::GetKernelPreferredWorkGroupSizeMultiple {
                invoke: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            297u32 => Ok(ops::Op::RetainEvent {
                event: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            298u32 => Ok(ops::Op::ReleaseEvent {
                event: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            299u32 => Ok(ops::Op::CreateUserEvent),
            300u32 => Ok(ops::Op::IsValidEvent {
                event: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            301u32 => Ok(ops::Op::SetUserEventStatus {
                event: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                status: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            302u32 => Ok(ops::Op::CaptureEventProfilingInfo {
                event: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                profiling_info: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            303u32 => Ok(ops::Op::GetDefaultQueue),
            304u32 => Ok(ops::Op::BuildNDRange {
                global_work_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                local_work_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                global_work_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            305u32 => Ok(ops::Op::ImageSparseSampleImplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            306u32 => Ok(ops::Op::ImageSparseSampleExplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            307u32 => Ok(ops::Op::ImageSparseSampleDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            308u32 => Ok(ops::Op::ImageSparseSampleDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            309u32 => Ok(ops::Op::ImageSparseSampleProjImplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            310u32 => Ok(ops::Op::ImageSparseSampleProjExplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            311u32 => Ok(ops::Op::ImageSparseSampleProjDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            312u32 => Ok(ops::Op::ImageSparseSampleProjDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            313u32 => Ok(ops::Op::ImageSparseFetch {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            314u32 => Ok(ops::Op::ImageSparseGather {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            315u32 => Ok(ops::Op::ImageSparseDrefGather {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            316u32 => Ok(ops::Op::ImageSparseTexelsResident {
                resident_code: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            317u32 => Ok(ops::Op::NoLine),
            318u32 => Ok(ops::Op::AtomicFlagTestAndSet {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            319u32 => Ok(ops::Op::AtomicFlagClear {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            320u32 => Ok(ops::Op::ImageSparseRead {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            321u32 => Ok(ops::Op::SizeOf {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            323u32 => Ok(ops::Op::ConstantPipeStorage {
                packet_size: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                capacity: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            324u32 => Ok(ops::Op::CreatePipeFromPipeStorage {
                pipe_storage: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            325u32 => Ok(ops::Op::GetKernelLocalSizeForSubgroupCount {
                subgroup_count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            326u32 => Ok(ops::Op::GetKernelMaxNumSubgroups {
                invoke: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            328u32 => Ok(ops::Op::NamedBarrierInitialize {
                subgroup_count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            329u32 => Ok(ops::Op::MemoryNamedBarrier {
                named_barrier: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            330u32 => Ok(ops::Op::ModuleProcessed {
                process: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            332u32 => Ok(ops::Op::DecorateId {
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(dr::Operand::Decoration(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            333u32 => Ok(ops::Op::GroupNonUniformElect {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            334u32 => Ok(ops::Op::GroupNonUniformAll {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            335u32 => Ok(ops::Op::GroupNonUniformAny {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            336u32 => Ok(ops::Op::GroupNonUniformAllEqual {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            337u32 => Ok(ops::Op::GroupNonUniformBroadcast {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            338u32 => Ok(ops::Op::GroupNonUniformBroadcastFirst {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            339u32 => Ok(ops::Op::GroupNonUniformBallot {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            340u32 => Ok(ops::Op::GroupNonUniformInverseBallot {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            341u32 => Ok(ops::Op::GroupNonUniformBallotBitExtract {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            342u32 => Ok(ops::Op::GroupNonUniformBallotBitCount {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            343u32 => Ok(ops::Op::GroupNonUniformBallotFindLSB {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            344u32 => Ok(ops::Op::GroupNonUniformBallotFindMSB {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            345u32 => Ok(ops::Op::GroupNonUniformShuffle {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            346u32 => Ok(ops::Op::GroupNonUniformShuffleXor {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            347u32 => Ok(ops::Op::GroupNonUniformShuffleUp {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            348u32 => Ok(ops::Op::GroupNonUniformShuffleDown {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            349u32 => Ok(ops::Op::GroupNonUniformIAdd {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            350u32 => Ok(ops::Op::GroupNonUniformFAdd {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            351u32 => Ok(ops::Op::GroupNonUniformIMul {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            352u32 => Ok(ops::Op::GroupNonUniformFMul {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            353u32 => Ok(ops::Op::GroupNonUniformSMin {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            354u32 => Ok(ops::Op::GroupNonUniformUMin {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            355u32 => Ok(ops::Op::GroupNonUniformFMin {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            356u32 => Ok(ops::Op::GroupNonUniformSMax {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            357u32 => Ok(ops::Op::GroupNonUniformUMax {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            358u32 => Ok(ops::Op::GroupNonUniformFMax {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            359u32 => Ok(ops::Op::GroupNonUniformBitwiseAnd {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            360u32 => Ok(ops::Op::GroupNonUniformBitwiseOr {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            361u32 => Ok(ops::Op::GroupNonUniformBitwiseXor {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            362u32 => Ok(ops::Op::GroupNonUniformLogicalAnd {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            363u32 => Ok(ops::Op::GroupNonUniformLogicalOr {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            364u32 => Ok(ops::Op::GroupNonUniformLogicalXor {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            365u32 => Ok(ops::Op::GroupNonUniformQuadBroadcast {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            366u32 => Ok(ops::Op::GroupNonUniformQuadSwap {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            400u32 => Ok(ops::Op::CopyLogical {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            401u32 => Ok(ops::Op::PtrEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            402u32 => Ok(ops::Op::PtrNotEqual {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            403u32 => Ok(ops::Op::PtrDiff {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4160u32 => Ok(ops::Op::ColorAttachmentReadEXT {
                attachment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sample: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4161u32 => Ok(ops::Op::DepthAttachmentReadEXT {
                sample: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4162u32 => Ok(ops::Op::StencilAttachmentReadEXT {
                sample: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4421u32 => Ok(ops::Op::SubgroupBallotKHR {
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4422u32 => Ok(ops::Op::SubgroupFirstInvocationKHR {
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4428u32 => Ok(ops::Op::SubgroupAllKHR {
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4429u32 => Ok(ops::Op::SubgroupAnyKHR {
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4430u32 => Ok(ops::Op::SubgroupAllEqualKHR {
                predicate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4431u32 => Ok(ops::Op::GroupNonUniformRotateKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4432u32 => Ok(ops::Op::SubgroupReadInvocationKHR {
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4445u32 => Ok(ops::Op::TraceRayKHR {
                accel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_flags: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cull_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                miss_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmax: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4446u32 => Ok(ops::Op::ExecuteCallableKHR {
                sbt_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                callable_data: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4447u32 => Ok(ops::Op::ConvertUToAccelerationStructureKHR {
                accel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4450u32 => Ok(ops::Op::SDot {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_vector_format: match operands.next() {
                    Some(dr::Operand::PackedVectorFormat(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4451u32 => Ok(ops::Op::UDot {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_vector_format: match operands.next() {
                    Some(dr::Operand::PackedVectorFormat(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4452u32 => Ok(ops::Op::SUDot {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_vector_format: match operands.next() {
                    Some(dr::Operand::PackedVectorFormat(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4453u32 => Ok(ops::Op::SDotAccSat {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                accumulator: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_vector_format: match operands.next() {
                    Some(dr::Operand::PackedVectorFormat(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4454u32 => Ok(ops::Op::UDotAccSat {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                accumulator: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_vector_format: match operands.next() {
                    Some(dr::Operand::PackedVectorFormat(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4455u32 => Ok(ops::Op::SUDotAccSat {
                vector_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                accumulator: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_vector_format: match operands.next() {
                    Some(dr::Operand::PackedVectorFormat(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4457u32 => Ok(ops::Op::CooperativeMatrixLoadKHR {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_layout: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                stride: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
                memory_operand: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4458u32 => Ok(ops::Op::CooperativeMatrixStoreKHR {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_layout: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                stride: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
                memory_operand: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4459u32 => Ok(ops::Op::CooperativeMatrixMulAddKHR {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                c: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cooperative_matrix_operands: match operands.next() {
                    Some(dr::Operand::CooperativeMatrixOperands(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            4460u32 => Ok(ops::Op::CooperativeMatrixLengthKHR {
                ty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4473u32 => Ok(ops::Op::RayQueryInitializeKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                accel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_flags: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cull_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4474u32 => Ok(ops::Op::RayQueryTerminateKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4475u32 => Ok(ops::Op::RayQueryGenerateIntersectionKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_t: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4476u32 => Ok(ops::Op::RayQueryConfirmIntersectionKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4477u32 => Ok(ops::Op::RayQueryProceedKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4479u32 => Ok(ops::Op::RayQueryGetIntersectionTypeKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4480u32 => Ok(ops::Op::ImageSampleWeightedQCOM {
                texture: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinates: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                weights: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4481u32 => Ok(ops::Op::ImageBoxFilterQCOM {
                texture: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinates: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                box_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4482u32 => Ok(ops::Op::ImageBlockMatchSSDQCOM {
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                target_coordinates: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reference: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reference_coordinates: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                block_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4483u32 => Ok(ops::Op::ImageBlockMatchSADQCOM {
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                target_coordinates: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reference: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reference_coordinates: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                block_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5000u32 => Ok(ops::Op::GroupIAddNonUniformAMD {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5001u32 => Ok(ops::Op::GroupFAddNonUniformAMD {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5002u32 => Ok(ops::Op::GroupFMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5003u32 => Ok(ops::Op::GroupUMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5004u32 => Ok(ops::Op::GroupSMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5005u32 => Ok(ops::Op::GroupFMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5006u32 => Ok(ops::Op::GroupUMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5007u32 => Ok(ops::Op::GroupSMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5011u32 => Ok(ops::Op::FragmentMaskFetchAMD {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5012u32 => Ok(ops::Op::FragmentFetchAMD {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                fragment_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5056u32 => Ok(ops::Op::ReadClockKHR {
                scope: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5075u32 => Ok(ops::Op::FinalizeNodePayloadsAMDX {
                payload_array: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5078u32 => Ok(ops::Op::FinishWritingNodePayloadAMDX {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5090u32 => Ok(ops::Op::InitializeNodePayloadsAMDX {
                payload_array: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                visibility: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload_count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                node_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5249u32 => Ok(ops::Op::HitObjectRecordHitMotionNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                acceleration_structure: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                instance_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                primitive_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                geometry_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_kind: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                current_time: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_object_attributes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5250u32 => Ok(ops::Op::HitObjectRecordHitWithIndexMotionNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                acceleration_structure: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                instance_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                primitive_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                geometry_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_kind: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                current_time: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_object_attributes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5251u32 => Ok(ops::Op::HitObjectRecordMissMotionNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                current_time: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5252u32 => Ok(ops::Op::HitObjectGetWorldToObjectNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5253u32 => Ok(ops::Op::HitObjectGetObjectToWorldNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5254u32 => Ok(ops::Op::HitObjectGetObjectRayDirectionNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5255u32 => Ok(ops::Op::HitObjectGetObjectRayOriginNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5256u32 => Ok(ops::Op::HitObjectTraceRayMotionNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                acceleration_structure: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_flags: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cullmask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                miss_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                time: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5257u32 => Ok(ops::Op::HitObjectGetShaderRecordBufferHandleNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5258u32 => Ok(ops::Op::HitObjectGetShaderBindingTableRecordIndexNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5259u32 => Ok(ops::Op::HitObjectRecordEmptyNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5260u32 => Ok(ops::Op::HitObjectTraceRayNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                acceleration_structure: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_flags: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cullmask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                miss_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5261u32 => Ok(ops::Op::HitObjectRecordHitNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                acceleration_structure: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                instance_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                primitive_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                geometry_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_kind: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_object_attributes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5262u32 => Ok(ops::Op::HitObjectRecordHitWithIndexNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                acceleration_structure: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                instance_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                primitive_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                geometry_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_kind: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_record_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_object_attributes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5263u32 => Ok(ops::Op::HitObjectRecordMissNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_min: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                t_max: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5264u32 => Ok(ops::Op::HitObjectExecuteShaderNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5265u32 => Ok(ops::Op::HitObjectGetCurrentTimeNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5266u32 => Ok(ops::Op::HitObjectGetAttributesNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_object_attribute: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5267u32 => Ok(ops::Op::HitObjectGetHitKindNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5268u32 => Ok(ops::Op::HitObjectGetPrimitiveIndexNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5269u32 => Ok(ops::Op::HitObjectGetGeometryIndexNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5270u32 => Ok(ops::Op::HitObjectGetInstanceIdNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5271u32 => Ok(ops::Op::HitObjectGetInstanceCustomIndexNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5272u32 => Ok(ops::Op::HitObjectGetWorldRayDirectionNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5273u32 => Ok(ops::Op::HitObjectGetWorldRayOriginNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5274u32 => Ok(ops::Op::HitObjectGetRayTMaxNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5275u32 => Ok(ops::Op::HitObjectGetRayTMinNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5276u32 => Ok(ops::Op::HitObjectIsEmptyNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5277u32 => Ok(ops::Op::HitObjectIsHitNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5278u32 => Ok(ops::Op::HitObjectIsMissNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5279u32 => Ok(ops::Op::ReorderThreadWithHitObjectNV {
                hit_object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hint: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
                bits: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            5280u32 => Ok(ops::Op::ReorderThreadWithHintNV {
                hint: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                bits: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5283u32 => Ok(ops::Op::ImageSampleFootprintNV {
                sampled_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                granularity: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coarse: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(dr::Operand::ImageOperands(value)) => {
                        let operands = operands
                            .map(|op| match *op {
                                dr::Operand::IdRef(second) => Ok(second),
                                _ => Err(OperandError::WrongType),
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Some((*value, operands))
                    }
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            5295u32 => Ok(ops::Op::SetMeshOutputsEXT {
                vertex_count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                primitive_count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5296u32 => Ok(ops::Op::GroupNonUniformPartitionNV {
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5299u32 => Ok(ops::Op::WritePackedPrimitiveIndices4x8NV {
                index_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_indices: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5300u32 => Ok(ops::Op::FetchMicroTriangleVertexPositionNV {
                accel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                instance_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                geometry_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                primitive_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                barycentric: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5301u32 => Ok(ops::Op::FetchMicroTriangleVertexBarycentricNV {
                accel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                instance_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                geometry_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                primitive_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                barycentric: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5334u32 => Ok(ops::Op::ReportIntersectionKHR {
                hit: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_kind: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5335u32 => Ok(ops::Op::IgnoreIntersectionNV),
            5336u32 => Ok(ops::Op::TerminateRayNV),
            5337u32 => Ok(ops::Op::TraceNV {
                accel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_flags: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cull_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                miss_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmax: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5338u32 => Ok(ops::Op::TraceMotionNV {
                accel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_flags: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cull_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                miss_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmax: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                time: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5339u32 => Ok(ops::Op::TraceRayMotionNV {
                accel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_flags: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cull_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                miss_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_origin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmin: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmax: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                time: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5340u32 => Ok(ops::Op::RayQueryGetIntersectionTriangleVertexPositionsKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5344u32 => Ok(ops::Op::ExecuteCallableNV {
                sbt_index: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                callable_data_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5359u32 => Ok(ops::Op::CooperativeMatrixLoadNV {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                column_major: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            5360u32 => Ok(ops::Op::CooperativeMatrixStoreNV {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                stride: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                column_major: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(dr::Operand::MemoryAccess(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            5361u32 => Ok(ops::Op::CooperativeMatrixMulAddNV {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                c: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5362u32 => Ok(ops::Op::CooperativeMatrixLengthNV {
                ty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5364u32 => Ok(ops::Op::BeginInvocationInterlockEXT),
            5365u32 => Ok(ops::Op::EndInvocationInterlockEXT),
            5381u32 => Ok(ops::Op::IsHelperInvocationEXT),
            5391u32 => Ok(ops::Op::ConvertUToImageNV {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5392u32 => Ok(ops::Op::ConvertUToSamplerNV {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5393u32 => Ok(ops::Op::ConvertImageToUNV {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5394u32 => Ok(ops::Op::ConvertSamplerToUNV {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5395u32 => Ok(ops::Op::ConvertUToSampledImageNV {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5396u32 => Ok(ops::Op::ConvertSampledImageToUNV {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5397u32 => Ok(ops::Op::SamplerImageAddressingModeNV {
                bit_width: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5571u32 => Ok(ops::Op::SubgroupShuffleINTEL {
                data: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invocation_id: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5572u32 => Ok(ops::Op::SubgroupShuffleDownINTEL {
                current: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                next: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5573u32 => Ok(ops::Op::SubgroupShuffleUpINTEL {
                previous: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                current: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5574u32 => Ok(ops::Op::SubgroupShuffleXorINTEL {
                data: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5575u32 => Ok(ops::Op::SubgroupBlockReadINTEL {
                ptr: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5576u32 => Ok(ops::Op::SubgroupBlockWriteINTEL {
                ptr: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                data: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5577u32 => Ok(ops::Op::SubgroupImageBlockReadINTEL {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5578u32 => Ok(ops::Op::SubgroupImageBlockWriteINTEL {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                data: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5580u32 => Ok(ops::Op::SubgroupImageMediaBlockReadINTEL {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                width: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                height: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5581u32 => Ok(ops::Op::SubgroupImageMediaBlockWriteINTEL {
                image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                width: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                height: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                data: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5585u32 => Ok(ops::Op::UCountLeadingZerosINTEL {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5586u32 => Ok(ops::Op::UCountTrailingZerosINTEL {
                operand: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5587u32 => Ok(ops::Op::AbsISubINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5588u32 => Ok(ops::Op::AbsUSubINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5589u32 => Ok(ops::Op::IAddSatINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5590u32 => Ok(ops::Op::UAddSatINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5591u32 => Ok(ops::Op::IAverageINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5592u32 => Ok(ops::Op::UAverageINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5593u32 => Ok(ops::Op::IAverageRoundedINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5594u32 => Ok(ops::Op::UAverageRoundedINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5595u32 => Ok(ops::Op::ISubSatINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5596u32 => Ok(ops::Op::USubSatINTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5597u32 => Ok(ops::Op::IMul32x16INTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5598u32 => Ok(ops::Op::UMul32x16INTEL {
                operand_1: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5600u32 => Ok(ops::Op::ConstantFunctionPointerINTEL {
                function: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5601u32 => Ok(ops::Op::FunctionPointerCallINTEL {
                operand_1: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            5609u32 => Ok(ops::Op::AsmTargetINTEL {
                asm_target: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5610u32 => Ok(ops::Op::AsmINTEL {
                asm_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                asm_instructions: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                constraints: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5611u32 => Ok(ops::Op::AsmCallINTEL {
                asm: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                argument_0: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            5614u32 => Ok(ops::Op::AtomicFMinEXT {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5615u32 => Ok(ops::Op::AtomicFMaxEXT {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5630u32 => Ok(ops::Op::AssumeTrueKHR {
                condition: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5631u32 => Ok(ops::Op::ExpectKHR {
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                expected_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5632u32 => Ok(ops::Op::DecorateString {
                target: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(dr::Operand::Decoration(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5633u32 => Ok(ops::Op::MemberDecorateString {
                struct_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                member: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(dr::Operand::Decoration(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5699u32 => Ok(ops::Op::VmeImageINTEL {
                image_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sampler: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5700u32 => Ok(ops::Op::TypeVmeImageINTEL {
                image_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5701u32 => Ok(ops::Op::TypeAvcImePayloadINTEL),
            5702u32 => Ok(ops::Op::TypeAvcRefPayloadINTEL),
            5703u32 => Ok(ops::Op::TypeAvcSicPayloadINTEL),
            5704u32 => Ok(ops::Op::TypeAvcMcePayloadINTEL),
            5705u32 => Ok(ops::Op::TypeAvcMceResultINTEL),
            5706u32 => Ok(ops::Op::TypeAvcImeResultINTEL),
            5707u32 => Ok(ops::Op::TypeAvcImeResultSingleReferenceStreamoutINTEL),
            5708u32 => Ok(ops::Op::TypeAvcImeResultDualReferenceStreamoutINTEL),
            5709u32 => Ok(ops::Op::TypeAvcImeSingleReferenceStreaminINTEL),
            5710u32 => Ok(ops::Op::TypeAvcImeDualReferenceStreaminINTEL),
            5711u32 => Ok(ops::Op::TypeAvcRefResultINTEL),
            5712u32 => Ok(ops::Op::TypeAvcSicResultINTEL),
            5713u32 => Ok(
                ops::Op::SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL {
                    slice_type: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    qp: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5714u32 => Ok(
                ops::Op::SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL {
                    reference_base_penalty: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5715u32 => Ok(ops::Op::SubgroupAvcMceGetDefaultInterShapePenaltyINTEL {
                slice_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                qp: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5716u32 => Ok(ops::Op::SubgroupAvcMceSetInterShapePenaltyINTEL {
                packed_shape_penalty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5717u32 => Ok(
                ops::Op::SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL {
                    slice_type: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    qp: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5718u32 => Ok(ops::Op::SubgroupAvcMceSetInterDirectionPenaltyINTEL {
                direction_cost: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5719u32 => Ok(
                ops::Op::SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL {
                    slice_type: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    qp: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5720u32 => Ok(
                ops::Op::SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL {
                    slice_type: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    qp: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5721u32 => Ok(ops::Op::SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL),
            5722u32 => Ok(ops::Op::SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL),
            5723u32 => Ok(ops::Op::SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL),
            5724u32 => Ok(ops::Op::SubgroupAvcMceSetMotionVectorCostFunctionINTEL {
                packed_cost_center_delta: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_cost_table: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cost_precision: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5725u32 => Ok(ops::Op::SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL {
                slice_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                qp: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5726u32 => Ok(ops::Op::SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL),
            5727u32 => Ok(ops::Op::SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL),
            5728u32 => Ok(ops::Op::SubgroupAvcMceSetAcOnlyHaarINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5729u32 => Ok(
                ops::Op::SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL {
                    source_field_polarity: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5730u32 => Ok(
                ops::Op::SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL {
                    reference_field_polarity: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5731u32 => Ok(
                ops::Op::SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL {
                    forward_reference_field_polarity: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    backward_reference_field_polarity: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5732u32 => Ok(ops::Op::SubgroupAvcMceConvertToImePayloadINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5733u32 => Ok(ops::Op::SubgroupAvcMceConvertToImeResultINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5734u32 => Ok(ops::Op::SubgroupAvcMceConvertToRefPayloadINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5735u32 => Ok(ops::Op::SubgroupAvcMceConvertToRefResultINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5736u32 => Ok(ops::Op::SubgroupAvcMceConvertToSicPayloadINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5737u32 => Ok(ops::Op::SubgroupAvcMceConvertToSicResultINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5738u32 => Ok(ops::Op::SubgroupAvcMceGetMotionVectorsINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5739u32 => Ok(ops::Op::SubgroupAvcMceGetInterDistortionsINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5740u32 => Ok(ops::Op::SubgroupAvcMceGetBestInterDistortionsINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5741u32 => Ok(ops::Op::SubgroupAvcMceGetInterMajorShapeINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5742u32 => Ok(ops::Op::SubgroupAvcMceGetInterMinorShapeINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5743u32 => Ok(ops::Op::SubgroupAvcMceGetInterDirectionsINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5744u32 => Ok(ops::Op::SubgroupAvcMceGetInterMotionVectorCountINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5745u32 => Ok(ops::Op::SubgroupAvcMceGetInterReferenceIdsINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5746u32 => Ok(
                ops::Op::SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL {
                    packed_reference_ids: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    packed_reference_parameter_field_polarities: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5747u32 => Ok(ops::Op::SubgroupAvcImeInitializeINTEL {
                src_coord: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                partition_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sad_adjustment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5748u32 => Ok(ops::Op::SubgroupAvcImeSetSingleReferenceINTEL {
                ref_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                search_window_config: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5749u32 => Ok(ops::Op::SubgroupAvcImeSetDualReferenceINTEL {
                fwd_ref_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                bwd_ref_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                id_search_window_config: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5750u32 => Ok(ops::Op::SubgroupAvcImeRefWindowSizeINTEL {
                search_window_config: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                dual_ref: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5751u32 => Ok(ops::Op::SubgroupAvcImeAdjustRefOffsetINTEL {
                ref_offset: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                src_coord: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ref_window_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5752u32 => Ok(ops::Op::SubgroupAvcImeConvertToMcePayloadINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5753u32 => Ok(ops::Op::SubgroupAvcImeSetMaxMotionVectorCountINTEL {
                max_motion_vector_count: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5754u32 => Ok(ops::Op::SubgroupAvcImeSetUnidirectionalMixDisableINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5755u32 => Ok(
                ops::Op::SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL {
                    threshold: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5756u32 => Ok(ops::Op::SubgroupAvcImeSetWeightedSadINTEL {
                packed_sad_weights: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5757u32 => Ok(ops::Op::SubgroupAvcImeEvaluateWithSingleReferenceINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5758u32 => Ok(ops::Op::SubgroupAvcImeEvaluateWithDualReferenceINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                fwd_ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                bwd_ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5759u32 => Ok(
                ops::Op::SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL {
                    src_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    streamin_components: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5760u32 => Ok(
                ops::Op::SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL {
                    src_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    fwd_ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    bwd_ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    streamin_components: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5761u32 => Ok(
                ops::Op::SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL {
                    src_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5762u32 => Ok(
                ops::Op::SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL {
                    src_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    fwd_ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    bwd_ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5763u32 => Ok(
                ops::Op::SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL {
                    src_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    streamin_components: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5764u32 => Ok(
                ops::Op::SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL {
                    src_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    fwd_ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    bwd_ref_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    streamin_components: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5765u32 => Ok(ops::Op::SubgroupAvcImeConvertToMceResultINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5766u32 => Ok(ops::Op::SubgroupAvcImeGetSingleReferenceStreaminINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5767u32 => Ok(ops::Op::SubgroupAvcImeGetDualReferenceStreaminINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5768u32 => Ok(ops::Op::SubgroupAvcImeStripSingleReferenceStreamoutINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5769u32 => Ok(ops::Op::SubgroupAvcImeStripDualReferenceStreamoutINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5770u32 => Ok(
                ops::Op::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    major_shape: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5771u32 => Ok(
                ops::Op::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    major_shape: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5772u32 => Ok(
                ops::Op::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    major_shape: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5773u32 => Ok(
                ops::Op::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    major_shape: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    direction: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5774u32 => Ok(
                ops::Op::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    major_shape: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    direction: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5775u32 => Ok(
                ops::Op::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    major_shape: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    direction: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5776u32 => Ok(ops::Op::SubgroupAvcImeGetBorderReachedINTEL {
                image_select: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5777u32 => Ok(ops::Op::SubgroupAvcImeGetTruncatedSearchIndicationINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5778u32 => Ok(
                ops::Op::SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5779u32 => Ok(
                ops::Op::SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5780u32 => Ok(
                ops::Op::SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL {
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5781u32 => Ok(ops::Op::SubgroupAvcFmeInitializeINTEL {
                src_coord: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                motion_vectors: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                major_shapes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                minor_shapes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pixel_resolution: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sad_adjustment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5782u32 => Ok(ops::Op::SubgroupAvcBmeInitializeINTEL {
                src_coord: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                motion_vectors: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                major_shapes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                minor_shapes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pixel_resolution: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                bidirectional_weight: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sad_adjustment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5783u32 => Ok(ops::Op::SubgroupAvcRefConvertToMcePayloadINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5784u32 => Ok(ops::Op::SubgroupAvcRefSetBidirectionalMixDisableINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5785u32 => Ok(ops::Op::SubgroupAvcRefSetBilinearFilterEnableINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5786u32 => Ok(ops::Op::SubgroupAvcRefEvaluateWithSingleReferenceINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5787u32 => Ok(ops::Op::SubgroupAvcRefEvaluateWithDualReferenceINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                fwd_ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                bwd_ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5788u32 => Ok(ops::Op::SubgroupAvcRefEvaluateWithMultiReferenceINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_reference_ids: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5789u32 => Ok(
                ops::Op::SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL {
                    src_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    packed_reference_ids: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    packed_reference_field_polarities: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5790u32 => Ok(ops::Op::SubgroupAvcRefConvertToMceResultINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5791u32 => Ok(ops::Op::SubgroupAvcSicInitializeINTEL {
                src_coord: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5792u32 => Ok(ops::Op::SubgroupAvcSicConfigureSkcINTEL {
                skip_block_partition_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                skip_motion_vector_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                motion_vectors: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                bidirectional_weight: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sad_adjustment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5793u32 => Ok(ops::Op::SubgroupAvcSicConfigureIpeLumaINTEL {
                luma_intra_partition_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intra_neighbour_availabilty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                left_edge_luma_pixels: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                upper_left_corner_luma_pixel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                upper_edge_luma_pixels: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                upper_right_edge_luma_pixels: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sad_adjustment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5794u32 => Ok(ops::Op::SubgroupAvcSicConfigureIpeLumaChromaINTEL {
                luma_intra_partition_mask: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intra_neighbour_availabilty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                left_edge_luma_pixels: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                upper_left_corner_luma_pixel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                upper_edge_luma_pixels: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                upper_right_edge_luma_pixels: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                left_edge_chroma_pixels: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                upper_left_corner_chroma_pixel: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                upper_edge_chroma_pixels: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sad_adjustment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5795u32 => Ok(ops::Op::SubgroupAvcSicGetMotionVectorMaskINTEL {
                skip_block_partition_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5796u32 => Ok(ops::Op::SubgroupAvcSicConvertToMcePayloadINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5797u32 => Ok(ops::Op::SubgroupAvcSicSetIntraLumaShapePenaltyINTEL {
                packed_shape_penalty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5798u32 => Ok(ops::Op::SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL {
                luma_mode_penalty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                luma_packed_neighbor_modes: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                luma_packed_non_dc_penalty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5799u32 => Ok(ops::Op::SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL {
                chroma_mode_base_penalty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5800u32 => Ok(ops::Op::SubgroupAvcSicSetBilinearFilterEnableINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5801u32 => Ok(ops::Op::SubgroupAvcSicSetSkcForwardTransformEnableINTEL {
                packed_sad_coefficients: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5802u32 => Ok(ops::Op::SubgroupAvcSicSetBlockBasedRawSkipSadINTEL {
                block_based_skip_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5803u32 => Ok(ops::Op::SubgroupAvcSicEvaluateIpeINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5804u32 => Ok(ops::Op::SubgroupAvcSicEvaluateWithSingleReferenceINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5805u32 => Ok(ops::Op::SubgroupAvcSicEvaluateWithDualReferenceINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                fwd_ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                bwd_ref_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5806u32 => Ok(ops::Op::SubgroupAvcSicEvaluateWithMultiReferenceINTEL {
                src_image: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_reference_ids: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5807u32 => Ok(
                ops::Op::SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL {
                    src_image: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    packed_reference_ids: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    packed_reference_field_polarities: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    payload: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            5808u32 => Ok(ops::Op::SubgroupAvcSicConvertToMceResultINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5809u32 => Ok(ops::Op::SubgroupAvcSicGetIpeLumaShapeINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5810u32 => Ok(ops::Op::SubgroupAvcSicGetBestIpeLumaDistortionINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5811u32 => Ok(ops::Op::SubgroupAvcSicGetBestIpeChromaDistortionINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5812u32 => Ok(ops::Op::SubgroupAvcSicGetPackedIpeLumaModesINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5813u32 => Ok(ops::Op::SubgroupAvcSicGetIpeChromaModeINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5814u32 => Ok(ops::Op::SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5815u32 => Ok(ops::Op::SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5816u32 => Ok(ops::Op::SubgroupAvcSicGetInterRawSadsINTEL {
                payload: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5818u32 => Ok(ops::Op::VariableLengthArrayINTEL {
                lenght: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5819u32 => Ok(ops::Op::SaveMemoryINTEL),
            5820u32 => Ok(ops::Op::RestoreMemoryINTEL {
                ptr: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5840u32 => Ok(ops::Op::ArbitraryFloatSinCosPiINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                from_sign: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5841u32 => Ok(ops::Op::ArbitraryFloatCastINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5842u32 => Ok(ops::Op::ArbitraryFloatCastFromIntINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                from_sign: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5843u32 => Ok(ops::Op::ArbitraryFloatCastToIntINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5846u32 => Ok(ops::Op::ArbitraryFloatAddINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5847u32 => Ok(ops::Op::ArbitraryFloatSubINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5848u32 => Ok(ops::Op::ArbitraryFloatMulINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5849u32 => Ok(ops::Op::ArbitraryFloatDivINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5850u32 => Ok(ops::Op::ArbitraryFloatGTINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5851u32 => Ok(ops::Op::ArbitraryFloatGEINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5852u32 => Ok(ops::Op::ArbitraryFloatLTINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5853u32 => Ok(ops::Op::ArbitraryFloatLEINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5854u32 => Ok(ops::Op::ArbitraryFloatEQINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5855u32 => Ok(ops::Op::ArbitraryFloatRecipINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5856u32 => Ok(ops::Op::ArbitraryFloatRSqrtINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5857u32 => Ok(ops::Op::ArbitraryFloatCbrtINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5858u32 => Ok(ops::Op::ArbitraryFloatHypotINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5859u32 => Ok(ops::Op::ArbitraryFloatSqrtINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5860u32 => Ok(ops::Op::ArbitraryFloatLogINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5861u32 => Ok(ops::Op::ArbitraryFloatLog2INTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5862u32 => Ok(ops::Op::ArbitraryFloatLog10INTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5863u32 => Ok(ops::Op::ArbitraryFloatLog1pINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5864u32 => Ok(ops::Op::ArbitraryFloatExpINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5865u32 => Ok(ops::Op::ArbitraryFloatExp2INTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5866u32 => Ok(ops::Op::ArbitraryFloatExp10INTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5867u32 => Ok(ops::Op::ArbitraryFloatExpm1INTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5868u32 => Ok(ops::Op::ArbitraryFloatSinINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5869u32 => Ok(ops::Op::ArbitraryFloatCosINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5870u32 => Ok(ops::Op::ArbitraryFloatSinCosINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5871u32 => Ok(ops::Op::ArbitraryFloatSinPiINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5872u32 => Ok(ops::Op::ArbitraryFloatCosPiINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5873u32 => Ok(ops::Op::ArbitraryFloatASinINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5874u32 => Ok(ops::Op::ArbitraryFloatASinPiINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5875u32 => Ok(ops::Op::ArbitraryFloatACosINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5876u32 => Ok(ops::Op::ArbitraryFloatACosPiINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5877u32 => Ok(ops::Op::ArbitraryFloatATanINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5878u32 => Ok(ops::Op::ArbitraryFloatATanPiINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5879u32 => Ok(ops::Op::ArbitraryFloatATan2INTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5880u32 => Ok(ops::Op::ArbitraryFloatPowINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5881u32 => Ok(ops::Op::ArbitraryFloatPowRINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m2: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5882u32 => Ok(ops::Op::ArbitraryFloatPowNINTEL {
                a: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                m1: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mout: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                enable_subnormals: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_mode: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rounding_accuracy: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5887u32 => Ok(ops::Op::LoopControlINTEL {
                loop_control_parameters: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            5911u32 => Ok(ops::Op::AliasDomainDeclINTEL {
                name: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            5912u32 => Ok(ops::Op::AliasScopeDeclINTEL {
                alias_domain: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                name: match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            5913u32 => Ok(ops::Op::AliasScopeListDeclINTEL {
                alias_scope1_alias_scope2: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            5923u32 => Ok(ops::Op::FixedSqrtINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5924u32 => Ok(ops::Op::FixedRecipINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5925u32 => Ok(ops::Op::FixedRsqrtINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5926u32 => Ok(ops::Op::FixedSinINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5927u32 => Ok(ops::Op::FixedCosINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5928u32 => Ok(ops::Op::FixedSinCosINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5929u32 => Ok(ops::Op::FixedSinPiINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5930u32 => Ok(ops::Op::FixedCosPiINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5931u32 => Ok(ops::Op::FixedSinCosPiINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5932u32 => Ok(ops::Op::FixedLogINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5933u32 => Ok(ops::Op::FixedExpINTEL {
                input_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                s: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                r_i: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                q: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                o: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5934u32 => Ok(ops::Op::PtrCastToCrossWorkgroupINTEL {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5938u32 => Ok(ops::Op::CrossWorkgroupCastToPtrINTEL {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5946u32 => Ok(ops::Op::ReadPipeBlockingINTEL {
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5947u32 => Ok(ops::Op::WritePipeBlockingINTEL {
                packet_size: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5949u32 => Ok(ops::Op::FPGARegINTEL {
                result: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                input: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6016u32 => Ok(ops::Op::RayQueryGetRayTMinKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6017u32 => Ok(ops::Op::RayQueryGetRayFlagsKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6018u32 => Ok(ops::Op::RayQueryGetIntersectionTKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6019u32 => Ok(ops::Op::RayQueryGetIntersectionInstanceCustomIndexKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6020u32 => Ok(ops::Op::RayQueryGetIntersectionInstanceIdKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6021u32 => Ok(
                ops::Op::RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR {
                    ray_query: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                    intersection: (match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    })
                    .ok_or(OperandError::Missing)?,
                },
            ),
            6022u32 => Ok(ops::Op::RayQueryGetIntersectionGeometryIndexKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6023u32 => Ok(ops::Op::RayQueryGetIntersectionPrimitiveIndexKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6024u32 => Ok(ops::Op::RayQueryGetIntersectionBarycentricsKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6025u32 => Ok(ops::Op::RayQueryGetIntersectionFrontFaceKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6026u32 => Ok(ops::Op::RayQueryGetIntersectionCandidateAABBOpaqueKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6027u32 => Ok(ops::Op::RayQueryGetIntersectionObjectRayDirectionKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6028u32 => Ok(ops::Op::RayQueryGetIntersectionObjectRayOriginKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6029u32 => Ok(ops::Op::RayQueryGetWorldRayDirectionKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6030u32 => Ok(ops::Op::RayQueryGetWorldRayOriginKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6031u32 => Ok(ops::Op::RayQueryGetIntersectionObjectToWorldKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6032u32 => Ok(ops::Op::RayQueryGetIntersectionWorldToObjectKHR {
                ray_query: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                intersection: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6035u32 => Ok(ops::Op::AtomicFAddEXT {
                pointer: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6096u32 => Ok(ops::Op::CompositeConstructContinuedINTEL {
                constituents: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            6116u32 => Ok(ops::Op::ConvertFToBF16INTEL {
                float_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6117u32 => Ok(ops::Op::ConvertBF16ToFINTEL {
                b_float16_value: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6142u32 => Ok(ops::Op::ControlBarrierArriveINTEL {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6143u32 => Ok(ops::Op::ControlBarrierWaitINTEL {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(dr::Operand::IdMemorySemantics(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6401u32 => Ok(ops::Op::GroupIMulKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6402u32 => Ok(ops::Op::GroupFMulKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6403u32 => Ok(ops::Op::GroupBitwiseAndKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6404u32 => Ok(ops::Op::GroupBitwiseOrKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6405u32 => Ok(ops::Op::GroupBitwiseXorKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6406u32 => Ok(ops::Op::GroupLogicalAndKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6407u32 => Ok(ops::Op::GroupLogicalOrKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6408u32 => Ok(ops::Op::GroupLogicalXorKHR {
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(dr::Operand::GroupOperation(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            _ => Err(InstructionError::WrongOpcode),
        }
    }
    pub fn lift_type(&mut self, raw: &dr::Instruction) -> Result<Type, InstructionError> {
        let mut operands = raw.operands.iter();
        match raw.class.opcode as u32 {
            19u32 => Ok(Type::Void),
            20u32 => Ok(Type::Bool),
            21u32 => Ok(Type::Int {
                width: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                signedness: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            22u32 => Ok(Type::Float {
                width: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            23u32 => Ok(Type::Vector {
                component_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                component_count: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            24u32 => Ok(Type::Matrix {
                column_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                column_count: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            25u32 => Ok(Type::Image {
                sampled_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                dim: (match operands.next() {
                    Some(dr::Operand::Dim(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                depth: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                arrayed: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ms: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sampled: (match operands.next() {
                    Some(dr::Operand::LiteralBit32(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_format: (match operands.next() {
                    Some(dr::Operand::ImageFormat(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                access_qualifier: match operands.next() {
                    Some(dr::Operand::AccessQualifier(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                },
            }),
            26u32 => Ok(Type::Sampler),
            27u32 => Ok(Type::SampledImage {
                image_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            28u32 => Ok(Type::Array {
                element_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                length: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.constants.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            29u32 => Ok(Type::RuntimeArray {
                element_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            30u32 => Ok(Type::Struct {
                member_0_type_member_1_type: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => {
                            Some(StructMember::new(self.types.lookup_token(*value)))
                        }
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            31u32 => Ok(Type::Opaque {
                the_name_of_the_opaque_type: (match operands.next() {
                    Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            32u32 => Ok(Type::Pointer {
                storage_class: (match operands.next() {
                    Some(dr::Operand::StorageClass(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ty: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            33u32 => Ok(Type::Function {
                return_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                parameter_0_type_parameter_1_type: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            34u32 => Ok(Type::Event),
            35u32 => Ok(Type::DeviceEvent),
            36u32 => Ok(Type::ReserveId),
            37u32 => Ok(Type::Queue),
            38u32 => Ok(Type::Pipe {
                qualifier: (match operands.next() {
                    Some(dr::Operand::AccessQualifier(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            39u32 => Ok(Type::ForwardPointer {
                pointer_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                storage_class: (match operands.next() {
                    Some(dr::Operand::StorageClass(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            322u32 => Ok(Type::PipeStorage),
            327u32 => Ok(Type::NamedBarrier),
            4456u32 => Ok(Type::CooperativeMatrixKHR {
                component_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                scope: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rows: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                columns: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                usage: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4472u32 => Ok(Type::RayQueryKHR),
            5281u32 => Ok(Type::HitObjectNV),
            5341u32 => Ok(Type::AccelerationStructureKHR),
            5358u32 => Ok(Type::CooperativeMatrixNV {
                component_type: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                execution: (match operands.next() {
                    Some(dr::Operand::IdScope(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rows: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                columns: (match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6086u32 => Ok(Type::BufferSurfaceINTEL {
                access_qualifier: (match operands.next() {
                    Some(dr::Operand::AccessQualifier(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            6090u32 => Ok(Type::StructContinuedINTEL {
                member_0_type_member_1_type: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(dr::Operand::IdRef(value)) => Some(*value),
                        Some(_) => return Err(OperandError::WrongType.into()),
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            _ => Err(InstructionError::WrongOpcode),
        }
    }
    #[allow(unused)]
    pub fn lift_extension(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Extension, InstructionError> {
        if raw.class.opcode as u32 != 10u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Extension {
            name: (match operands.next() {
                Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_ext_inst_import(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExtInstImport, InstructionError> {
        if raw.class.opcode as u32 != 11u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExtInstImport {
            name: (match operands.next() {
                Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_ext_inst(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExtInst, InstructionError> {
        if raw.class.opcode as u32 != 12u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExtInst {
            set: (match operands.next() {
                Some(dr::Operand::IdRef(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            instruction: (match operands.next() {
                Some(dr::Operand::LiteralExtInstInteger(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            operand_1_operand_2: {
                let mut vec = Vec::new();
                while let Some(item) = match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                } {
                    vec.push(item);
                }
                vec
            },
        })
    }
    #[allow(unused)]
    pub fn lift_memory_model(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::MemoryModel, InstructionError> {
        if raw.class.opcode as u32 != 14u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::MemoryModel {
            addressing_model: (match operands.next() {
                Some(dr::Operand::AddressingModel(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            memory_model: (match operands.next() {
                Some(dr::Operand::MemoryModel(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_entry_point(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::EntryPoint, InstructionError> {
        if raw.class.opcode as u32 != 15u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::EntryPoint {
            execution_model: (match operands.next() {
                Some(dr::Operand::ExecutionModel(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            entry_point: (match operands.next() {
                Some(dr::Operand::IdRef(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            name: (match operands.next() {
                Some(dr::Operand::LiteralString(value)) => Some(value.clone()),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            interface: {
                let mut vec = Vec::new();
                while let Some(item) = match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                } {
                    vec.push(item);
                }
                vec
            },
        })
    }
    #[allow(unused)]
    pub fn lift_execution_mode(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExecutionMode, InstructionError> {
        if raw.class.opcode as u32 != 16u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExecutionMode {
            entry_point: (match operands.next() {
                Some(dr::Operand::IdRef(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            mode: (match operands.next() {
                Some(dr::Operand::ExecutionMode(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_capability(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Capability, InstructionError> {
        if raw.class.opcode as u32 != 17u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Capability {
            capability: (match operands.next() {
                Some(dr::Operand::Capability(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_function(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Function, InstructionError> {
        if raw.class.opcode as u32 != 54u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Function {
            function_control: (match operands.next() {
                Some(dr::Operand::FunctionControl(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            function_type: (match operands.next() {
                Some(dr::Operand::IdRef(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_function_parameter(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::FunctionParameter, InstructionError> {
        if raw.class.opcode as u32 != 55u32 {
            return Err(InstructionError::WrongOpcode);
        }
        Ok(instructions::FunctionParameter {})
    }
    #[allow(unused)]
    pub fn lift_function_end(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::FunctionEnd, InstructionError> {
        if raw.class.opcode as u32 != 56u32 {
            return Err(InstructionError::WrongOpcode);
        }
        Ok(instructions::FunctionEnd {})
    }
    #[allow(unused)]
    pub fn lift_function_call(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::FunctionCall, InstructionError> {
        if raw.class.opcode as u32 != 57u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::FunctionCall {
            function: (match operands.next() {
                Some(dr::Operand::IdRef(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            argument_0_argument_1: {
                let mut vec = Vec::new();
                while let Some(item) = match operands.next() {
                    Some(dr::Operand::IdRef(value)) => Some(*value),
                    Some(_) => return Err(OperandError::WrongType.into()),
                    None => None,
                } {
                    vec.push(item);
                }
                vec
            },
        })
    }
    #[allow(unused)]
    pub fn lift_execution_mode_id(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExecutionModeId, InstructionError> {
        if raw.class.opcode as u32 != 331u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExecutionModeId {
            entry_point: (match operands.next() {
                Some(dr::Operand::IdRef(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            mode: (match operands.next() {
                Some(dr::Operand::ExecutionMode(value)) => Some(*value),
                Some(_) => return Err(OperandError::WrongType.into()),
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
}
