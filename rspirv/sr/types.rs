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

use super::{ConstantToken, Decoration};

/// The class to represent a SPIR-V type.
///
/// Derived types hold `TypeToken`s of their base types instead of direct
/// references or `Rc` references, because of the difficulty or impossibility
/// of labelling explicit lifetimes or back references.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    pub(in crate::sr) ty: TypeEnum,
    /// Sets of decorations.
    pub(in crate::sr) decorations: Vec<Decoration>,
}

/// A token for representing a SPIR-V type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TypeToken {
    index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::sr) struct StructMember {
    pub(in crate::sr) token: TypeToken,
    pub(in crate::sr) decorations: Vec<Decoration>,
}

impl StructMember {
    pub(in crate::sr) fn new(token: TypeToken) -> Self {
        StructMember {
            token,
            decorations: Vec::new(),
        }
    }
}

include!("autogen_type_enum_check.rs");

impl Type {
    pub fn is_numerical_type(&self) -> bool {
        self.is_int_type() || self.is_float_type()
    }

    pub fn is_scalar_type(&self) -> bool {
        self.is_bool_type() || self.is_numerical_type()
    }

    pub fn is_aggregate_type(&self) -> bool {
        self.is_structure_type() || self.is_array_type() || self.is_runtime_array_type()
    }

    pub fn is_composite_type(&self) -> bool {
        self.is_aggregate_type() || self.is_vector_type() || self.is_matrix_type()
    }
}

impl TypeToken {
    pub(in crate::sr) fn new(index: usize) -> Self {
        TypeToken { index: index }
    }

