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
use super::error::Error as DecodeError;

use std::result;

use grammar::InstructionTable as GInstTable;
use grammar::OperandKind as GOpKind;
use grammar::OperandQuantifier as GOpCount;

type GInstRef = &'static grammar::Instruction<'static>;

#[derive(Debug)]
pub enum State {
    Complete,
    HeaderIncomplete,
    HeaderIncorrect,
    InstructionIncomplete,
    OpcodeUnknown,
    OperandExpected,
    OperandError(DecodeError),
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

macro_rules! try_decode {
    ($e: expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(State::OperandError(err))
    });
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
                let result = self.parse_operands(grammar);
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

    fn parse_operands(&mut self, grammar: GInstRef) -> Result<mr::Instruction> {
        let mut rtype = None;
        let mut rid = None;
        let mut concrete_operands = vec![];

        let mut logical_operand_index: usize = 0;
        while logical_operand_index < grammar.operands.len() {
            let logical_operand = &grammar.operands[logical_operand_index];
            let has_more_operands = !self.decoder.limit_reached();
            if has_more_operands {
                match logical_operand.kind {
                    GOpKind::IdResultType => rtype = Some(try_decode!(self.decoder.id())),
                    GOpKind::IdResult => rid = Some(try_decode!(self.decoder.id())),
                    _ => concrete_operands.append(
                        &mut try!(self.parse_operand(logical_operand.kind))),
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
}

include!("parse_operand.rs");

pub fn parse(binary: Vec<u8>, consumer: &mut Consumer) -> Result<()> {
    Parser::new(binary, consumer).parse()
}
