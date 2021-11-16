use crate::sr::storage::{Storage, Token};

use std::{
    borrow::Borrow,
    collections::hash_map::{Entry, HashMap, VacantEntry},
    hash::BuildHasherDefault,
};

use rustc_hash::FxHasher;

/// A wrapper around `Storage` that tracks associated SPIR-V <id>
/// with the elements, allowing the lookup of `Token<T>` by <id>.
pub struct LiftStorage<T, L = Token<T>> {
    values: Storage<T>,
    lookup: HashMap<spirv::Word, L, BuildHasherDefault<FxHasher>>,
}

impl<T, L: Borrow<Token<T>>> LiftStorage<T, L> {
    pub(in crate::lift) fn new() -> Self {
        LiftStorage {
            values: Storage::new(),
            lookup: HashMap::default(),
        }
    }

    pub(in crate::lift) fn unwrap(self) -> Storage<T> {
        self.values
    }

    pub(in crate::lift) fn lookup(&self, id: spirv::Word) -> (&T, &L) {
        let info = &self.lookup[&id];
        (&self.values[*info.borrow()], info)
    }

    pub(in crate::lift) fn lookup_safe(&self, id: spirv::Word) -> Option<(&T, &L)> {
        let info = self.lookup.get(&id)?;
        Some((&self.values[*info.borrow()], info))
    }

    pub(in crate::lift) fn append(
        &mut self,
        id: spirv::Word,
        value: T,
    ) -> (Token<T>, VacantEntry<spirv::Word, L>) {
        let token = self.values.append(value);
        match self.lookup.entry(id) {
            Entry::Occupied(_) => panic!("Id {:?} is already used", id),
            Entry::Vacant(e) => (token, e),
        }
    }
}

impl<T> LiftStorage<T> {
    pub(in crate::lift) fn append_id(&mut self, id: spirv::Word, value: T) -> Token<T> {
        let (token, entry) = self.append(id, value);
        entry.insert(token);
        token
    }

    pub(in crate::lift) fn lookup_token(&self, id: spirv::Word) -> Token<T> {
        self.lookup[&id]
    }
}
