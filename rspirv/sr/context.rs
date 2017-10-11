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

use std::collections::BTreeSet;

use super::{Type, TypeToken, Constant, ConstantToken};
use sr::constants::ConstantEnum;
use sr::types::TypeEnum;

/// The context class for SPIR-V structured representation.
///
/// This class holds all allocations for types, constants, decorations,
/// instructions, etc. in structured representation. Thus, those objects
/// are created using methods of this class. Tokens are returned for the
/// object to be created, which can then be used to access the real object
/// using the context again. Tokens are indeed indices into the vectors
/// of objects inside the context. The context serves as the memory arena.
#[derive(Debug)]
pub struct Context {
    /// All type objects.
    types: Vec<Type>,
    constants: Vec<Constant>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            types: vec![],
            constants: vec![],
        }
    }
}

include!("type_creation.rs");

impl Context {
    pub fn type_struct<T: AsRef<[TypeToken]>>(&mut self, field_types: T) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Struct { field_types: field_types.as_ref().to_vec() },
            decorations: BTreeSet::new(),
        });
        TypeToken::new(self.types.len() - 1)
    }

    /// Returns the reference to the real type represented by the given token.
    pub fn get_type(&self, token: TypeToken) -> &Type {
        // Note: we assume the vector doesn't shrink so we always have a valid index.
        &self.types[token.get()]
    }
}

macro_rules! fetch_or_append {
    ($container: expr, $val: expr) => {
        if let Some(index) = $container.iter().position(|x| *x == $val) {
            ConstantToken::new(index)
        } else {
            $container.push($val);
            ConstantToken::new($container.len() - 1)
        }
    }
}

