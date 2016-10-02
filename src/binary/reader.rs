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
        let mut builder = mr::Builder::new();
        let header = try!(self.read_header(&mut producer));
        println!("{:?}", header);
        builder.initialize(header);

        loop {
            match self.read_inst(&mut producer) {
                Ok((opcode, operands)) => {
                    match builder.add_instruction(opcode, operands) {
                        mr::BuilderState::Normal => continue,
                        mr::BuilderState::OpcodeUnknown => return Err(State::OpcodeUnknown),
                        mr::BuilderState::OperandExpected => return Err(State::OperandExpected),
                    }
                }
                Err(State::Complete) => break,
                Err(error) => return Err(error),
            }
        }

        Ok(builder.finalize())
    }
}
