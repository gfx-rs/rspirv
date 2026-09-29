//! The SPIR-V header.
//!
//! This crate contains Rust definitions of all SPIR-V structs, enums,
//! and constants.
//!
//! These bindings have been generated from the <https://github.com/KhronosGroup/SPIRV-Headers> tag
//! or release described by this crates' version metadata (the value after the `+`).

#![no_std]
#![allow(non_camel_case_types)]
#![deny(clippy::std_instead_of_core, clippy::alloc_instead_of_core)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use bitflags::bitflags;

include!("autogen_spirv.rs");

impl From<Op> for Word {
    // Exists because of repr()
    fn from(value: Op) -> Self {
        value as Word
    }
}
