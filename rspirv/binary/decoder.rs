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

/// The SPIR-V binary decoder.
///
/// Takes in a vector of bytes, and serves requests for raw SPIR-V words
/// or values of a specific SPIR-V enum type.
///
/// Different from the [`Parser`](struct.Parser.html),
/// this decoder is low-level; it has no knowledge of the SPIR-V grammar.
/// Given a vector of bytes, it solely responds to word decoding requests
/// via method calls: both raw words requests and decoding the raw words
/// into a value of a specific SPIR-V enum type.
///
/// It also provides a limit mechanism. Users can set a limit, and then
/// requesting words. If that limit is reached before the end of the
/// stream, [`State::LimitReached`](enum.ParseState.html) will be
/// returned.
pub struct Decoder {
    /// Raw bytes to decode
    bytes: Vec<u8>,
    /// Offset for next byte to decode
    offset: usize,
    /// Remaining limit of number of words before error
    limit: Option<usize>,
}

impl Decoder {
    /// Creates a new `Decoder` instance.
    pub fn new(bytes: Vec<u8>) -> Decoder {
        Decoder {
            bytes: bytes,
            offset: 0,
            limit: None,
        }
    }

    /// Returns the offset of the byte to decode next.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Decodes and returns the next raw SPIR-V word.
    pub fn word(&mut self) -> Result<spirv::Word> {
        if self.has_limit() {
            if self.limit_reached() {
                return Err(Error::LimitReached(self.offset));
            } else {
                *self.limit.as_mut().unwrap() -= 1
            }
        }

        if self.offset >= self.bytes.len() {
            Err(Error::StreamExpected(self.offset))
        } else if self.offset + WORD_NUM_BYTES > self.bytes.len() {
            Err(Error::StreamExpected(self.offset))
        } else {
            self.offset += WORD_NUM_BYTES;
            Ok((0..WORD_NUM_BYTES).fold(0, |word, i| {
                (word << 8) | (self.bytes[self.offset - i - 1]) as u32
            }))
        }
    }

    /// Decodes and returns the next `n` raw SPIR-V words.
    pub fn words(&mut self, n: usize) -> Result<Vec<spirv::Word>> {
        let mut words = Vec::new();
        for _ in 0..n {
            words.push(try!(self.word()));
        }
        Ok(words)
    }
}

impl Decoder {
    /// Sets the limit to `num_words` words.
    ///
    /// The decoder will return [`State::LimitReached`](enum.ParseState.html)
    /// after `num_words` words have been requested, if having not consumed
    /// the whole stream.
    pub fn set_limit(&mut self, num_words: usize) {
        self.limit = Some(num_words)
    }

    /// Clear the previously set limit (if any).
    pub fn clear_limit(&mut self) {
        self.limit = None
    }

    /// Returns true if a limit has been set on this decoder.
    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    /// Returns true if the previously set limit has been reached.
    ///
    /// This will always return false if no limit has been ever set.
    pub fn limit_reached(&self) -> bool {
        if let Some(left) = self.limit {
            left == 0
        } else {
            false
        }
    }
}

impl Decoder {
    /// Decodes and returns the next SPIR-V word as an id.
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

    /// Decodes and returns a literal string.
    ///
    /// This method will consume as many words as necessary, util a null
    /// character (`\0`) is reached, or errored out.
    pub fn string(&mut self) -> Result<String> {
        let start_index = self.offset;
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
        String::from_utf8(bytes).map_err(
                |e| Error::DecodeStringFailed(start_index, format!("{}", e)))
    }

    /// Decodes and returns the next SPIR-V word as a 32-bit
    /// literal integer.
    pub fn integer(&mut self) -> Result<u32> {
        self.word()
    }

    /// Decodes and returns the next SPIR-V word as a 32-bit
    /// context-dependent number.
    // TODO(antiagainst): This should return the correct typed number.
    pub fn context_dependent_number(&mut self) -> Result<u32> {
        self.word()
    }

    /// Decodes and returns the next SPIR-V word as a 32-bit
    /// spec-constant-op integer.
    pub fn spec_constant_op_integer(&mut self) -> Result<u32> {
        self.word()
    }

    /// Decodes and returns the next SPIR-V word as a 32-bit
    /// extended-instruction-set number.
    pub fn ext_inst_integer(&mut self) -> Result<u32> {
        self.word()
    }
}

include!("decode_operand.rs");
