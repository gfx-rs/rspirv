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

pub struct Reader<'a> {
    producer: producer::Producer,
    builder: mr::Builder<'a>,
}

impl<'a> Reader<'a> {
    pub fn new() -> Reader<'a> {
        Reader {
            producer: producer::Producer::new(),
            builder: mr::Builder::new(),
        }
    }

    fn split_into_word_count_and_opcode(&self, word: spirv::Word) -> (u16, u16) {
        ((word >> 16) as u16, (word & 0xffff) as u16)
    }

    fn process_header(&mut self) -> Result<mr::ModuleHeader> {
        if let Ok(words) = self.producer.get_next_n_words(HEADER_NUM_WORDS) {
            if words[0] != MAGIC_NUMBER {
                return Err(State::HeaderIncorrect);
            }
            let header = mr::ModuleHeader {
                magic_number: words[0],
                version: words[1],
                generator: words[2],
                bound: words[3],
                reserved_word: words[4],
            };
            Ok(header)
        } else {
            Err(State::HeaderIncomplete)
        }
    }

    fn process_instruction(&mut self) -> Result<(u16, Vec<spirv::Word>)> {
        if let Ok(word) = self.producer.get_next_word() {
            let (wc, opcode) = self.split_into_word_count_and_opcode(word);
            if let Ok(words) = self.producer.get_next_n_words((wc - 1) as usize) {
                Ok((opcode, words))
            } else {
                Err(State::InstructionIncomplete)
            }
        } else {
            Err(State::Complete)
        }
    }

    pub fn process(&mut self, binary: Vec<u8>) -> Result<mr::Module<'a>> {
        self.producer.set_data(binary);
        let header = try!(self.process_header());
        println!("{:?}", header);
        self.builder.initialize(header);

        loop {
            match self.process_instruction() {
                Ok((opcode, operands)) => {
                    match self.builder.add_instruction(opcode, operands) {
                        mr::BuilderState::Normal => continue,
                        mr::BuilderState::OpcodeUnknown => return Err(State::OpcodeUnknown),
                        mr::BuilderState::OperandExpected => return Err(State::OperandExpected),
                    }
                }
                Err(State::Complete) => break,
                Err(error) => return Err(error),
            }
        }

        Ok(self.builder.finalize().unwrap())
    }
}
