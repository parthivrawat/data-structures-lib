# Data Structures Library

A polyglot, production-ready collection of fundamental data structures implemented for Python, TypeScript, Go, and Rust.

## Packages

| Language | Package | Registry |
|---|---|---|
| Python | `data-structures-lib` | [PyPI](https://pypi.org/project/data-structures-lib) |
| TypeScript | `data-structures-lib` | [npm](https://www.npmjs.com/package/data-structures-lib) |
| Go | `github.com/parthivrawat/data-structures-lib/go` | [pkg.go.dev](https://pkg.go.dev/github.com/parthivrawat/data-structures-lib/go) |
| Rust | `data-structures-lib` | [crates.io](https://crates.io/crates/data-structures-lib) |

## Overview

This repository provides a consistent, minimal API for common data structures across four major languages. Each implementation is zero-dependency, fully tested, and packaged for its respective registry.

## Implemented Data Structures

- **Linear**: `DynamicArray`, `SinglyLinkedList`, `DoublyLinkedList`, `CircularLinkedList`, `Stack`, `Queue`
- **Hashing**: `HashMap`, `HashSet`
- **Trees**: `BinarySearchTree`, `AVLTree` (or `AvlTree` in some languages)
- **Heaps**: `MinHeap`, `MaxHeap`
- **Tries**: `Trie`
- **Graphs**: `AdjacencyListGraph`, `AdjacencyMatrixGraph`
- **Caching**: `LRUCache` (or `LruCache` in Rust)
- **Probabilistic**: `BloomFilter`

## Repository Layout

```
data-structures-lib/
├── python/     # PyPI package
├── typescript/ # npm package
├── go/         # Go module
├── rust/       # crates.io package
└── README.md   # This file
```

## Development

Each language directory contains its own build and test commands:

- **Python**: `pip install -e ".[dev]"` then `pytest`
- **TypeScript**: `npm install` then `npm test` and `npm run build`
- **Go**: `go test ./...` and `go build`
- **Rust**: `cargo test` and `cargo build`

## License

MIT License. See each language directory's `LICENSE` file for details.
