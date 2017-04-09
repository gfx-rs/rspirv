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

//! Data representation of various SPIR-V language constructs.
//!
//! By language constructs, I mean general language concepts like module,
//! function, basic block, instruction, and operands. This is different
//! from the "control flow constructs" mentioned in the SPIR-V
//! [specification](https://goo.gl/YQRcZT).
//!
//! This data representation is designed to be lightweight; there are
//! no excessive sanity check or cross referrences within each language
//! construct. It is intended to be used as a plain data vehicle of
//! SPIR-V language constructs in the memory.
//!
//! Required components of a language construct may still be wrapped around
//! using `Option`; it makes the data representation more flexible since
//! we don't always require valid language constructs.
//!
//! Apart from definitions of various language constructs, this module also
//! provides a [loader](struct.Loader.html) for loading SPIR-V binaries
//! (together with the [parser](../binary/struct.Parser.html)) and a
//! [builder](struct.Builder.html) for building a SPIR-V data representation
//! interactively.

pub use self::builder::Builder;
pub use self::constructs::{BasicBlock, Function, Instruction, InstIter};
pub use self::constructs::{Module, ModuleHeader, Operand};
pub use self::loader::{Error, load_bytes, load_words, Loader};

mod builder;
mod constructs;
mod loader;
