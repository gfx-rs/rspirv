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

//! Module for SPIR-V binary processing.
//!
//! This module provides a [`Decoder`](struct.Decoder.html) and a
//! [`Parser`](struct.Parser.html):
//!
//! * The decoder is a low-level binary processing tool; it has no knowlege
//!   of the SPIR-V grammar. It only serves SPIR-V word requests.
//! * The parser is a high-level binary processing tool; it has knowledge
//!   of the SPIR-V grammar. It works with the
//!   [`Consumer`](trait.Consumer.html) to process a SPIR-V binary on the
//!   instruction level.

pub use self::decoder::Decoder;
pub use self::error::Error as DecodeError;
pub use self::parser::{Consumer, parse_bytes, parse_words, Parser};
pub use self::parser::Action as ParseAction;
pub use self::parser::Result as ParseResult;
pub use self::parser::State as ParseState;

pub use self::disassemble::Disassemble;

mod decoder;
mod disassemble;
mod error;
mod parser;
mod tracker;
