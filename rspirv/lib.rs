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

//! Library APIs for SPIR-V module processing functionalities.
//!
//! This library provides:
//!
//! * The whole SPIR-V [grammar](grammar/index.html) (instruction layouts
//!   and their operands)
//! * A [memory representation](mr/index.html) of SPIR-V modules and its
//!   loader and builder
//! * SPIR-V [binary](binary/index.html) module decoding and parsing
//!   functionalities
//!
//! # Examples
//!
//! Building a SPIR-V module, assembling it, parsing it, and then
//! disassembling it:
//!
//! ```
//! extern crate rspirv;
//! extern crate spirv_headers as spirv;
//!
//! use rspirv::binary::Assemble;
//! use rspirv::binary::Disassemble;
//!
//! fn main() {
//!     // Building
//!     let mut b = rspirv::mr::Builder::new();
//!     b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);
//!     let void = b.type_void();
//!     let voidf = b.type_function(void, vec![void]);
//!     b.begin_function(void,
//!                      (spirv::FUNCTION_CONTROL_DONT_INLINE |
//!                       spirv::FUNCTION_CONTROL_CONST),
//!                      voidf)
//!      .unwrap();
//!     b.begin_basic_block().unwrap();
//!     b.ret().unwrap();
//!     b.end_function().unwrap();
//!     let module = b.module();
//!
//!     // Assembling
//!     let code = module.assemble();
//!     assert!(code.len() > 20);  // Module header contains 5 words
//!     assert_eq!(spirv::MAGIC_NUMBER, code[0]);
//!
//!     // Parsing
//!     let mut loader = rspirv::mr::Loader::new();
//!     rspirv::binary::parse_words(&code, &mut loader).unwrap();
//!     let module = loader.module();
//!
//!     // Disassembling
//!     assert_eq!(module.disassemble(),
//!                "; SPIR-V\n\
//!                 ; Version: 1.1\n\
//!                 ; Generator: Unknown\n\
//!                 ; Bound: 5\n\
//!                 OpMemoryModel Logical GLSL450\n\
//!                 %1 = OpTypeVoid\n\
//!                 %2 = OpTypeFunction %1 %1\n\
//!                 %3 = OpFunction  %1  DontInline|Const %2\n\
//!                 %4 = OpLabel\n\
//!                 OpReturn\n\
//!                 OpFunctionEnd");
//! }
//! ```

#[cfg(test)]
#[macro_use]
extern crate assert_matches;
extern crate num;
#[macro_use]
extern crate num_derive;
extern crate spirv_headers as spirv;

pub mod binary;
pub mod grammar;
pub mod mr;

mod utils;
