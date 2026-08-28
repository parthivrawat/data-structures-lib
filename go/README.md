# Data Structures Library for Go

A comprehensive, zero-dependency collection of fundamental data structures for Go. The package is designed for production use and is ready for indexing on [pkg.go.dev](https://pkg.go.dev).

## Installation

```bash
go get github.com/parthivrawat/data-structures-lib/go
```

## Quick Start

```go
package main

import (
    "fmt"
    "github.com/parthivrawat/data-structures-lib/go"
)

func main() {
    arr := datastructures.NewDynamicArray[int]()
    arr.Append(10)
    arr.Append(20)
    v, _ := arr.Get(0)
    fmt.Println(v) // 10

    h := datastructures.NewMinHeap[int]()
    h.Push(5)
    h.Push(1)
    min, _ := h.Pop()
    fmt.Println(min) // 1

    tree := datastructures.NewAVLTree[int]()
    tree.Insert(10)
    tree.Insert(20)
    tree.Insert(30)
    fmt.Println(tree.InOrder()) // [10 20 30]

    cache, _ := datastructures.NewLRUCache[string, int](2)
    cache.Set("a", 1)
    cache.Set("b", 2)
    cache.Set("c", 3) // evicts "a"
    fmt.Println(cache.Has("a")) // false
}
```

## Features

- **Zero runtime dependencies**: Uses only the Go standard library
- **Comprehensive coverage**: Linear, hashing, trees, heaps, tries, graphs, caches, and probabilistic structures
- **Generic APIs**: Type-safe with Go 1.18+ generics
- **Well tested**: Full `go test` coverage
- **Idiomatic errors**: Methods return `error` values where appropriate

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

```go
package main

import (
    "fmt"
    "github.com/parthivrawat/data-structures-lib/go"
)

func main() {
    tree := datastructures.NewBinarySearchTree[int]()
    tree.Insert(5)
    tree.Insert(3)
    tree.Insert(7)
    fmt.Println(tree.Search(4)) // false
    fmt.Println(tree.InOrder()) // [3 5 7]
}
```

### Graph

```go
package main

import (
    "fmt"
    "github.com/parthivrawat/data-structures-lib/go"
)

func main() {
    g := datastructures.NewAdjacencyListGraph[string](false)
    g.AddEdge("a", "b")
    g.AddEdge("a", "c")
    fmt.Println(g.BFS("a")) // [a b c]
}
```

### Bloom Filter

```go
package main

import (
    "fmt"
    "github.com/parthivrawat/data-structures-lib/go"
)

func main() {
    bf, _ := datastructures.NewBloomFilter(1000, 0.01)
    bf.Add("hello")
    fmt.Println(bf.Has("hello")) // true
    fmt.Println(bf.Has("world")) // probably false
}
```

## Development

```bash
go test ./...
go build
```

## License

MIT License. See [LICENSE](./LICENSE) for details.