impl Context {
    pub fn constant_bool(&mut self, val: bool) -> ConstantToken {
        let v = Constant { c: ConstantEnum::Bool(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn constant_i32(&mut self, val: i32) -> ConstantToken {
        let v = Constant { c: ConstantEnum::I32(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn constant_u32(&mut self, val: u32) -> ConstantToken {
        let v = Constant { c: ConstantEnum::U32(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn constant_f32(&mut self, val: f32) -> ConstantToken {
        let v = Constant { c: ConstantEnum::F32(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn constant_composite<T: AsRef<[ConstantToken]>>(&mut self, val: T) -> ConstantToken {
        let v = Constant { c: ConstantEnum::Composite(val.as_ref().to_vec()) };
        fetch_or_append!(self.constants, v)
    }

    pub fn constant_null(&mut self, val: TypeToken) -> ConstantToken {
        let v = Constant { c: ConstantEnum::Null(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn constant_sampler(
        &mut self,
        addressing_mode: spirv::SamplerAddressingMode,
        param: u32,
        filter_mode: spirv::SamplerFilterMode,
    ) -> ConstantToken {
        let v = Constant { c: ConstantEnum::Sampler(addressing_mode, param, filter_mode) };
        fetch_or_append!(self.constants, v)
    }

    pub fn spec_constant_bool(&mut self, val: bool) -> ConstantToken {
        let v = Constant { c: ConstantEnum::SpecBool(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn spec_constant_i32(&mut self, val: i32) -> ConstantToken {
        let v = Constant { c: ConstantEnum::SpecI32(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn spec_constant_u32(&mut self, val: u32) -> ConstantToken {
        let v = Constant { c: ConstantEnum::SpecU32(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn spec_constant_f32(&mut self, val: f32) -> ConstantToken {
        let v = Constant { c: ConstantEnum::SpecF32(val) };
        fetch_or_append!(self.constants, v)
    }

    pub fn spec_constant_composite<T: AsRef<[ConstantToken]>>(&mut self, val: T) -> ConstantToken {
        let v = Constant { c: ConstantEnum::SpecComposite(val.as_ref().to_vec()) };
        fetch_or_append!(self.constants, v)
    }

    pub fn spec_constant_op<T: AsRef<[ConstantToken]>>(
        &mut self,
        op: spirv::Op,
        operands: T,
    ) -> ConstantToken {
        let v = Constant { c: ConstantEnum::SpecOp(op, operands.as_ref().to_vec()) };
        fetch_or_append!(self.constants, v)
    }

    /// Returns the reference to the real constant represented by the given token.
    pub fn get_constant(&self, token: ConstantToken) -> &Constant {
        // Note: we assume the vector doesn't shrink so we always have a valid index.
        &self.constants[token.get()]
    }
}

#[cfg(test)]
mod tests {
    use spirv;
    use sr::{Context, TypeToken};

    #[test]
    fn test_get_type() {
        let mut c = Context::new();
        let i32t = c.type_int(32, 1);
        let t = c.get_type(i32t);
        assert!(t.is_int_type());
    }

    #[test]
    fn test_void_type_uniqueness() {
        let mut c = Context::new();
        let t1 = c.type_void();
        let t2 = c.type_void();
        assert_eq!(t1, t2);
        let t3 = c.type_int(32, 1);
        assert!(t1 != t3);
    }

    #[test]
    fn test_int_type_uniqueness() {
        let mut c = Context::new();
        let t1 = c.type_int(32, 1);
        let t2 = c.type_int(32, 1);
        assert_eq!(t1, t2);
        let t3 = c.type_int(32, 0);
        assert!(t1 != t3);
    }

    #[test]
    fn test_float_type_uniqueness() {
        let mut c = Context::new();
        let t1 = c.type_float(64);
        let t2 = c.type_float(64);
        assert_eq!(t1, t2);
        let t3 = c.type_float(32);
        assert!(t1 != t3);
    }

    #[test]
    fn test_vector_type_uniqueness() {
        let mut c = Context::new();
        let token = TypeToken::new(0);
        let t1 = c.type_vector(token, 4);
        let t2 = c.type_vector(token, 4);
        assert_eq!(t1, t2);
        let t3 = c.type_vector(token, 3);
        assert!(t1 != t3);
        let token = TypeToken::new(1);
        let t4 = c.type_vector(token, 3);
        assert!(t3 != t4);
        assert!(t2 != t3);
    }

    #[test]
    fn test_matrix_type_uniqueness() {
        let mut c = Context::new();
        let token = TypeToken::new(0);
        let t1 = c.type_matrix(token, 4);
        let t2 = c.type_matrix(token, 4);
        assert_eq!(t1, t2);
        let t3 = c.type_matrix(token, 3);
        assert!(t1 != t3);
        let token = TypeToken::new(1);
        let t4 = c.type_matrix(token, 3);
        assert!(t3 != t4);
        assert!(t2 != t3);
    }

    #[test]
    fn test_struct_type_non_uniqueness() {
        let mut c = Context::new();
        let token = TypeToken::new(0);
        let t1 = c.type_struct(&vec![token]);
        let t2 = c.type_struct(&vec![token]);
        assert!(t1 != t2);
        let t3 = c.type_struct(&vec![token, token]);
        assert!(t1 != t3);
        assert!(t2 != t3);
    }

    #[test]
    fn test_get_constant() {
        let mut c = Context::new();
        let i32c = c.constant_i32(5);
        let v = c.get_constant(i32c);
        assert!(v.is_i32_constant());
    }

    #[test]
    fn test_bool_constant_uniqueness() {
        let mut c = Context::new();
        let v1 = c.constant_bool(true);
        let v2 = c.constant_bool(true);
        assert_eq!(v1, v2);
        let v3 = c.constant_bool(false);
        assert!(v1 != v3);
    }

    #[test]
    fn test_i32_constant_uniqueness() {
        let mut c = Context::new();
        let v1 = c.constant_i32(-42);
        let v2 = c.constant_i32(-42);
        assert_eq!(v1, v2);
        let v3 = c.constant_i32(42);
        assert!(v1 != v3);
    }

    #[test]
    fn test_u32_constant_uniqueness() {
        let mut c = Context::new();
        let v1 = c.constant_u32(42);
        let v2 = c.constant_u32(42);
        assert_eq!(v1, v2);
        let v3 = c.constant_u32(24);
        assert!(v1 != v3);
    }

    #[test]
    fn test_f32_constant_uniqueness() {
        let mut c = Context::new();
        let v1 = c.constant_f32(1.2);
        let v2 = c.constant_f32(1.2);
        assert_eq!(v1, v2);
        let v3 = c.constant_f32(1.20001);
        assert!(v1 != v3);
    }

    #[test]
    fn test_composite_constant_uniqueness() {
        let mut c = Context::new();
        let component = c.constant_f32(1.2);
        let v1 = c.constant_composite(vec![component]);
        let v2 = c.constant_composite(vec![component]);
        assert_eq!(v1, v2);
        let v3 = c.constant_composite(vec![component, component]);
        assert!(v1 != v3);
    }

    #[test]
    fn test_null_constant_uniqueness() {
        let mut c = Context::new();
        let t1 = c.type_int(32, 1);
        let v1 = c.constant_null(t1);
        let v2 = c.constant_null(t1);
        assert_eq!(v1, v2);
        let t2 = c.type_int(32, 0);
        let v3 = c.constant_null(t2);
        assert!(v1 != v3);
    }

    #[test]
    fn test_sampler_constant_uniqueness() {
        let mut c = Context::new();
        let v1 = c.constant_sampler(
            spirv::SamplerAddressingMode::Clamp,
            0,
            spirv::SamplerFilterMode::Nearest,
        );
        let v2 = c.constant_sampler(
            spirv::SamplerAddressingMode::Clamp,
            0,
            spirv::SamplerFilterMode::Nearest,
        );
        assert_eq!(v1, v2);
        let v3 = c.constant_sampler(
            spirv::SamplerAddressingMode::Clamp,
            0,
            spirv::SamplerFilterMode::Linear,
        );
        assert!(v1 != v3);
    }

    #[test]
    fn test_spec_constant_bool_uniqueness() {
        let mut c = Context::new();
        let v1 = c.spec_constant_bool(true);
        let v2 = c.spec_constant_bool(true);
        assert_eq!(v1, v2);
        let v3 = c.spec_constant_bool(false);
        assert!(v1 != v3);
    }

    #[test]
    fn test_spec_constant_i32_uniqueness() {
        let mut c = Context::new();
        let v1 = c.spec_constant_i32(-42);
        let v2 = c.spec_constant_i32(-42);
        assert_eq!(v1, v2);
        let v3 = c.spec_constant_i32(42);
        assert!(v1 != v3);
    }

    #[test]
    fn test_spec_constant_u32_uniqueness() {
        let mut c = Context::new();
        let v1 = c.spec_constant_u32(42);
        let v2 = c.spec_constant_u32(42);
        assert_eq!(v1, v2);
        let v3 = c.spec_constant_u32(24);
        assert!(v1 != v3);
    }

    #[test]
    fn test_spec_constant_f32_uniqueness() {
        let mut c = Context::new();
        let v1 = c.spec_constant_f32(1.2);
        let v2 = c.spec_constant_f32(1.2);
        assert_eq!(v1, v2);
        let v3 = c.spec_constant_f32(1.20001);
        assert!(v1 != v3);
    }

    #[test]
    fn test_spec_constant_composite_uniqueness() {
        let mut c = Context::new();
        let component = c.spec_constant_f32(1.2);
        let v1 = c.spec_constant_composite(vec![component]);
        let v2 = c.spec_constant_composite(vec![component]);
        assert_eq!(v1, v2);
        let v3 = c.spec_constant_composite(vec![component, component]);
        assert!(v1 != v3);
    }

    #[test]
    fn test_spec_constant_op_uniqueness() {
        let mut c = Context::new();
        let component = c.spec_constant_f32(1.2);
        let v1 = c.spec_constant_op(spirv::Op::FAdd, vec![component, component]);
        let v2 = c.spec_constant_op(spirv::Op::FAdd, vec![component, component]);
        assert_eq!(v1, v2);
        let v3 = c.spec_constant_op(spirv::Op::FSub, vec![component, component]);
        assert!(v1 != v3);
    }
}
