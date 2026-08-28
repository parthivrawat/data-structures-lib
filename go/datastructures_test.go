package datastructures

import (
	"reflect"
	"testing"
)

func TestDynamicArray(t *testing.T) {
	arr := NewDynamicArray[int]()
	arr.Append(1)
	arr.Append(2)
	if arr.Len() != 2 {
		t.Fatalf("expected length 2, got %d", arr.Len())
	}
	v, err := arr.Get(0)
	if err != nil || v != 1 {
		t.Fatalf("expected 1, got %v", v)
	}
	if err := arr.Insert(1, 2); err != nil {
		t.Fatalf("insert failed: %v", err)
	}
	if _, err := arr.Pop(); err != nil {
		t.Fatalf("pop failed: %v", err)
	}
	if !reflect.DeepEqual(arr.ToSlice(), []int{1, 2}) {
		t.Fatalf("expected [1 2], got %v", arr.ToSlice())
	}
}

func TestSinglyLinkedList(t *testing.T) {
	list := NewSinglyLinkedList[int]()
	list.Append(1)
	list.Append(2)
	if list.Len() != 2 {
		t.Fatalf("expected length 2, got %d", list.Len())
	}
	v, _ := list.Get(0)
	if v != 1 {
		t.Fatalf("expected 1, got %v", v)
	}
	if err := list.Remove(2); err != nil {
		t.Fatalf("remove failed: %v", err)
	}
	if list.Len() != 1 {
		t.Fatalf("expected length 1, got %d", list.Len())
	}
}

func TestDoublyLinkedList(t *testing.T) {
	list := NewDoublyLinkedList[int]()
	list.Append(1)
	list.Prepend(2)
	if !reflect.DeepEqual(list.ToSlice(), []int{2, 1}) {
		t.Fatalf("expected [2 1], got %v", list.ToSlice())
	}
	list.Remove(1)
	if list.Len() != 1 {
		t.Fatalf("expected length 1, got %d", list.Len())
	}
}

func TestCircularLinkedList(t *testing.T) {
	list := NewCircularLinkedList[int]()
	list.Append(1)
	list.Append(2)
	list.Append(3)
	if err := list.Remove(2); err != nil {
		t.Fatalf("remove failed: %v", err)
	}
	if !reflect.DeepEqual(list.ToSlice(), []int{1, 3}) {
		t.Fatalf("expected [1 3], got %v", list.ToSlice())
	}
}

func TestStack(t *testing.T) {
	s := NewStack[int]()
	s.Push(1)
	s.Push(2)
	v, _ := s.Pop()
	if v != 2 {
		t.Fatalf("expected 2, got %v", v)
	}
	if _, err := s.Peek(); err != nil {
		t.Fatalf("peek failed: %v", err)
	}
}

func TestQueue(t *testing.T) {
	q := NewQueue[int]()
	q.Enqueue(1)
	q.Enqueue(2)
	v, _ := q.Dequeue()
	if v != 1 {
		t.Fatalf("expected 1, got %v", v)
	}
	if _, err := q.Peek(); err != nil || q.Len() != 1 {
		t.Fatalf("queue state wrong")
	}
}

func TestHashMap(t *testing.T) {
	m := NewHashMap[string, int]()
	m.Set("a", 1)
	m.Set("b", 2)
	v, ok := m.Get("a")
	if !ok || v != 1 {
		t.Fatalf("expected 1, got %v", v)
	}
	if m.Len() != 2 {
		t.Fatalf("expected length 2, got %d", m.Len())
	}
}

func TestHashSet(t *testing.T) {
	s := NewHashSet[int]()
	s.Add(1)
	s.Add(2)
	if !s.Has(1) {
		t.Fatalf("expected 1 in set")
	}
	s.Remove(2)
	if s.Has(2) {
		t.Fatalf("expected 2 removed")
	}
}

func TestBinarySearchTree(t *testing.T) {
	tree := NewBinarySearchTree[int]()
	tree.Insert(5)
	tree.Insert(3)
	tree.Insert(7)
	if !tree.Search(3) {
		t.Fatalf("expected 3 in tree")
	}
	if !reflect.DeepEqual(tree.InOrder(), []int{3, 5, 7}) {
		t.Fatalf("expected in-order [3 5 7], got %v", tree.InOrder())
	}
	tree.Delete(3)
	if tree.Search(3) {
		t.Fatalf("expected 3 deleted")
	}
}

func TestAVLTree(t *testing.T) {
	tree := NewAVLTree[int]()
	for i := 0; i < 100; i++ {
		tree.Insert(i)
	}
	if tree.Height() > 20 {
		t.Fatalf("tree is not balanced: height %d", tree.Height())
	}
	tree.Delete(50)
	if tree.Search(50) {
		t.Fatalf("expected 50 deleted")
	}
}

func TestMinHeap(t *testing.T) {
	h := NewMinHeap[int]()
	h.Push(5)
	h.Push(1)
	h.Push(3)
	v, _ := h.Pop()
	if v != 1 {
		t.Fatalf("expected 1, got %v", v)
	}
}

func TestMaxHeap(t *testing.T) {
	h := NewMaxHeap[int]()
	h.Push(5)
	h.Push(1)
	h.Push(3)
	v, _ := h.Pop()
	if v != 5 {
		t.Fatalf("expected 5, got %v", v)
	}
}

func TestTrie(t *testing.T) {
	trie := NewTrie()
	trie.Insert("cat")
	trie.Insert("car")
	if !trie.Search("cat") {
		t.Fatalf("expected cat in trie")
	}
	if trie.Search("ca") {
		t.Fatalf("ca should not be a word")
	}
	if !trie.StartsWith("ca") {
		t.Fatalf("expected prefix ca")
	}
	words := trie.Words("ca")
	if len(words) != 2 {
		t.Fatalf("expected 2 words, got %d", len(words))
	}
}

func TestAdjacencyListGraph(t *testing.T) {
	g := NewAdjacencyListGraph[string](false)
	g.AddEdge("a", "b")
	g.AddEdge("a", "c")
	g.AddEdge("b", "d")
	bfs := g.BFS("a")
	if len(bfs) != 4 {
		t.Fatalf("expected 4 vertices, got %v", bfs)
	}
}

func TestAdjacencyMatrixGraph(t *testing.T) {
	g := NewAdjacencyMatrixGraph[int](false)
	g.AddEdge(1, 2, 1)
	g.AddEdge(1, 3, 1)
	n := g.Neighbors(1)
	if len(n) != 2 {
		t.Fatalf("expected 2 neighbors, got %v", n)
	}
}

func TestLRUCache(t *testing.T) {
	cache, err := NewLRUCache[string, int](2)
	if err != nil {
		t.Fatalf("create cache failed: %v", err)
	}
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)
	if cache.Has("a") {
		t.Fatalf("expected a evicted")
	}
	cache.Get("b")
	cache.Set("d", 4)
	if cache.Has("c") {
		t.Fatalf("expected c evicted")
	}
}

func TestBloomFilter(t *testing.T) {
	bf, err := NewBloomFilter(100, 0.05)
	if err != nil {
		t.Fatalf("create bloom filter failed: %v", err)
	}
	for i := 0; i < 50; i++ {
		bf.Add("item_" + string(rune('0'+i%10)))
	}
	if !bf.Has("item_0") && !bf.Has("item_1") {
		t.Fatalf("expected inserted item found")
	}
}
