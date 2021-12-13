extern crate hex;
extern crate uuid;
extern crate web_sys;

use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

/// A cache of values with efficient lookup by value and index.
#[derive(Debug, Clone)]
pub(crate) struct IndexedCache<T> {
    /// Store of values for access by index.
    pub cache: Vec<T>,
    /// Store of values for access by value.
    lookup: HashMap<T, usize>,
}

impl<T> IndexedCache<T>
where
    T: Clone + Eq + Hash + Ord,
{
    pub fn new() -> Self {
        IndexedCache {
            cache: Default::default(),
            lookup: Default::default(),
        }
    }

    pub fn from(cache: Vec<T>) -> Self {
        let lookup = cache
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect();
        IndexedCache { cache, lookup }
    }

    /// Cache the value and return the index of it in the cache.
    pub fn cache(&mut self, item: T) -> usize {
        if let Some(n) = self.lookup.get(&item) {
            *n
        } else {
            let n = self.cache.len();
            self.cache.push(item.clone());
            self.lookup.insert(item, n);
            n
        }
    }

    /// Find the index of an item.
    pub fn lookup(&self, item: T) -> Option<usize> {
        self.lookup.get(&item).copied()
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Find a value by index.
    pub fn get(&self, index: usize) -> &T {
        // TODO: what if the index isn't in the range?
        // could add an assert with a useful explanation of why this shouldn't happen
        &self.cache[index]
    }

    /// Return a copy of this cache based on a sorted list.
    pub fn sorted(&self) -> IndexedCache<T> {
        let mut sorted = Self::new();
        self.cache.iter().sorted().cloned().for_each(|item| {
            let n = sorted.cache.len();
            sorted.cache.push(item.clone());
            sorted.lookup.insert(item, n);
        });
        sorted
    }

    pub fn encode_index(&self) -> Vec<usize> {
        let sorted: Vec<_> = self.cache.iter().sorted().cloned().collect();
        self.cache
            .iter()
            .map(|a| sorted.iter().position(|r| r == a).unwrap())
            .collect()
    }
}

impl<T> IntoIterator for IndexedCache<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.into_iter()
    }
}

impl<T> Index<usize> for IndexedCache<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.cache[i]
    }
}
