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
    Sampler {
        addressing_mode: spirv::SamplerAddressingMode,
        normalized: bool,
        filter_mode: spirv::SamplerFilterMode,
    },
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
