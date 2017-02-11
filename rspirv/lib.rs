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
pub mod ir;

mod utils;
