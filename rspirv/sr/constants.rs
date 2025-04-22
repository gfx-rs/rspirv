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
        matches!(self, Constant::Bool { .. } | Constant::SpecBool { .. })
    }

    pub fn is_int_constant(&self) -> bool {
        matches!(self, Constant::Int { .. } | Constant::SpecInt { .. })
    }

    pub fn is_uint_constant(&self) -> bool {
        matches!(self, Constant::UInt { .. } | Constant::SpecUInt { .. })
    }

    pub fn is_float_constant(&self) -> bool {
        matches!(self, Constant::Float { .. } | Constant::SpecFloat { .. })
    }

    pub fn is_composite_constant(&self) -> bool {
        matches!(
            self,
            Constant::Composite { .. } | Constant::SpecComposite { .. }
        )
    }

    pub fn is_null_constant(&self) -> bool {
        matches!(self, Constant::Null)
    }

    pub fn is_sampler_constant(&self) -> bool {
        matches!(self, Constant::Sampler { .. })
    }

    pub fn is_spec_constant(&self) -> bool {
        matches!(
            self,
            Constant::SpecBool { .. }
                | Constant::SpecInt { .. }
                | Constant::SpecUInt { .. }
                | Constant::SpecFloat { .. }
                | Constant::SpecComposite { .. }
                | Constant::SpecOp { .. }
        )
    }

    pub fn is_spec_op_constant(&self) -> bool {
        matches!(self, Constant::SpecOp { .. })
    }
}
