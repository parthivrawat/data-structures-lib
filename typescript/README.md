# Data Structures Library

A comprehensive, zero-dependency collection of fundamental data structures for TypeScript. The package is built for both CommonJS and ESM, with full declaration files, and is ready for publication on npm.

## Installation

```bash
npm install data-structures-lib
```

## Quick Start

```typescript
import { DynamicArray, HashMap, MinHeap, Trie, LRUCache } from 'data-structures-lib';

const arr = new DynamicArray<number>();
arr.append(10);
arr.append(20);
console.log(arr.get(0)); // 10

const map = new HashMap<string, string>();
map.set('name', 'Alice');
console.log(map.get('name')); // Alice

const heap = new MinHeap<number>();
heap.push(5);
heap.push(1);
console.log(heap.pop()); // 1

const trie = new Trie();
trie.insert('cat');
trie.insert('car');
console.log(trie.startsWith('ca')); // true

const cache = new LRUCache<string, number>(2);
cache.set('a', 1);
cache.set('b', 2);
cache.set('c', 3); // evicts 'a'
console.log(cache.has('a')); // false
```

## Features

- **Zero runtime dependencies**: No external packages required
- **Comprehensive coverage**: Linear, hashing, trees, heaps, tries, graphs, caches, and probabilistic structures
- **Strongly typed**: Generic APIs with full TypeScript declarations
- **Dual format**: CommonJS (`dist/index.js`) and ESM (`dist/index.mjs`) with `dist/index.d.ts`
- **Well tested**: Vitest test suite covering common and edge cases

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

```typescript
import { BinarySearchTree } from 'data-structures-lib';

const tree = new BinarySearchTree<number>();
[5, 3, 7, 1, 4].forEach(v => tree.insert(v));
console.log(tree.has(4)); // true
```

### AVL Tree

```typescript
import { AVLTree } from 'data-structures-lib';

const tree = new AVLTree<number>();
[10, 20, 30, 40, 50].forEach(v => tree.insert(v));
console.log(tree.height()); // small balanced height
```

### Graph

```typescript
import { AdjacencyListGraph } from 'data-structures-lib';

const g = new AdjacencyListGraph<string>();
g.addEdge('a', 'b');
g.addEdge('a', 'c');
console.log(g.bfs('a')); // ['a', 'b', 'c']
```

### Bloom Filter

```typescript
import { BloomFilter } from 'data-structures-lib';

const bf = new BloomFilter(1000, 0.01);
bf.add('hello');
console.log(bf.has('hello')); // true
console.log(bf.has('world')); // probably false
```

## Development

```bash
npm install
npm test
npm run build
```

## License

MIT License. See [LICENSE](./LICENSE) for details.
