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

use super::decoder;

use std::result;

use grammar::InstructionTable as GInstTable;
use grammar::OperandKind as GOpKind;
use grammar::OperandQuantifier as GOpCount;

type GInstRef = &'static grammar::Instruction<'static>;

#[derive(Clone, Copy, Debug)]
pub enum State {
    Complete,
    HeaderIncomplete,
    HeaderIncorrect,
    InstructionIncomplete,
    OpcodeUnknown,
    OperandExpected,
}

pub type Result<T> = result::Result<T, State>;

const HEADER_NUM_WORDS: usize = 5;
const MAGIC_NUMBER: spirv::Word = 0x07230203;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseAction {
    Continue,
    Stop,
    Fail,
}

pub trait Consumer {
    fn consume_header(&mut self, module: mr::ModuleHeader) -> ParseAction;
    fn consume_instruction(&mut self, inst: mr::Instruction) -> ParseAction;
}

struct Parser<'a> {
    decoder: decoder::Decoder,
    consumer: &'a mut Consumer,
}

impl<'a> Parser<'a> {
    pub fn new(binary: Vec<u8>, consumer: &'a mut Consumer) -> Parser<'a> {
        Parser {
            decoder: decoder::Decoder::new(binary),
            consumer: consumer,
        }
    }

    pub fn parse(mut self) -> Result<()> {
        let header = try!(self.parse_header());
        if self.consumer.consume_header(header) == ParseAction::Stop {
            return Ok(());
        }

        loop {
            let result = self.parse_inst();
            match result {
                Ok(inst) => {
                    if self.consumer.consume_instruction(inst) == ParseAction::Stop {
                        return Ok(());
                    }
                }
                Err(State::Complete) => break,
                Err(error) => return Err(error),
            };
        }
        Ok(())
    }

    fn split_into_word_count_and_opcode(word: spirv::Word) -> (u16, u16) {
        ((word >> 16) as u16, (word & 0xffff) as u16)
    }

    fn parse_header(&mut self) -> Result<mr::ModuleHeader> {
        if let Ok(words) = self.decoder.words(HEADER_NUM_WORDS) {
            if words[0] != MAGIC_NUMBER {
                return Err(State::HeaderIncorrect);
            }
            Ok(mr::ModuleHeader::new(words[0], words[1], words[2], words[3], words[4]))
        } else {
            Err(State::HeaderIncomplete)
        }
    }

    fn parse_inst(&mut self) -> Result<mr::Instruction> {
        if let Ok(word) = self.decoder.word() {
            let (wc, opcode) = Parser::split_into_word_count_and_opcode(word);
            assert!(wc > 0);
            if let Some(grammar) = GInstTable::lookup_opcode(opcode) {
                self.decoder.set_limit((wc - 1) as usize);
                let result = self.decode_words_to_operands(grammar);
                assert!(self.decoder.limit_reached());
                self.decoder.clear_limit();
                result
            } else {
                Err(State::OpcodeUnknown)
            }
        } else {
            Err(State::Complete)
        }
    }

    fn decode_words_to_operands(&mut self, grammar: GInstRef) -> Result<mr::Instruction> {
        let mut rtype = None;
        let mut rid = None;
        let mut concrete_operands = Vec::new();

        let mut logical_operand_index: usize = 0;
        while logical_operand_index < grammar.operands.len() {
            let logical_operand = &grammar.operands[logical_operand_index];
            let has_more_operands = !self.decoder.limit_reached();
            if has_more_operands {
                match logical_operand.kind {
                    GOpKind::IdResultType => rtype = self.decoder.id().ok(),
                    GOpKind::IdResult => rid = self.decoder.id().ok(),
                    _ => {
                        concrete_operands.push(self.decode_operand(logical_operand.kind)
                                                   .unwrap());
                        if let mr::Operand::Decoration(decoration) = *concrete_operands.last()
                                                                                       .unwrap() {
                            concrete_operands.append(
                                &mut self.decode_decoration_arguments(
                                     decoration).unwrap());
                        }
                    }
                }
                match logical_operand.quantifier {
                    GOpCount::One | GOpCount::ZeroOrOne => logical_operand_index += 1,
                    GOpCount::ZeroOrMore => continue,
                }
            } else {
                // We still have logical operands to match but no no more words.
                match logical_operand.quantifier {
                    GOpCount::One => return Err(State::OperandExpected),
                    GOpCount::ZeroOrOne | GOpCount::ZeroOrMore => break,
                }
            }
        }
        Ok(mr::Instruction::new(grammar, rtype, rid, concrete_operands))
    }

    fn decode_operand(&mut self, kind: GOpKind) -> Result<mr::Operand> {
        Ok(match kind {
            GOpKind::IdResultType => mr::Operand::IdResultType(self.decoder.id().unwrap()),
            GOpKind::IdResult => mr::Operand::IdResult(self.decoder.id().unwrap()),
            GOpKind::IdRef |
            GOpKind::IdMemorySemantics |
            GOpKind::IdScope => mr::Operand::IdRef(self.decoder.id().unwrap()),
            GOpKind::Scope => mr::Operand::Scope(self.decoder.scope().unwrap()),
            GOpKind::MemorySemantics => {
                mr::Operand::MemorySemantics(self.decoder.memory_semantics().unwrap())
            }
            GOpKind::LiteralString => mr::Operand::LiteralString(self.decoder.string().unwrap()),
            GOpKind::LiteralContextDependentNumber => {
                mr::Operand::LiteralContextDependentNumber(self.decoder
                                                               .context_dependent_number()
                                                               .unwrap())
            }
            GOpKind::Capability => mr::Operand::Capability(self.decoder.capability().unwrap()),
            GOpKind::Decoration => mr::Operand::Decoration(self.decoder.decoration().unwrap()),
            GOpKind::AddressingModel => {
                mr::Operand::AddressingModel(self.decoder
                                                 .addressing_model()
                                                 .unwrap())
            }
            GOpKind::MemoryModel => mr::Operand::MemoryModel(self.decoder.memory_model().unwrap()),
            GOpKind::ExecutionMode => {
                mr::Operand::ExecutionMode(self.decoder
                                               .execution_mode()
                                               .unwrap())
            }
            GOpKind::ExecutionModel => {
                mr::Operand::ExecutionModel(self.decoder.execution_model().unwrap())
            }
            GOpKind::SourceLanguage => {
                mr::Operand::SourceLanguage(self.decoder
                                                .source_language()
                                                .unwrap())
            }
            GOpKind::LiteralInteger => mr::Operand::LiteralInteger(self.decoder.integer().unwrap()),
            GOpKind::StorageClass => {
                mr::Operand::StorageClass(self.decoder.storage_class().unwrap())
            }
            GOpKind::ImageOperands => {
                mr::Operand::ImageOperands(self.decoder.image_operands().unwrap())
            }
            GOpKind::FPFastMathMode => {
                mr::Operand::FPFastMathMode(self.decoder.fpfast_math_mode().unwrap())
            }
            GOpKind::SelectionControl => {
                mr::Operand::SelectionControl(self.decoder.selection_control().unwrap())
            }
            GOpKind::LoopControl => mr::Operand::LoopControl(self.decoder.loop_control().unwrap()),
            GOpKind::FunctionControl => {
                mr::Operand::FunctionControl(self.decoder.function_control().unwrap())
            }
            GOpKind::MemoryAccess => {
                mr::Operand::MemoryAccess(self.decoder.memory_access().unwrap())
            }
            GOpKind::KernelProfilingInfo => {
                mr::Operand::KernelProfilingInfo(self.decoder
                                                     .kernel_profiling_info()
                                                     .unwrap())
            }
            GOpKind::Dim => mr::Operand::Dim(self.decoder.dim().unwrap()),
            GOpKind::SamplerAddressingMode => {
                mr::Operand::SamplerAddressingMode(self.decoder
                                                       .sampler_addressing_mode()
                                                       .unwrap())
            }
            GOpKind::SamplerFilterMode => {
                mr::Operand::SamplerFilterMode(self.decoder.sampler_filter_mode().unwrap())
            }
            GOpKind::ImageFormat => mr::Operand::ImageFormat(self.decoder.image_format().unwrap()),
            GOpKind::ImageChannelOrder => {
                mr::Operand::ImageChannelOrder(self.decoder.image_channel_order().unwrap())
            }
            GOpKind::ImageChannelDataType => {
                mr::Operand::ImageChannelDataType(self.decoder
                                                      .image_channel_data_type()
                                                      .unwrap())
            }
            GOpKind::FPRoundingMode => {
                mr::Operand::FPRoundingMode(self.decoder.fprounding_mode().unwrap())
            }
            GOpKind::LinkageType => mr::Operand::LinkageType(self.decoder.linkage_type().unwrap()),
            GOpKind::AccessQualifier => {
                mr::Operand::AccessQualifier(self.decoder.access_qualifier().unwrap())
            }
            GOpKind::FunctionParameterAttribute => {
                mr::Operand::FunctionParameterAttribute(self.decoder
                                                            .function_parameter_attribute()
                                                            .unwrap())
            }
            GOpKind::BuiltIn => mr::Operand::BuiltIn(self.decoder.built_in().unwrap()),
            GOpKind::GroupOperation => {
                mr::Operand::GroupOperation(self.decoder.group_operation().unwrap())
            }
            GOpKind::KernelEnqueueFlags => {
                mr::Operand::KernelEnqueueFlags(self.decoder.kernel_enqueue_flags().unwrap())
            }
            GOpKind::LiteralExtInstInteger |
            GOpKind::LiteralSpecConstantOpInteger |
            GOpKind::PairLiteralIntegerIdRef |
            GOpKind::PairIdRefLiteralInteger |
            GOpKind::PairIdRefIdRef => {
                println!("unimplemented operand kind: {:?}", kind);
                unimplemented!();
            }
        })
    }

    fn decode_decoration_arguments(&mut self,
                                   decoration: spirv::Decoration)
                                   -> Result<Vec<mr::Operand>> {
        match decoration {
            spirv::Decoration::BuiltIn => {
                Ok(vec![mr::Operand::BuiltIn(self.decoder.built_in().unwrap())])
            }
            spirv::Decoration::Block => Ok(vec![]),
            _ => unimplemented!(),

        }
    }
}

pub fn parse(binary: Vec<u8>, consumer: &mut Consumer) -> Result<()> {
    Parser::new(binary, consumer).parse()
}
