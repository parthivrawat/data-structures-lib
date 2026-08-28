use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::Error;

/// A Bloom filter for membership queries.
#[derive(Debug)]
pub struct BloomFilter {
    size: usize,
    hash_count: usize,
    bits: Vec<u8>,
    items_added: usize,
}

impl BloomFilter {
    /// Creates a new BloomFilter.
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Result<Self, Error> {
        if expected_items == 0 {
            return Err(Error::InvalidArgument);
        }
        if false_positive_rate <= 0.0 || false_positive_rate >= 1.0 {
            return Err(Error::InvalidArgument);
        }
        let m = (-(expected_items as f64) * false_positive_rate.ln() / (2.0_f64.ln().powi(2))).ceil()
            as usize;
        let k = ((m as f64 / expected_items as f64) * 2.0_f64.ln()).round() as usize;
        let k = k.max(1);
        Ok(Self {
            size: m,
            hash_count: k,
            bits: vec![0; (m + 7) / 8],
            items_added: 0,
        })
    }

    /// Adds an item to the filter.
    pub fn add(&mut self, item: &str) {
        for i in 0..self.hash_count {
            let position = self.hash(item, i);
            self.bits[position / 8] |= 1 << (position % 8);
        }
        self.items_added += 1;
    }

    /// Returns true if the item might be in the filter.
    pub fn has(&self, item: &str) -> bool {
        for i in 0..self.hash_count {
            let position = self.hash(item, i);
            if self.bits[position / 8] & (1 << (position % 8)) == 0 {
                return false;
            }
        }
        true
    }

    /// Returns the number of items added.
    pub fn count(&self) -> usize {
        self.items_added
    }

    /// Returns the estimated false-positive probability.
    pub fn expected_fpp(&self) -> f64 {
        let exponent = -(self.hash_count as f64 * self.items_added as f64) / self.size as f64;
        (1.0 - exponent.exp()).powi(self.hash_count as i32)
    }

    fn hash(&self, item: &str, seed: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        (hasher.finish() as usize) % self.size
    }
}
