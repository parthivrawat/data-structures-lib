# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-09-05

### Fixed

- Fixed Rust `unsafe` zeroing in linked lists.

### Changed

- Aligned package versions across Python, TypeScript, Go, and Rust.
- Bumped all language package versions to `1.1.0` and removed the top-level `VERSION` / `python/VERSION` files.
- Improved `SinglyLinkedList` append to O(1) in Python, TypeScript, and Go.
- Improved `CircularLinkedList.__contains__` in Python to avoid list allocation.
- Replaced Go `reflect.DeepEqual` with `==` in linked lists and `DynamicArray`.
- Cleaned up Python `DynamicArray` internal typing.
- Migrated Python packaging metadata to `pyproject.toml` and added CI.

### Added

- Added TypeScript `IndexOutOfRangeError`, `NotFoundError`, and `InvalidArgumentError`.
- Added cross-language API and error mapping tables to README.
- Added `SECURITY.md` documenting the library's security posture and reporting process.
- Expanded Rust test suite from 17 to 87 tests, including property/fuzz tests for unordered structures.

### Documentation

- Documented Go package-name / import-path mismatch.
- Added `CHANGELOG.md` and `CONTRIBUTING.md`.

## [1.0.0] - 2026-09-05

### Added

- Initial release: fundamental data structures implemented for Python, TypeScript, Go, and Rust.
  - Linear: `DynamicArray`, `SinglyLinkedList`, `DoublyLinkedList`, `CircularLinkedList`, `Stack`, `Queue`
  - Hashing: `HashMap`, `HashSet`
  - Trees: `BinarySearchTree`, `AVLTree`
  - Heaps: `MinHeap`, `MaxHeap`
  - Tries: `Trie`
  - Graphs: `AdjacencyListGraph`, `AdjacencyMatrixGraph`
  - Caching: `LRUCache`
  - Probabilistic: `BloomFilter`
