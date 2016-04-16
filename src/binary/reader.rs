use builder;
use mr;
use spirv;
use binary::producer;

use std::result;

pub enum State {
    HeaderIncomplete,
    HeaderIncorrect,
}

pub type Result<T> = result::Result<T, State>;

const HEADER_NUM_WORDS: usize = 5;
const MAGIC_NUMBER: spirv::Word = 0x07230203;

pub struct Reader<'a> {
    producer: producer::Producer,
    builder: builder::Builder<'a>,
}

impl<'a> Reader<'a> {
    pub fn new() -> Reader<'a> {
        Reader {
            producer: producer::Producer::new(),
            builder: builder::Builder::new(),
        }
    }

    fn split_into_word_count_and_opcode(word: spirv::Word) -> (u16, u16) {
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

    pub fn process(&mut self, binary: Vec<u8>) -> Result<mr::Module> {
        self.producer.set_data(binary);
        let header = try!(self.process_header());
        println!("{:?}", header);
        self.builder.initialize(header);

        Ok(self.builder.finalize().unwrap())
    }
}
