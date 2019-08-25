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

use super::{Constant, Decoration, storage::Token};

#[derive(Clone, Debug)]
pub struct StructMember {
    pub token: Token<Type>,
    pub decorations: Vec<Decoration>,
}

include!("autogen_types.rs");

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;
        match (self, other) {
            (Bool, Bool) => true,
            (Int { width: sw, signedness: ss }, Int { width: ow, signedness: os }) => sw == ow && ss == os,
            (Float { width: s }, Float { width: o }) => s == o,
            (Vector { component_type: st, component_count: sc }, Vector { component_type: ot, component_count: oc }) => st == ot && sc == oc,
            (Matrix { column_type: st, column_count: sc }, Matrix { column_type: ot, column_count: oc }) => st == ot && sc == oc,
            _ => false,
        }
    }
}

impl Type {
    pub fn is_numerical_type(&self) -> bool {
        match self {
            Type::Int {..} | Type::Float {..} => true,
            _ => false,
        }
    }

    pub fn is_scalar_type(&self) -> bool {
        match self {
            Type::Bool => true,
            _ => self.is_numerical_type(),
        }
    }

    pub fn is_aggregate_type(&self) -> bool {
        match self {
            Type::Struct {..} | Type::Array {..} | Type::RuntimeArray {..} => true,
            _ => false,
        }
    }

    pub fn is_composite_type(&self) -> bool {
        match self {
            Type::Vector {..} | Type::Matrix {..} => true,
            _ => self.is_aggregate_type(),
        }
    }
}
