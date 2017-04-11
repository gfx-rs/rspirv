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

use spirv;

/// SPIR-V types.
#[derive(Debug, Eq, PartialEq)]
pub struct Type {
    ty: Ty,
}

include!("ty.rs");

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

#[cfg(test)]
mod tests {
    use super::Type;

    #[test]
    fn test_void_type_is_only_void_type() {
        let t = Type::void();
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
        let t = Type::int(32, 0);
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
        let t = Type::void();
        assert!(!t.is_numerical_type());
        let t = Type::int(32, 1);
        assert!(t.is_numerical_type());
        let t = Type::float(64);
        assert!(t.is_numerical_type());
        let t = Type::bool();
        assert!(!t.is_numerical_type());
    }

    #[test]
    fn test_is_scalar_type() {
        let t = Type::void();
        assert!(!t.is_scalar_type());
        let t = Type::int(32, 1);
        assert!(t.is_scalar_type());
        let t = Type::float(64);
        assert!(t.is_scalar_type());
        let t = Type::bool();
        assert!(t.is_scalar_type());
    }

    #[test]
    fn test_is_aggregate_type() {
        let t = Type::void();
        assert!(!t.is_aggregate_type());
        let t = Type::int(32, 1);
        assert!(!t.is_aggregate_type());
        let t = Type::float(64);
        assert!(!t.is_aggregate_type());
        let t = Type::bool();
        assert!(!t.is_aggregate_type());
        let t = Type::structure(vec![1, 2, 3]);
        assert!(t.is_aggregate_type());
        let t = Type::array(1, 32);
        assert!(t.is_aggregate_type());
        let t = Type::runtime_array(1);
        assert!(t.is_aggregate_type());
        let t = Type::vector(1, 4);
        assert!(!t.is_aggregate_type());
        let t = Type::matrix(1, 4);
        assert!(!t.is_aggregate_type());
    }

    #[test]
    fn test_is_composite_type() {
        let t = Type::void();
        assert!(!t.is_composite_type());
        let t = Type::int(32, 1);
        assert!(!t.is_composite_type());
        let t = Type::float(64);
        assert!(!t.is_composite_type());
        let t = Type::bool();
        assert!(!t.is_composite_type());
        let t = Type::structure(vec![1, 2, 3]);
        assert!(t.is_composite_type());
        let t = Type::array(1, 32);
        assert!(t.is_composite_type());
        let t = Type::runtime_array(1);
        assert!(t.is_composite_type());
        let t = Type::vector(1, 4);
        assert!(t.is_composite_type());
        let t = Type::matrix(1, 4);
        assert!(t.is_composite_type());
    }
}
