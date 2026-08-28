//! Data Structures Library for Rust
//!
//! A comprehensive, zero-dependency collection of fundamental data structures.

use std::fmt;

/// Errors that can be returned by data structure operations.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Empty,
    NotFound,
    OutOfBounds,
    InvalidArgument,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Empty => write!(f, "structure is empty"),
            Error::NotFound => write!(f, "not found"),
            Error::OutOfBounds => write!(f, "index out of range"),
            Error::InvalidArgument => write!(f, "invalid argument"),
        }
    }
}

impl std::error::Error for Error {}

pub mod caches;
pub mod graphs;
pub mod hashing;
pub mod heaps;
pub mod linear;
pub mod probabilistic;
pub mod trees;
pub mod tries;

pub use caches::LruCache;
pub use graphs::{AdjacencyListGraph, AdjacencyMatrixGraph};
pub use hashing::{HashMap, HashSet};
pub use heaps::{Heap, MaxHeap, MinHeap};
pub use linear::{
    CircularLinkedList, DoublyLinkedList, DynamicArray, Queue, SinglyLinkedList, Stack,
};
pub use probabilistic::BloomFilter;
pub use trees::{AvlTree, BinarySearchTree};
pub use tries::Trie;
