//! The SPIR-V header.
//!
//! This crate contains Rust definitions of all SPIR-V structs, enums,
//! and constants.
//!
//! The version-metadata of this crate specifies the [SPIRV-Headers] git-tag
//! it is generated from. The corresponding SPIR-V specification is
#![doc = concat!("Version ", version_major!(), ".", version_minor!())]
#![doc = concat!("Revision ", version_revision!(), ".")]
//!
//! [SPIRV-Headers]: https://github.com/KhronosGroup/SPIRV-Headers

#![no_std]
#![allow(non_camel_case_types)]
#![deny(clippy::std_instead_of_core, clippy::alloc_instead_of_core)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use bitflags::bitflags;

pub const MAJOR_VERSION: u8 = version_major!();
pub const MINOR_VERSION: u8 = version_minor!();
pub const REVISION: u8 = version_revision!();

include!("autogen_spirv.rs");
pub(crate) use {version_major, version_minor, version_revision};

impl From<Op> for Word {
    // Exists because of repr()
    fn from(value: Op) -> Self {
        value as Word
    }
}
