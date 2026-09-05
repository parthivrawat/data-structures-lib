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

## API Naming by Language

Each implementation follows its language's idioms, but the operations are equivalent. The table below maps common operations to the public method names used in each package.

| Operation | Python | TypeScript | Go | Rust |
|---|---|---|---|---|
| length / size | `len(x)` (`__len__`), `is_empty()` | `length` (getter), `isEmpty()` | `Len()`, `IsEmpty()` | `len()`, `is_empty()` |
| append | `append(value)` | `append(value)` | `Append(value)` | `append(value)` |
| prepend (linked lists) | `prepend(value)` | `prepend(value)` | `Prepend(value)` | `prepend(value)` |
| insert | `insert(index, value)` | `insert(index, value)` | `Insert(index, value)` | `insert(index, value)` |
| remove (by value) | `remove(value)` | `remove(value)` | `Remove(value)` | `remove(&value)` |
| pop (by index) | `pop(index=-1)` | `pop(index = -1)` | `Pop(index)` (arrays: variadic `Pop()`) | `pop(Option<index>)` |
| get / set | `x[i]` / `x[i] = v` (`__getitem__` / `__setitem__`) | `get(index)` / `set(index, value)` | `Get(index)` / `Set(index, value)` | `get(index)` / `set(index, value)` |
| stack push / pop / peek | `push` / `pop` / `peek` | `push` / `pop` / `peek` | `Push` / `Pop` / `Peek` | `push` / `pop` / `peek` |
| queue enqueue / dequeue / peek | `enqueue` / `dequeue` / `peek` | `enqueue` / `dequeue` / `peek` | `Enqueue` / `Dequeue` / `Peek` | `enqueue` / `dequeue` / `peek` |
| search / has / contains | `in` (`__contains__`), `find`, `search` (trees), `has` (maps/filters) | `find`, `has`, `search` (tries), `[Symbol.iterator]` | `Find`, `Has`, `Search`, `HasEdge` | `find`, `has`, `search`, `has_edge` |
| in-order traversal (trees) | `iter(tree)` (`__iter__` yields in-order) | `for...of` (`Symbol.iterator` yields in-order) | `InOrder()` | `in_order()` |

## Error Semantics by Language

Failure modes are surfaced differently per language: Python and TypeScript raise/throw exceptions, while Go and Rust return errors (`error` / `Result<T, Error>`).

| Failure mode | Python | TypeScript | Go | Rust |
|---|---|---|---|---|
| empty structure (pop/peek/dequeue) | `EmptyStructureError` | `EmptyStructureError` | `ErrEmptyStructure` | `Error::Empty` |
| index out of range | `IndexError` | `IndexOutOfRangeError` | `ErrIndexOutOfRange` | `Error::OutOfBounds` |
| value / key not found | `ValueError` (sequences), `KeyError` (maps/sets) | `NotFoundError` | `ErrNotFound`, `ErrKeyNotFound` | `Error::NotFound` |
| invalid argument | `ValueError` | `InvalidArgumentError` | `ErrInvalidCapacity` | `Error::InvalidArgument` |

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
