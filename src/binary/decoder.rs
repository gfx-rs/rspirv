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

use std::result;
use super::error::Error;

pub type Result<T> = result::Result<T, Error>;

const WORD_NUM_BYTES: usize = 4;

pub struct Decoder {
    bytes: Vec<u8>,
    index: usize, // Index for next byte to decode.
    limit: Option<usize>,
}

impl Decoder {
    pub fn new(bytes: Vec<u8>) -> Decoder {
        Decoder {
            bytes: bytes,
            index: 0,
            limit: None,
        }
    }

    pub fn set_limit(&mut self, num_words: usize) {
        self.limit = Some(num_words)
    }

    pub fn clear_limit(&mut self) {
        self.limit = None
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    pub fn limit_reached(&self) -> bool {
        if let Some(left) = self.limit {
            left == 0
        } else {
            false
        }
    }

    pub fn word(&mut self) -> Result<spirv::Word> {
        if self.has_limit() {
            if self.limit_reached() {
                return Err(Error::LimitReached(self.index));
            } else {
                *self.limit.as_mut().unwrap() -= 1
            }
        }

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

impl Decoder {
    pub fn id(&mut self) -> Result<spirv::Word> {
        self.word()
    }

    /// Splits the given word into a vector of bytes in little-endian format.
    ///
    /// NOTE: I know it's duplicate work to group bytes into words and then
    /// split them again. But it's nice to have word() take full control of
    /// limit checking.
    fn split_word_to_bytes(word: spirv::Word) -> Vec<u8> {
        (0..WORD_NUM_BYTES).map(|i| ((word >> (8 * i)) & 0xff) as u8).collect()
    }

    pub fn string(&mut self) -> Result<String> {
        let start_index = self.index;
        let mut bytes = vec![];
        while let Ok(word) = self.word() {
            bytes.append(&mut Decoder::split_word_to_bytes(word));
            if bytes.last() == Some(&0) {
                break;
            }
        }
        while !bytes.is_empty() && bytes.last() == Some(&0) {
            bytes.pop();
        }
        String::from_utf8(bytes)
            .map_err(|e| Error::DecodeStringFailed(start_index, e))
    }

    pub fn integer(&mut self) -> Result<u32> {
        self.word()
    }

    // TODO(antiagainst): This should return the correct typed number.
    pub fn context_dependent_number(&mut self) -> Result<u32> {
        self.word()
    }

    pub fn spec_constant_op_integer(&mut self) -> Result<u32> {
        self.word()
    }

    pub fn ext_inst_integer(&mut self) -> Result<u32> {
        self.word()
    }
}

include!("decode_operand.rs");
