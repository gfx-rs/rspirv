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

use crate::spirv;

use super::storage::Token;

/// Represents a SPIR-V constant.
#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Bool(bool),
    // TODO: Change to arbitrary width datatype
    Int(i32),
    UInt(u32),
    Float(f32),
    Composite(Vec<Token<Constant>>),
    Null,
    Sampler(spirv::SamplerAddressingMode, u32, spirv::SamplerFilterMode),
    SpecBool(bool),
    SpecInt(i32),
    SpecUInt(u32),
    SpecFloat(f32),
    SpecComposite(Vec<Token<Constant>>),
    SpecOp(spirv::Op, Vec<Token<Constant>>),
}

impl Constant {
    pub fn is_bool_constant(&self) -> bool {
        match self {
            Constant::Bool { .. } |
            Constant::SpecBool { .. } => true,
            _ => false,
        }
    }

    pub fn is_int_constant(&self) -> bool {
        match self {
            Constant::Int { .. } |
            Constant::SpecInt { .. } => true,
            _ => false,
        }
    }

    pub fn is_uint_constant(&self) -> bool {
        match self {
            Constant::UInt { .. } |
            Constant::SpecUInt { .. } => true,
            _ => false,
        }
    }

    pub fn is_float_constant(&self) -> bool {
        match self {
            Constant::Float { .. } |
            Constant::SpecFloat { .. } => true,
            _ => false,
        }
    }

    pub fn is_composite_constant(&self) -> bool {
        match self {
            Constant::Composite { .. } |
            Constant::SpecComposite { .. } => true,
            _ => false,
        }
    }

    pub fn is_null_constant(&self) -> bool {
        match self {
            Constant::Null { .. } => true,
            _ => false,
        }
    }

    pub fn is_sampler_constant(&self) -> bool {
        match self {
            Constant::Sampler { .. } => true,
            _ => false,
        }
    }

    pub fn is_spec_constant(&self) -> bool {
        match self {
            Constant::SpecBool { .. } |
            Constant::SpecInt { .. } |
            Constant::SpecUInt { .. } |
            Constant::SpecFloat { .. } |
            Constant::SpecComposite { .. } |
            Constant::SpecOp { .. } => true,
            _ => false,
        }
    }

    pub fn is_spec_op_constant(&self) -> bool {
        match self {
            Constant::SpecOp { .. } => true,
            _ => false,
        }
    }
}
