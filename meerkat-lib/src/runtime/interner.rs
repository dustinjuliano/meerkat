//! String interning module. Depends on Symbol module for type safe
//! symbolic identifiers. Features zero-copy strings. Relies on caller
//! supplied storage for strings. Allows forward and reverse lookup.

//! This implementation is intentionally copying strings to avoid
//! viral lifetime annotations; we can upgrade it to slices in the future.

use std::collections::HashMap;
use crate::runtime::symbol::Symbol;

pub struct Interner {
    index: HashMap<String, u32>,
    strings: Vec<String>,
    next_id: u32,
}

impl Interner {
    /// Initialize new interner. 0 is reserved for empty string.
    pub fn new() -> Self {
        Self {
            index: HashMap::from([(String::new(), 0)]),
            strings: vec![String::new()],
            next_id: 1,
        }
    }

    /// Reserve `n` number of identifiers. Initializes them internally
    /// to the empty string.
    pub fn with_reserve(n: u32) -> Self {
        let count = n + 1;
        let mut index = HashMap::with_capacity(count as usize);
        index.insert(String::new(), 0);
        Self {
            index,
            strings: vec![String::new(); count as usize],
            next_id: count,
        }
    }

    /// Intern a string and return its unique Symbol.
    /// If the string already exists, it returns the existing Symbol.
    pub fn insert(&mut self, s: &str) -> Symbol {
        if let Some(&id) = self.index.get(s) {
            return Symbol(id);
        }

        let id = self.next_id;
        self.index.insert(s.to_string(), id);
        self.strings.push(s.to_string());
        self.next_id += 1;
        Symbol(id)
    }

    /// Get the string associated with a Symbol.
    /// Returns an empty string if the Symbol is out of bounds.
    pub fn get(&self, id: Symbol) -> &str {
        self.strings.get(id.0 as usize).map(|s| s.as_str()).unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interner_has_empty_string_at_zero() {
        let interner = Interner::new();
        assert_eq!(interner.get(Symbol(0)), "");
        assert_eq!(interner.next_id, 1);
    }

    #[test]
    fn test_basic_insert_and_get() {
        let mut interner = Interner::new();
        let sym = interner.insert("hello");

        assert_eq!(sym, Symbol(1));
        assert_eq!(interner.get(sym), "hello");
    }

    #[test]
    fn test_deduplication() {
        let mut interner = Interner::new();
        let sym1 = interner.insert("rust");
        let sym2 = interner.insert("rust");

        assert_eq!(sym1, sym2, "Redundant strings must return the same Symbol");
        assert_eq!(interner.strings.len(), 2, "Vector should only contain empty string and 'rust'");
    }

    #[test]
    fn test_with_reserve_gaps() {
        // Reserve 5 slots (IDs 1, 2, 3, 4, 5)
        let mut interner = Interner::with_reserve(5);

        assert_eq!(interner.next_id, 6);
        assert_eq!(interner.strings.len(), 6);

        // Symbol 3 should exist but be an empty string placeholder
        assert_eq!(interner.get(Symbol(3)), "");

        // New inserts should happen AFTER the reserved range
        let sym = interner.insert("after_reserve");
        assert_eq!(sym, Symbol(6));
        assert_eq!(interner.get(Symbol(6)), "after_reserve");
    }

    #[test]
    fn test_out_of_bounds_safety() {
        let interner = Interner::new();
        // Index 99 doesn't exist, should return empty string sentinel
        assert_eq!(interner.get(Symbol(99)), "");
    }

    #[test]
    fn test_multiple_unique_inserts() {
        let mut interner = Interner::new();
        let s1 = interner.insert("a");
        let s2 = interner.insert("b");
        let s3 = interner.insert("c");

        assert_eq!(interner.get(s1), "a");
        assert_eq!(interner.get(s2), "b");
        assert_eq!(interner.get(s3), "c");
        assert_ne!(s1, s2);
    }
}
