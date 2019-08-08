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

use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    marker::PhantomData,
    ops,
};

use fxhash::FxHasher;
use spirv;

use crate::{
    sr::constants::{Constant, ConstantEnum},
    sr::types::{StructMember, TypeEnum, Type},
};

type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
/// An unique index in the storage array that a token points to.
///
/// This type is independent of `spirv::Word`. `spirv::Word` is used in data
// representation. It holds a SPIR-V and refers to that instruction. In
// structured representation, we use Token to refer to an SPIR-V instruction.
// Index is an implementation detail to Token.
type Index = u32;

/// A strongly typed reference to a SPIR-V element.
#[derive(Debug)]
pub struct Token<T> {
    index: Index,
    marker: PhantomData<T>,
}

impl<T> Clone for Token<T> {
    fn clone(&self) -> Self {
        Token {
            index: self.index,
            marker: self.marker,
        }
    }
}
impl<T> Copy for Token<T> {}
impl<T> PartialEq for Token<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl<T> Eq for Token<T> {}

impl<T> Token<T> {
    #[cfg(test)]
    pub const DUMMY: Self = Token {
        index: !0,
        marker: PhantomData,
    };

    fn new(index: Index) -> Self {
        Token {
            index,
            marker: PhantomData,
        }
    }
}

/// A structure holding some kind of SPIR-V entity (e.g., type, constant,
/// instruction, etc.) that can be referenced.
#[derive(Debug)]
pub struct Storage<T> {
    /// Values of this storage.
    data: Vec<T>,
    /// Reverse lookup table that associates SPIR-V <id> with interal indices
    /// in the main `data` table.
    lookup: FastHashMap<spirv::Word, Index>,
}

impl<T> Storage<T> {
    fn new() -> Self {
        Storage {
            data: Vec::new(),
            lookup: FastHashMap::default(),
        }
    }

    /// Associates the given value to the given SPIR-V <id> inside this storage
    /// and returns a token for representing this value.
    pub fn assign(&mut self, raw_index: spirv::Word, value: T) -> Token<T> {
        let index = self.data.len() as Index;
        self.data.push(value);
        let old = self.lookup.insert(raw_index, index);
        assert_eq!(None, old);
        Token::new(index)
    }

    /// Adds a new value to the storage, returning a typed token.
    ///
    /// The value is not linked to any SPIR-V module.
    pub fn append(&mut self, value: T) -> Token<T> {
        let index = self.data.len() as Index;
        self.data.push(value);
        Token::new(index)
    }

    /// Adds a value with a check for uniqueness: returns a token pointing to
    /// an existing element if its value matches the given one, or adds a new
    /// element otherwise.
    pub fn fetch_or_append(&mut self, value: T) -> Token<T> where T: PartialEq {
        if let Some(index) = self.data.iter().position(|d| d == &value) {
            Token::new(index as Index)
        } else {
            self.append(value)
        }
    }
}

impl<T> ops::Index<Token<T>> for Storage<T> {
    type Output = T;
    fn index(&self, token: Token<T>) -> &T {
        &self.data[token.index as usize]
    }
}


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
    pub types: Storage<Type>,
    pub constants: Storage<Constant>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            types: Storage::new(),
            constants: Storage::new(),
        }
    }
}

include!("autogen_context.rs");

impl Context {
    pub fn type_struct<T: AsRef<[Token<Type>]>>(&mut self, field_types: T) -> Token<Type> {
        self.types.append(Type {
            ty: TypeEnum::Struct {
                field_types: field_types
                    .as_ref()
                    .iter()
                    .map(|ft| StructMember::new(*ft))
                    .collect(),
            },
            decorations: Vec::new(),
        })
    }
}

impl Context {
    pub fn constant_bool(&mut self, val: bool) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::Bool(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn constant_i32(&mut self, val: i32) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::I32(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn constant_u32(&mut self, val: u32) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::U32(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn constant_f32(&mut self, val: f32) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::F32(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn constant_composite<T: AsRef<[Token<Constant>]>>(&mut self, val: T) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::Composite(val.as_ref().to_vec()) };
        self.constants.fetch_or_append(v)
    }

    pub fn constant_null(&mut self, val: Token<Type>) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::Null(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn constant_sampler(
        &mut self,
        addressing_mode: spirv::SamplerAddressingMode,
        param: u32,
        filter_mode: spirv::SamplerFilterMode,
    ) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::Sampler(addressing_mode, param, filter_mode) };
        self.constants.fetch_or_append(v)
    }

    pub fn spec_constant_bool(&mut self, val: bool) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::SpecBool(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn spec_constant_i32(&mut self, val: i32) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::SpecI32(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn spec_constant_u32(&mut self, val: u32) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::SpecU32(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn spec_constant_f32(&mut self, val: f32) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::SpecF32(val) };
        self.constants.fetch_or_append(v)
    }

    pub fn spec_constant_composite<T: AsRef<[Token<Constant>]>>(&mut self, val: T) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::SpecComposite(val.as_ref().to_vec()) };
        self.constants.fetch_or_append(v)
    }

    pub fn spec_constant_op<T: AsRef<[Token<Constant>]>>(
        &mut self,
        op: spirv::Op,
        operands: T,
    ) -> Token<Constant> {
        let v = Constant { c: ConstantEnum::SpecOp(op, operands.as_ref().to_vec()) };
        self.constants.fetch_or_append(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::spirv;
    use crate::sr::{Context, Token};

    #[test]
    fn test_get_type() {
        let mut c = Context::new();
        let i32t = c.type_int(32, 1);
        let t = &c.types[i32t];
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
        let token = Token::new(0);
        let t1 = c.type_vector(token, 4);
        let t2 = c.type_vector(token, 4);
        assert_eq!(t1, t2);
        let t3 = c.type_vector(token, 3);
        assert!(t1 != t3);
        let token = Token::new(1);
        let t4 = c.type_vector(token, 3);
        assert!(t3 != t4);
        assert!(t2 != t3);
    }

    #[test]
    fn test_matrix_type_uniqueness() {
        let mut c = Context::new();
        let token = Token::new(0);
        let t1 = c.type_matrix(token, 4);
        let t2 = c.type_matrix(token, 4);
        assert_eq!(t1, t2);
        let t3 = c.type_matrix(token, 3);
        assert!(t1 != t3);
        let token = Token::new(1);
        let t4 = c.type_matrix(token, 3);
        assert!(t3 != t4);
        assert!(t2 != t3);
    }

    #[test]
    fn test_struct_type_non_uniqueness() {
        let mut c = Context::new();
        let token = Token::new(0);
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
        let v = &c.constants[i32c];
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
