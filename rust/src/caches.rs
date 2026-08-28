use std::collections::{HashMap, VecDeque};

use crate::Error;

/// A least-recently-used cache with a fixed capacity.
#[derive(Debug)]
pub struct LruCache<K: Clone + Eq + std::hash::Hash, V> {
    capacity: usize,
    cache: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Clone + Eq + std::hash::Hash, V> LruCache<K, V> {
    /// Creates a new LruCache.
    pub fn new(capacity: usize) -> Result<Self, Error> {
        if capacity == 0 {
            return Err(Error::InvalidArgument);
        }
        Ok(Self {
            capacity,
            cache: HashMap::new(),
            order: VecDeque::new(),
        })
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Returns true if the key exists.
    pub fn has(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    /// Returns a reference to the value for a key, marking it as recently used.
    pub fn get(&mut self, key: &K) -> Result<&V, Error> {
        if !self.cache.contains_key(key) {
            return Err(Error::NotFound);
        }
        self.touch(key);
        Ok(self.cache.get(key).unwrap())
    }

    /// Sets a key-value pair.
    pub fn set(&mut self, key: K, value: V) {
        if self.cache.contains_key(&key) {
            self.cache.insert(key.clone(), value);
            self.touch(&key);
            return;
        }
        if self.cache.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.cache.remove(&oldest);
            }
        }
        self.cache.insert(key.clone(), value);
        self.order.push_back(key);
    }

    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            let k = self.order.remove(pos).unwrap();
            self.order.push_back(k);
        }
    }
}