    pub(in crate::sr) fn get(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use crate::sr::{Context, ConstantToken};

    #[test]
    fn test_void_type_is_only_void_type() {
        let mut c = Context::new();
        let voidt = c.type_void();
        let t = c.get_type(voidt);
        assert!(t.is_void_type());
        assert!(!t.is_bool_type());
        assert!(!t.is_int_type());
        assert!(!t.is_float_type());
        assert!(!t.is_vector_type());
        assert!(!t.is_matrix_type());
        assert!(!t.is_image_type());
        assert!(!t.is_sampler_type());
        assert!(!t.is_sampled_image_type());
        assert!(!t.is_array_type());
        assert!(!t.is_runtime_array_type());
        assert!(!t.is_structure_type());
        assert!(!t.is_opaque_type());
        assert!(!t.is_pointer_type());
        assert!(!t.is_function_type());
        assert!(!t.is_event_type());
        assert!(!t.is_device_event_type());
        assert!(!t.is_reserve_id_type());
        assert!(!t.is_queue_type());
        assert!(!t.is_pipe_type());
        assert!(!t.is_forward_pointer_type());
        assert!(!t.is_pipe_storage_type());
        assert!(!t.is_named_barrier_type());
    }

    #[test]
    fn test_int_type_is_only_int_type() {
        let mut c = Context::new();
        let u32t = c.type_int(32, 0);
        let t = c.get_type(u32t);
        assert!(!t.is_void_type());
        assert!(!t.is_bool_type());
        assert!(t.is_int_type());
        assert!(!t.is_float_type());
        assert!(!t.is_vector_type());
        assert!(!t.is_matrix_type());
        assert!(!t.is_image_type());
        assert!(!t.is_sampler_type());
        assert!(!t.is_sampled_image_type());
        assert!(!t.is_array_type());
        assert!(!t.is_runtime_array_type());
        assert!(!t.is_structure_type());
        assert!(!t.is_opaque_type());
        assert!(!t.is_pointer_type());
        assert!(!t.is_function_type());
        assert!(!t.is_event_type());
        assert!(!t.is_device_event_type());
        assert!(!t.is_reserve_id_type());
        assert!(!t.is_queue_type());
        assert!(!t.is_pipe_type());
        assert!(!t.is_forward_pointer_type());
        assert!(!t.is_pipe_storage_type());
        assert!(!t.is_named_barrier_type());
    }

    #[test]
    fn test_is_numerical_type() {
        let mut c = Context::new();
        let voidt = c.type_void();
        {
            let t = c.get_type(voidt);
            assert!(!t.is_numerical_type());
        }
        let i32t = c.type_int(32, 1);
        {
            let t = c.get_type(i32t);
            assert!(t.is_numerical_type());
        }
        let f64t = c.type_float(64);
        {
            let t = c.get_type(f64t);
            assert!(t.is_numerical_type());
        }
        let boolt = c.type_bool();
        {
            let t = c.get_type(boolt);
            assert!(!t.is_numerical_type());
        }
    }

    #[test]
    fn test_is_scalar_type() {
        let mut c = Context::new();
        let voidt = c.type_void();
        {
            let t = c.get_type(voidt);
            assert!(!t.is_scalar_type());
        }
        let i32t = c.type_int(32, 1);
        {
            let t = c.get_type(i32t);
            assert!(t.is_scalar_type());
        }
        let f64t = c.type_float(64);
        {
            let t = c.get_type(f64t);
            assert!(t.is_scalar_type());
        }
        let boolt = c.type_bool();
        {
            let t = c.get_type(boolt);
            assert!(t.is_scalar_type());
        }
    }

    #[test]
    fn test_is_aggregate_type() {
        let mut c = Context::new();
        let voidt = c.type_void();
        {
            let t = c.get_type(voidt);
            assert!(!t.is_aggregate_type());
        }
        let i32t = c.type_int(32, 1);
        {
            let t = c.get_type(i32t);
            assert!(!t.is_aggregate_type());
        }
        let f64t = c.type_float(64);
        {
            let t = c.get_type(f64t);
            assert!(!t.is_aggregate_type());
        }
        let boolt = c.type_bool();
        {
            let t = c.get_type(boolt);
            assert!(!t.is_aggregate_type());
        }
        let structt = c.type_struct(&vec![i32t, i32t, i32t]);
        {
            let t = c.get_type(structt);
            assert!(t.is_aggregate_type());
        }
        let arrt = c.type_array(i32t, ConstantToken::new(16));
        {
            let t = c.get_type(arrt);
            assert!(t.is_aggregate_type());
        }
        let rtarrt = c.type_runtime_array(i32t);
        {
            let t = c.get_type(rtarrt);
            assert!(t.is_aggregate_type());
        }
        let v4i32t = c.type_vector(i32t, 4);
        {
            let t = c.get_type(v4i32t);
            assert!(!t.is_aggregate_type());
        }
        let matt = c.type_matrix(v4i32t, 4);
        {
            let t = c.get_type(matt);
            assert!(!t.is_aggregate_type());
        }
    }

    #[test]
    fn test_is_composite_type() {
        let mut c = Context::new();
        let voidt = c.type_void();
        {
            let t = c.get_type(voidt);
            assert!(!t.is_composite_type());
        }
        let i32t = c.type_int(32, 1);
        {
            let t = c.get_type(i32t);
            assert!(!t.is_composite_type());
        }
        let f64t = c.type_float(64);
        {
            let t = c.get_type(f64t);
            assert!(!t.is_composite_type());
        }
        let boolt = c.type_bool();
        {
            let t = c.get_type(boolt);
            assert!(!t.is_composite_type());
        }
        let structt = c.type_struct(&vec![i32t, i32t, i32t]);
        {
            let t = c.get_type(structt);
            assert!(t.is_composite_type());
        }
        let arrt = c.type_array(i32t, ConstantToken::new(16));
        {
            let t = c.get_type(arrt);
            assert!(t.is_composite_type());
        }
        let rtarrt = c.type_runtime_array(i32t);
        {
            let t = c.get_type(rtarrt);
            assert!(t.is_composite_type());
        }
        let v4i32t = c.type_vector(i32t, 4);
        {
            let t = c.get_type(v4i32t);
            assert!(t.is_composite_type());
        }
        let matt = c.type_matrix(v4i32t, 4);
        {
            let t = c.get_type(matt);
            assert!(t.is_composite_type());
        }
    }
}
