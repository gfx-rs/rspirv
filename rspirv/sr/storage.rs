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

use std::marker::PhantomData;

use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
};

use fxhash::FxHasher;

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

    pub(in crate::sr) fn new(index: Index) -> Self {
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
    pub(in crate::sr) fn new() -> Self {
        Storage {
            data: Vec::new(),
            lookup: FastHashMap::default(),
        }
    }

    /// Look up a value by the given SPIR-V <id>.
    pub fn lookup(&mut self, raw_index: spirv::Word) -> Option<Token<T>> {
        self.lookup.get(&raw_index).cloned().map(Token::new)
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

impl<T> std::ops::Index<Token<T>> for Storage<T> {
    type Output = T;
    fn index(&self, token: Token<T>) -> &T {
        &self.data[token.index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_non_unique() {
        let mut storage: Storage<f64> = Storage::new();
        let t1 = storage.append(0.0);
        let t2 = storage.append(0.0);
        assert!(t1 != t2);
        assert!(storage[t1] == storage[t2]);
    }

    #[test]
    fn append_unique() {
        let mut storage: Storage<f64> = Storage::new();
        let t1 = storage.append(std::f64::NAN);
        let t2 = storage.append(std::f64::NAN);
        assert!(t1 != t2);
        assert!(storage[t1] != storage[t2]);
    }

    #[test]
    fn fetch_or_append_non_unique() {
        let mut storage: Storage<f64> = Storage::new();
        let t1 = storage.fetch_or_append(0.0);
        let t2 = storage.fetch_or_append(0.0);
        assert!(t1 == t2);
        assert!(storage[t1] == storage[t2])
    }

    #[test]
    fn fetch_or_append_unique() {
        let mut storage: Storage<f64> = Storage::new();
        let t1 = storage.fetch_or_append(std::f64::NAN);
        let t2 = storage.fetch_or_append(std::f64::NAN);
        assert!(t1 != t2);
        assert!(storage[t1] != storage[t2]);
    }

    #[test]
    fn lookup_exists() {
        let mut storage: Storage<f64> = Storage::new();
        let t1 = storage.assign(2, 0.0);
        let t2 = storage.append(0.1);
        assert!(t1 != t2);
        assert!(storage.lookup(2) == Some(t1));
        assert!(storage[t1] == 0.0);
    }

    #[test]
    fn lookup_not_exists() {
        let mut storage: Storage<f64> = Storage::new();
        let _t1 = storage.append(0.1);
        assert!(storage.lookup(0) == None);
    }
}