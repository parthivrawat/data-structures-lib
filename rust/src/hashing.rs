use std::collections::{HashMap as StdMap, HashSet as StdSet};

/// A hash map.
#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
    data: StdMap<K, V>,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    /// Creates a new HashMap.
    pub fn new() -> Self {
        Self {
            data: StdMap::new(),
        }
    }

    /// Sets a key-value pair.
    pub fn set(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    /// Gets a reference to the value for a key.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Returns true if the key exists.
    pub fn has(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Removes a key.
    pub fn delete(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a Vec of all keys.
    pub fn keys(&self) -> Vec<&K> {
        self.data.keys().collect()
    }

    /// Returns a Vec of all values.
    pub fn values(&self) -> Vec<&V> {
        self.data.values().collect()
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A hash set.
#[derive(Debug, Clone)]
pub struct HashSet<T> {
    data: StdSet<T>,
}

impl<T> HashSet<T>
where
    T: Eq + std::hash::Hash,
{
    /// Creates a new HashSet.
    pub fn new() -> Self {
        Self {
            data: StdSet::new(),
        }
    }

    /// Adds a value.
    pub fn add(&mut self, value: T) {
        self.data.insert(value);
    }

    /// Removes a value.
    pub fn remove(&mut self, value: &T) -> bool {
        self.data.remove(value)
    }

    /// Returns true if the value is in the set.
    pub fn has(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    /// Returns the number of values.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a Vec of all values.
    pub fn to_vec(&self) -> Vec<&T> {
        self.data.iter().collect()
    }
}

impl<T> Default for HashSet<T>
where
    T: Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
