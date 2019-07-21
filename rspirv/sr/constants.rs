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

use super::{Token, Type};

/// The class to represent a SPIR-V constant.
#[derive(Clone, Debug, PartialEq)]
pub struct Constant {
    pub(in crate::sr) c: ConstantEnum,
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::sr) enum ConstantEnum {
    Bool(bool),
    I32(i32),
    U32(u32),
    F32(f32),
    Composite(Vec<Token<Constant>>),
    Null(Token<Type>),
    Sampler(spirv::SamplerAddressingMode, u32, spirv::SamplerFilterMode),
    SpecBool(bool),
    SpecI32(i32),
    SpecU32(u32),
    SpecF32(f32),
    SpecComposite(Vec<Token<Constant>>),
    SpecOp(spirv::Op, Vec<Token<Constant>>),
}

impl Constant {
    pub fn is_bool_constant(&self) -> bool {
        match self.c {
            ConstantEnum::Bool { .. } |
            ConstantEnum::SpecBool { .. } => true,
            _ => false,
        }
    }

    pub fn is_i32_constant(&self) -> bool {
        match self.c {
            ConstantEnum::I32 { .. } |
            ConstantEnum::SpecI32 { .. } => true,
            _ => false,
        }
    }

    pub fn is_u32_constant(&self) -> bool {
        match self.c {
            ConstantEnum::U32 { .. } |
            ConstantEnum::SpecU32 { .. } => true,
            _ => false,
        }
    }

    pub fn is_f32_constant(&self) -> bool {
        match self.c {
            ConstantEnum::F32 { .. } |
            ConstantEnum::SpecF32 { .. } => true,
            _ => false,
        }
    }

    pub fn is_composite_constant(&self) -> bool {
        match self.c {
            ConstantEnum::Composite { .. } |
            ConstantEnum::SpecComposite { .. } => true,
            _ => false,
        }
    }

    pub fn is_null_constant(&self) -> bool {
        match self.c {
            ConstantEnum::Null { .. } => true,
            _ => false,
        }
    }

    pub fn is_sampler_constant(&self) -> bool {
        match self.c {
            ConstantEnum::Sampler { .. } => true,
            _ => false,
        }
    }

    pub fn is_spec_constant(&self) -> bool {
        match self.c {
            ConstantEnum::SpecBool { .. } |
            ConstantEnum::SpecI32 { .. } |
            ConstantEnum::SpecU32 { .. } |
            ConstantEnum::SpecF32 { .. } |
            ConstantEnum::SpecComposite { .. } |
            ConstantEnum::SpecOp { .. } => true,
            _ => false,
        }
    }

    pub fn is_spec_op_constant(&self) -> bool {
        match self.c {
            ConstantEnum::SpecOp { .. } => true,
            _ => false,
        }
    }
}
