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
use spirv;

use super::producer;

use std::result;

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

pub struct Reader {}

impl Reader {
    pub fn new() -> Reader {
        Reader {}
    }

    fn split_into_word_count_and_opcode(word: spirv::Word) -> (u16, u16) {
        ((word >> 16) as u16, (word & 0xffff) as u16)
    }

    fn read_header(&self, producer: &mut producer::Producer) -> Result<mr::ModuleHeader> {
        if let Ok(words) = producer.get_next_n_words(HEADER_NUM_WORDS) {
            if words[0] != MAGIC_NUMBER {
                return Err(State::HeaderIncorrect);
            }
            Ok(mr::ModuleHeader::new(words[0], words[1], words[2], words[3], words[4]))
        } else {
            Err(State::HeaderIncomplete)
        }
    }

    fn read_inst(&self, producer: &mut producer::Producer) -> Result<(u16, Vec<spirv::Word>)> {
        if let Ok(word) = producer.get_next_word() {
            let (wc, opcode) = Reader::split_into_word_count_and_opcode(word);
            if let Ok(words) = producer.get_next_n_words((wc - 1) as usize) {
                Ok((opcode, words))
            } else {
                Err(State::InstructionIncomplete)
            }
        } else {
            Err(State::Complete)
        }
    }

    pub fn read(&self, binary: Vec<u8>) -> Result<mr::Module> {
        let mut producer = producer::Producer::new(binary);
        let mut loader = mr::Loader::new();
        let header = try!(self.read_header(&mut producer));
        loader.initialize(header);

        loop {
            match self.read_inst(&mut producer) {
                Ok((opcode, operands)) => {
                    match loader.add_instruction(opcode, operands) {
                        mr::LoaderState::Normal => continue,
                        mr::LoaderState::OpcodeUnknown => return Err(State::OpcodeUnknown),
                        mr::LoaderState::OperandExpected => return Err(State::OperandExpected),
                    }
                }
                Err(State::Complete) => break,
                Err(error) => return Err(error),
            }
        }

        Ok(loader.finalize())
    }
}
