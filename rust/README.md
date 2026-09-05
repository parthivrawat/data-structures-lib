# Data Structures Library for Rust

A comprehensive, zero-dependency collection of fundamental data structures for Rust. The crate is published on [crates.io](https://crates.io) and documented on [docs.rs](https://docs.rs).

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
data-structures-lib = "1.1.0"
```

## Quick Start

```rust
use data_structures_lib::{DynamicArray, MinHeap, AvlTree, LruCache, Trie};

let mut arr = DynamicArray::new();
arr.append(10);
arr.append(20);
assert_eq!(arr.get(0), Ok(&10));

let mut heap = MinHeap::new();
heap.push(5);
heap.push(1);
assert_eq!(heap.pop(), Ok(1));

let mut tree = AvlTree::new();
tree.insert(10);
tree.insert(20);
tree.insert(30);
assert_eq!(tree.in_order(), vec![10, 20, 30]);

let mut cache = LruCache::new(2).unwrap();
cache.set("a", 1);
cache.set("b", 2);
cache.set("c", 3); // evicts "a"
assert!(!cache.has(&"a"));

let mut trie = Trie::new();
trie.insert("cat");
trie.insert("car");
assert!(trie.starts_with("ca"));
```

## Features

- **Zero runtime dependencies**: Uses only the Rust standard library
- **Comprehensive coverage**: Linear, hashing, trees, heaps, tries, graphs, caches, and probabilistic structures
- **Generic APIs**: Strongly-typed with trait bounds
- **Well tested**: Integration tests for common and edge cases
- **Clear errors**: Methods return `Result<T, Error>` where appropriate

## Supported Data Structures

### Linear
- `DynamicArray`: automatically resizing array
- `SinglyLinkedList`, `DoublyLinkedList`, `CircularLinkedList`: linked lists
- `Stack`: LIFO stack
- `Queue`: FIFO queue

### Hashing
- `HashMap`: key-value map
- `HashSet`: unique value set

### Trees
- `BinarySearchTree`: standard binary search tree
- `AvlTree`: self-balancing AVL tree

### Heaps
- `MinHeap`, `MaxHeap`: binary heaps

### Others
- `Trie`: prefix tree
- `AdjacencyListGraph`, `AdjacencyMatrixGraph`: graph representations
- `LruCache`: least-recently-used cache
- `BloomFilter`: probabilistic membership filter

## Usage Examples

### Binary Search Tree

```rust
use data_structures_lib::BinarySearchTree;

let mut tree = BinarySearchTree::new();
[5, 3, 7, 1, 4].iter().for_each(|&v| tree.insert(v));
assert!(tree.search(&4));
```

### Graph

```rust
use data_structures_lib::AdjacencyListGraph;

let mut g = AdjacencyListGraph::new(false);
g.add_edge("a", "b");
g.add_edge("a", "c");
assert_eq!(g.bfs(&"a"), vec!["a", "b", "c"]);
```

### Bloom Filter

```rust
use data_structures_lib::BloomFilter;

let mut bf = BloomFilter::new(1000, 0.01).unwrap();
bf.add("hello");
assert!(bf.has("hello"));
assert!(!bf.has("world"));
```

## Development

```bash
cargo test
cargo build
```

## License

MIT License. See [LICENSE](./LICENSE) for details.
