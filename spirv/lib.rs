// Copyright 2017 Google Inc.
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

#![doc(html_root_url = "https://docs.rs/spirv_headers/1.3/")]

//! The SPIR-V header.
//!
//! This crate contains Rust definitions of all SPIR-V structs, enums,
//! and constants.
//!
//! The version of this crate is the version of SPIR-V it contains.

#![allow(non_camel_case_types)]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
extern crate bitflags;
extern crate num;
#[macro_use]
extern crate num_derive;

include!("spirv.rs");
