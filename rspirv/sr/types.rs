use crate::spirv;

use super::{storage::Token, Constant, Decoration};

#[derive(Clone, Debug)]
pub struct StructMember {
    pub token: Token<Type>,
    pub decorations: Vec<Decoration>,
}

impl StructMember {
    pub fn new(ty: Token<Type>) -> Self {
        StructMember {
            token: ty,
            decorations: Vec::new(),
        }
    }
}

include!("autogen_types.rs");

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;
        match (self, other) {
            (Bool, Bool) => true,
            (
                Int {
                    width: sw,
                    signedness: ss,
                },
                Int {
                    width: ow,
                    signedness: os,
                },
            ) => sw == ow && ss == os,
            (
                Float {
                    width: sw,
                    floating_point_encoding: se,
                },
                Float {
                    width: ow,
                    floating_point_encoding: oe,
                },
            ) => sw == ow && se == oe,
            (
                Vector {
                    component_type: st,
                    component_count: sc,
                },
                Vector {
                    component_type: ot,
                    component_count: oc,
                },
            ) => st == ot && sc == oc,
            (
                Matrix {
                    column_type: st,
                    column_count: sc,
                },
                Matrix {
                    column_type: ot,
                    column_count: oc,
                },
            ) => st == ot && sc == oc,
            _ => false,
        }
    }
}

impl Type {
    pub fn is_numerical_type(&self) -> bool {
        matches!(self, Type::Int { .. } | Type::Float { .. })
    }

    pub fn is_scalar_type(&self) -> bool {
        match self {
            Type::Bool => true,
            _ => self.is_numerical_type(),
        }
    }

    pub fn is_aggregate_type(&self) -> bool {
        matches!(
            self,
            Type::Struct { .. } | Type::Array { .. } | Type::RuntimeArray { .. }
        )
    }

    pub fn is_composite_type(&self) -> bool {
        match self {
            Type::Vector { .. } | Type::Matrix { .. } => true,
            _ => self.is_aggregate_type(),
        }
    }
}
