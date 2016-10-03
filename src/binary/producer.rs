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

#[derive(Clone, Copy, Debug)]
pub enum Error {
    StreamExpected,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expected more bytes in the stream")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "expected more bytes in the stream"
    }
}

pub type Result<T> = result::Result<T, Error>;

const WORD_NUM_BYTES: usize = 4;

pub struct Producer {
    data: Vec<u8>,
    position: usize,
}

impl Producer {
    pub fn new(data: Vec<u8>) -> Producer {
        Producer {
            data: data,
            position: 0,
        }
    }

    pub fn get_next_word(&mut self) -> Result<spirv::Word> {
        if self.position >= self.data.len() {
            Err(Error::StreamExpected)
        } else if self.position + WORD_NUM_BYTES > self.data.len() {
            Err(Error::StreamExpected)
        } else {
            self.position += WORD_NUM_BYTES;
            Ok((0..WORD_NUM_BYTES).fold(0, |word, i| {
                (word << 8) | (self.data[self.position - i - 1]) as u32
            }))
        }
    }

    pub fn get_next_n_words(&mut self, n: usize) -> Result<Vec<spirv::Word>> {
        let mut words = Vec::new();
        for _ in 0..n {
            words.push(try!(self.get_next_word()));
        }
        Ok(words)
    }
}
