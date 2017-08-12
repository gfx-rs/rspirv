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
