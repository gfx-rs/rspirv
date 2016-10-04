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

use spirv;

use std::{fmt, error, result};

use super::error::Error;

pub type Result<T> = result::Result<T, Error>;

const WORD_NUM_BYTES: usize = 4;

pub struct WordDecoder {
    bytes: Vec<u8>,
    index: usize, // Index for next byte to decode.
}

impl WordDecoder {
    pub fn new(bytes: Vec<u8>) -> WordDecoder {
        WordDecoder {
            bytes: bytes,
            index: 0,
        }
    }

    pub fn word(&mut self) -> Result<spirv::Word> {
        if self.index >= self.bytes.len() {
            Err(Error::StreamExpected(self.index))
        } else if self.index + WORD_NUM_BYTES > self.bytes.len() {
            Err(Error::StreamExpected(self.index))
        } else {
            self.index += WORD_NUM_BYTES;
            Ok((0..WORD_NUM_BYTES).fold(0, |word, i| {
                (word << 8) | (self.bytes[self.index - i - 1]) as u32
            }))
        }
    }

    pub fn words(&mut self, n: usize) -> Result<Vec<spirv::Word>> {
        let mut words = Vec::new();
        for _ in 0..n {
            words.push(try!(self.word()));
        }
        Ok(words)
    }
}

pub struct OperandDecoder {
    words: Vec<spirv::Word>,
    index: usize,
}

impl Iterator for OperandDecoder {
    type Item = spirv::Word;

    fn next(&mut self) -> Option<spirv::Word> {
        if self.empty() {
            None
        } else {
            self.index += 1;
            Some(self.words[self.index - 1])
        }
    }
}

impl OperandDecoder {
    pub fn new(words: Vec<spirv::Word>) -> OperandDecoder {
        OperandDecoder {
            words: words,
            index: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.index >= self.words.len()
    }

    fn split_word_to_bytes(&self, word: spirv::Word) -> Vec<u8> {
        (0..WORD_NUM_BYTES).map(|i| ((word >> (8 * i)) & 0xff) as u8).collect()
    }

    pub fn string(&mut self) -> Option<String> {
        let mut bytes = Vec::new();
        while let Some(word) = self.next() {
            bytes.append(&mut self.split_word_to_bytes(word));
            if bytes.last() == Some(&0) {
                break;
            }
        }
        while !bytes.is_empty() && bytes.last() == Some(&0) {
            bytes.pop();
        }
        String::from_utf8(bytes).ok()
    }

    pub fn id(&mut self) -> Option<spirv::Word> {
        self.next()
    }

    pub fn literal_integer(&mut self) -> Option<u32> {
        self.next()
    }

    pub fn context_dependent_number(&mut self) -> Option<u32> {
        // TODO(antiagainst): This should return the correct typed number.
        self.next()
    }
}

include!("decode_operand.rs");
