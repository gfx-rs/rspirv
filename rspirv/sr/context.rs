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

use super::{Type, TypeToken, ConstantToken};
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
}

impl Context {
    pub fn new() -> Context {
        Context { types: vec![] }
    }

    pub fn type_struct(&mut self, field_types: &[TypeToken]) -> TypeToken {
        self.types.push(Type {
            ty: TypeEnum::Struct { field_types: field_types.to_owned() },
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

include!("type_creation.rs");

#[cfg(test)]
mod tests {
    use sr::{Context, TypeToken};

    #[test]
    fn test_get_type() {
        let mut c = Context::new();
        let i32t = c.type_int(32, 1);
        let t = c.get_type(i32t);
        assert!(t.is_int_type());
    }

    #[test]
    fn test_void_uniqueness() {
        let mut c = Context::new();
        let t1 = c.type_void();
        let t2 = c.type_void();
        assert_eq!(t1, t2);
        let t3 = c.type_int(32, 1);
        assert!(t1 != t3);
    }

    #[test]
    fn test_int_uniqueness() {
        let mut c = Context::new();
        let t1 = c.type_int(32, 1);
        let t2 = c.type_int(32, 1);
        assert_eq!(t1, t2);
        let t3 = c.type_int(32, 0);
        assert!(t1 != t3);
    }

    #[test]
    fn test_float_uniqueness() {
        let mut c = Context::new();
        let t1 = c.type_float(64);
        let t2 = c.type_float(64);
        assert_eq!(t1, t2);
        let t3 = c.type_float(32);
        assert!(t1 != t3);
    }

    #[test]
    fn test_vector_uniqueness() {
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
    fn test_matrix_uniqueness() {
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
    fn test_struct_non_uniqueness() {
        let mut c = Context::new();
        let token = TypeToken::new(0);
        let t1 = c.type_struct(&vec![token]);
        let t2 = c.type_struct(&vec![token]);
        assert!(t1 != t2);
        let t3 = c.type_struct(&vec![token, token]);
        assert!(t1 != t3);
        assert!(t2 != t3);
    }
}
