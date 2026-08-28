# Data Structures Library

A comprehensive, zero-dependency collection of fundamental data structures for Python. The package is designed to be production-ready, strongly typed, and easy to use.

## Installation

```bash
pip install data-structures-lib
```

## Quick Start

```python
from data_structures_lib import DynamicArray, HashMap, MinHeap, Trie, LRUCache

arr = DynamicArray()
arr.append(10)
arr.append(20)
print(arr[0])  # 10

m = HashMap()
m['name'] = 'Alice'
print(m['name'])  # Alice

heap = MinHeap()
heap.push(5)
heap.push(1)
print(heap.pop())  # 1

t = Trie()
t.insert('cat')
t.insert('car')
print(t.starts_with('ca'))  # True

cache = LRUCache(capacity=2)
cache['a'] = 1
cache['b'] = 2
cache['c'] = 3  # evicts 'a'
print('a' in cache)  # False
```

## Features

- **Zero runtime dependencies**: No external packages required
- **Comprehensive coverage**: Linear, hashing, trees, heaps, tries, graphs, caches, and probabilistic structures
- **Type safe**: Includes `py.typed` marker for type checkers
- **Well tested**: Full test coverage for normal use, edge cases, and invalid operations
- **Clear error messages**: Explicit, helpful exceptions

## Supported Data Structures

### Linear
- `DynamicArray`: automatically resizing array
- `SinglyLinkedList`, `DoublyLinkedList`, `CircularLinkedList`: linked lists
- `Stack`: LIFO stack
- `Queue`: FIFO queue

### Hashing
- `HashMap`: separate-chaining hash map
- `HashSet`: separate-chaining hash set

### Trees
- `BinarySearchTree`: standard binary search tree
- `AVLTree`: self-balancing AVL tree

### Heaps
- `MinHeap`, `MaxHeap`: binary heaps

### Others
- `Trie`: prefix tree
- `AdjacencyListGraph`, `AdjacencyMatrixGraph`: graph representations
- `LRUCache`: least-recently-used cache
- `BloomFilter`: probabilistic membership filter

## Usage Examples

### Binary Search Tree

```python
from data_structures_lib import BinarySearchTree

tree = BinarySearchTree()
for value in [5, 3, 7, 1, 4]:
    tree.insert(value)
print(tree.search(4))  # True
```

### AVL Tree

```python
from data_structures_lib import AVLTree

tree = AVLTree()
for value in [10, 20, 30, 40, 50]:
    tree.insert(value)
print(tree.height())  # small balanced height
```

### Graph

```python
from data_structures_lib import AdjacencyListGraph

g = AdjacencyListGraph()
g.add_edge('a', 'b')
g.add_edge('a', 'c')
print(g.bfs('a'))  # ['a', 'b', 'c']
```

### Bloom Filter

```python
from data_structures_lib import BloomFilter

bf = BloomFilter(expected_items=1000, false_positive_rate=0.01)
bf.add('hello')
print(bf.has('hello'))  # True
print(bf.has('world'))  # probably False
```

## Development

```bash
pip install -e ".[dev]"
pytest test_data_structures_lib.py -v
```

## License

MIT License. See [LICENSE](./LICENSE) for details.
