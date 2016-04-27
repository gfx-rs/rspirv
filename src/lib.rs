#![feature(custom_derive, plugin)]
#![plugin(num_macros)]

#[macro_use]
extern crate bitflags;
extern crate num;

pub mod binary;
pub mod grammar;
pub mod mr;
pub mod spirv;
